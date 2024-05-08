//! Defines the OS Layper log API.
//!
//! Every OS should provide log_print func
//!
//! # Examples
//!
//! ```
//! pub fn log_print(level: LogLevel, args: fmt::Arguments<'_>) {
//!     unsafe {
//!         match  level {
//!             LogLevel::Error =>  call_printk(&format_strings::ERR, __LOG_PREFIX, args),
//!             LogLevel::Warn => call_printk(&format_strings::WARNING, __LOG_PREFIX, args),
//!             LogLevel::Info => call_printk(&format_strings::INFO, __LOG_PREFIX, args),
//!             LogLevel::Debug => call_printk(&format_strings::DEBUG, __LOG_PREFIX, args),
//!         }
//!     }
//! }
//!
//! ```
//!
//!# Use Example
//!
//! ```
//! #[macro_use]
//! extern crate osl;
//!
//! fn func() {
//!     log_info!("Hello!")
//! }
//!

#[cfg(feature = "linux")]
pub use crate::linux::log::*;

/// Log Level
pub enum LogLevel {
    /// The "error" level
    Error,
    /// The "warn" level.
    Warn,
    /// The "info" level.
    Info,
    /// The "debug" level.
    Debug,
}


/// Prints an error-level message.
///
/// Use this level for informational messages.
///
/// # Examples
///
/// ```
/// log_err!("hello {}\n", "there");
/// ```
#[macro_export]
macro_rules! log_err (
    ($($arg:tt)*) => (
        $crate::log::log_print($crate::log::LogLevel::Error, format_args!($($arg)*))
    )
);

/// Prints an warn-level message.
///
/// Use this level for informational messages.
///
/// # Examples
///
/// ```
/// log_warn!("hello {}\n", "there");
/// ```
#[macro_export]
macro_rules! log_warn (
    ($($arg:tt)*) => (
        $crate::log::log_print($crate::log::LogLevel::Warn, format_args!($($arg)*))
    )
);

/// Prints an info-level message.
///
/// Use this level for informational messages.
///
/// # Examples
///
/// ```
/// log_info!("hello {}\n", "there");
/// ```
#[macro_export]
macro_rules! log_info (
    ($($arg:tt)*) => (
        $crate::log::log_print($crate::log::LogLevel::Info, format_args!($($arg)*))
    )
);

/// Prints an info-level message.
///
/// Use this level for informational messages.
///
/// # Examples
///
/// ```
/// log_debug!("hello {}\n", "there");
/// ```
#[macro_export]
macro_rules! log_debug (
    ($($arg:tt)*) => (
        $crate::log::log_print($crate::log::LogLevel::Debug, format_args!($($arg)*))
    )
);
