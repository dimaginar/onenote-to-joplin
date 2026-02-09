# OneNote to Joplin Migration Readiness Tool - Implementation Plan

## Phase Overview

| Phase | Description | Dependencies |
|-------|-------------|-------------|
| 1 | Project Scaffolding | None |
| 2 | Rust Backend | Phase 1 |
| 3 | Frontend Foundation | Phase 1 |
| 4 | UI Components | Phase 3 |
| 5 | Integration & Polish | Phase 2 + Phase 4 |
| 6 | Testing & Release | Phase 5 |

---

## Phase 1: Project Scaffolding

### 1.1 Create Tauri v2 Project

```bash
# On Windows (required for this project)
pnpm create tauri-app onenote-to-joplin --template react-ts
cd onenote-to-joplin
```

Or manually initialize if already in the directory:
```bash
pnpm create vite . --template react-ts
pnpm add -D @tauri-apps/cli@latest
pnpm tauri init
```

### 1.2 Install Frontend Dependencies

```bash
# Core
pnpm add react@19 react-dom@19 zustand@5

# Dev / Build
pnpm add -D typescript@5.8 vite@7 @vitejs/plugin-react
pnpm add -D tailwindcss@4 @tailwindcss/vite

# UI
pnpm add lucide-react class-variance-authority clsx tailwind-merge

# Tauri plugins (frontend side)
pnpm add @tauri-apps/api@2 @tauri-apps/plugin-dialog@2
```

### 1.3 Configure shadcn/ui

```bash
pnpm dlx shadcn@latest init
# Select: New York style, Zinc color, CSS variables = yes
```

Add only needed components:
```bash
pnpm dlx shadcn@latest add button card badge progress alert separator
```

### 1.4 Configure Tailwind CSS 4

**`src/App.css`:**
```css
@import "tailwindcss";
```

**`vite.config.ts`:**
```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [react(), tailwindcss()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
});
```

### 1.5 Configure Rust Dependencies

**`src-tauri/Cargo.toml`:**
```toml
[package]
name = "onenote-to-joplin"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = { version = "2", features = ["devtools"] }
tauri-plugin-dialog = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
tokio = { version = "1", features = ["full"] }

[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = [
    "Win32_System_Com",
    "Win32_System_Registry",
    "Win32_System_SystemInformation",
    "Win32_Foundation",
] }
```

### 1.6 Tauri Configuration

**`src-tauri/tauri.conf.json`** (key fields):
```json
{
  "productName": "OneNote Migration Readiness",
  "identifier": "com.onenote-to-joplin.readiness",
  "build": {
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [{
      "title": "OneNote Migration Readiness",
      "width": 720,
      "height": 580,
      "resizable": true,
      "minWidth": 600,
      "minHeight": 480
    }]
  },
  "bundle": {
    "active": true,
    "targets": ["nsis"],
    "icon": ["icons/icon.ico"]
  }
}
```

**`src-tauri/capabilities/default.json`:**
```json
{
  "identifier": "default",
  "description": "Default capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "dialog:allow-save"
  ]
}
```

### 1.7 Dark Mode Setup

**`index.html`:**
```html
<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>OneNote Migration Readiness</title>
</head>
<body class="bg-background text-foreground">
  <div id="root"></div>
  <script type="module" src="/src/main.tsx"></script>
</body>
</html>
```

### Files Created/Modified
- `package.json` -- dependencies
- `vite.config.ts` -- Vite + Tailwind plugin
- `tsconfig.json` -- TypeScript config
- `components.json` -- shadcn config
- `index.html` -- dark class on root
- `src/App.css` -- Tailwind import
- `src-tauri/Cargo.toml` -- Rust deps
- `src-tauri/tauri.conf.json` -- window config
- `src-tauri/capabilities/default.json` -- permissions

### Definition of Done
- `pnpm dev` runs Vite dev server (frontend only)
- `pnpm tauri build` compiles on Windows (may show empty window)
- Tailwind dark mode renders correctly
- shadcn/ui Button renders in the app

---

## Phase 2: Rust Backend

### 2.1 Type Definitions

**Create `src-tauri/src/types.rs`:**
- `CheckStatus` enum (Pass/Fail/Warning)
- `CheckResult` struct (id, label, status, message, remediation)
- `ScanResult` struct (checks vec, timestamp, os_info, overall)
- `ScanError` enum (RegistryAccessDenied, ComInitFailed, Unexpected)
- All types derive `Serialize, Deserialize, Clone, Debug`

### 2.2 OS Check Module

