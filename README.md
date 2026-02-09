# OneNote to Joplin Readiness

A Windows desktop tool that checks if your system is ready to migrate from Microsoft OneNote to [Joplin](https://joplinapp.org/). It verifies installed software, COM automation support, and OneNote sync settings — so you can identify and fix issues before starting a migration.

## What It Checks

| Check | What it verifies |
|-------|-----------------|
| **Joplin** | Joplin desktop app is installed |
| **Windows OS** | Windows 10 or 11, build version |
| **OneNote (Desktop)** | OneNote desktop app is installed (not the Store version) |
| **Word** | Microsoft Word is installed (used by some migration tools) |
| **COM Automation** | COM bridge is functional (required for programmatic access to OneNote) |
| **OneNote Auto-Sync** | Sync-on-close is enabled in OneNote settings |
| **OneNote Full Download** | Notebooks are fully downloaded locally (not cloud-only stubs) |

Each check returns a clear status — pass, fail, warning, or skipped — with guided remediation steps for any issues found.

## Transparency & Safety

This project was developed with the assistance of AI coding tools. The entire source code is public so you can review exactly what the application does.

- **Read-only**: The tool only reads registry keys and checks for installed software. It does not modify your system, OneNote data, or any files.
- **Verified code**: If you are tech-savvy, feel free to audit the Rust backend (`src-tauri/src/`) and the React frontend (`src/`).
- **Open Source**: Full transparency — the code speaks for itself.

## License & Distribution

This source code is licensed under the [MIT License](LICENSE) — feel free to inspect, fork, and contribute.

The **Dimaginar Readiness Tool** (the compiled `.exe` available at [dimaginar.com](https://dimaginar.com)) is the official, supported distribution. It includes a user-friendly installer and direct support.

If you appreciate the work that went into this tool, purchasing the compiled version is the best way to support the project.

## Building from Source

The source code is provided for transparency and community audit. If you prefer to build it yourself, see the [Build Guide](docs/BUILD.md) for full instructions.

You will need Windows (not WSL), Visual Studio Build Tools, Rust, Node.js, and pnpm. The build produces both a standalone `.exe` and an NSIS installer.

## Tech Stack

- [Tauri 2](https://v2.tauri.app/) — Rust backend with native Windows APIs
- React 19 + TypeScript — Frontend
- Zustand — State management
- Tailwind CSS 4 — Styling
