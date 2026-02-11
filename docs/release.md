We are excited to announce the first alpha release of OneNote to Joplin Readiness! This tool checks whether your Windows system is ready to migrate from Microsoft OneNote to Joplin, identifying and helping you fix issues before you start.

## Highlights

* **6 Readiness Checks**: Validates Joplin, Windows OS, OneNote Desktop (with COM automation), Word (with COM automation), Auto-Sync, and Full Download settings.
* **Smart Office Detection**: 3-layer detection strategy (traditional registry, Click-to-Run configuration, and live COM activation) supports traditional installs, Microsoft 365, and enterprise deployments. COM activation is the definitive test — if it works, the export will work.
* **Safety First**: Read-only tool — it checks registry keys and installed software but never modifies your system or data.
* **Guided Remediation**: Each failed check includes step-by-step fix instructions with clickable links. Non-standard Office installs (e.g. MSIX/Store) get a clear warning instead of a false failure.
* **Migration Guide**: Built-in 4-step guide walks you through downloading the exporter, preparing your notebooks, exporting, and importing into Joplin.
* **Pure Windows Focus**: Optimized for Windows 10 (2004+) and Windows 11.
* **Fast & Native**: Built with Rust (Tauri 2) for instant scan results — no Electron, no bloat.
* **Transparency**: Open-source code available for community audit.

## Features

* Master-detail results view with checks grouped by status (fail, warning, pass, skipped).
* Markdown report export with scan results and remediation steps.
* Multi-step guided fix wizard for each failed check.
* Smart detection: registry lookups, Click-to-Run configuration, filesystem probing, and live COM object testing.
* Windows 11 detection even when registry reports "Windows 10".
* Skips sync checks gracefully when OneNote Desktop is not installed.
* Keyboard shortcuts: `Ctrl+R` to scan, `Ctrl+S` to save report.
* Dark theme with custom scrollbars.

## Security & Privacy

This application only reads Windows registry keys and tests COM object availability. It does not write to the registry, modify files, or collect any personal data. The full source code is available for review.

> **Note for Windows Users**: This app is currently not signed with a Code Signing Certificate. You may see a "Windows protected your PC" (SmartScreen) warning. You can safely bypass this by clicking "More info" > "Run anyway."
