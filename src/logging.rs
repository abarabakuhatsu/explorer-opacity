//! Minimal, dependency-free file logger.
//!
//! Logs go to `%LOCALAPPDATA%\explorer-opacity\explorer-opacity.log`. Logging
//! is best effort: a failure to write must never take the process down.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static SINK: OnceLock<Mutex<Option<File>>> = OnceLock::new();

/// Rotate (truncate) the log once it grows past this size.
const MAX_LOG_BYTES: u64 = 1_048_576;

/// Directory containing the log file.
pub fn log_dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join("explorer-opacity")
}

/// Full path to the log file.
pub fn log_path() -> PathBuf {
    log_dir().join("explorer-opacity.log")
}

/// Initialize the logger. Safe to call more than once.
pub fn init() {
    SINK.get_or_init(|| {
        let path = log_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0) > MAX_LOG_BYTES {
            let _ = std::fs::remove_file(&path);
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .ok();
        Mutex::new(file)
    });
    write("logger initialized");
}

/// Append one line to the log.
pub fn write(message: &str) {
    let sink = SINK.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = sink.lock() {
        if let Some(file) = guard.as_mut() {
            let _ = writeln!(file, "{} {}", timestamp(), message);
            let _ = file.flush();
        }
    }
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
    fn log_path_is_under_localappdata() {
        assert!(log_path()
            .to_string_lossy()
            .to_lowercase()
            .contains("explorer-opacity"));
    }
}
