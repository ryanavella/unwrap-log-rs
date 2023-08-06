//! Non-panicking alternatives to `Option` and `Result` unwrapping, which log at warn level.
//!
//! ## Example
//!
//! ```rust
//! use unwrap_log::{OptionExt, ResultExt};
//! use env_logger::Builder;
//! use log::LevelFilter::Warn;
//!
//! Builder::new().filter_level(Warn).init();
//!
//! let x: i32 = None.unwrap_or_default_log();
//! assert_eq!(x, 0);
//!
//! let y: i32 = Err("oops").unwrap_or_default_log();
//! assert_eq!(y, 0);
//! ```
//!
//! Output:
//! ```text
//! [1970-01-01T00:00:00Z WARN  my_crate] src\main.rs:8:23 encountered `None`
//! [1970-01-01T00:00:00Z WARN  my_crate] src\main.rs:11:30 encountered `Err("oops")`
//! ```
#![no_std]

/// Extension trait providing tracing alternatives to `Option` unwrap methods.
pub trait OptionExt {
    /// The type of the "present" output, intended to be `T` for a `Option<T>`.
    type Output;
    /// Returns the contained `Some` value, or logs at the warn level and returns a default value.
    fn unwrap_or_default_log(self) -> Self::Output;
    /// Returns the contained `Some` value, or logs at the warn level and computes a default value from a closure.
    fn unwrap_or_else_log(self, f: impl FnOnce() -> Self::Output) -> Self::Output;
    /// Returns the contained `Some` value, or logs at the warn level and returns the provided default.
    fn unwrap_or_log(self, default: Self::Output) -> Self::Output;
}

/// Extension trait providing tracing alternatives to `Result` unwrap methods.
pub trait ResultExt {
    /// The type of the "successful" output, intended to be `T` for a `Result<T, E>`.
    type Output;
    /// Returns the contained `Ok` value, or logs at the warn level and returns a default value.
    fn unwrap_or_default_log(self) -> Self::Output;
    /// Returns the contained `Ok` value, or logs at the warn level and computes a default value from a closure.
    fn unwrap_or_else_log(self, f: impl FnOnce() -> Self::Output) -> Self::Output;
    /// Returns the contained `Ok` value, or logs at the warn level and returns the provided default.
    fn unwrap_or_log(self, default: Self::Output) -> Self::Output;
}

/// Like `ResultExt` for `Result<T, E>`, but doesn't require `E: Debug`.
///
/// This is provided for users who want to avoid logging sensitive information,
/// or who want to slim down their log files.
pub trait ResultExtNoDbg {
    /// The type of the "successful" output, intended to be `T` for a `Result<T, E>`.
    type Output;
    /// Returns the contained `Ok` value, or logs at the warn level and returns a default value.
    fn unwrap_or_default_log(self) -> Self::Output;
    /// Returns the contained `Ok` value, or logs at the warn level and computes a default value from a closure.
    fn unwrap_or_else_log(self, f: impl FnOnce() -> Self::Output) -> Self::Output;
    /// Returns the contained `Ok` value, or logs at the warn level and returns the provided default.
    fn unwrap_or_log(self, default: Self::Output) -> Self::Output;
}

impl<T: Default> OptionExt for Option<T> {
    type Output = T;

    #[track_caller]
    fn unwrap_or_default_log(self) -> T {
        if let Some(x) = self {
            x
        } else {
            option_error();
            T::default()
        }
    }

    #[track_caller]
    fn unwrap_or_else_log(self, f: impl FnOnce() -> T) -> T {
        if let Some(x) = self {
            x
        } else {
            option_error();
            f()
        }
    }

    #[track_caller]
    fn unwrap_or_log(self, default: T) -> T {
        if let Some(x) = self {
            x
        } else {
            option_error();
            default
        }
    }
}

impl<T: Default, E: core::fmt::Debug> ResultExt for Result<T, E> {
    type Output = T;

    #[track_caller]
    fn unwrap_or_default_log(self) -> T {
        match self {
            Ok(x) => x,
            Err(err) => {
                result_error(&err);
                T::default()
            }
        }
    }

    #[track_caller]
    fn unwrap_or_else_log(self, f: impl FnOnce() -> T) -> T {
        match self {
            Ok(x) => x,
            Err(err) => {
                result_error(&err);
                f()
            }
        }
    }

    #[track_caller]
    fn unwrap_or_log(self, default: T) -> T {
        match self {
            Ok(x) => x,
            Err(err) => {
                result_error(&err);
                default
            }
        }
    }
}

impl<T: Default, E> ResultExtNoDbg for Result<T, E> {
    type Output = T;

    #[track_caller]
    fn unwrap_or_default_log(self) -> T {
        if let Ok(x) = self {
            x
        } else {
            no_dbg_error();
            T::default()
        }
    }

    #[track_caller]
    fn unwrap_or_else_log(self, f: impl FnOnce() -> T) -> T {
        if let Ok(x) = self {
            x
        } else {
            no_dbg_error();
            f()
        }
    }

    #[track_caller]
    fn unwrap_or_log(self, default: T) -> T {
        if let Ok(x) = self {
            x
        } else {
            no_dbg_error();
            default
        }
    }
}

#[cold]
#[inline(never)]
#[track_caller]
fn option_error() {
    let caller = core::panic::Location::caller();
    log::warn!("{caller} encountered `None`");
}

#[cold]
#[inline(never)]
#[track_caller]
fn result_error(err: &dyn core::fmt::Debug) {
    let caller = core::panic::Location::caller();
    log::warn!("{caller} encountered `Err({err:?})`");
}

#[cold]
#[inline(never)]
#[track_caller]
fn no_dbg_error() {
    let caller = core::panic::Location::caller();
    log::warn!("{caller} encountered `Err(_)`");
}
