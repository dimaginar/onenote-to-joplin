# OneNote to Joplin Migration Readiness Tool - Architecture

## 1. Overview

A Tauri v2 desktop application that validates a Windows environment before running `onenote-md-exporter`. The Rust backend performs system diagnostics (registry reads, COM initialization); the React frontend renders results and guides the user through remediation.

---

## 2. Rust Backend Architecture

### 2.1 Tauri IPC Commands

```rust
// src-tauri/src/commands/mod.rs
pub mod checks;
pub mod report;

// src-tauri/src/commands/checks.rs
#[tauri::command]
pub async fn run_readiness_scan() -> Result<ScanResult, ScanError> { ... }

// src-tauri/src/commands/report.rs
#[tauri::command]
pub async fn generate_report(results: ScanResult) -> Result<String, ScanError> { ... }

#[tauri::command]
pub async fn save_report(markdown: String, path: String) -> Result<(), ScanError> { ... }
```

### 2.2 Domain Types

```rust
// src-tauri/src/types.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Fail,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub id: String,           // "windows_os" | "onenote" | "word" | "com_bridge"
    pub label: String,        // Human-readable name
    pub status: CheckStatus,
    pub message: String,      // Detail: version found, error reason, etc.
    pub remediation: Option<String>, // Fix instructions if Fail/Warning
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub checks: Vec<CheckResult>,
    pub timestamp: String,    // ISO 8601
    pub os_info: String,      // e.g. "Windows 11 23H2"
    pub overall: CheckStatus, // Pass only if ALL checks pass
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanError {
    RegistryAccessDenied(String),
    ComInitFailed(String),
    Unexpected(String),
}
```

### 2.3 Check Modules

```
src-tauri/src/checks/
  mod.rs          -- re-exports, orchestrates all checks
  os_check.rs     -- Windows version via registry or RtlGetVersion
  onenote_check.rs -- OneNote registry detection + UWP discrimination
  word_check.rs   -- Word registry detection
  com_check.rs    -- COM CoCreateInstance for OneNote.Application & Word.Application
```

#### OS Check (`os_check.rs`)
- Read `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion`
  - Keys: `CurrentBuild`, `DisplayVersion`, `ProductName`
- Pass: build >= 19041 (Win10 2004+) or >= 22000 (Win11)
- Fail: anything else

#### OneNote Check (`onenote_check.rs`)
- Scan `HKLM\SOFTWARE\Microsoft\Office\{16.0,15.0}\OneNote\InstallRoot` for `Path` value
- If no desktop path found, check for UWP: `HKCU\Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages\*OneNote*`
- Pass: Desktop v15.0+ found
- Warning: Desktop found but version < 16.0 (2016)
- Fail: Only UWP found, or not installed

#### Word Check (`word_check.rs`)
- Scan `HKLM\SOFTWARE\Microsoft\Office\{16.0,15.0}\Word\InstallRoot` for `Path` value
- Pass: Desktop v15.0+ found
- Fail: Not found

#### COM Check (`com_check.rs`)
- `CoInitializeEx(COINIT_APARTMENTTHREADED)` (STA required for Office COM)
- `CoCreateInstance` for CLSID of `OneNote.Application`
- `CoCreateInstance` for CLSID of `Word.Application`
- Immediately release and `CoUninitialize`
- Pass: Both succeed
- Fail: Either fails, with specific error message

### 2.4 Rust Crate Dependencies

```toml
# src-tauri/Cargo.toml [dependencies]
tauri = { version = "2", features = ["devtools"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = [
    "Win32_System_Com",
    "Win32_System_Registry",
    "Win32_System_SystemInformation",
    "Win32_Foundation",
] }
```

### 2.5 Report Generation

`generate_report` converts `ScanResult` into a Markdown string:

```markdown
# OneNote Migration Readiness Report
Generated: 2026-02-02T10:30:00Z
System: Windows 11 23H2

## Results

| Check | Status | Detail |
|-------|--------|--------|
| Windows OS | PASS | Windows 11 Build 22631 |
| OneNote Desktop | PASS | Version 16.0, Path: C:\Program Files\... |
| Word Desktop | PASS | Version 16.0, Path: C:\Program Files\... |
| COM Bridge | FAIL | OneNote.Application: Class not registered |

## Remediation Steps
1. **COM Bridge**: Repair your Office installation...
```

---

## 3. Frontend Component Tree

