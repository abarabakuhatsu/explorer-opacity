//! Configuration file handling (`explorer-opacity.toml`, portable: stored next
//! to the executable).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Bumped whenever the on-disk schema changes in an incompatible way.
pub const CONFIG_VERSION: u32 = 1;
pub const DEFAULT_OPACITY: u8 = 85;
pub const DEFAULT_OPACITY_STEP: u8 = 5;
/// Lower bound is non-zero on purpose: `0` would render a window invisible and
/// impossible to recover without restarting Explorer.
pub const MIN_OPACITY: u8 = 10;
pub const MAX_OPACITY: u8 = 100;
pub const DEFAULT_CONFIG_FILE: &str = "explorer-opacity.toml";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hotkeys {
    #[serde(default = "default_toggle_hotkey")]
    pub toggle: String,
    #[serde(default = "default_increase_hotkey")]
    pub increase: String,
    #[serde(default = "default_decrease_hotkey")]
    pub decrease: String,
}

fn default_toggle_hotkey() -> String {
    "Ctrl+Alt+T".to_string()
}
fn default_increase_hotkey() -> String {
    "Ctrl+Alt+Up".to_string()
}
fn default_decrease_hotkey() -> String {
    "Ctrl+Alt+Down".to_string()
}

impl Default for Hotkeys {
    fn default() -> Self {
        Self {
            toggle: default_toggle_hotkey(),
            increase: default_increase_hotkey(),
            decrease: default_decrease_hotkey(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Targets {
    #[serde(default = "default_include")]
    pub include: Vec<String>,
    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,
}

fn default_include() -> Vec<String> {
    vec!["CabinetWClass".to_string()]
}

fn default_exclude() -> Vec<String> {
    vec![
        "Progman".to_string(),
        "WorkerW".to_string(),
        "Shell_TrayWnd".to_string(),
        "TaskManagerWindow".to_string(),
    ]
}

impl Default for Targets {
    fn default() -> Self {
        Self {
            include: default_include(),
            exclude: default_exclude(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Logging {
    /// When `false`, no log file is written at all.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Log destination. Empty means `<exe dir>\explorer-opacity.log`.
    #[serde(default)]
    pub path: String,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            enabled: true,
            path: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub config_version: u32,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_opacity")]
    pub opacity: u8,
    #[serde(default = "default_step")]
    pub opacity_step: u8,
    #[serde(default)]
    pub hotkeys: Hotkeys,
    #[serde(default)]
    pub targets: Targets,
    #[serde(default)]
    pub logging: Logging,
}

fn default_version() -> u32 {
    CONFIG_VERSION
}
fn default_true() -> bool {
    true
}
fn default_opacity() -> u8 {
    DEFAULT_OPACITY
}
fn default_step() -> u8 {
    DEFAULT_OPACITY_STEP
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_version: CONFIG_VERSION,
            enabled: true,
            opacity: DEFAULT_OPACITY,
            opacity_step: DEFAULT_OPACITY_STEP,
            hotkeys: Hotkeys::default(),
            targets: Targets::default(),
            logging: Logging::default(),
        }
    }
}

impl Config {
    /// Normalize values that may have been edited by hand.
    pub fn sanitize(&mut self) {
        self.opacity = crate::filter::clamp_opacity(self.opacity);
        self.opacity_step = self.opacity_step.clamp(1, 50);
        self.logging.path = self.logging.path.trim().to_string();
        if self.targets.include.is_empty() && self.targets.exclude.is_empty() {
            // Both empty would mean "apply to every window". Restore the safe
            // default instead of surprising the user.
            self.targets = Targets::default();
        }
    }

    /// Parse a config from a TOML string, applying defaults and sanitization.
    pub fn from_toml_str(text: &str) -> Result<Self, toml::de::Error> {
        let mut cfg: Config = toml::from_str(text)?;
        cfg.sanitize();
        Ok(cfg)
    }

    /// Serialize the config to a canonical TOML document (one key per line).
    pub fn to_toml_string(&self) -> String {
        toml::to_string_pretty(self).unwrap_or_else(|_| String::new())
    }
}

/// Directory that holds the config file (next to the executable for portability).
pub fn config_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Full path to the config file.
pub fn config_path() -> PathBuf {
    config_dir().join(DEFAULT_CONFIG_FILE)
}

/// Load the config without touching the filesystem beyond reading.
///
/// Missing file yields the defaults with no error; a parse error yields the
/// defaults plus a message and leaves the broken file untouched.
pub fn load() -> (Config, Option<String>) {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(text) => match Config::from_toml_str(&text) {
            Ok(cfg) => (cfg, None),
            Err(e) => (
                Config::default(),
                Some(format!("invalid config {}: {e}", path.display())),
            ),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (Config::default(), None),
        Err(e) => (
            Config::default(),
            Some(format!("could not read {}: {e}", path.display())),
        ),
    }
}

/// Load the config, creating a default file when missing.
pub fn load_or_create() -> (Config, Option<String>) {
    let path = config_path();
    if path.exists() {
        return load();
    }
    let cfg = Config::default();
    let text = cfg.to_toml_string();
    if text.is_empty() {
        return (cfg, Some("failed to serialize default config".to_string()));
    }
    if let Err(e) = std::fs::write(&path, text) {
        return (
            cfg,
            Some(format!("could not write {}: {e}", path.display())),
        );
    }
    (cfg, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sane() {
        let c = Config::default();
        assert_eq!(c.opacity, 85);
        assert!(c.enabled);
        assert!(c.targets.include.contains(&"CabinetWClass".to_string()));
        assert_eq!(c.targets.exclude.len(), 4);
    }

    #[test]
    fn logging_defaults_to_enabled_and_exe_dir() {
        let c = Config::default();
        assert!(c.logging.enabled);
        assert!(c.logging.path.is_empty());
    }

    #[test]
    fn logging_section_parses_and_trims() {
        let text = r#"
[logging]
enabled = false
path = "  C:\\logs\\eo.log  "
"#;
        let c = Config::from_toml_str(text).unwrap();
        assert!(!c.logging.enabled);
        assert_eq!(c.logging.path, r"C:\logs\eo.log");
    }

    #[test]
    fn sanitize_clamps_opacity() {
        let mut c = Config {
            opacity: 0,
            opacity_step: 0,
            ..Config::default()
        };
        c.sanitize();
        assert_eq!(c.opacity, MIN_OPACITY);
        assert_eq!(c.opacity_step, 1);
    }

    #[test]
    fn both_lists_empty_restores_defaults() {
        let mut c = Config::default();
        c.targets.include.clear();
        c.targets.exclude.clear();
        c.sanitize();
        assert!(!c.targets.include.is_empty());
    }

    #[test]
    fn toml_roundtrip() {
        let c = Config::default();
        let text = c.to_toml_string();
        let back = Config::from_toml_str(&text).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn legacy_mode_keys_are_ignored() {
        // Older configs may still contain mode/[mode_options]; they must not
        // make parsing fail now that the backdrop mode has been removed.
        let text = r#"
enabled = true
mode = "backdrop"
opacity = 90
[mode_options]
backdrop = "mica-alt"
"#;
        let c = Config::from_toml_str(text).unwrap();
        assert_eq!(c.opacity, 90);
        assert!(c.enabled);
    }

    #[test]
    fn partial_config_fills_defaults() {
        let text = "opacity = 70\n";
        let c = Config::from_toml_str(text).unwrap();
        assert_eq!(c.opacity, 70);
        assert_eq!(c.opacity_step, DEFAULT_OPACITY_STEP);
        assert_eq!(c.hotkeys.toggle, "Ctrl+Alt+T");
    }
}
