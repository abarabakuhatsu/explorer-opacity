//! Autostart via `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.

use std::path::PathBuf;
use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows_sys::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
    HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_SET_VALUE, REG_SZ,
};

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "ExplorerOpacity";

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Path to the running executable.
pub fn exe_path() -> PathBuf {
    std::env::current_exe().unwrap_or_default()
}

fn quoted_exe() -> String {
    format!("\"{}\"", exe_path().display())
}

/// Whether the autostart entry points at any executable.
pub fn is_enabled() -> bool {
    read_value().map(|v| !v.is_empty()).unwrap_or(false)
}

fn read_value() -> Option<String> {
    unsafe {
        let sub = wide(RUN_KEY);
        let mut hkey: HKEY = std::ptr::null_mut();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            sub.as_ptr(),
            0,
            KEY_QUERY_VALUE,
            &mut hkey,
        ) != 0
        {
            return None;
        }

        let name = wide(VALUE_NAME);
        let mut ty: u32 = 0;
        let mut buf = vec![0u8; 4096];
        let mut len = buf.len() as u32;
        let rc = RegQueryValueExW(
            hkey,
            name.as_ptr(),
            std::ptr::null(),
            &mut ty,
            buf.as_mut_ptr(),
            &mut len,
        );
        RegCloseKey(hkey);
        if rc != 0 {
            return None;
        }

        buf.truncate(len as usize);
        let (pairs, _) = buf.as_chunks::<2>();
        let units: Vec<u16> = pairs.iter().map(|c| u16::from_le_bytes(*c)).collect();
        Some(
            String::from_utf16_lossy(&units)
                .trim_end_matches('\0')
                .to_string(),
        )
    }
}

/// Whether the registered command matches the currently running executable.
pub fn points_at_self() -> bool {
    read_value()
        .map(|v| v.eq_ignore_ascii_case(&quoted_exe()))
        .unwrap_or(false)
}

/// Register the current executable, refreshing the path (handles a moved exe).
pub fn enable() -> Result<(), String> {
    set(true)
}

/// Remove the autostart entry.
pub fn disable() -> Result<(), String> {
    set(false)
}

/// Flip autostart state. Returns the new state. Idempotent on the current exe.
pub fn toggle() -> Result<bool, String> {
    if points_at_self() {
        disable()?;
        Ok(false)
    } else {
        enable()?;
        Ok(true)
    }
}

fn set(enable: bool) -> Result<(), String> {
    unsafe {
        let sub = wide(RUN_KEY);
        let mut hkey: HKEY = std::ptr::null_mut();
        if RegOpenKeyExW(HKEY_CURRENT_USER, sub.as_ptr(), 0, KEY_SET_VALUE, &mut hkey) != 0 {
            return Err("could not open HKCU Run key".to_string());
        }

        let name = wide(VALUE_NAME);
        let rc = if enable {
            let data = wide(&quoted_exe());
            let bytes = std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 2);
            RegSetValueExW(
                hkey,
                name.as_ptr(),
                0,
                REG_SZ,
                bytes.as_ptr(),
                bytes.len() as u32,
            )
        } else {
            RegDeleteValueW(hkey, name.as_ptr())
        };
        RegCloseKey(hkey);

        let ok = rc == 0 || (!enable && rc == ERROR_FILE_NOT_FOUND);
        if ok {
            Ok(())
        } else {
            Err(format!("registry update failed (code {rc})"))
        }
    }
}
