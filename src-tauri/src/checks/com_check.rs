use crate::types::{CheckResult, CheckStatus};

#[cfg(windows)]
pub fn check() -> CheckResult {
    use windows::Win32::System::Com::*;

    unsafe {
        // Initialize COM in STA mode (required for Office COM objects)
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return CheckResult {
                id: "com_bridge".into(),
                label: "COM Automation".into(),
                status: CheckStatus::Fail,
                message: format!("COM initialization failed: {:?}", hr),
                remediation: Some(
                    "COM subsystem could not be initialized. Try restarting your computer.".into(),
                ),
            };
        }

        let onenote_ok = test_com_object("OneNote.Application");
        let word_ok = test_com_object("Word.Application");

        CoUninitialize();

        match (onenote_ok, word_ok) {
            (Ok(()), Ok(())) => CheckResult {
                id: "com_bridge".into(),
                label: "COM Automation".into(),
                status: CheckStatus::Pass,
                message: "OneNote.Application and Word.Application are accessible.".into(),
                remediation: None,
            },
            (Err(onenote_err), Err(word_err)) => CheckResult {
                id: "com_bridge".into(),
                label: "COM Automation".into(),
                status: CheckStatus::Fail,
                message: format!(
                    "Both COM objects failed. OneNote: {}. Word: {}.",
                    onenote_err, word_err
                ),
                remediation: Some(
                    "Office COM objects are not accessible. Try repairing your Office \
                     installation: Open Settings > Apps > Microsoft Office > Modify > \
                     Online Repair."
                        .into(),
                ),
            },
            (Err(e), Ok(())) => CheckResult {
                id: "com_bridge".into(),
                label: "COM Automation".into(),
                status: CheckStatus::Fail,
                message: format!("OneNote.Application failed: {}. Word.Application OK.", e),
                remediation: Some(
                    "OneNote COM automation is not accessible. If you have the Store/UWP \
                     version, you need the Desktop version instead. Otherwise, try repairing \
                     your Office installation."
                        .into(),
                ),
            },
            (Ok(()), Err(e)) => CheckResult {
                id: "com_bridge".into(),
                label: "COM Automation".into(),
                status: CheckStatus::Fail,
                message: format!("OneNote.Application OK. Word.Application failed: {}.", e),
                remediation: Some(
                    "Word COM automation is not accessible. Try repairing your Office \
                     installation: Open Settings > Apps > Microsoft Office > Modify > \
                     Online Repair."
                        .into(),
                ),
            },
        }
    }
}

#[cfg(windows)]
unsafe fn test_com_object(prog_id: &str) -> Result<(), String> {
    use windows::Win32::System::Com::*;
    use windows::core::*;

    let prog_id_wide: HSTRING = prog_id.into();
    let clsid = CLSIDFromProgID(&prog_id_wide).map_err(|e| format!("CLSIDFromProgID: {}", e))?;

    let result: windows::core::Result<IUnknown> =
        CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER);
    match result {
        Ok(_obj) => Ok(()),
        Err(e) => Err(format!("CoCreateInstance: {}", e)),
    }
}

#[cfg(not(windows))]
pub fn check() -> CheckResult {
    CheckResult {
        id: "com_bridge".into(),
        label: "COM Bridge".into(),
        status: CheckStatus::Fail,
        message: "Not running on Windows - COM is not available.".into(),
        remediation: Some("This tool must be run on Windows.".into()),
    }
}
