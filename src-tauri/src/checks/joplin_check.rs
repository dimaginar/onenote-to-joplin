use crate::types::{CheckResult, CheckStatus};

#[cfg(windows)]
pub fn check() -> CheckResult {
    // Strategy 1: Registry Uninstall keys (most reliable for installed copies)
    if let Some((version, location)) = find_joplin_in_registry() {
        return CheckResult {
            id: "joplin".into(),
            label: "Joplin".into(),
            status: CheckStatus::Pass,
            message: format!("{} found at {}", version, location),
            remediation: None,
        };
    }

    // Strategy 2: Common installation paths on disk
    if let Some(path) = find_joplin_on_disk() {
        return CheckResult {
            id: "joplin".into(),
            label: "Joplin".into(),
            status: CheckStatus::Pass,
            message: format!("Found at {}", path),
            remediation: None,
        };
    }

    // Strategy 3: Profile directory exists (Joplin was used before but exe not found)
    if joplin_profile_exists() {
        return CheckResult {
            id: "joplin".into(),
            label: "Joplin".into(),
            status: CheckStatus::Warning,
            message: "Joplin profile directory found, but application not detected.".into(),
            remediation: Some(
                "A Joplin configuration folder exists but the application was not found. \
                 Joplin may have been uninstalled or is a portable installation in a \
                 non-standard location. Reinstall from https://joplinapp.org/download/ \
                 or verify it is accessible."
                    .into(),
            ),
        };
    }

    // Nothing found
    CheckResult {
        id: "joplin".into(),
        label: "Joplin Desktop".into(),
        status: CheckStatus::Fail,
        message: "Joplin desktop application not found.".into(),
        remediation: Some(
            "Install Joplin from https://joplinapp.org/download/. The desktop application \
             is required as the migration target. Both the per-user install and the \
             system-wide install are supported."
                .into(),
        ),
    }
}

// ---------------------------------------------------------------------------
// Registry detection (User + System Uninstall keys)
// ---------------------------------------------------------------------------

#[cfg(windows)]
fn find_joplin_in_registry() -> Option<(String, String)> {
    use windows::Win32::System::Registry::*;

    // User install writes to HKCU, system install writes to HKLM
    let roots = [HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE];

    for root in &roots {
        if let Some(result) = scan_uninstall_keys(*root) {
            return Some(result);
        }
    }
    None
}

#[cfg(windows)]
fn scan_uninstall_keys(
    root: windows::Win32::System::Registry::HKEY,
) -> Option<(String, String)> {
    use windows::Win32::Foundation::*;
    use windows::Win32::System::Registry::*;
    use windows::core::*;

    unsafe {
        let subkey = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall");
        let mut key = HKEY::default();
        let status = RegOpenKeyExW(root, subkey, 0, KEY_READ, &mut key);
        if status != ERROR_SUCCESS {
            return None;
        }

        let mut index = 0u32;
        let mut name_buf = vec![0u16; 512];

        loop {
            let mut name_len = name_buf.len() as u32;
            let result = RegEnumKeyExW(
                key,
                index,
                PWSTR::from_raw(name_buf.as_mut_ptr()),
                &mut name_len,
                None,
                PWSTR::null(),
                None,
                None,
            );

            if result != ERROR_SUCCESS {
                break;
            }

            let subkey_name =
                String::from_utf16_lossy(&name_buf[..name_len as usize]);

            let full_path = format!(
                "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}",
                subkey_name
            );
            let full_path_wide: Vec<u16> =
                full_path.encode_utf16().chain(std::iter::once(0)).collect();

            let mut sub_key = HKEY::default();
            let open = RegOpenKeyExW(
                root,
                PCWSTR::from_raw(full_path_wide.as_ptr()),
                0,
                KEY_READ,
                &mut sub_key,
            );

            if open == ERROR_SUCCESS {
                let display_name = read_reg_sz(sub_key, w!("DisplayName"));
                let install_location =
                    read_reg_sz(sub_key, w!("InstallLocation"));
                let display_version =
                    read_reg_sz(sub_key, w!("DisplayVersion"));
                let _ = RegCloseKey(sub_key);

                if let Some(ref name) = display_name {
                    if name.to_lowercase().contains("joplin") {
                        let version_str = display_version
                            .map(|v| format!("Joplin {}", v))
                            .unwrap_or_else(|| "Joplin".into());
                        let location_str = install_location
                            .unwrap_or_else(|| "(registry entry found)".into());
                        let _ = RegCloseKey(key);
                        return Some((version_str, location_str));
                    }
                }
            }

            index += 1;
        }

        let _ = RegCloseKey(key);
        None
    }
}

#[cfg(windows)]
unsafe fn read_reg_sz(
    key: windows::Win32::System::Registry::HKEY,
    value_name: windows::core::PCWSTR,
) -> Option<String> {
    use windows::Win32::Foundation::*;
    use windows::Win32::System::Registry::*;

    let mut buf_size: u32 = 0;
    let mut value_type = REG_VALUE_TYPE::default();

    let _ = RegQueryValueExW(
        key,
        value_name,
        None,
        Some(&mut value_type),
        None,
        Some(&mut buf_size),
    );

    if buf_size == 0 {
        return None;
    }

    let mut buffer = vec![0u8; buf_size as usize];
    let status = RegQueryValueExW(
        key,
        value_name,
        None,
        Some(&mut value_type),
        Some(buffer.as_mut_ptr()),
        Some(&mut buf_size),
    );

    if status != ERROR_SUCCESS {
        return None;
    }

    let wide: Vec<u16> = buffer
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    let s = String::from_utf16_lossy(&wide)
        .trim_end_matches('\0')
        .to_string();

    if s.is_empty() { None } else { Some(s) }
}

// ---------------------------------------------------------------------------
// Filesystem detection (common install paths)
// ---------------------------------------------------------------------------

#[cfg(windows)]
fn find_joplin_on_disk() -> Option<String> {
    use std::path::PathBuf;

    let mut candidates: Vec<PathBuf> = Vec::new();

    // User install (standard Joplin installer, "Install for me only")
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        candidates.push(
            PathBuf::from(&local)
                .join("Programs")
                .join("joplin")
                .join("Joplin.exe"),
        );
    }

    // System-wide install ("Install for all users")
    if let Ok(pf) = std::env::var("PROGRAMFILES") {
        candidates.push(
            PathBuf::from(&pf).join("Joplin").join("Joplin.exe"),
        );
    }

    // 32-bit program files on 64-bit Windows
    if let Ok(pf86) = std::env::var("PROGRAMFILES(X86)") {
        candidates.push(
            PathBuf::from(&pf86).join("Joplin").join("Joplin.exe"),
        );
    }

    for path in candidates {
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Profile directory fallback
// ---------------------------------------------------------------------------

#[cfg(windows)]
fn joplin_profile_exists() -> bool {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let profile = std::path::PathBuf::from(appdata).join("joplin-desktop");
        return profile.is_dir();
    }
    false
}

// ---------------------------------------------------------------------------
// Non-Windows stub
// ---------------------------------------------------------------------------

#[cfg(not(windows))]
pub fn check() -> CheckResult {
    CheckResult {
        id: "joplin".into(),
        label: "Joplin Desktop".into(),
        status: CheckStatus::Fail,
        message: "Not running on Windows - cannot check Joplin.".into(),
        remediation: Some("This tool must be run on Windows.".into()),
    }
}
