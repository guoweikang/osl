use kernel::prelude::*;

pub use kernel::prelude::Error;

impl From<crate::error::Errno> for Error {
    fn from(errno: crate::error::Errno) -> Self {
        match errno {
            crate::error::Errno::InvalidArgs => EINVAL,
        }
    }
}