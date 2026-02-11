use crate::types::{CheckResult, CheckStatus};

#[cfg(windows)]
pub fn check() -> CheckResult {
    let registry_info = detect_via_registry();
    let com_result = super::registry::test_com_activation("Word.Application");

    match (&registry_info, &com_result) {
        (Some(info), Ok(())) => CheckResult {
            id: "word".into(),
            label: "Word".into(),
            status: CheckStatus::Pass,
            message: format!("{} — COM automation verified.", info),
            remediation: None,
        },
        (None, Ok(())) => CheckResult {
            id: "word".into(),
            label: "Word".into(),
            status: CheckStatus::Warning,
            message: "Word COM automation works but installation not found via standard registry paths.".into(),
            remediation: Some(
                "The export should work, but your Office installation appears non-standard \
                 (e.g. Microsoft Store or MSIX deployment). For best reliability, consider \
                 reinstalling Office as a standard Click-to-Run installation from office.com."
                    .into(),
            ),
        },
        (Some(info), Err(e)) => CheckResult {
            id: "word".into(),
            label: "Word".into(),
            status: CheckStatus::Fail,
            message: format!("{} — but COM automation failed: {}", info, e),
            remediation: Some(
                "Word is installed but COM automation is not working. Try repairing \
                 your Office installation: Settings \u{2192} Apps \u{2192} Microsoft Office \
                 \u{2192} Modify \u{2192} Online Repair."
                    .into(),
            ),
        },
        (None, Err(_)) => CheckResult {
            id: "word".into(),
            label: "Word".into(),
            status: CheckStatus::Fail,
            message: "Word desktop application not found.".into(),
            remediation: Some(
                "Install Microsoft Office (Desktop) with Word included. \
                 Office 2013 or later is required. Word is needed for \
                 data rendering during the export process."
                    .into(),
            ),
        },
    }
}

#[cfg(windows)]
fn detect_via_registry() -> Option<String> {
    for version in &["16.0", "15.0"] {
        if let Some(path) = super::registry::find_office_install_root("Word", version) {
            return Some(format!("Version {} at {}", version, path));
        }
    }
    if let Some(c2r) = super::registry::find_click_to_run() {
        if !super::registry::is_c2r_app_excluded(&c2r.product_ids, "Word") {
            let ver = c2r.version.as_deref().unwrap_or("unknown");
            return Some(format!("Click-to-Run {} (v{})", c2r.product_ids, ver));
        }
    }
    None
}

#[cfg(not(windows))]
pub fn check() -> CheckResult {
    CheckResult {
        id: "word".into(),
        label: "Word".into(),
        status: CheckStatus::Fail,
        message: "Not running on Windows — cannot check Word.".into(),
        remediation: Some("This tool must be run on Windows.".into()),
    }
}
