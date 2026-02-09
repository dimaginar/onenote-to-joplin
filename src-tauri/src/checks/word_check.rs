use crate::types::{CheckResult, CheckStatus};

#[cfg(windows)]
pub fn check() -> CheckResult {
    for version in &["16.0", "15.0"] {
        if let Some(path) = super::registry::find_office_install_root("Word", version) {
            return CheckResult {
                id: "word".into(),
                label: "Word".into(),
                status: CheckStatus::Pass,
                message: format!("Version {} found at {}", version, path),
                remediation: None,
            };
        }
    }

    CheckResult {
        id: "word".into(),
        label: "Word Desktop".into(),
        status: CheckStatus::Fail,
        message: "Word desktop application not found.".into(),
        remediation: Some(
            "Install Microsoft Office (Desktop) with Word included. \
             Office 2013 (v15.0) or later is required. Word is needed for \
             data rendering during the export process."
                .into(),
        ),
    }
}

#[cfg(not(windows))]
pub fn check() -> CheckResult {
    CheckResult {
        id: "word".into(),
        label: "Word Desktop".into(),
        status: CheckStatus::Fail,
        message: "Not running on Windows - cannot check Word.".into(),
        remediation: Some("This tool must be run on Windows.".into()),
    }
}
