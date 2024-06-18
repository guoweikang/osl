//! Defines the OS Layper sync API.
//!
//! Every OS must provide the following features
//!
//! - Complete mechanism:
//!  must supply a stuct called OslCompletion and implement GeneralComplete
//!
//! - SpinLock
//!  must supply a stuct called SpinLock and implement lock
//!
//! - Arc
//!

use crate::error::Result;

cfg_if::cfg_if! {
    if #[cfg(feature = "linux")] {
        pub use crate::linux::complete::*;
        pub use kernel::sync::Arc;
        pub use kernel::{new_spinlock, sync::SpinLock}
    } else if  #[cfg(feature = "arceos")] {
        pub use alloc::sync::Arc;
        pub use crate::new_spinlock;
        pub use crate::arceos::sync::SpinLock;
        pub use crate::arceos::complete::*;
    }
}

/// Complete trait that os must implement
pub trait GeneralComplete {
    /// Complete use own lock protect
    /// So here use Arc pointer as result
    fn new() -> Result<Arc<Self>>;
    /// complete reinit
    fn reinit(&self);
    /// wait completion finish
    /// timeout: seconds
    fn wait_for_completion_timeout(&self, timeout: u32) -> Result<()>;
    /// wait unitil complete
    fn wait_for_completion(&self);
    /// finish complete
    fn complete(&self);
}
