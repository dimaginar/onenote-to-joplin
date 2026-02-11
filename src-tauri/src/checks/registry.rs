#[cfg(windows)]
pub fn find_office_install_root(app: &str, version: &str) -> Option<String> {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    use windows::core::*;

    let paths = [
        format!(
            "SOFTWARE\\Microsoft\\Office\\{}\\{}\\InstallRoot",
            version, app
        ),
        format!(
            "SOFTWARE\\WOW6432Node\\Microsoft\\Office\\{}\\{}\\InstallRoot",
            version, app
        ),
    ];

    for subkey_str in &paths {
        unsafe {
            let subkey: Vec<u16> = subkey_str.encode_utf16().chain(std::iter::once(0)).collect();
            let mut key = HKEY::default();
            let status = RegOpenKeyExW(
                HKEY_LOCAL_MACHINE,
                PCWSTR::from_raw(subkey.as_ptr()),
                0,
                KEY_READ,
                &mut key,
            );
            if status != ERROR_SUCCESS {
                continue;
            }

            let value_name = w!("Path");
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

            if buf_size > 0 {
                let mut buffer = vec![0u8; buf_size as usize];
                let status = RegQueryValueExW(
                    key,
                    value_name,
                    None,
                    Some(&mut value_type),
                    Some(buffer.as_mut_ptr()),
                    Some(&mut buf_size),
                );
                if status == ERROR_SUCCESS {
                    let _ = RegCloseKey(key);
                    let wide: Vec<u16> = buffer
                        .chunks_exact(2)
                        .map(|c| u16::from_le_bytes([c[0], c[1]]))
                        .collect();
                    return Some(
                        String::from_utf16_lossy(&wide)
                            .trim_end_matches('\0')
                            .to_string(),
                    );
                }
            }
            let _ = RegCloseKey(key);
        }
    }
    None
}

#[cfg(not(windows))]
pub fn find_office_install_root(_app: &str, _version: &str) -> Option<String> {
    None
}

