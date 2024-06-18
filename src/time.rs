//! Defines the OS Layper time API.
//!
//! Every OS should provide these time func
//!
//! /// Return current_time of ns
//! fn current_time() -> u64
//! /// Return current_time add  us, return ns
//! fn time_add_us(us: u64) -> u64;
//!
//! # Use example
//! ```
//! fn func() {
//!     osl::time::current_time_us();
//! }
//! ```

/// One usec to nsec
pub const NSEC_PER_USEC: u64 = 1000;

cfg_if::cfg_if! {
    if #[cfg(feature = "linux")] {
        pub use crate::linux::time::*;
    } else if  #[cfg(feature = "arceos")] {
        pub use crate::arceos::time::*;
    }
}
