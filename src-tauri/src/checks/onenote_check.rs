use crate::types::{CheckResult, CheckStatus};

#[cfg(windows)]
pub fn check() -> CheckResult {
    let registry_info = detect_via_registry();
    let com_result = super::registry::test_com_activation("OneNote.Application");

    match (&registry_info, &com_result) {
        (Some(info), Ok(())) => CheckResult {
            id: "onenote".into(),
            label: "OneNote (Desktop)".into(),
            status: CheckStatus::Pass,
            message: format!("{} — COM automation verified.", info),
            remediation: None,
        },
        (None, Ok(())) => CheckResult {
            id: "onenote".into(),
            label: "OneNote (Desktop)".into(),
            status: CheckStatus::Warning,
            message: "OneNote COM automation works but installation not found via standard registry paths.".into(),
            remediation: Some(
                "The export should work, but your Office installation appears non-standard \
                 (e.g. Microsoft Store or MSIX deployment). For best reliability, consider \
                 reinstalling Office as a standard Click-to-Run installation from office.com."
                    .into(),
            ),
        },
        (Some(info), Err(e)) => CheckResult {
            id: "onenote".into(),
            label: "OneNote (Desktop)".into(),
            status: CheckStatus::Fail,
            message: format!("{} — but COM automation failed: {}", info, e),
            remediation: Some(
                "OneNote is installed but COM automation is not working. Try repairing \
                 your Office installation: Settings \u{2192} Apps \u{2192} Microsoft Office \
                 \u{2192} Modify \u{2192} Online Repair."
                    .into(),
            ),
        },
        (None, Err(_)) => {
            if is_uwp_onenote_installed() {
                CheckResult {
                    id: "onenote".into(),
                    label: "OneNote (Desktop)".into(),
                    status: CheckStatus::Fail,
                    message: "Only UWP/Store version of OneNote detected. Desktop version is required.".into(),
                    remediation: Some(
                        "Install Microsoft Office (Desktop) with OneNote. The Microsoft Store version \
                         (OneNote for Windows 10) does not support COM automation. You need Office 2013, \
                         2016, 2019, 2021, or Microsoft 365 desktop apps."
                            .into(),
                    ),
                }
            } else {
                CheckResult {
                    id: "onenote".into(),
                    label: "OneNote (Desktop)".into(),
                    status: CheckStatus::Fail,
                    message: "OneNote desktop application not found.".into(),
                    remediation: Some(
                        "Install Microsoft Office (Desktop) with OneNote included. \
                         Office 2013 or later is required."
                            .into(),
                    ),
                }
            }
        }
    }
}

#[cfg(windows)]
fn detect_via_registry() -> Option<String> {
    for version in &["16.0", "15.0"] {
        if let Some(path) = super::registry::find_office_install_root("OneNote", version) {
            return Some(format!("Version {} at {}", version, path));
        }
    }
    if let Some(c2r) = super::registry::find_click_to_run() {
        if !super::registry::is_c2r_app_excluded(&c2r.product_ids, "OneNote") {
            let ver = c2r.version.as_deref().unwrap_or("unknown");
            return Some(format!("Click-to-Run {} (v{})", c2r.product_ids, ver));
        }
    }
    None
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
        label: "OneNote (Desktop)".into(),
        status: CheckStatus::Fail,
        message: "Not running on Windows — cannot check OneNote.".into(),
        remediation: Some("This tool must be run on Windows.".into()),
    }
}