```
App
 +-- StatusBar                        // Bottom bar: "Ready" / "Scanning..." / error
 +-- MainContent
      +-- EmptyState                   // Shown before first scan
      |    +-- ScanButton              // "Run Readiness Scan"
      +-- ScanningState                // Shown during scan
      |    +-- ProgressIndicator       // Animated spinner + current check name
      +-- ResultsView                  // Shown after scan completes
      |    +-- ResultsSummary          // Overall status badge + timestamp
      |    +-- CheckCard (x4)          // Individual check: icon, label, status, message
      |    +-- ActionBar               // "Re-Scan" + "Download Report" buttons
      |    +-- WizardPrompt            // "Issues found. View fix guide?" (if failures)
      +-- WizardView                   // Step-by-step fix guide (if user opts in)
           +-- WizardStepper           // Step indicator (1/N)
           +-- WizardStep              // Current step: instructions + external link
           +-- WizardNav               // Back / Next / Skip / "Re-Scan" buttons
```

### shadcn/ui Components Needed
- `Button`, `Card`, `Badge`, `Separator`
- `Progress` (for scanning animation)
- `Alert` (for warnings/errors in status bar)
- No `Menubar` needed -- app has very few actions

---

## 4. Zustand Store Shape

```typescript
// src/stores/types.ts

export type CheckStatus = "pass" | "fail" | "warning";

export interface CheckResult {
  id: string;
  label: string;
  status: CheckStatus;
  message: string;
  remediation: string | null;
}

export interface ScanResult {
  checks: CheckResult[];
  timestamp: string;
  osInfo: string;
  overall: CheckStatus;
}

export type AppView = "empty" | "scanning" | "results" | "wizard";

export interface AppState {
  // View state
  view: AppView;

  // Scan data
  scanResult: ScanResult | null;
  scanError: string | null;
  currentCheckIndex: number; // 0-3 during scanning, for progress display

  // Wizard state
  wizardStep: number;
  failedChecks: CheckResult[]; // Subset of checks with fail/warning

  // Status bar
  statusMessage: string;
  statusType: "info" | "error" | "success";

  // Actions
  startScan: () => Promise<void>;
  resetScan: () => void;
  enterWizard: () => void;
  exitWizard: () => void;
  nextWizardStep: () => void;
  prevWizardStep: () => void;
  generateReport: () => Promise<string>;
  saveReport: () => Promise<void>;
}
```

```typescript
// src/stores/appStore.ts

import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import type { AppState, ScanResult } from "./types";

export const useAppStore = create<AppState>((set, get) => ({
  view: "empty",
  scanResult: null,
  scanError: null,
  currentCheckIndex: 0,
  wizardStep: 0,
  failedChecks: [],
  statusMessage: "Ready",
  statusType: "info",

  startScan: async () => {
    set({ view: "scanning", scanError: null, currentCheckIndex: 0,
          statusMessage: "Scanning...", statusType: "info" });
    try {
      const result = await invoke<ScanResult>("run_readiness_scan");
      const failed = result.checks.filter(c => c.status !== "pass");
      set({
        view: "results",
        scanResult: result,
        failedChecks: failed,
        statusMessage: result.overall === "pass"
          ? "All checks passed" : `${failed.length} issue(s) found`,
        statusType: result.overall === "pass" ? "success" : "error",
      });
    } catch (err) {
      set({
        view: "empty",
        scanError: String(err),
        statusMessage: `Scan failed: ${err}`,
        statusType: "error",
      });
    }
  },

  resetScan: () => set({
    view: "empty", scanResult: null, scanError: null,
    statusMessage: "Ready", statusType: "info",
  }),

  enterWizard: () => set({ view: "wizard", wizardStep: 0 }),
  exitWizard: () => set({ view: "results" }),
  nextWizardStep: () => {
    const { wizardStep, failedChecks } = get();
    if (wizardStep < failedChecks.length - 1) set({ wizardStep: wizardStep + 1 });
  },
  prevWizardStep: () => {
    const { wizardStep } = get();
    if (wizardStep > 0) set({ wizardStep: wizardStep - 1 });
  },

  generateReport: async () => {
    const { scanResult } = get();
    if (!scanResult) throw new Error("No scan result");
    return invoke<string>("generate_report", { results: scanResult });
  },

  saveReport: async () => {
    const markdown = await get().generateReport();
    const path = await save({
      defaultPath: "readiness-report.md",
      filters: [{ name: "Markdown", extensions: ["md"] }],
    });
    if (path) {
      await invoke("save_report", { markdown, path });
      set({ statusMessage: "Report saved", statusType: "success" });
    }
  },
}));
```

---

## 5. Hook Architecture

