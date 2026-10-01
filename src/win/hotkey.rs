//! Global hotkey parsing and registration.

use windows_sys::Win32::Foundation::{BOOL, HWND};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey};

pub const MOD_ALT: u32 = 0x0001;
pub const MOD_CONTROL: u32 = 0x0002;
pub const MOD_SHIFT: u32 = 0x0004;
pub const MOD_WIN: u32 = 0x0008;
pub const MOD_NOREPEAT: u32 = 0x4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotkeySpec {
    pub modifiers: u32,
    pub vk: u32,
}

/// Parse strings like `Ctrl+Alt+T`, `Ctrl+Shift+Up` or `Win+F12`.
pub fn parse(spec: &str) -> Option<HotkeySpec> {
    let mut modifiers = 0u32;
    let mut vk: Option<u32> = None;

    for raw in spec.split('+') {
        let token = raw.trim();
        if token.is_empty() {
            return None;
        }
        match token.to_ascii_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= MOD_CONTROL,
            "alt" => modifiers |= MOD_ALT,
            "shift" => modifiers |= MOD_SHIFT,
            "win" | "super" | "meta" => modifiers |= MOD_WIN,
            other => {
                if vk.is_some() {
                    return None;
                }
                vk = key_to_vk(other);
                vk?;
            }
        }
    }

    Some(HotkeySpec { modifiers, vk: vk? })
}

fn key_to_vk(key: &str) -> Option<u32> {
    let bytes = key.as_bytes();
    if bytes.len() == 1 {
        let c = bytes[0];
        if c.is_ascii_alphabetic() {
            return Some(c.to_ascii_uppercase() as u32);
        }
        if c.is_ascii_digit() {
            return Some(c as u32);
        }
    }
    match key {
        "up" => Some(0x26),
        "down" => Some(0x28),
        "left" => Some(0x25),
        "right" => Some(0x27),
        "space" => Some(0x20),
        "tab" => Some(0x09),
        "enter" | "return" => Some(0x0D),
        "esc" | "escape" => Some(0x1B),
        "home" => Some(0x24),
        "end" => Some(0x23),
        "pageup" => Some(0x21),
        "pagedown" => Some(0x22),
        fkey if fkey.starts_with('f') => {
            let n: u32 = fkey[1..].parse().ok()?;
            (1..=24).contains(&n).then_some(0x70 + n - 1)
        }
        _ => None,
    }
}

/// Register a hotkey on `hwnd`. `MOD_NOREPEAT` is always added so a held chord
/// fires once.
pub fn register(hwnd: HWND, id: i32, spec: &HotkeySpec) -> Result<(), String> {
    let ok: BOOL = unsafe { RegisterHotKey(hwnd, id, spec.modifiers | MOD_NOREPEAT, spec.vk) };
    if ok == 0 {
        Err(format!(
            "RegisterHotKey failed (id={id}, vk={:#x}, mods={:#x})",
            spec.vk, spec.modifiers
        ))
    } else {
        Ok(())
    }
}

pub fn unregister(hwnd: HWND, id: i32) {
    unsafe {
        UnregisterHotKey(hwnd, id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_toggle() {
        let h = parse("Ctrl+Alt+T").unwrap();
        assert_eq!(h.modifiers, MOD_CONTROL | MOD_ALT);
        assert_eq!(h.vk, 'T' as u32);
    }

    #[test]
    fn parses_arrows_and_fkeys() {
        assert_eq!(parse("Ctrl+Alt+Up").unwrap().vk, 0x26);
        assert_eq!(parse("Ctrl+Alt+Down").unwrap().vk, 0x28);
        assert_eq!(parse("Win+F12").unwrap().vk, 0x7B);
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse("Ctrl+Foo").is_none());
        assert!(parse("Ctrl+").is_none());
        assert!(parse("A+B").is_none());
        assert!(parse("Ctrl+F99").is_none());
    }

    #[test]
    fn ordering_of_modifiers_does_not_matter() {
        assert_eq!(parse("Alt+Ctrl+T"), parse("Ctrl+Alt+T"));
    }
}
