//! This is an OS interface abstraction layer developed for cross-kernel drivers. 
//!
//! Include: 
//! - error: error type use by drivers
//!

#![no_std]

#[cfg(feature="linux")]
mod linux;

pub mod error;