```
src/hooks/
  useReadinessScan.ts   -- Wraps startScan, exposes scanning state slices
  useWizard.ts          -- Wraps wizard navigation, computes current step data
  useReport.ts          -- Wraps report generation + save dialog
  useKeyboard.ts        -- Global keyboard shortcuts (Ctrl+R = re-scan, Ctrl+S = save)
  useStatusBar.ts       -- Subscribes to statusMessage/statusType
```

Each hook is a thin selector/action wrapper around `useAppStore`:

```typescript
// src/hooks/useReadinessScan.ts
export function useReadinessScan() {
  const view = useAppStore(s => s.view);
  const scanResult = useAppStore(s => s.scanResult);
  const scanError = useAppStore(s => s.scanError);
  const startScan = useAppStore(s => s.startScan);
  const resetScan = useAppStore(s => s.resetScan);
  return { view, scanResult, scanError, startScan, resetScan };
}
```

---

## 6. IPC Contract

| Tauri Command | Direction | Input | Output |
|---|---|---|---|
| `run_readiness_scan` | FE -> BE | (none) | `Result<ScanResult, ScanError>` |
| `generate_report` | FE -> BE | `{ results: ScanResult }` | `Result<String, ScanError>` |
| `save_report` | FE -> BE | `{ markdown: String, path: String }` | `Result<(), ScanError>` |

Only 3 commands. The scan is atomic -- all 4 checks run sequentially in one invocation and return together.

---

## 7. State Machine

```
    [Empty] --(click "Run Scan")--> [Scanning]
       ^                                |
       |                           (success / error)
       |                                |
   (click "Reset")              [Results]
       |                           /        \
       +--------<--(back)---[Wizard]    (click "Download Report")
                              |   ^
                        (next/prev steps)
```

**States:**
- `empty` -- Initial. Shows `EmptyState` with scan button.
- `scanning` -- In progress. Shows `ProgressIndicator`. No user actions available.
- `results` -- Complete. Shows `ResultsView` with cards + action bar. Wizard prompt visible if failures exist.
- `wizard` -- Remediation. Shows `WizardView` stepping through failed checks. Can return to `results` or trigger re-scan (-> `scanning`).

---

## 8. File Structure

```
onenote-to-joplin/
  package.json
  pnpm-lock.yaml
  vite.config.ts
  tsconfig.json
  tailwind.config.ts
  postcss.config.js
  index.html
  components.json              # shadcn/ui config
  src/
    main.tsx
    App.tsx
    App.css                    # Tailwind @layer directives
    components/
      ui/                      # shadcn/ui: button, card, badge, progress, alert, separator
      scan/
        EmptyState.tsx
        ScanButton.tsx
        ScanningState.tsx
        ProgressIndicator.tsx
      results/
        ResultsView.tsx
        ResultsSummary.tsx
        CheckCard.tsx
        ActionBar.tsx
        WizardPrompt.tsx
      wizard/
        WizardView.tsx
        WizardStepper.tsx
        WizardStep.tsx
        WizardNav.tsx
      layout/
        StatusBar.tsx
    hooks/
      useReadinessScan.ts
      useWizard.ts
      useReport.ts
      useKeyboard.ts
      useStatusBar.ts
    stores/
      types.ts
      appStore.ts
    utils/
      tauri.ts                 # __TAURI_INTERNALS__ guard for dev fallback
  src-tauri/
    Cargo.toml
    tauri.conf.json
    build.rs
    capabilities/
      default.json             # dialog:allow-save, core:default
    icons/
    src/
      main.rs
      lib.rs                   # register commands + plugins
      types.rs
      commands/
        mod.rs
        checks.rs
        report.rs
      checks/
        mod.rs
        os_check.rs
        onenote_check.rs
        word_check.rs
        com_check.rs
  .claude-flow/                # claude-flow orchestration (existing)
```

---

## 9. Tauri Capabilities

```json
// src-tauri/capabilities/default.json
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

No filesystem, shell, or clipboard plugins needed. Registry and COM access happen natively in Rust without Tauri plugins.

---

## 10. Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Atomic scan (all checks in one call) | Yes | Simpler IPC, no streaming needed for 4 fast checks |
| `windows` crate over `winreg` | `windows` | Single crate for both registry AND COM; official Microsoft crate |
| STA threading for COM | Required | Office COM objects require single-threaded apartment |
| Report as Markdown string | Yes | Generate in Rust, save via dialog plugin; no temp files |
| No menubar | Correct | Only 2-3 actions; buttons in content area suffice |
| Dark mode only | Yes | Per spec; matches Joplin/dev tool aesthetic |
