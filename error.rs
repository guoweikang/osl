//! Defines the OS Layper general error type.
//! 
//! Every OS should provide Error type and an error code conversion method
//! such as the following implementation on Linux.
//!
//! # Examples
//! 
//! ```
//! use kernel::prelude::error{Error,code};
//!
//! impl From<crate::error::Errno> for Error {
//!     fn from(errno: crate::error::Errno) -> Self {
//!         match errno {
//!             crate::error::Errno::InvalidArgs => code::EINVAL,
//!             ...
//!         }
//!     }
//! }
//! ```
//!

/// The general error type.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Errno {
    /// Invalid arguments.
    InvalidArgs,
}

#[cfg(feature="linux")]
pub use crate::linux::error::*;

/// A [`Result`] with an [`Error`] error type.
pub type Result<T> = core::result::Result<T, Error>;
