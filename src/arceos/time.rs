use crate::time::NSEC_PER_USEC;
use axhal::time::{current_time_nanos};

/// Arceos time
pub fn time_add_us(us: u64) -> u64 {
    current_time_nanos() + us * NSEC_PER_USEC
}

/// Arceos time
pub fn current_time() -> u64 {
    current_time_nanos()
}
