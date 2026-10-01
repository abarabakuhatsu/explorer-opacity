use explorer_opacity::win::{autostart, hook};

fn print_help() {
    println!(
        "\
explorer-opacity {}

Usage:
  explorer-opacity                 Run the resident tool (default)
  explorer-opacity install-autostart
  explorer-opacity uninstall-autostart
  explorer-opacity status
  explorer-opacity --version

Configuration is read from explorer-opacity.toml next to the executable.",
        env!("CARGO_PKG_VERSION")
    );
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();
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
        "status" => {
            println!("autostart entry present: {}", autostart::is_enabled());
            println!(
                "autostart points at this exe: {}",
                autostart::points_at_self()
            );
            println!("log: {}", explorer_opacity::logging::log_path().display());
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
