# Build Guide (WSL2)

How to build OneNote to Joplin Readiness from source using WSL2 on Windows.

## Prerequisites

Install in WSL2 (Ubuntu/Debian):

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add x86_64-pc-windows-gnu x86_64-pc-windows-msvc

# Node.js 20+ and pnpm
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo bash -
sudo apt install -y nodejs
corepack enable

# Cross-compilation tools (for NSIS installer)
sudo apt install -y nsis lld llvm clang
cargo install cargo-xwin

# MinGW (for exe-only builds)
sudo apt install -y gcc-mingw-w64-x86-64
```

## Install Dependencies

```bash
cd onenote-to-joplin
pnpm install
```

## Build Options

### Standalone exe only (fast, no installer)

```bash
pnpm tauri build --target x86_64-pc-windows-gnu --no-bundle
```

Output: `src-tauri/target/x86_64-pc-windows-gnu/release/onenote-to-joplin.exe`

### Exe + NSIS installer

```bash
pnpm tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc
```

Output:
- `src-tauri/target/x86_64-pc-windows-msvc/release/onenote-to-joplin.exe`
- `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/OneNote to Joplin Readiness_0.1.0_x64-setup.exe`

The installer places the app in `%LOCALAPPDATA%` (no admin rights needed).

### Development mode

```bash
pnpm tauri dev
```

Hot-reloads the frontend. Requires a display server (e.g. VcXsrv) or run from Windows terminal.

## Troubleshooting

| Error | Fix |
|-------|-----|
| `linker 'x86_64-w64-mingw32-gcc' not found` | `sudo apt install gcc-mingw-w64-x86-64` |
| `error running makensis` | `sudo apt install nsis` |
| `cargo-xwin not found` | `cargo install cargo-xwin` |
| `pnpm not found` | `corepack enable` or `npm i -g pnpm` |
