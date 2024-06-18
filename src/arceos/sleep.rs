use axtask::sleep;

/// Linux usleep
pub fn usleep(us: u64) {
    sleep(core::time::Duration::from_nanos(us));
}
