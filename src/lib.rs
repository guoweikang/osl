//! This is an OS interface abstraction layer developed for cross-kernel drivers.
//!
//! Include:
//! - error: error type used by drivers
//! - log: log interface used by drivers

#![no_std]

#[cfg(feature = "linux")]
mod linux;

pub mod error;
pub mod log;
pub mod sleep;
pub mod time;
