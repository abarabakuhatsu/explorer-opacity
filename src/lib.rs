//! Explorer Opacity - keep File Explorer windows translucent on Windows.
//!
//! The crate is split into a pure, testable core ([`config`], [`filter`]) and a
//! thin Win32 layer ([`win`]) that deals with layered-window alpha.

pub mod config;
pub mod filter;
pub mod logging;
pub mod win;

pub use config::Config;
