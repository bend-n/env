//! Inspection and manipulation of the process's environment.
//!
//! This module contains functions to inspect various aspects such as
//! environment variables, process arguments, the current directory, and various
//! other important directories.
//!
//! There are several functions and structs in this module that have a
//! counterpart ending in `os`. Those ending in `os` will return an [`OsString`](std::ffi::OsString)
//! and those without will return a [`String`].
//!
//! This crate provides <code>[mod@std::env]::{[set_var], [remove_var]}</code> safely.
#![forbid(clippy::missing_safety_doc)]
pub use std::env::*;
use std::ffi::OsStr;

/// Whether the operating system has a thread-safe environment. This allows bypassing the check for if the process is multi-threaded.
const SAFE: bool = matches!(
    std::env::consts::OS.as_bytes(),
    b"illumos" | b"netbsd" | b"windows"
);

/// Sets the environment variable `key` to the value `value` for the currently running
/// process. (safely!)
///
/// # Safety
///
/// This function is safe to call.
/// This function returns [`None`] if it would be unsafe to call this function.
///
/// See also [`std::env::set_var`].
///
/// # Panics
///
/// This function may panic if `key` is empty, contains an ASCII equals sign `'='`
/// or the NUL character `'\0'`, or when `value` contains the NUL character.
///
/// # Examples
///
/// ```
/// let key = "KEY";
/// env::set_var(key, "VALUE").expect("ok");
/// assert_eq!(env::var(key), Ok("VALUE".to_string()));
/// ```
#[must_use = "this function may not always run"]
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) -> Option<()> {
    (SAFE || num_threads::is_single_threaded() == Some(true))
        .then(|| unsafe { std::env::set_var(key, value) })
}

/// Removes an environment variable from the environment of the currently running process. (safely!)
///
/// # Safety
///
/// This function is safe to call.
/// This function returns [`None`] if it would be unsafe to call this function.
///
/// See also [`std::env::remove_var`].
///
/// # Panics
///
/// This function may panic if `key` is empty, contains an ASCII equals sign
/// `'='` or the NUL character `'\0'`, or when the value contains the NUL
/// character.
///
/// # Examples
///
/// ```
/// let key = "KEY";
/// env::set_var(key, "VALUE").expect("ok");
/// assert_eq!(env::var(key), Ok("VALUE".to_string()));
/// env::remove_var(key).expect("ok");
/// assert!(env::var(key).is_err());
/// ```
#[must_use = "this function may not always run"]
pub fn remove_var<K: AsRef<OsStr>>(key: K) -> Option<()> {
    // SAFETY: this function is safe to call in single threaded environments or on good OS's.
    (SAFE || num_threads::is_single_threaded() == Some(true))
        .then(|| unsafe { std::env::remove_var(key) })
}
