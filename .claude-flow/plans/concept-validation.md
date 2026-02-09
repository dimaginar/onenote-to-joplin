# OneNote to Joplin Migration Readiness Tool - Concept Validation

## Feasibility Assessment Summary

| Area | Verdict | Notes |
|------|---------|-------|
| Windows Registry from Rust | **Go** | `windows` crate provides full registry API |
| COM Object Initialization | **Go** | `windows` crate supports `CoCreateInstance` natively |
| UWP vs Desktop Detection | **Go with caveats** | Registry heuristics work but have edge cases |
| WSL-to-Windows Cross-Compile | **No-Go** | Must build natively on Windows with MSVC |
| Overall | **Go** | Viable project, build constraint is the main issue |

---

## 1. Windows Registry Access from Rust

### Verdict: Go

### Recommended Approach

Use the `windows` crate (Microsoft's official Rust bindings) with the `Win32_System_Registry` feature.

```rust
use windows::Win32::System::Registry::*;
use windows::core::*;

fn read_registry_string(hkey: HKEY, subkey: &str, value: &str) -> Result<String> {
    unsafe {
        let subkey = HSTRING::from(subkey);
        let mut key = HKEY::default();
        RegOpenKeyExW(hkey, &subkey, 0, KEY_READ, &mut key)?;
        // ... RegQueryValueExW to read string value
    }
}
```

### Specific Registry Paths

**Windows OS Version:**
- `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion`
  - `CurrentBuild` (DWORD string) -- "22631" for Win11 23H2
  - `DisplayVersion` -- "23H2"
  - `ProductName` -- "Windows 11 Pro"

**Office Version Detection (Desktop):**
- `HKLM\SOFTWARE\Microsoft\Office\16.0\OneNote\InstallRoot` -> `Path` value
- `HKLM\SOFTWARE\Microsoft\Office\16.0\Word\InstallRoot` -> `Path` value
- Also check `15.0` (Office 2013) path variant
- On 64-bit Windows with 32-bit Office: `HKLM\SOFTWARE\WOW6432Node\Microsoft\Office\...`

**Click-to-Run Detection:**
- `HKLM\SOFTWARE\Microsoft\Office\ClickToRun\Configuration`
  - `VersionToReport` -- exact version string like "16.0.17830.20166"
  - `Platform` -- "x86" or "x64"

### Crate Comparison

| Crate | Recommendation |
|-------|---------------|
| `windows` (v0.58+) | **Use this.** Official Microsoft crate. Covers registry AND COM in one dependency. |
| `winreg` | Good for registry-only work, but we also need COM, so `windows` covers both. Skip. |
| `winapi` | Legacy. Unmaintained. Avoid. |

### Risk: None significant
Registry reads are non-destructive, require no elevated privileges for HKLM read access, and are fast (<10ms per key).

---

## 2. COM Object Initialization from Rust

### Verdict: Go

### Strategy

```rust
use windows::Win32::System::Com::*;
use windows::core::*;

// OneNote.Application CLSID
const CLSID_ONENOTE: GUID = GUID::from_u128(0x0BE35203_8F91_11CE_9DE3_00AA004BB851);
// Actual CLSID must be looked up from registry at runtime via CLSIDFromProgID

fn test_com_object(prog_id: &str) -> Result<bool> {
    unsafe {
        // Must use STA for Office COM objects
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let clsid = CLSIDFromProgID(&HSTRING::from(prog_id))?;
        let result: Result<IUnknown> = CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER);

        CoUninitialize();
        Ok(result.is_ok())
    }
}
```

### Critical Requirements

1. **STA Threading**: Office COM objects (OneNote, Word) require `COINIT_APARTMENTTHREADED`. Using MTA will cause `RPC_E_WRONG_THREAD` or silent failures.

2. **`CLSCTX_LOCAL_SERVER`**: Office apps are out-of-process COM servers. Use `CLSCTX_LOCAL_SERVER`, not `CLSCTX_INPROC_SERVER`.

3. **Immediate Release**: After confirming the object can be created, drop it immediately. Don't hold references to Office application objects.

4. **Tauri Thread Consideration**: Tauri commands run on a Tokio threadpool (MTA by default). The COM check must either:
   - Use `#[tauri::command]` without `async` (runs on main thread, but blocks) -- **not ideal**
   - Spawn a dedicated `std::thread` with `CoInitializeEx(STA)` inside -- **recommended**

### Recommended Pattern

```rust
#[tauri::command]
pub async fn run_readiness_scan() -> Result<ScanResult, ScanError> {
    // Spawn a dedicated thread for COM (STA requirement)
    tokio::task::spawn_blocking(|| {
        // This thread gets STA COM initialization
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED)? };

        let os = check_os()?;
        let onenote = check_onenote()?;
        let word = check_word()?;
        let com = check_com()?;

        unsafe { CoUninitialize() };

        Ok(ScanResult { checks: vec![os, onenote, word, com], ... })
    }).await.unwrap()
}
```

### Known Behavior

- **First COM init may launch the Office app briefly** (splash screen visible for ~1-2 seconds). This is normal for out-of-process COM. Document this in the UI ("Word may briefly appear during COM test").
- **If Office is already running**, COM creation is faster and reuses the running instance.
- **Antivirus**: COM access to Office is standard automation behavior. No false positives expected from Windows Defender.

---

## 3. UWP vs Desktop OneNote Detection

### Verdict: Go with Caveats

### Detection Strategy

**Desktop OneNote indicators:**
- Registry key exists: `HKLM\SOFTWARE\Microsoft\Office\{15.0,16.0}\OneNote\InstallRoot`
- Executable path points to `ONENOTE.EXE` under `Program Files`

**UWP OneNote indicators:**
- No desktop registry key found
- Package exists under: `HKCU\Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages\Microsoft.Office.OneNote_*`
- Alternatively, the executable is under `C:\Program Files\WindowsApps\...`

**Heuristic approach (recommended):**
1. Check for desktop registry key first (fast, definitive)
2. If not found, check for UWP package in registry
3. If neither found, report "Not installed"

### Caveats

1. **Both can be installed simultaneously.** A user may have UWP OneNote AND Desktop OneNote. The tool should detect the desktop version specifically and report it, even if UWP is also present.

2. **OneNote for Windows 10 (UWP) vs OneNote (Desktop).** Since Office 2019/M365, Microsoft has been shipping the "new" OneNote as a desktop app (not UWP). The registry check for `InstallRoot` covers this correctly.

3. **Office 365 Click-to-Run** installs desktop OneNote via virtual registry (VFS). The standard `HKLM` check still works because C2R projects these keys. No special handling needed.

### What `onenote-md-exporter` Checks

The exporter relies on COM `OneNote.Application` being available. Our COM Bridge check (Check #4) is the definitive test. The UWP detection is supplementary -- it tells the user *why* COM failed ("You have the Store version, you need Desktop").

---

## 4. WSL-to-Windows Cross-Compilation

### Verdict: No-Go for this project

### Why Cross-Compile Fails

The `windows` crate requires the Windows SDK headers and MSVC-compatible linking. The `x86_64-pc-windows-gnu` target (MinGW):
- **Cannot link against COM libraries** (`ole32.lib`, `oleaut32.lib`) without the Windows SDK
- **Missing `windows.h` and related headers** that the `windows` crate's build script expects
- **Tauri v2's WiX/NSIS bundlers** require Windows-native tools

### Recommended Build Strategy

**Option A: Native Windows Build (Recommended)**
```powershell
# On Windows with Visual Studio Build Tools installed
pnpm tauri build
```
- Install: Visual Studio Build Tools 2022 + "Desktop development with C++"
- Install: Rust via `rustup` (MSVC toolchain, default on Windows)
- Install: Node.js + pnpm

**Option B: GitHub Actions CI**
```yaml
# .github/workflows/build.yml
jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v2
      - uses: actions/setup-node@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: pnpm install
      - run: pnpm tauri build
```
This is the practical solution for developers working primarily in WSL.

**Option C: Dev in WSL, build from Windows**
- Keep source code in WSL filesystem
- Access via `\\wsl$\Ubuntu\home\barney\onenote-to-joplin` from Windows
- Run `pnpm tauri build` from Windows terminal pointing at that path

### Impact on Development

- `pnpm tauri dev` (hot-reload dev mode) must also run on Windows, not WSL
- Frontend-only development (Vite dev server) can run in WSL for rapid iteration
- Rust backend testing requires Windows

---

## 5. Known Limitations & Risks

### Risk Matrix

| Risk | Severity | Likelihood | Mitigation |
|------|----------|-----------|------------|
| Office app launches during COM test | Low | High | Document in UI; minimize+close immediately |
| 32-bit Office on 64-bit Windows | Medium | Medium | Check both `HKLM\SOFTWARE` and `WOW6432Node` registry paths |
| Office repair needed (COM registered but broken) | Medium | Low | COM check will catch this; remediation step = "Repair Office" |
| User runs as non-admin | Low | Medium | Registry reads work without admin; COM creation works without admin |
| Virtual Office (App-V/C2R) registry differences | Medium | Medium | Click-to-Run projects standard keys; test on C2R installs |
| Antivirus blocks COM creation | Low | Very Low | Unlikely for standard Office COM; document as edge case |

### Tauri v2 Specific Considerations

- **No Tauri plugin needed** for registry or COM. All access is through native Rust code in the Tauri command handlers.
- **Capability permissions**: Only `dialog:allow-save` needed (for report save). No filesystem, shell, or clipboard plugins required.
- **Window focus**: COM `CoCreateInstance` for Office may briefly steal window focus. Consider calling it on a background thread to avoid UI jank.

### COM Threading Model

- **Must use STA** (Single-Threaded Apartment). Office COM objects are apartment-threaded.
- Tauri async commands run on Tokio's thread pool (effectively MTA).
- **Solution**: Use `spawn_blocking` to run COM code on a dedicated thread with STA initialization.
- Always call `CoUninitialize` in the same thread that called `CoInitializeEx`.

---

## 6. Conclusion

The project is **technically feasible** with one significant constraint: **builds must happen on Windows** (not cross-compiled from WSL). The recommended workflow is:

1. **Develop frontend** in WSL with `pnpm dev` (Vite only, no Tauri)
2. **Build full app** on Windows natively, or via GitHub Actions `windows-latest`
3. **Test** the `.exe` on a real Windows machine with various Office configurations

All four diagnostic checks (OS, OneNote, Word, COM) are implementable using the `windows` crate alone. The UWP detection has minor edge cases but combined with the COM bridge test, provides a reliable assessment.
