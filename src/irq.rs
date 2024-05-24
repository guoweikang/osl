//! Defines the OS Layper general irq.
//!
//! Every OS should provide Return type and an ReturnEnum conversion method
//! such as the following implementation on Linux.
//!
//!
//!

/// The general irq return type.
/// The return value from interrupt handlers.
///
///
pub enum ReturnEnum {
    /// The interrupt was not from this device or was not handled.
    None,
    /// The interrupt was handled by this device.
    Handled,
    /// The handler wants the handler thread to wake up.
    WakeThread,
}

#[cfg(feature = "linux")]
pub use crate::linux::irq::Return;

/// Give an errno, return OS Error
pub fn to_irq_return(val: ReturnEnum) -> Return {
    Return::from(val)
}
