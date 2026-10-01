//! Tray icon and context menu.

use windows_sys::Win32::Foundation::{HWND, POINT};
use windows_sys::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY,
    NOTIFYICONDATAW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, DestroyMenu, GetCursorPos, LoadIconW, PostMessageW,
    SetForegroundWindow, TrackPopupMenu, IDI_APPLICATION, MF_CHECKED, MF_SEPARATOR, MF_STRING,
    TPM_RETURNCMD, TPM_RIGHTBUTTON, WM_APP, WM_CONTEXTMENU, WM_RBUTTONUP,
};

/// Custom message id used for tray callbacks.
pub const WM_TRAYICON: u32 = WM_APP + 100;
const TRAY_UID: u32 = 1;

const CMD_TOGGLE: usize = 1;
const CMD_OPACITY_UP: usize = 2;
const CMD_OPACITY_DOWN: usize = 3;
const CMD_RELOAD: usize = 4;
const CMD_AUTOSTART: usize = 5;
const CMD_OPEN_LOG: usize = 6;
const CMD_EXIT: usize = 7;

/// Result of the tray menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuCommand {
    Toggle,
    OpacityUp,
    OpacityDown,
    Reload,
    ToggleAutostart,
    OpenLog,
    Exit,
}

fn copy_wide(dst: &mut [u16], s: &str) {
    let encoded: Vec<u16> = s.encode_utf16().collect();
    let n = encoded.len().min(dst.len().saturating_sub(1));
    dst[..n].copy_from_slice(&encoded[..n]);
    dst[n] = 0;
}

fn base_data(hwnd: HWND) -> NOTIFYICONDATAW {
    let mut nid: NOTIFYICONDATAW = unsafe { core::mem::zeroed() };
    nid.cbSize = core::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = TRAY_UID;
    nid
}

/// Add the tray icon. Call again after Explorer restarts.
pub fn add(hwnd: HWND, tip: &str) -> bool {
    let mut nid = base_data(hwnd);
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = WM_TRAYICON;
    nid.hIcon = unsafe { LoadIconW(std::ptr::null_mut(), IDI_APPLICATION) };
    copy_wide(&mut nid.szTip, tip);
    unsafe { Shell_NotifyIconW(NIM_ADD, &nid) != 0 }
}

/// Update the tooltip of an existing tray icon.
pub fn update_tip(hwnd: HWND, tip: &str) -> bool {
    let mut nid = base_data(hwnd);
    nid.uFlags = NIF_TIP;
    copy_wide(&mut nid.szTip, tip);
    unsafe { Shell_NotifyIconW(NIM_MODIFY, &nid) != 0 }
}

/// Remove the tray icon.
pub fn remove(hwnd: HWND) {
    let nid = base_data(hwnd);
    unsafe {
        Shell_NotifyIconW(NIM_DELETE, &nid);
    }
}

/// Show the context menu and translate the selection.
pub fn show_menu(hwnd: HWND, enabled: bool, autostart: bool) -> Option<MenuCommand> {
    unsafe {
        let menu = CreatePopupMenu();
        if menu.is_null() {
            return None;
        }

        let toggle_flags = MF_STRING | if enabled { MF_CHECKED } else { 0 };
        append(menu, toggle_flags, CMD_TOGGLE, "Enabled");
        append(menu, MF_SEPARATOR, 0, "");
        append(menu, MF_STRING, CMD_OPACITY_UP, "Increase opacity");
        append(menu, MF_STRING, CMD_OPACITY_DOWN, "Decrease opacity");
        append(menu, MF_SEPARATOR, 0, "");
        append(menu, MF_STRING, CMD_RELOAD, "Reload config");
        append(
            menu,
            MF_STRING | if autostart { MF_CHECKED } else { 0 },
            CMD_AUTOSTART,
            "Start with Windows",
        );
        append(menu, MF_STRING, CMD_OPEN_LOG, "Open log");
        append(menu, MF_SEPARATOR, 0, "");
        append(menu, MF_STRING, CMD_EXIT, "Exit");

        let mut pt = POINT { x: 0, y: 0 };
        GetCursorPos(&mut pt);
        SetForegroundWindow(hwnd);
        let selected = TrackPopupMenu(
            menu,
            TPM_RETURNCMD | TPM_RIGHTBUTTON,
            pt.x,
            pt.y,
            0,
            hwnd,
            std::ptr::null(),
        );
        PostMessageW(hwnd, 0, 0, 0);
        DestroyMenu(menu);

        match selected as usize {
            CMD_TOGGLE => Some(MenuCommand::Toggle),
            CMD_OPACITY_UP => Some(MenuCommand::OpacityUp),
            CMD_OPACITY_DOWN => Some(MenuCommand::OpacityDown),
            CMD_RELOAD => Some(MenuCommand::Reload),
            CMD_AUTOSTART => Some(MenuCommand::ToggleAutostart),
            CMD_OPEN_LOG => Some(MenuCommand::OpenLog),
            CMD_EXIT => Some(MenuCommand::Exit),
            _ => None,
        }
    }
}

unsafe fn append(
    menu: windows_sys::Win32::UI::WindowsAndMessaging::HMENU,
    flags: u32,
    id: usize,
    text: &str,
) {
    let wide: Vec<u16> = if text.is_empty() {
        vec![0]
    } else {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    };
    let ptr = if text.is_empty() {
        std::ptr::null()
    } else {
        wide.as_ptr()
    };
    AppendMenuW(menu, flags, id, ptr);
}

/// `true` when `lparam` (tray callback) indicates a menu-open mouse event.
pub fn is_menu_event(lparam: isize) -> bool {
    let event = (lparam as usize) & 0xFFFF;
    event == WM_RBUTTONUP as usize || event == WM_CONTEXTMENU as usize
}
