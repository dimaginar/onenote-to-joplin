pub mod registry;
pub mod joplin_check;
pub mod os_check;
pub mod onenote_check;
pub mod word_check;
pub mod com_check;
pub mod sync_check;

use crate::types::{CheckStatus, ScanResult, ScanError};

pub fn run_all_checks() -> Result<ScanResult, ScanError> {
    let joplin = joplin_check::check();
    let os = os_check::check();
    let onenote = onenote_check::check();
    let word = word_check::check();
    let com = com_check::check();
    let sync_auto = sync_check::check_auto_sync();
    let sync_download = sync_check::check_full_download();

    let checks = vec![joplin, os, onenote, word, com, sync_auto, sync_download];
    let overall = if checks.iter().any(|c| c.status == CheckStatus::Fail) {
        CheckStatus::Fail
    } else if checks.iter().any(|c| c.status == CheckStatus::Warning) {
        CheckStatus::Warning
    } else if checks.iter().all(|c| c.status == CheckStatus::Skipped) {
        CheckStatus::Skipped
    } else {
        CheckStatus::Pass
    };

    let os_info = get_os_info();
    let timestamp = chrono::Utc::now().to_rfc3339();

    Ok(ScanResult {
        checks,
        timestamp,
        os_info,
        overall,
    })
}

#[cfg(windows)]
fn get_os_info() -> String {
    os_check::get_os_display_string()
}

#[cfg(not(windows))]
fn get_os_info() -> String {
    "Non-Windows OS (checks unavailable)".to_string()
}
