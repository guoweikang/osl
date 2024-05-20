//! Defines the OS Layper general irq.
//!

/// The general irq return type.
/// The return value from interrupt handlers.
pub enum Return {
    /// The interrupt was not from this device or was not handled.
    None,
    /// The interrupt was handled by this device.
    Handled,
    /// The handler wants the handler thread to wake up.
    WakeThread,
}
