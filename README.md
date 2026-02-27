# OneNote to Joplin Readiness

A Windows desktop tool that checks if your system is ready to migrate from Microsoft OneNote to [Joplin](https://joplinapp.org/). It verifies installed software, COM automation, and OneNote sync settings — so you can identify and fix issues before starting a migration.

## ✅ What It Checks

| Check                     | What it verifies                                                                         |
| ------------------------- | ---------------------------------------------------------------------------------------- |
| **Joplin**                | Joplin desktop app is installed                                                          |
| **Windows OS**            | Windows 10 or 11, build version                                                          |
| **OneNote (Desktop)**     | OneNote desktop app is installed with working COM automation (not the Store/UWP version) |
| **Word**                  | Microsoft Word is installed with working COM automation (needed for export rendering)    |
| **OneNote Auto-Sync**     | Sync-on-close is enabled in OneNote settings                                             |
| **OneNote Full Download** | Notebooks are fully downloaded locally (not cloud-only stubs)                            |

Each check returns a clear status — pass, fail, warning, or skipped — with guided remediation steps for any issues found.

## 🔍 Transparency & Safety

This project was developed with the assistance of AI coding tools. To build trust, the entire source code is public for community audit.

**Why the "Unknown Publisher" warning?** To remove the Windows SmartScreen warning, an app must be signed with a Code Signing Certificate. These certificates are expensive annual subscriptions. As an independent developer, I currently do not have one — your [donations](#-support-development) help make this possible.

- **Read-only**: The tool only reads registry keys and checks for installed software. It does not modify your system, OneNote data, or any files.
- **Verified code**: If you are tech-savvy, feel free to audit the Rust backend (`src-tauri/src/`) and the React frontend (`src/`).

## 🚀 Quick Start

1. Download `onenote-to-joplin.exe` from the [Releases](https://github.com/dimaginar/onenote-to-joplin/releases) section.
   - **Recommended browsers**: Firefox or Vivaldi typically allow the download without additional warnings.
   - **Microsoft Edge**: You may see a warning that the file is "not commonly downloaded." Click the three dots (...) next to the download, select **Keep**, then **Show more** > **Keep anyway**.
   - **Work laptops**: On some managed environments, downloading .exe files from GitHub may be blocked by system policy.
2. Run the tool. If Windows SmartScreen appears, click **More info** > **Run anyway**.
3. Click **Scan** and review the results. Follow the guided remediation for any failed checks.

## 📄 License & Distribution

This project is licensed under the [MIT License](LICENSE).

A ready-to-use compiled version will soon be available at [dimaginar.com](https://dimaginar.com). If you find this tool useful, making a donation or purchasing the compiled version is the best way to support the project.

## ☕ Support Development
If this tool helped you prepare for a smooth OneNote-to-Joplin migration, consider supporting its development. You can grab the packaged installer version on Payhip.

[Get it on Payhip](https://payhip.com/b/LOgwY)

## 🔧 Building from Source

See the [Build Guide](docs/human_guide.md) for instructions on building from WSL2.

## 🛠️ Tech Stack

- [Tauri 2](https://v2.tauri.app/) + Rust — native Windows registry, COM automation, and system detection
- React 19 + TypeScript — Frontend
- Zustand — State management
- Tailwind CSS 4 — Styling
