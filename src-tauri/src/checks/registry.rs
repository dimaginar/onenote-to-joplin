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
