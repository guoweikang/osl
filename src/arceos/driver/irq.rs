//! Linux irq implementation
//!

/// Arceos now didn't check irq return
pub type Return = ();

impl From<crate::driver::irq::ReturnEnum> for Return {
    fn from(ret: crate::driver::irq::ReturnEnum) -> Self {
        match ret {
            crate::driver::irq::ReturnEnum::None => (),
            crate::driver::irq::ReturnEnum::Handled => (),
            crate::driver::irq::ReturnEnum::WakeThread => (),
        }
    }
}
