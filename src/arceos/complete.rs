use axsync::Completion;
use alloc::sync::Arc;
use crate::error::{to_error, Errno, Result};

/// Osl Complete
pub struct OslCompletion {
    val: Completion
}

impl crate::sync::GeneralComplete for OslCompletion {
    fn new() -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            val: Completion::new(),
        }))
    }

    fn reinit(&self) {
        self.val.reinit();
    }

    fn complete(&self) {
        self.val.complete();
    }

    fn wait_for_completion(&self) {
        self.val.wait_for_completion();
    }

    fn wait_for_completion_timeout(&self, timeout: u32) -> Result<()> {
        if self.val.wait_for_completion_timeout(timeout as u64) {
            return to_error(Errno::TimeOut);
        };
        Ok(())
    }
}
