//! Defines the OS Layper sync API.
//!
//! Every OS must provide the following features
//! 
//! - Complete mechanism
//!  must supply a stuct called OslCompletion and implement GeneralComplete
//!

use crate::error::Result;

/// Complete trait that os must implement
pub trait GeneralComplete:Default {
    /// complete init
    fn init(&mut self);
    /// complete reinit
    fn reinit(&mut self);
    /// wait completion finish
    /// timeout: seconds
    fn wait_for_completion_timeout(&mut self, timeout: u32) -> Result<()>;
    /// wait unitil complete
    fn wait_for_completion(&mut self);
    /// finish complete
    fn complete(&mut self);
}

#[cfg(feature = "linux")]
pub use crate::linux::complete::*;
