
use axsync::spin;

pub type  SpinLock<T> = spin::SpinNoPreempt<T>;

#[macro_export]
macro_rules! new_spinlock {
    ($inner:expr $(, $name:literal)? $(,)?) => {
        $crate::sync::SpinLock::new($inner)
    };
}
