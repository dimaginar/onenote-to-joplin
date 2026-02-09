# Build Guide

This document explains how to build the OneNote to Joplin Readiness tool from source on Windows.

> **Important**: You must build on native Windows with the MSVC toolchain. Building from WSL2 can produce a standalone `.exe` but cannot create the NSIS installer because `makensis.exe` is a Windows-native binary that WSL2 cannot execute.

## Prerequisites

Install these on **Windows** (PowerShell or Command Prompt, not WSL):

### 1. Git

Download and install from [git-scm.com](https://git-scm.com/downloads).

### 2. Visual Studio Build Tools 2022

Download from [Visual Studio Downloads](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

During installation, select **"Desktop development with C++"**. This provides:
- MSVC compiler and linker
- Windows SDK
- CMake (used by some Rust crates)

This is the largest dependency (~2-6 GB). You do **not** need the full Visual Studio IDE — only the Build Tools.

### 3. Rust

Install via [rustup.rs](https://rustup.rs/). Run the installer and accept defaults.

Verify after install:
```powershell
rustc --version
rustup show
```

The default host triple should be `x86_64-pc-windows-msvc`. If it shows `gnu` instead, switch with:
```powershell
rustup default stable-x86_64-pc-windows-msvc
```

### 4. Node.js

Install version 20 LTS or later from [nodejs.org](https://nodejs.org/).

Verify:
```powershell
node --version
```

### 5. pnpm

After installing Node.js, enable pnpm:
```powershell
corepack enable
```

Verify:
```powershell
pnpm --version
```

If `corepack enable` fails (some Node installs don't include it), use:
```powershell
npm install -g pnpm
```

## Build Steps

### Clone the repository
```powershell
git clone https://github.com/<your-username>/onenote-to-joplin.git
cd onenote-to-joplin
```

### Install JavaScript dependencies
```powershell
pnpm install
```

### Build the application
```powershell
pnpm tauri build
```

This compiles the Rust backend and React frontend, then bundles everything into:

| Output | Location |
|--------|----------|
| NSIS Installer | `src-tauri\target\release\bundle\nsis\OneNote to Joplin Readiness_0.1.0_x64-setup.exe` |
| Standalone exe | `src-tauri\target\release\onenote-to-joplin.exe` |

The **NSIS installer** places the app in your user profile (`%LOCALAPPDATA%`). No admin rights are needed.

The **standalone exe** runs directly without installation.

### Development mode (optional)

For development with hot-reload:
```powershell
pnpm tauri dev
```

## Troubleshooting

| Error | Solution |
|-------|----------|
| `MSVC not found` or linker errors | Install Visual Studio Build Tools 2022 with "Desktop development with C++" workload |
| `rustc` or `cargo` not found | Install Rust from rustup.rs and restart your terminal |
| `pnpm` not found | Run `corepack enable` or `npm install -g pnpm` |
| `error running makensis.exe` | You are building from WSL — switch to native Windows PowerShell |
| Rust target shows `gnu` | Run `rustup default stable-x86_64-pc-windows-msvc` |
| `node` not found | Install Node.js 20+ from nodejs.org and restart your terminal |
| Build succeeds but no installer | Check that `tauri.conf.json` has `"targets": ["nsis"]` in the bundle section |

## What You Do NOT Need

- Full Visual Studio IDE (Build Tools alone are sufficient)
- NSIS separately (Tauri downloads it automatically during the build)
- MinGW or MSYS2 (those are for the GNU target — we use MSVC)
- WebView2 SDK (included on Windows 10 1803+ and Windows 11)
