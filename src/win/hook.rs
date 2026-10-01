//! The resident application: hidden message window, shell hook, tray, hotkeys,
//! periodic safety sweep and window state management.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::config::{self, Config, MAX_OPACITY, MIN_OPACITY};
use crate::logln;
use crate::win::window::WindowId;
use crate::win::{autostart, hotkey, tray, window};

use windows_sys::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HWND, LPARAM, LRESULT, WPARAM,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Threading::CreateMutexW;
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, IsWindow, KillTimer,
    PostQuitMessage, RegisterClassW, RegisterShellHookWindow, RegisterWindowMessageW, SetTimer,
    TranslateMessage, HSHELL_WINDOWCREATED, MSG, SW_SHOWNORMAL, WM_CLOSE, WM_DESTROY,
    WM_ENDSESSION, WM_HOTKEY, WM_TIMER, WNDCLASSW, WS_EX_TOOLWINDOW, WS_POPUP,
};

const SWEEP_TIMER_ID: usize = 1;
const SWEEP_INTERVAL_MS: u32 = 30_000;
const HOTKEY_TOGGLE: i32 = 1;
const HOTKEY_INCREASE: i32 = 2;
const HOTKEY_DECREASE: i32 = 3;

thread_local! {
    static APP: RefCell<Option<App>> = const { RefCell::new(None) };
}

struct App {
    cfg: Config,
    /// Runtime on/off state (mirrors `cfg.enabled` initially).
    enabled: bool,
    alpha_states: HashMap<WindowId, window::AlphaState>,
    hwnd: HWND,
    shell_hook_msg: u32,
    taskbar_created_msg: u32,
    autostart: bool,
}

impl App {
    fn new(cfg: Config, hwnd: HWND, shell_hook_msg: u32, taskbar_created_msg: u32) -> Self {
        Self {
            enabled: cfg.enabled,
            cfg,
            alpha_states: HashMap::new(),
            hwnd,
            shell_hook_msg,
            taskbar_created_msg,
            autostart: autostart::points_at_self(),
        }
    }

    fn setup(&mut self) {
        unsafe {
            if RegisterShellHookWindow(self.hwnd) == 0 {
                logln!("RegisterShellHookWindow failed");
            }
            SetTimer(self.hwnd, SWEEP_TIMER_ID, SWEEP_INTERVAL_MS, None);
        }
        if !tray::add(self.hwnd, &self.tip()) {
            logln!("tray::add failed");
        }
        self.register_hotkeys();
        self.sweep();
        logln!(
            "started: opacity={}%, enabled={}",
            self.cfg.opacity,
            self.enabled
        );
    }

    fn register_hotkeys(&self) {
        let specs = [
            (HOTKEY_TOGGLE, self.cfg.hotkeys.toggle.as_str()),
            (HOTKEY_INCREASE, self.cfg.hotkeys.increase.as_str()),
            (HOTKEY_DECREASE, self.cfg.hotkeys.decrease.as_str()),
        ];
        for (id, text) in specs {
            match hotkey::parse(text) {
                Some(spec) => {
                    if let Err(e) = hotkey::register(self.hwnd, id, &spec) {
                        logln!("hotkey '{text}': {e}");
                    }
                }
                None => logln!("invalid hotkey '{text}'"),
            }
        }
    }

    fn unregister_hotkeys(&self) {
        for id in [HOTKEY_TOGGLE, HOTKEY_INCREASE, HOTKEY_DECREASE] {
            hotkey::unregister(self.hwnd, id);
        }
    }

    fn tip(&self) -> String {
        if self.enabled {
            format!("Explorer Opacity - ON ({}%)", self.cfg.opacity)
        } else {
            "Explorer Opacity - OFF".to_string()
        }
    }

    fn apply_window(&mut self, hwnd: HWND) {
        if !self.enabled {
            return;
        }
        let id = window::id_of(hwnd);
        if self.alpha_states.contains_key(&id) {
            return;
        }
        let class = window::class_name(hwnd);
        if !crate::filter::matches_class(
            &class,
            &self.cfg.targets.include,
            &self.cfg.targets.exclude,
        ) {
            return;
        }

        let alpha = crate::filter::percent_to_alpha(self.cfg.opacity);
        match window::apply_alpha(hwnd, alpha) {
            Ok(state) => {
                self.alpha_states.insert(id, state);
                logln!("alpha={alpha} applied to {class}");
            }
            Err(e) => logln!("apply_alpha on {class} failed: {e}"),
        }
    }

