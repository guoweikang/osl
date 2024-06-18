use axerrno::AxError;

pub type Error = AxError; 

impl From<crate::error::Errno> for Error {
    fn from(errno: crate::error::Errno) -> Self {
        match errno {
            crate::error::Errno::InvalidArgs => AxError::InvalidInput, 
            crate::error::Errno::NoSuchDevice => AxError::NotFound,
            crate::error::Errno::TimeOut => AxError::Timeout,
            crate::error::Errno::Busy => AxError::Busy,
            crate::error::Errno::Io => AxError::Io,
            crate::error::Errno::Again => AxError::Again,
        }
    }
}