#[cfg(windows)]
pub fn read_reg_dword(root: windows::Win32::System::Registry::HKEY, subkey: &str, value_name: &str) -> Option<u32> {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    use windows::core::*;

    unsafe {
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey = HKEY::default();
        let status = RegOpenKeyExW(
            root,
            PCWSTR::from_raw(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if status != ERROR_SUCCESS {
            return None;
        }

        let value_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut data: u32 = 0;
        let mut data_size: u32 = std::mem::size_of::<u32>() as u32;
        let mut reg_type = REG_VALUE_TYPE::default();

        let status = RegQueryValueExW(
            hkey,
            PCWSTR::from_raw(value_wide.as_ptr()),
            None,
            Some(&mut reg_type),
            Some(&mut data as *mut u32 as *mut u8),
            Some(&mut data_size),
        );

        let _ = RegCloseKey(hkey);

        if status == ERROR_SUCCESS && reg_type == REG_DWORD {
            Some(data)
        } else {
            None
        }
    }
}

#[cfg(not(windows))]
pub fn read_reg_dword(_root: u32, _subkey: &str, _value_name: &str) -> Option<u32> {
    None
}

/// Info about a Click-to-Run Office installation
pub struct ClickToRunInfo {
    pub install_path: String,
    pub product_ids: String,
    pub version: Option<String>,
}

/// Read a REG_SZ string value from a registry key
#[cfg(windows)]
fn read_reg_string(root: windows::Win32::System::Registry::HKEY, subkey: &str, value_name: &str) -> Option<String> {
    use windows::Win32::System::Registry::*;
    use windows::Win32::Foundation::*;
    use windows::core::*;

    unsafe {
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey = HKEY::default();
        let status = RegOpenKeyExW(
            root,
            PCWSTR::from_raw(subkey_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if status != ERROR_SUCCESS {
            return None;
        }

        let value_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut buf_size: u32 = 0;
        let mut value_type = REG_VALUE_TYPE::default();
        let _ = RegQueryValueExW(
            hkey,
            PCWSTR::from_raw(value_wide.as_ptr()),
            None,
            Some(&mut value_type),
            None,
            Some(&mut buf_size),
        );

        if buf_size == 0 {
            let _ = RegCloseKey(hkey);
            return None;
        }

        let mut buffer = vec![0u8; buf_size as usize];
        let status = RegQueryValueExW(
            hkey,
            PCWSTR::from_raw(value_wide.as_ptr()),
            None,
            Some(&mut value_type),
            Some(buffer.as_mut_ptr()),
            Some(&mut buf_size),
        );
        let _ = RegCloseKey(hkey);

        if status == ERROR_SUCCESS && (value_type == REG_SZ || value_type == REG_EXPAND_SZ) {
            let wide: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            Some(String::from_utf16_lossy(&wide).trim_end_matches('\0').to_string())
        } else {
            None
        }
    }
}

/// Detect Click-to-Run Office installation
#[cfg(windows)]
pub fn find_click_to_run() -> Option<ClickToRunInfo> {
    use windows::Win32::System::Registry::*;

    let c2r_key = "SOFTWARE\\Microsoft\\Office\\ClickToRun\\Configuration";

    let install_path = read_reg_string(HKEY_LOCAL_MACHINE, c2r_key, "InstallationPath")?;
    let product_ids = read_reg_string(HKEY_LOCAL_MACHINE, c2r_key, "ProductReleaseIds")
        .unwrap_or_default();
    let version = read_reg_string(HKEY_LOCAL_MACHINE, c2r_key, "VersionToReport");

    Some(ClickToRunInfo {
        install_path,
        product_ids,
        version,
    })
}

#[cfg(not(windows))]
pub fn find_click_to_run() -> Option<ClickToRunInfo> {
    None
}

/// Check if a specific app is excluded from a C2R installation.
/// Checks ExcludedApps for each product ID (e.g. "O365ProPlusRetail.ExcludedApps").
#[cfg(windows)]
pub fn is_c2r_app_excluded(product_ids: &str, app_name: &str) -> bool {
    use windows::Win32::System::Registry::*;

    let c2r_key = "SOFTWARE\\Microsoft\\Office\\ClickToRun\\Configuration";

    for product_id in product_ids.split(',') {
        let product_id = product_id.trim();
        if product_id.is_empty() {
            continue;
        }
        let value_name = format!("{}.ExcludedApps", product_id);
        if let Some(excluded) = read_reg_string(HKEY_LOCAL_MACHINE, c2r_key, &value_name) {
            if excluded.to_lowercase().contains(&app_name.to_lowercase()) {
                return true;
            }
        }
        // If ExcludedApps doesn't exist for this product, the app is included
    }
    false
}

#[cfg(not(windows))]
pub fn is_c2r_app_excluded(_product_ids: &str, _app_name: &str) -> bool {
    false
}

/// Test COM activation for an Office app. Returns Ok(()) if COM works.
#[cfg(windows)]
pub fn test_com_activation(prog_id: &str) -> Result<(), String> {
    use windows::Win32::System::Com::*;
    use windows::core::*;

    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(format!("COM init failed: {:?}", hr));
        }

        let prog_id_wide: HSTRING = prog_id.into();
        let clsid = match CLSIDFromProgID(&prog_id_wide) {
            Ok(c) => c,
            Err(e) => {
                CoUninitialize();
                return Err(format!("CLSIDFromProgID: {}", e));
            }
        };

        let result: windows::core::Result<IUnknown> =
            CoCreateInstance(&clsid, None, CLSCTX_LOCAL_SERVER);

        CoUninitialize();

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("CoCreateInstance: {}", e)),
        }
    }
}

#[cfg(not(windows))]
pub fn test_com_activation(_prog_id: &str) -> Result<(), String> {
    Err("Not on Windows".into())
}
