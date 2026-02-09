# OneNote to Joplin Readiness

A Windows desktop tool that checks if your system is ready to migrate from OneNote to Joplin. It verifies installed software, COM automation support, and OneNote sync settings so you can troubleshoot before starting a migration.

## What It Checks

| Check | What it verifies |
|-------|-----------------|
| **Joplin** | Joplin desktop app is installed |
| **Windows OS** | Windows 10/11, build version |
| **OneNote (Desktop)** | OneNote desktop app is installed (not the Store version) |
| **Word** | Microsoft Word is installed (used by some migration tools) |
| **COM Automation** | COM bridge is functional (required for programmatic access) |
| **OneNote Auto-Sync** | OneNote sync-on-close is enabled |
| **OneNote Full Download** | Notebooks are fully downloaded (not cloud-only stubs) |

## Building from Source

This project is distributed as source code only. You build it yourself on Windows.

### Prerequisites

Install these on **Windows** (not WSL):

1. **Git** -- [git-scm.com](https://git-scm.com/downloads)

2. **Visual Studio Build Tools 2022** -- [Download](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - During install, select **"Desktop development with C++"**
   - This provides the MSVC compiler and Windows SDK (~2-6 GB)

3. **Rust** -- [rustup.rs](https://rustup.rs/)
   - Run the installer and accept defaults (ensures the `x86_64-pc-windows-msvc` target)

4. **Node.js 20+** -- [nodejs.org](https://nodejs.org/) (LTS recommended)

5. **pnpm** -- Open a terminal after installing Node.js and run:
   ```powershell
   corepack enable
   ```

### Build

```powershell
git clone https://github.com/<your-username>/onenote-to-joplin.git
cd onenote-to-joplin
pnpm install
pnpm tauri build
```

The build produces:
- **Installer**: `src-tauri\target\release\bundle\nsis\OneNote to Joplin Readiness_0.1.0_x64-setup.exe`
- **Standalone exe**: `src-tauri\target\release\onenote-to-joplin.exe`

The installer places the app in your user profile (no admin rights needed). The standalone exe runs directly without installation.

### Troubleshooting

| Error | Fix |
|-------|-----|
| `MSVC not found` | Install Visual Studio Build Tools with "Desktop development with C++" |
| `rustup not found` | Install Rust from rustup.rs and restart your terminal |
| `pnpm not found` | Run `corepack enable` or install with `npm install -g pnpm` |
| Linker errors | Make sure you're building on native Windows, not WSL |

## Tech Stack

- [Tauri 2](https://v2.tauri.app/) (Rust backend)
- React 19 + TypeScript
- Zustand (state management)
- Tailwind CSS 4

## License

MIT