    fn sweep(&mut self) {
        self.alpha_states
            .retain(|&id, _| unsafe { IsWindow(window::hwnd_of(id)) } != 0);

        if !self.enabled {
            return;
        }
        for hwnd in window::matching_windows(&self.cfg) {
            self.apply_window(hwnd);
        }
    }

    fn restore_all(&mut self) {
        for (id, state) in self.alpha_states.drain() {
            let hwnd = window::hwnd_of(id);
            if unsafe { IsWindow(hwnd) } != 0 {
                window::restore_alpha(hwnd, &state);
            }
        }
    }

    fn toggle(&mut self) {
        self.enabled = !self.enabled;
        if self.enabled {
            self.sweep();
        } else {
            self.restore_all();
        }
    }

    fn adjust_opacity(&mut self, delta: i32) {
        let next = (self.cfg.opacity as i32 + delta).clamp(MIN_OPACITY as i32, MAX_OPACITY as i32);
        self.set_opacity(next as u8);
    }

    fn set_opacity(&mut self, opacity: u8) {
        self.cfg.opacity = crate::filter::clamp_opacity(opacity);
        if self.enabled {
            let alpha = crate::filter::percent_to_alpha(self.cfg.opacity);
            for &id in self.alpha_states.keys() {
                window::set_layered_alpha(window::hwnd_of(id), alpha);
            }
        }
    }

    fn reload(&mut self) {
        let (cfg, err) = config::load_or_create();
        if let Some(e) = err {
            logln!("{e}");
        }
        self.restore_all();
        self.unregister_hotkeys();
        self.cfg = cfg;
        self.enabled = self.cfg.enabled;
        self.autostart = autostart::points_at_self();
        crate::logging::configure(&self.cfg.logging);
        self.register_hotkeys();
        if self.enabled {
            self.sweep();
        }
        logln!("config reloaded");
    }

    fn toggle_autostart(&mut self) {
        match autostart::toggle() {
            Ok(value) => {
                self.autostart = value;
                logln!("autostart={value}");
            }
            Err(e) => logln!("autostart toggle failed: {e}"),
        }
    }

    fn shutdown(&mut self) {
        self.restore_all();
        self.unregister_hotkeys();
        unsafe {
            KillTimer(self.hwnd, SWEEP_TIMER_ID);
        }
        tray::remove(self.hwnd);
        logln!("shutdown complete");
    }
}

fn with_app<R>(f: impl FnOnce(&mut App) -> R) -> Option<R> {
    APP.with(|cell| {
        let mut borrow = cell.borrow_mut();
        borrow.as_mut().map(f)
    })
}

fn update_tray_tip() {
    if let Some((hwnd, tip)) = with_app(|a| (a.hwnd, a.tip())) {
        tray::update_tip(hwnd, &tip);
    }
}

fn on_hotkey(id: i32) {
    match id {
        HOTKEY_TOGGLE => with_app(|a| a.toggle()),
        HOTKEY_INCREASE => with_app(|a| {
            let step = a.cfg.opacity_step as i32;
            a.adjust_opacity(step);
        }),
        HOTKEY_DECREASE => with_app(|a| {
            let step = a.cfg.opacity_step as i32;
            a.adjust_opacity(-step);
        }),
        _ => None,
    };
    update_tray_tip();
}

fn on_tray(hwnd: HWND, lparam: LPARAM) {
    logln!("tray event lparam={lparam:#x}");
    if !tray::is_menu_event(lparam) {
        return;
    }
    let Some((enabled, autostart, log_enabled)) =
        with_app(|a| (a.enabled, a.autostart, a.cfg.logging.enabled))
    else {
        return;
    };
    let Some(command) = tray::show_menu(hwnd, enabled, autostart, log_enabled) else {
        logln!("tray menu dismissed without selection");
        return;
    };
    logln!("tray command: {command:?}");
    apply_command(command);
}

fn apply_command(command: tray::MenuCommand) {
    use tray::MenuCommand::*;
    match command {
        Toggle => {
            with_app(|a| a.toggle());
        }
        OpacityUp => {
            with_app(|a| {
                let step = a.cfg.opacity_step as i32;
                a.adjust_opacity(step);
            });
        }
        OpacityDown => {
            with_app(|a| {
                let step = a.cfg.opacity_step as i32;
                a.adjust_opacity(-step);
            });
        }
        Reload => {
            with_app(|a| a.reload());
        }
        ToggleAutostart => {
            with_app(|a| a.toggle_autostart());
        }
        OpenLog => open_log(),
        Exit => {
            with_app(|a| a.shutdown());
            unsafe {
                PostQuitMessage(0);
            }
            return;
        }
    }
    update_tray_tip();
}

