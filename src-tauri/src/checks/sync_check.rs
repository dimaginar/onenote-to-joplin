use crate::types::{CheckResult, CheckStatus};

#[cfg(windows)]
use windows::Win32::System::Registry::HKEY_CURRENT_USER;

#[cfg(windows)]
fn detect_onenote_version() -> Option<&'static str> {
    if super::registry::find_office_install_root("OneNote", "16.0").is_some() {
        Some("16.0")
    } else if super::registry::find_office_install_root("OneNote", "15.0").is_some() {
        Some("15.0")
    } else {
        None
    }
}

#[cfg(windows)]
fn version_suffix(version: &str) -> String {
    if version == "16.0" {
        format!("(Office {})", version)
    } else {
        format!("(Office {} \u{2014} unverified; check manually if unsure)", version)
    }
}

#[cfg(windows)]
pub fn check_auto_sync() -> CheckResult {
    let version = match detect_onenote_version() {
        Some(v) => v,
        None => {
            return CheckResult {
                id: "sync_auto".into(),
                label: "OneNote Auto-Sync".into(),
                status: CheckStatus::Skipped,
                message: "Skipped \u{2014} OneNote Desktop not installed".into(),
                remediation: None,
            };
        }
    };

    let suffix = version_suffix(version);
    let subkey = format!(
        "Software\\Microsoft\\Office\\{}\\OneNote\\Options\\Save",
        version
    );
    let value = super::registry::read_reg_dword(HKEY_CURRENT_USER, &subkey, "SyncStateOffline");

    match value {
        Some(0) | None => CheckResult {
            id: "sync_auto".into(),
            label: "OneNote Auto-Sync".into(),
            status: CheckStatus::Pass,
            message: format!("Automatic sync is enabled {}", suffix),
            remediation: None,
        },
        Some(_) => CheckResult {
            id: "sync_auto".into(),
            label: "OneNote Auto-Sync".into(),
            status: CheckStatus::Warning,
            message: format!("Automatic sync is disabled {}", suffix),
            remediation: Some(
                "Open OneNote \u{2192} File \u{2192} Options \u{2192} Sync and enable 'Sync notebooks automatically'. This ensures your notebooks are up-to-date before migration.".into()
            ),
        },
    }
}

#[cfg(windows)]
pub fn check_full_download() -> CheckResult {
    let version = match detect_onenote_version() {
        Some(v) => v,
        None => {
            return CheckResult {
                id: "sync_download".into(),
                label: "OneNote Full Download".into(),
                status: CheckStatus::Skipped,
                message: "Skipped \u{2014} OneNote Desktop not installed".into(),
                remediation: None,
            };
        }
    };

    let suffix = version_suffix(version);
    let subkey = format!(
        "Software\\Microsoft\\Office\\{}\\OneNote\\Options",
        version
    );
    let value = super::registry::read_reg_dword(HKEY_CURRENT_USER, &subkey, "DeferFdoDownload");

    match value {
        Some(0) => CheckResult {
            id: "sync_download".into(),
            label: "OneNote Full Download".into(),
            status: CheckStatus::Pass,
            message: format!("Full file and image download is enabled {}", suffix),
            remediation: None,
        },
        _ => CheckResult {
            id: "sync_download".into(),
            label: "OneNote Full Download".into(),
            status: CheckStatus::Warning,
            message: format!("Full file and image download is not enabled {}", suffix),
            remediation: Some(
                "Open OneNote \u{2192} File \u{2192} Options \u{2192} Sync and enable 'Download all files and images'. This ensures all attachments and embedded images are cached locally before migration, preventing missing content.".into()
            ),
        },
    }
}

#[cfg(not(windows))]
pub fn check_auto_sync() -> CheckResult {
    CheckResult {
        id: "sync_auto".into(),
        label: "OneNote Auto-Sync".into(),
        status: CheckStatus::Skipped,
        message: "Skipped \u{2014} OneNote Desktop not installed".into(),
        remediation: None,
    }
}

#[cfg(not(windows))]
pub fn check_full_download() -> CheckResult {
    CheckResult {
        id: "sync_download".into(),
        label: "OneNote Full Download".into(),
        status: CheckStatus::Skipped,
        message: "Skipped \u{2014} OneNote Desktop not installed".into(),
        remediation: None,
    }
}