**Create `src-tauri/src/checks/os_check.rs`:**
- Read `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion`
- Extract `CurrentBuild`, `DisplayVersion`, `ProductName`
- Evaluate: build >= 19041 = Pass, else Fail
- Return `CheckResult` with id="windows_os"

### 2.3 OneNote Check Module

**Create `src-tauri/src/checks/onenote_check.rs`:**
- Check `HKLM\SOFTWARE\Microsoft\Office\16.0\OneNote\InstallRoot` (and `15.0`)
- Check `WOW6432Node` variant for 32-bit Office
- If not found, check for UWP package in registry
- Return Pass (desktop found), Warning (old version), Fail (UWP only or missing)
- Include remediation text for Fail cases

### 2.4 Word Check Module

**Create `src-tauri/src/checks/word_check.rs`:**
- Same pattern as OneNote but for Word registry keys
- Check `HKLM\SOFTWARE\Microsoft\Office\{16.0,15.0}\Word\InstallRoot`
- Check `WOW6432Node` variant

### 2.5 COM Check Module

**Create `src-tauri/src/checks/com_check.rs`:**
- Initialize COM with `CoInitializeEx(COINIT_APARTMENTTHREADED)`
- Use `CLSIDFromProgID` for "OneNote.Application" and "Word.Application"
- Attempt `CoCreateInstance` with `CLSCTX_LOCAL_SERVER`
- Release immediately, `CoUninitialize`
- Return Pass if both succeed, Fail with specific error if either fails

### 2.6 Check Orchestrator

**Create `src-tauri/src/checks/mod.rs`:**
- `pub fn run_all_checks() -> Result<ScanResult, ScanError>`
- Runs os, onenote, word, com checks sequentially
- Computes `overall` status (Pass only if all pass)
- Stamps `timestamp` with `chrono::Utc::now()`

### 2.7 Tauri Commands

**Create `src-tauri/src/commands/checks.rs`:**
```rust
#[tauri::command]
pub async fn run_readiness_scan() -> Result<ScanResult, ScanError> {
    tokio::task::spawn_blocking(|| {
        crate::checks::run_all_checks()
    }).await.unwrap()
}
```

**Create `src-tauri/src/commands/report.rs`:**
- `generate_report(results: ScanResult) -> Result<String, ScanError>` -- builds Markdown string
- `save_report(markdown: String, path: String) -> Result<(), ScanError>` -- writes to file

### 2.8 Register Commands

