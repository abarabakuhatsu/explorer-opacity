//! Pure window-class filtering rules.
//!
//! Matching semantics (documented in the README):
//! 1. If the class is listed in `exclude`, it is never touched.
//! 2. Otherwise, if `include` is non-empty, only classes listed in `include`
//!    match.
//! 3. If `include` is empty, every class that is not excluded matches.

/// Returns `true` when a top-level window of `class_name` should be managed.
pub fn matches_class(class_name: &str, include: &[String], exclude: &[String]) -> bool {
    if exclude.iter().any(|c| c == class_name) {
        return false;
    }
    if include.is_empty() {
        return true;
    }
    include.iter().any(|c| c == class_name)
}

/// Clamp a raw configured opacity percentage into the safe range.
///
/// `0` would make a window fully invisible and impossible to recover by hand,
/// so the minimum is deliberately non-zero.
pub fn clamp_opacity(percent: u8) -> u8 {
    percent.clamp(crate::config::MIN_OPACITY, crate::config::MAX_OPACITY)
}

/// Convert an opacity percentage (inclusive 0..=100) into the layered-window
/// alpha byte (`0..=255`).
pub fn percent_to_alpha(percent: u8) -> u8 {
    let p = percent.min(100) as u32;
    ((p * 255 + 50) / 100) as u8
}

/// Convert a layered-window alpha byte back into a rounded percentage.
pub fn alpha_to_percent(alpha: u8) -> u8 {
    ((alpha as u32 * 100 + 127) / 255) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn exclude_wins_over_include() {
        let include = s(&["CabinetWClass"]);
        let exclude = s(&["Progman"]);
        assert!(!matches_class("Progman", &include, &exclude));
        assert!(matches_class("CabinetWClass", &include, &exclude));
        assert!(!matches_class("Notepad", &include, &exclude));
    }

    #[test]
    fn empty_include_matches_everything_not_excluded() {
        let include: Vec<String> = vec![];
        let exclude = s(&["Shell_TrayWnd"]);
        assert!(matches_class("Anything", &include, &exclude));
        assert!(!matches_class("Shell_TrayWnd", &include, &exclude));
    }

    #[test]
    fn clamp_keeps_no_invisible_window() {
        assert_eq!(clamp_opacity(0), crate::config::MIN_OPACITY);
        assert_eq!(clamp_opacity(100), 100);
        assert_eq!(clamp_opacity(85), 85);
    }

    #[test]
    fn percent_alpha_roundtrip_is_stable() {
        for p in [10u8, 25, 50, 85, 100] {
            let a = percent_to_alpha(p);
            assert_eq!(alpha_to_percent(a), p, "percent {p} -> alpha {a}");
        }
        assert_eq!(percent_to_alpha(100), 255);
        assert_eq!(percent_to_alpha(0), 0);
    }
}
