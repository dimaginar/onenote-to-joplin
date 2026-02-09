use crate::types::{CheckResult, CheckStatus};

#[cfg(windows)]
pub fn check() -> CheckResult {
    let (build, display_version, product_name, ubr) = match read_os_version() {
        Ok(v) => v,
        Err(e) => {
            return CheckResult {
                id: "windows_os".into(),
                label: "Windows OS".into(),
                status: CheckStatus::Fail,
                message: format!("Could not read OS version: {}", e),
                remediation: Some("Ensure you are running Windows 10 or 11.".into()),
            };
        }
    };

    let build_num: u32 = build.parse().unwrap_or(0);

    let product_name = if build_num >= 22000 {
        product_name.replace("Windows 10", "Windows 11")
    } else {
        product_name
    };

    let build_str = match ubr {
        Some(u) => format!("{}.{}", build, u),
        None => build.clone(),
    };

    let version_suffix = if display_version.is_empty() {
        String::new()
    } else {
        format!(" {}", display_version)
    };

    if build_num >= 19041 {
        CheckResult {
            id: "windows_os".into(),
            label: "Windows OS".into(),
            status: CheckStatus::Pass,
            message: format!("{}{} (Build {})", product_name, version_suffix, build_str),
            remediation: None,
        }
    } else if build_num >= 10240 {
        CheckResult {
            id: "windows_os".into(),
            label: "Windows OS".into(),
            status: CheckStatus::Warning,
            message: format!("{}{} (Build {}) - consider updating", product_name, version_suffix, build_str),
            remediation: Some(
                "Update Windows 10 to version 2004 or later via Windows Update.".into(),
            ),
        }
    } else {
        CheckResult {
            id: "windows_os".into(),
            label: "Windows OS".into(),
            status: CheckStatus::Fail,
            message: format!("Unsupported OS: {}{} (Build {})", product_name, version_suffix, build_str),
            remediation: Some("Windows 10 (version 2004+) or Windows 11 is required.".into()),
        }
    }
}

#[cfg(windows)]
fn read_os_version() -> Result<(String, String, String, Option<u32>), String> {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    use windows::core::*;

    unsafe {
        let subkey = w!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
        let mut key = HKEY::default();
        let status = RegOpenKeyExW(HKEY_LOCAL_MACHINE, subkey, 0, KEY_READ, &mut key);
        if status != ERROR_SUCCESS {
            return Err(format!("RegOpenKeyExW failed: {:?}", status));
        }

        let build = read_reg_string(key, w!("CurrentBuild")).unwrap_or_default();
        let display = read_reg_string(key, w!("DisplayVersion")).unwrap_or_default();
        let product = read_reg_string(key, w!("ProductName")).unwrap_or_default();

        let _ = RegCloseKey(key);

        let ubr = super::registry::read_reg_dword(
            HKEY_LOCAL_MACHINE,
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "UBR",
        );

        Ok((build, display, product, ubr))
    }
}

#[cfg(windows)]
unsafe fn read_reg_string(
    key: windows::Win32::System::Registry::HKEY,
    value_name: windows::core::PCWSTR,
) -> Result<String, String> {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;

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
        return Err("Empty value".into());
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
        return Err(format!("RegQueryValueExW failed: {:?}", status));
    }

    let wide: Vec<u16> = buffer
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    Ok(String::from_utf16_lossy(&wide)
        .trim_end_matches('\0')
        .to_string())
}

#[cfg(windows)]
pub fn get_os_display_string() -> String {
    match read_os_version() {
        Ok((build, display, product, ubr)) => {
            let build_num: u32 = build.parse().unwrap_or(0);
            let product = if build_num >= 22000 {
                product.replace("Windows 10", "Windows 11")
            } else {
                product
            };
            let build_str = match ubr {
                Some(u) => format!("{}.{}", build, u),
                None => build,
            };
            if display.is_empty() {
                format!("{} (Build {})", product, build_str)
            } else {
                format!("{} {} (Build {})", product, display, build_str)
            }
        },
        Err(_) => "Windows (version unknown)".into(),
    }
}

#[cfg(not(windows))]
pub fn check() -> CheckResult {
    CheckResult {
        id: "windows_os".into(),
        label: "Windows OS".into(),
        status: CheckStatus::Fail,
        message: "Not running on Windows".into(),
        remediation: Some("This tool must be run on Windows 10 or 11.".into()),
    }
}

#[cfg(not(windows))]
pub fn get_os_display_string() -> String {
    "Non-Windows OS".to_string()
}
