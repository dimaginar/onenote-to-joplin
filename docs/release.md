# v0.2.0

Security hardening release. No functional changes — the tool works the same, but is now locked down properly.

## Changes

### Security
- **Content Security Policy enabled** — strict CSP (`default-src 'self'`, `script-src 'self'`, `style-src 'self' 'unsafe-inline'`, `img-src 'self' data:`) prevents potential XSS attacks.
- **URL opening restricted** — the opener capability now only allows `joplinapp.org` and the `onenote-md-exporter` GitHub page. Previously any URL could be opened.

### Housekeeping
- Suppressed dead-code compiler warnings for cross-platform stubs (all flagged functions are actively used on Windows).
- Version bumped across all config files and the in-app status bar.

## What It Checks

6 readiness checks for migrating OneNote to Joplin:

| Check | What it verifies |
|-------|-----------------|
| Joplin | Joplin is installed |
| Windows OS | Windows 10 (2004+) or 11 |
| OneNote (Desktop) | Desktop OneNote with COM automation |
| Word | Word with COM automation |
| OneNote Auto-Sync | Automatic sync is enabled |
| OneNote Full Download | Full file/image download is enabled |

## Security & Privacy

This application only reads Windows registry keys and tests COM object availability. It does not write to the registry, modify files, or collect any personal data. The full source code is available for review.

> **Note for Windows Users**: This app is not yet signed with a Code Signing Certificate. You may see a "Windows protected your PC" (SmartScreen) warning. Click "More info" then "Run anyway."