**Update `src-tauri/src/lib.rs`:**
```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::checks::run_readiness_scan,
            commands::report::generate_report,
            commands::report::save_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Files Created/Modified
- `src-tauri/src/types.rs`
- `src-tauri/src/checks/mod.rs`
- `src-tauri/src/checks/os_check.rs`
- `src-tauri/src/checks/onenote_check.rs`
- `src-tauri/src/checks/word_check.rs`
- `src-tauri/src/checks/com_check.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/checks.rs`
- `src-tauri/src/commands/report.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/main.rs`

### Definition of Done
- `cargo build` succeeds on Windows
- `cargo test` passes for OS check (with mock registry if needed)
- `run_readiness_scan` Tauri command is callable from frontend

---

## Phase 3: Frontend Foundation

### 3.1 TypeScript Types

**Create `src/stores/types.ts`:**
- Mirror Rust types: `CheckStatus`, `CheckResult`, `ScanResult`
- Define `AppView = "empty" | "scanning" | "results" | "wizard"`
- Define `AppState` interface with all state fields and actions

### 3.2 Zustand Store

**Create `src/stores/appStore.ts`:**
- Implement `AppState` with `create<AppState>()`
- Actions: `startScan`, `resetScan`, `enterWizard`, `exitWizard`, `nextWizardStep`, `prevWizardStep`, `generateReport`, `saveReport`
- `startScan` calls `invoke("run_readiness_scan")`
- `saveReport` uses `@tauri-apps/plugin-dialog` for save path

### 3.3 Custom Hooks

**Create hooks:**
- `src/hooks/useReadinessScan.ts` -- scan state selectors + actions
- `src/hooks/useWizard.ts` -- wizard navigation + current step
- `src/hooks/useReport.ts` -- report generation + save
- `src/hooks/useKeyboard.ts` -- Ctrl+R (re-scan), Ctrl+S (save report)
- `src/hooks/useStatusBar.ts` -- status message + type

### 3.4 App Layout Shell

**Update `src/App.tsx`:**
```tsx
function App() {
  const { view } = useReadinessScan();
  return (
    <div className="h-screen flex flex-col bg-background text-foreground overflow-hidden">
      <main className="flex-1 flex items-center justify-center overflow-auto p-6">
        {view === "empty" && <EmptyState />}
        {view === "scanning" && <ScanningState />}
        {view === "results" && <ResultsView />}
        {view === "wizard" && <WizardView />}
      </main>
      <StatusBar />
    </div>
  );
}
```

### 3.5 Tauri Dev Guard

**Create `src/utils/tauri.ts`:**
```typescript
export const isTauri = () => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
```

Use in store to provide mock data when running `pnpm dev` outside Tauri (for frontend iteration in WSL).

### Files Created/Modified
- `src/stores/types.ts`
- `src/stores/appStore.ts`
- `src/hooks/useReadinessScan.ts`
- `src/hooks/useWizard.ts`
- `src/hooks/useReport.ts`
- `src/hooks/useKeyboard.ts`
- `src/hooks/useStatusBar.ts`
- `src/App.tsx`
- `src/utils/tauri.ts`

### Definition of Done
- App renders dark-mode full-viewport layout
- Store is wired up with mock data
- View switches between states correctly
- Keyboard shortcuts registered

---

## Phase 4: UI Components

### 4.1 Empty State

**Create `src/components/scan/EmptyState.tsx` + `ScanButton.tsx`:**
- Centered layout with app icon/title
- Brief description text
- Large "Run Readiness Scan" button (shadcn `Button` variant="default", size="lg")
- Lucide `ScanSearch` icon on button

### 4.2 Scanning State

**Create `src/components/scan/ScanningState.tsx` + `ProgressIndicator.tsx`:**
- Centered spinner/progress animation
- Text showing current check name ("Checking Windows version...")
- shadcn `Progress` bar (indeterminate or 4-step)

### 4.3 Results View

**Create `src/components/results/ResultsView.tsx`:**
- Container for all result components

**Create `src/components/results/ResultsSummary.tsx`:**
- Overall status badge (green "All Clear" / red "Issues Found")
- Scan timestamp

**Create `src/components/results/CheckCard.tsx`:**
- shadcn `Card` with:
  - Status icon (Lucide: `CircleCheck` green, `CircleX` red, `AlertTriangle` yellow)
  - Check label + detail message
  - Expandable remediation text if Fail/Warning

**Create `src/components/results/ActionBar.tsx`:**
- "Re-Scan" button (secondary)
- "Download Report" button (outline, `Download` icon)

**Create `src/components/results/WizardPrompt.tsx`:**
- Conditionally shown when failures exist
- "N issues need attention. View step-by-step guide?"
- "Start Guide" button

### 4.4 Wizard View

**Create `src/components/wizard/WizardView.tsx`:**
- Container for wizard components

**Create `src/components/wizard/WizardStepper.tsx`:**
- Horizontal step indicator (1 / N)
- Shows which check is being remediated
- Completed steps marked green

**Create `src/components/wizard/WizardStep.tsx`:**
- Current failed check details
- Step-by-step fix instructions (from `remediation` field)
- External links where relevant (e.g., "Download Office Deployment Tool")

**Create `src/components/wizard/WizardNav.tsx`:**
- "Back" / "Next" / "Skip" buttons
- "Re-Scan" button to verify fixes

### 4.5 Status Bar

**Create `src/components/layout/StatusBar.tsx`:**
- Fixed bottom bar, dark-muted background
- Left: status message text
- Right: version number
- Color-coded: default (muted), error (red), success (green)
- Default text: "Ready"

### Files Created
- `src/components/scan/EmptyState.tsx`
- `src/components/scan/ScanButton.tsx`
- `src/components/scan/ScanningState.tsx`
- `src/components/scan/ProgressIndicator.tsx`
- `src/components/results/ResultsView.tsx`
- `src/components/results/ResultsSummary.tsx`
- `src/components/results/CheckCard.tsx`
- `src/components/results/ActionBar.tsx`
- `src/components/results/WizardPrompt.tsx`
- `src/components/wizard/WizardView.tsx`
- `src/components/wizard/WizardStepper.tsx`
- `src/components/wizard/WizardStep.tsx`
- `src/components/wizard/WizardNav.tsx`
- `src/components/layout/StatusBar.tsx`

### Definition of Done
- All views render correctly with mock data
- State transitions work (empty -> scanning -> results -> wizard)
- Keyboard shortcuts work (Ctrl+R, Ctrl+S)
- No page-level scrollbar
- Responsive within min-width/min-height constraints

---

## Phase 5: Integration & Polish

### 5.1 Wire IPC

- Replace mock data in store with real `invoke()` calls
- Test with actual Tauri build on Windows
- Handle `invoke` errors gracefully (store sets `scanError`, status bar shows red)

### 5.2 Report Generation

- Implement Markdown template in Rust `generate_report` command
- Wire "Download Report" button -> `saveReport` store action -> native save dialog
- Test that `.md` file is well-formatted and contains all check results

### 5.3 Error Handling Edge Cases

- Registry access denied (non-standard permissions)
- COM timeout (Office takes long to initialize)
- No Office installed at all (all checks fail gracefully)
- Scan interrupted (user closes window during scan -- Tauri handles cleanup)

### 5.4 Status Bar Polish

- Show "Scanning... (1/4)" during each check if we add progress events
- Clear error messages with actionable text
- Auto-clear success messages after 5 seconds

### 5.5 Accessibility

- Focus management: focus scan button on load, focus first result card after scan
- Keyboard navigation through check cards and wizard steps
- ARIA labels on status icons

### Files Modified
- `src/stores/appStore.ts` (remove mocks, wire real IPC)
- `src/components/**` (minor focus/aria adjustments)

### Definition of Done
- Full scan works end-to-end on Windows with Office installed
- Full scan works end-to-end on Windows without Office (all fail gracefully)
- Report downloads correctly
- No console errors or unhandled promise rejections

---

## Phase 6: Testing & Release

### 6.1 Rust Unit Tests

**`src-tauri/src/checks/os_check.rs`:**
- Test version parsing logic with known build numbers
- Test edge cases (very old Windows, Windows Server)

**`src-tauri/src/checks/onenote_check.rs` + `word_check.rs`:**
- Test registry path resolution logic
- Note: actual registry reads require Windows; use `#[cfg(test)]` with mock values

**`src-tauri/src/commands/report.rs`:**
- Test Markdown generation with various result combinations
- Verify format is valid Markdown

### 6.2 Frontend Tests

**Component tests (Vitest + React Testing Library):**
- `EmptyState` renders scan button
- `CheckCard` renders correct icon/color for each status
- `ResultsView` shows wizard prompt when failures exist
- `WizardNav` disables "Back" on first step, "Next" on last step

**Store tests (Vitest):**
- `startScan` transitions view states correctly
- `enterWizard` / `exitWizard` toggle view
- `failedChecks` computed correctly from scan results

### 6.3 Integration Tests

- Mock Tauri `invoke` using `vi.mock("@tauri-apps/api/core")`
- Test full flow: click scan -> results appear -> enter wizard -> navigate steps -> download report

### 6.4 Manual Test Matrix

| Scenario | Expected |
|----------|----------|
| Windows 11 + Office 365 (C2R) + Desktop OneNote | All pass |
| Windows 11 + UWP OneNote only | OneNote=Fail, COM=Fail |
| Windows 11 + No Office | OneNote=Fail, Word=Fail, COM=Fail |
| Windows 10 + Office 2013 | All pass (v15.0) |
| Windows 10 + Office 2010 | OneNote=Fail, Word=Fail |
| Linux/macOS (dev fallback) | Mock data shown |

### 6.5 Build & Package

```bash
# On Windows
pnpm tauri build
# Output: src-tauri/target/release/bundle/nsis/OneNote-Migration-Readiness_0.1.0_x64-setup.exe
```

Or via GitHub Actions:
```yaml
name: Build
on: [push]
jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with: { version: 9 }
      - uses: actions/setup-node@v4
        with: { node-version: 20 }
      - uses: dtolnay/rust-toolchain@stable
      - run: pnpm install
      - run: pnpm tauri build
      - uses: actions/upload-artifact@v4
        with:
          name: windows-installer
          path: src-tauri/target/release/bundle/nsis/*.exe
```

### Definition of Done
- `cargo test` passes
- `pnpm test` passes (Vitest)
- Clean build produces `.exe` installer
- Installer runs on clean Windows 10/11 machine
- All manual test scenarios verified

---

## Key Decisions Log

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | Use `windows` crate (not `winreg`) | Single crate for both registry and COM access |
| 2 | Atomic scan (one IPC call, all checks) | 4 checks are fast; simpler than streaming |
| 3 | STA thread via `spawn_blocking` | Office COM requires apartment-threaded model |
| 4 | Native Windows build (no WSL cross-compile) | COM and registry linking requires MSVC/Windows SDK |
| 5 | Markdown report (not PDF/HTML) | Lightweight, fits Joplin/Obsidian ecosystem |
| 6 | No menubar | Too few actions; buttons in content area suffice |
| 7 | Mock Tauri in dev mode | Enables frontend development in WSL/browser |
| 8 | NSIS installer (not MSI) | Simpler, user-friendly, Tauri default |
