use crate::types::{CheckResult, CheckStatus};

#[cfg(windows)]
pub fn check() -> CheckResult {
    for version in &["16.0", "15.0"] {
        if let Some(path) = super::registry::find_office_install_root("OneNote", version) {
            return CheckResult {
                id: "onenote".into(),
                label: "OneNote (Desktop)".into(),
                status: CheckStatus::Pass,
                message: format!("Version {} found at {}", version, path),
                remediation: None,
            };
        }
    }

    if is_uwp_onenote_installed() {
        return CheckResult {
            id: "onenote".into(),
            label: "OneNote (Desktop)".into(),
            status: CheckStatus::Fail,
            message: "Only UWP/Store version of OneNote detected. Desktop version is required."
                .into(),
            remediation: Some(
                "Install Microsoft Office (Desktop) with OneNote. The Microsoft Store version \
                 (OneNote for Windows 10) does not support COM automation. You need Office 2013, \
                 2016, 2019, 2021, or Microsoft 365 desktop apps."
                    .into(),
            ),
        };
    }

    CheckResult {
        id: "onenote".into(),
        label: "OneNote Desktop".into(),
        status: CheckStatus::Fail,
        message: "OneNote desktop application not found.".into(),
        remediation: Some(
            "Install Microsoft Office (Desktop) with OneNote included. \
             Office 2013 (v15.0) or later is required."
                .into(),
        ),
    }
}

#[cfg(windows)]
fn is_uwp_onenote_installed() -> bool {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    use windows::core::*;

    unsafe {
        let subkey = w!("Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppModel\\Repository\\Packages");
        let mut key = HKEY::default();
        let status = RegOpenKeyExW(HKEY_CURRENT_USER, subkey, 0, KEY_READ, &mut key);
        if status != ERROR_SUCCESS {
            return false;
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

            let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
            if name.contains("OneNote") || name.contains("onenote") {
                let _ = RegCloseKey(key);
                return true;
            }

            index += 1;
        }

        let _ = RegCloseKey(key);
        false
    }
}

#[cfg(not(windows))]
pub fn check() -> CheckResult {
    CheckResult {
        id: "onenote".into(),
        label: "OneNote Desktop".into(),
        status: CheckStatus::Fail,
        message: "Not running on Windows - cannot check OneNote.".into(),
        remediation: Some("This tool must be run on Windows.".into()),
    }
}
