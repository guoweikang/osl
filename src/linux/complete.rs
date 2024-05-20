use crate::error::{
    Result,
    Errno,
    to_error,
};
use kernel::completion::Completion;

/// Osl Complete
pub struct OslCompletion {
    val: Completion,
}

impl Default for OslCompletion {
    fn default() -> Self {
        Self {
            val: Completion::default(),
        }
    }
}

impl crate::sync::GeneralComplete for OslCompletion {

    fn init(&mut self) {
        self.val.init();
    }

    fn reinit(&mut self) {
        self.val.reinit();
    }

    fn complete(&mut self) {
        self.val.complete();
    }

    fn wait_for_completion(&mut self) {
        self.val.wait_for_completion();
    }

    fn wait_for_completion_timeout(&mut self, timeout: u32) -> Result<()> {
        if self.val.wait_for_completion_timeout_sec(timeout as usize) == 0 {
            return to_error(Errno::TimeOut);
        };
        Ok(())
    }

}
