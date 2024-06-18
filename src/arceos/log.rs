pub use axlog::{debug, error, info, warn}; 

#[doc(hidden)]
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! log_print (
    ($crate::log::LogLevel::Error, $($arg:tt)*) => (
        $crate::log::error!($($arg)*)
    );
    ($crate::log::LogLevel::Warn, $($arg:tt)*) => (
        $crate::log::warn!($($arg)*)
    );
    ($crate::log::LogLevel::Info, $($arg:tt)*) => (
        $crate::log::info!($($arg)*)
    );
    ($crate::log::LogLevel::Debug, $($arg:tt)*) => (
        $crate::log::debug!($($arg)*)
    );
);
