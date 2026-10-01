//! Thin Win32 layer.
//!
//! `HWND` is an opaque handle passed by value to the OS; the functions in this
//! module never dereference it as a pointer, so the
//! `clippy::not_unsafe_ptr_arg_deref` lint does not apply.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub mod autostart;
pub mod hook;
pub mod hotkey;
pub mod tray;
pub mod window;