fn open_log() {
    let operation = wide("open");
    let path = wide(&crate::logging::log_path().to_string_lossy());
    unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            path.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        );
    }
}

fn on_explorer_restarted(hwnd: HWND) {
    let tip = with_app(|a| a.tip()).unwrap_or_default();
    tray::add(hwnd, &tip);
    unsafe {
        RegisterShellHookWindow(hwnd);
    }
    with_app(|a| a.sweep());
    logln!("explorer restarted: tray icon and shell hook re-registered");
}

fn on_window_created(created: HWND) {
    with_app(|a| a.apply_window(created));
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let (shell_msg, taskbar_msg) = APP.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|a| (a.shell_hook_msg, a.taskbar_created_msg))
            .unwrap_or((0, 0))
    });

    if msg == WM_HOTKEY {
        on_hotkey(wparam as i32);
        return 0;
    }
    if msg == WM_TIMER {
        with_app(|a| a.sweep());
        return 0;
    }
    if msg == tray::WM_TRAYICON {
        on_tray(hwnd, lparam);
        return 0;
    }
    if taskbar_msg != 0 && msg == taskbar_msg {
        on_explorer_restarted(hwnd);
        return 0;
    }
    if shell_msg != 0 && msg == shell_msg {
        if wparam as u32 == HSHELL_WINDOWCREATED {
            on_window_created(lparam as HWND);
        }
        return 0;
    }
    if msg == WM_CLOSE {
        with_app(|a| a.shutdown());
        PostQuitMessage(0);
        return 0;
    }
    if msg == WM_ENDSESSION {
        with_app(|a| a.shutdown());
        return 0;
    }
    if msg == WM_DESTROY {
        with_app(|a| a.shutdown());
        PostQuitMessage(0);
        return 0;
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Entry point: create the hidden window and run the message loop until exit.
pub fn run() {
    let (cfg, config_error) = config::load_or_create();
    crate::logging::configure(&cfg.logging);

    // Restore before an abort so a panic in the message loop cannot leave a
    // window permanently translucent.
    std::panic::set_hook(Box::new(|info| {
        crate::logging::write(&format!("panic: {info}"));
        APP.with(|cell| {
            if let Ok(mut borrow) = cell.try_borrow_mut() {
                if let Some(app) = borrow.as_mut() {
                    app.restore_all();
                }
            }
        });
    }));

    if let Some(e) = config_error {
        logln!("{e}");
    }

    unsafe {
        let module = GetModuleHandleW(std::ptr::null());

        let mutex_name = wide("Local\\explorer-opacity");
        let mutex = CreateMutexW(std::ptr::null(), 1, mutex_name.as_ptr());
        if mutex.is_null() {
            logln!("CreateMutexW failed; continuing without single-instance guard");
        } else if GetLastError() == ERROR_ALREADY_EXISTS {
            logln!("another instance is already running; exiting");
            CloseHandle(mutex);
            return;
        }

        let class_name = wide("ExplorerOpacityHidden");
        let wc = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: module,
            hIcon: tray::app_icon(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null(),
            lpszClassName: class_name.as_ptr(),
        };
        if RegisterClassW(&wc) == 0 {
            logln!("RegisterClassW failed");
            return;
        }

        let hwnd = CreateWindowExW(
            WS_EX_TOOLWINDOW,
            class_name.as_ptr(),
            std::ptr::null(),
            WS_POPUP,
            0,
            0,
            0,
            0,
            // A hidden top-level window (not HWND_MESSAGE): only top-level
            // windows receive broadcast messages such as TaskbarCreated, which
            // is required to recover after Explorer restarts.
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            module,
            std::ptr::null(),
        );
        if hwnd.is_null() {
            logln!("CreateWindowExW failed");
            return;
        }

        let shell_name = wide("SHELLHOOK");
        let shell_hook_msg = RegisterWindowMessageW(shell_name.as_ptr());
        let taskbar_name = wide("TaskbarCreated");
        let taskbar_created_msg = RegisterWindowMessageW(taskbar_name.as_ptr());

        APP.with(|cell| {
            *cell.borrow_mut() = Some(App::new(cfg, hwnd, shell_hook_msg, taskbar_created_msg));
        });
        with_app(|a| a.setup());

        let mut msg: MSG = core::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Belt and braces: restore in case WM_DESTROY did not run.
        with_app(|a| a.restore_all());
        if !mutex.is_null() {
            CloseHandle(mutex);
        }
    }
}
