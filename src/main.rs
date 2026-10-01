#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use explorer_opacity::win::{autostart, hook, window};

fn print_help() {
    println!(
        "\
explorer-opacity {}

Usage:
  explorer-opacity                 Run the resident tool (default)
  explorer-opacity install-autostart
  explorer-opacity uninstall-autostart
  explorer-opacity restore         Clear leftover transparency from Explorer windows
  explorer-opacity status
  explorer-opacity --version

Configuration is read from explorer-opacity.toml next to the executable.",
        env!("CARGO_PKG_VERSION")
    );
}

/// Attach to the console that launched this process so CLI subcommands print
/// there. When the exe is double-clicked there is no parent console, so nothing
/// happens and no window is shown.
fn attach_parent_console() {
    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
        use windows_sys::Win32::Storage::FileSystem::{
            CreateFileW, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE,
            OPEN_EXISTING,
        };
        use windows_sys::Win32::System::Console::{
            AttachConsole, GetStdHandle, SetStdHandle, ATTACH_PARENT_PROCESS, STD_ERROR_HANDLE,
            STD_OUTPUT_HANDLE,
        };

        // Respect an existing stdout (pipe / file redirection). Only bind to the
        // console when no usable handle was inherited.
        let existing = GetStdHandle(STD_OUTPUT_HANDLE);
        let has_stdout = !existing.is_null() && existing != INVALID_HANDLE_VALUE;

        if AttachConsole(ATTACH_PARENT_PROCESS) == 0 {
            return;
        }
        if has_stdout {
            return;
        }
        let name: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
        let handle = CreateFileW(
            name.as_ptr(),
            FILE_GENERIC_READ | FILE_GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null(),
            OPEN_EXISTING,
            0,
            std::ptr::null_mut(),
        );
        if handle != INVALID_HANDLE_VALUE {
            SetStdHandle(STD_OUTPUT_HANDLE, handle);
            SetStdHandle(STD_ERROR_HANDLE, handle);
        }
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();
    if !arg.is_empty() && arg != "run" {
        attach_parent_console();
    }

    match arg.as_str() {
        "" | "run" => hook::run(),
        "install-autostart" => match autostart::enable() {
            Ok(()) => println!("autostart enabled"),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        },
        "uninstall-autostart" => match autostart::disable() {
            Ok(()) => println!("autostart disabled"),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        },
        "restore" | "--restore" => {
            let (cfg, _) = explorer_opacity::config::load();
            let count = window::force_restore_matching(&cfg);
            println!("restored {count} window(s)");
        }
        "status" => {
            println!("autostart entry present: {}", autostart::is_enabled());
            println!(
                "autostart points at this exe: {}",
                autostart::points_at_self()
            );
            let (cfg, _) = explorer_opacity::config::load();
            let log_path = explorer_opacity::logging::resolve_log_path(
                &explorer_opacity::logging::exe_dir(),
                &cfg.logging.path,
            );
            println!("logging enabled: {}", cfg.logging.enabled);
            println!("log: {}", log_path.display());
            println!(
                "config: {}",
                explorer_opacity::config::config_path().display()
            );
        }
        "-h" | "--help" => print_help(),
        "-V" | "--version" => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        }
        other => {
            eprintln!("unknown argument: {other}\n");
            print_help();
            std::process::exit(2);
        }
    }
}
