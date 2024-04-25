use core::fmt;
use kernel::print::format_strings;
use kernel::print::call_printk;

use crate::log::LogLevel;

/// Prefix to appear before log messages printed from within the `kernel` crate.
const __LOG_PREFIX: &[u8] = b"osl_log\0";

/// refer osl::log
pub fn log_print(level: LogLevel, args: fmt::Arguments<'_>) {
    unsafe {
        match  level {
            LogLevel::Error =>  call_printk(&format_strings::ERR, __LOG_PREFIX, args),
            LogLevel::Warn => call_printk(&format_strings::WARNING, __LOG_PREFIX, args),
            LogLevel::Info => call_printk(&format_strings::INFO, __LOG_PREFIX, args),
            LogLevel::Debug => call_printk(&format_strings::DEBUG, __LOG_PREFIX, args),
        }
    }
}
