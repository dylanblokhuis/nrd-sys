use std::fmt;

/// NRD C API result code mapped to Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    Failure,
    InvalidArgument,
    Unsupported,
    NonUniqueIdentifier,
    /// Linked `libNRD` reports different `(major, minor)` than this crate’s headers.
    VersionMismatch {
        expected_major: u32,
        expected_minor: u32,
        linked_major: u8,
        linked_minor: u8,
        linked_build: u8,
    },
    Unknown(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Failure => write!(f, "NRD failure"),
            Error::InvalidArgument => write!(f, "NRD invalid argument"),
            Error::Unsupported => write!(f, "NRD unsupported"),
            Error::NonUniqueIdentifier => write!(f, "NRD non-unique denoiser identifier"),
            Error::VersionMismatch {
                expected_major,
                expected_minor,
                linked_major,
                linked_minor,
                linked_build,
            } => write!(
                f,
                "NRD version mismatch: crate expects {}.{}, linked library is {}.{}.{}",
                expected_major, expected_minor, linked_major, linked_minor, linked_build
            ),
            Error::Unknown(c) => write!(f, "NRD unknown result code {c}"),
        }
    }
}

impl std::error::Error for Error {}

pub(crate) fn result_from_ffi(code: u32) -> Result<(), Error> {
    use crate::ffi;
    match code {
        ffi::nrd_Result_SUCCESS => Ok(()),
        ffi::nrd_Result_FAILURE => Err(Error::Failure),
        ffi::nrd_Result_INVALID_ARGUMENT => Err(Error::InvalidArgument),
        ffi::nrd_Result_UNSUPPORTED => Err(Error::Unsupported),
        ffi::nrd_Result_NON_UNIQUE_IDENTIFIER => Err(Error::NonUniqueIdentifier),
        other => Err(Error::Unknown(other)),
    }
}
