//! Top-level window enumeration, class filtering and layered-window alpha.

use crate::config::Config;
use crate::logln;
use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM, TRUE};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, GetLayeredWindowAttributes, GetWindowLongPtrW,
    SetLayeredWindowAttributes, SetWindowLongPtrW, GWL_EXSTYLE, LWA_ALPHA, WS_EX_LAYERED,
};

/// Stable integer handle used as a map key.
pub type WindowId = isize;

pub fn id_of(hwnd: HWND) -> WindowId {
    hwnd as isize
}

pub fn hwnd_of(id: WindowId) -> HWND {
    id as HWND
}

/// Return the class name of a window (empty string on failure).
pub fn class_name(hwnd: HWND) -> String {
    let mut buf = [0u16; 256];
    let len = unsafe { GetClassNameW(hwnd, buf.as_mut_ptr(), buf.len() as i32) };
    if len <= 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buf[..len as usize])
}

/// Enumerate every top-level window.
pub fn top_level_windows() -> Vec<HWND> {
    let mut out: Vec<HWND> = Vec::new();
    unsafe {
        EnumWindows(Some(enum_proc), &mut out as *mut _ as isize);
    }
    out
}

unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let out = &mut *(lparam as *mut Vec<HWND>);
    out.push(hwnd);
    TRUE
}

/// Enumerate the top-level windows that match the configured target rules.
pub fn matching_windows(cfg: &Config) -> Vec<HWND> {
    top_level_windows()
        .into_iter()
        .filter(|&hwnd| {
            let class = class_name(hwnd);
            crate::filter::matches_class(&class, &cfg.targets.include, &cfg.targets.exclude)
        })
        .collect()
}

/// Snapshot of everything needed to undo a layered-alpha change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlphaState {
    /// `true` when we added `WS_EX_LAYERED` ourselves and must remove it later.
    pub added_layered: bool,
    /// Pre-existing `LWA` colour key (only meaningful when `!added_layered`).
    pub prev_key: u32,
    /// Pre-existing alpha byte (only meaningful when `!added_layered`).
    pub prev_alpha: u8,
    /// Pre-existing `LWA_*` flags; `0` means "unknown / not layered".
    pub prev_flags: u32,
}

/// Apply a uniform layered alpha to `hwnd`, returning the state needed to undo it.
pub fn apply_alpha(hwnd: HWND, alpha: u8) -> Result<AlphaState, String> {
    unsafe {
        let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let already_layered = (ex & WS_EX_LAYERED as isize) != 0;

        let mut state = AlphaState {
            added_layered: !already_layered,
            prev_key: 0,
            prev_alpha: 0,
            prev_flags: 0,
        };

        if already_layered {
            let mut key: u32 = 0;
            let mut a: u8 = 0;
            let mut flags: u32 = 0;
            if GetLayeredWindowAttributes(hwnd, &mut key, &mut a, &mut flags) != 0 {
                state.prev_key = key;
                state.prev_alpha = a;
                state.prev_flags = flags;
            }
        } else {
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex | WS_EX_LAYERED as isize);
        }

        if SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA) == 0 {
            if !already_layered {
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex);
            }
            return Err(format!(
                "SetLayeredWindowAttributes failed for hwnd {hwnd:p}"
            ));
        }

        Ok(state)
    }
}

/// Update the alpha of an already-layered window without touching tracked state.
pub fn set_layered_alpha(hwnd: HWND, alpha: u8) -> bool {
    unsafe { SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA) != 0 }
}

/// Re-apply the previous alpha, or remove `WS_EX_LAYERED` if we added it.
pub fn restore_alpha(hwnd: HWND, state: &AlphaState) {
    unsafe {
        if state.added_layered {
            let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex & !(WS_EX_LAYERED as isize));
        } else if state.prev_flags != 0 {
            SetLayeredWindowAttributes(hwnd, state.prev_key, state.prev_alpha, state.prev_flags);
        } else {
            logln!(
                "restore_alpha: original layered attributes unknown for hwnd {hwnd:p}; leaving as-is"
            );
        }
    }
}
