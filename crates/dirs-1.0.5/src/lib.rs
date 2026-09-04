//! The _dirs_ crate is
//!
//! - a tiny library with a minimal API (16 functions)
//! - that provides the platform-specific, user-accessible locations
//! - for finding and storing configuration, cache and other data
//! - on Linux, Windows (≥ Vista) and macOS.
//!
//! The library provides the location of these directories by leveraging the mechanisms defined by
//!
//! - the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications on Linux,
//! - the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911(v=vs.85).aspx) system on Windows, and
//! - the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6) on macOS.

#![allow
(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unexpected_cfgs,
    unused_imports,
)]

pub mod path
{
    pub use std::path::{ * };
}

#[cfg(target_os = "windows")] mod win;

#[cfg(target_os = "windows")]                                                     use win as sys;
#[cfg(target_os = "macos")]                                                       use mac as sys;
#[cfg(target_os = "redox")]                                                       use redox as sys;
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "redox")))] use lin as sys;

/// Returns the path to the user's home directory.
pub fn home_dir() -> Option<path::PathBuf> {
    sys::home_dir()
}
/// Returns the path to the user's cache directory.
pub fn cache_dir() -> Option<path::PathBuf> {
    sys::cache_dir()
}
/// Returns the path to the user's config directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value from the following table, or a `None`.
pub fn config_dir() -> Option<path::PathBuf> {
    sys::config_dir()
}
/// Returns the path to the user's data directory.
pub fn data_dir() -> Option<path::PathBuf> {
    sys::data_dir()
}
/// Returns the path to the user's local data directory.
pub fn data_local_dir() -> Option<path::PathBuf> {
    sys::data_local_dir()
}
/// Returns the path to the user's executable directory.
pub fn executable_dir() -> Option<path::PathBuf> {
    sys::executable_dir()
}
/// Returns the path to the user's runtime directory.
pub fn runtime_dir() -> Option<path::PathBuf> {
    sys::runtime_dir()
}
/// Returns the path to the user's audio directory.
pub fn audio_dir() -> Option<path::PathBuf> {
    sys::audio_dir()
}
/// Returns the path to the user's desktop directory.
pub fn desktop_dir() -> Option<path::PathBuf> {
    sys::desktop_dir()
}
/// Returns the path to the user's document directory.
pub fn document_dir() -> Option<path::PathBuf> {
    sys::document_dir()
}
/// Returns the path to the user's download directory.
pub fn download_dir() -> Option<path::PathBuf> {
    sys::download_dir()
}
/// Returns the path to the user's font directory.
pub fn font_dir() -> Option<path::PathBuf> {
    sys::font_dir()
}
/// Returns the path to the user's picture directory.
pub fn picture_dir() -> Option<path::PathBuf> {
    sys::picture_dir()
}
/// Returns the path to the user's public directory.
pub fn public_dir() -> Option<path::PathBuf> {
    sys::public_dir()
}
/// Returns the path to the user's template directory.
pub fn template_dir() -> Option<path::PathBuf> {
    sys::template_dir()
}

/// Returns the path to the user's video directory.
pub fn video_dir() -> Option<path::PathBuf> {
    sys::video_dir()
}