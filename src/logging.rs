//! Minimal, dependency-free file logger.
//!
//! The destination is configurable (`[logging]`). By default the log is written
//! next to the executable; logging can be disabled entirely. Logging is best
//! effort: a failure to write must never take the process down.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::Logging;

/// Rotate (truncate) the log once it grows past this size.
const MAX_LOG_BYTES: u64 = 1_048_576;
pub const LOG_FILE_NAME: &str = "explorer-opacity.log";

struct LogState {
    enabled: bool,
    path: Option<PathBuf>,
    file: Option<File>,
}

static STATE: OnceLock<Mutex<LogState>> = OnceLock::new();

fn state() -> &'static Mutex<LogState> {
    STATE.get_or_init(|| {
        Mutex::new(LogState {
            enabled: false,
            path: None,
            file: None,
        })
    })
}

/// Directory containing the executable (the base for the default log path).
pub fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Resolve a configured log destination into a concrete file path.
///
/// * empty -> `<exe_dir>\explorer-opacity.log`
/// * relative -> relative to `exe_dir`
/// * `%VAR%` -> expanded from the environment
/// * trailing separator or existing directory -> append `explorer-opacity.log`
/// * otherwise -> used as a file path
pub fn resolve_log_path(exe_dir: &Path, raw: &str) -> PathBuf {
    let expanded = expand_env(raw.trim());
    if expanded.is_empty() {
        return exe_dir.join(LOG_FILE_NAME);
    }
    let candidate = PathBuf::from(&expanded);
    let mut full = if candidate.is_absolute() {
        candidate
    } else {
        exe_dir.join(candidate)
    };
    let looks_like_dir = expanded.ends_with('\\') || expanded.ends_with('/') || full.is_dir();
    if looks_like_dir {
        full.push(LOG_FILE_NAME);
    }
    full
}

/// Expand Windows-style `%NAME%` environment references, leaving unknown ones
/// untouched.
fn expand_env(raw: &str) -> String {
    let parts: Vec<&str> = raw.split('%').collect();
    let mut out = String::new();
    for (idx, part) in parts.iter().enumerate() {
        if idx % 2 == 0 {
            out.push_str(part);
        } else if part.is_empty() {
            out.push('%');
        } else if let Ok(value) = std::env::var(part) {
            out.push_str(&value);
        } else {
            out.push('%');
            out.push_str(part);
            out.push('%');
        }
    }
    out
}

/// (Re)configure the logger from the `[logging]` section.
///
/// Safe to call again on config reload; a disabled logger closes any open file.
pub fn configure(cfg: &Logging) {
    let resolved = resolve_log_path(&exe_dir(), &cfg.path);
    {
        if let Ok(mut st) = state().lock() {
            st.file = None;
            st.enabled = cfg.enabled;
            st.path = Some(resolved.clone());
            if !cfg.enabled {
                return;
            }
            if let Some(parent) = resolved.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if std::fs::metadata(&resolved).map(|m| m.len()).unwrap_or(0) > MAX_LOG_BYTES {
                let _ = std::fs::remove_file(&resolved);
            }
            st.file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&resolved)
                .ok();
        }
    }
    if cfg.enabled {
        write("logger initialized");
    }
}

/// Append one line to the log (no-op when logging is disabled).
pub fn write(message: &str) {
    if let Ok(mut guard) = state().lock() {
        if let Some(file) = guard.file.as_mut() {
            let _ = writeln!(file, "{} {}", timestamp(), message);
            let _ = file.flush();
        }
    }
}

/// Whether logging is currently enabled.
pub fn is_enabled() -> bool {
    state().lock().map(|s| s.enabled).unwrap_or(false)
}

/// The effective log file path (resolved, or the default when not configured).
pub fn log_path() -> PathBuf {
    if let Ok(guard) = state().lock() {
        if let Some(path) = guard.path.as_ref() {
            return path.clone();
        }
    }
    resolve_log_path(&exe_dir(), "")
}

/// Convenience macro that formats and logs a line.
#[macro_export]
macro_rules! logln {
    ($($arg:tt)*) => {
        $crate::logging::write(&format!($($arg)*))
    };
}

fn timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}Z",
        y,
        m,
        d,
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Howard Hinnant's `civil_from_days` (proleptic Gregorian, days since epoch).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_epoch_and_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_723), (2024, 1, 1));
    }

    #[test]
    fn empty_path_defaults_next_to_exe() {
        let base = Path::new(r"C:\app");
        assert_eq!(resolve_log_path(base, ""), base.join(LOG_FILE_NAME));
        assert_eq!(resolve_log_path(base, "   "), base.join(LOG_FILE_NAME));
    }

    #[test]
    fn relative_path_is_relative_to_exe() {
        let base = Path::new(r"C:\app");
        assert_eq!(
            resolve_log_path(base, r"logs\eo.log"),
            base.join(r"logs\eo.log")
        );
    }

    #[test]
    fn trailing_separator_is_treated_as_directory() {
        let base = Path::new(r"C:\app");
        assert_eq!(
            resolve_log_path(base, r"logs\"),
            base.join("logs").join(LOG_FILE_NAME)
        );
    }

    #[test]
    fn absolute_path_is_kept() {
        let base = Path::new(r"C:\app");
        assert_eq!(
            resolve_log_path(base, r"D:\out\eo.log"),
            PathBuf::from(r"D:\out\eo.log")
        );
    }

    #[test]
    fn env_vars_are_expanded() {
        std::env::set_var("EO_TEST_LOG_DIR", r"C:\expanded");
        let base = Path::new(r"C:\app");
        assert_eq!(
            resolve_log_path(base, r"%EO_TEST_LOG_DIR%\eo.log"),
            PathBuf::from(r"C:\expanded\eo.log")
        );
        std::env::remove_var("EO_TEST_LOG_DIR");
    }

    #[test]
    fn unknown_env_vars_are_left_literal() {
        let base = Path::new(r"C:\app");
        assert_eq!(
            resolve_log_path(base, r"a%NOPE_NOT_SET%b"),
            base.join("a%NOPE_NOT_SET%b")
        );
    }
}
