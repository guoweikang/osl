pub use kernel::irq::Return;

impl From<crate::irq::ReturnEnum> for Return {
    fn from(ret: crate::irq::ReturnEnum) -> Self {
        match ret {
            crate::irq::ReturnEnum::None => Return::None,
            crate::irq::ReturnEnum::Handled => Return::Handled,
            crate::irq::ReturnEnum::WakeThread => Return::WakeThread,
        }
    }
}
