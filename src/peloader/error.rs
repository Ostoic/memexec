#[derive(Debug)]
pub enum Error {
    InvalidCString,
    LoadLibararyFail,
    GetProcAddressFail,
    NtAllocVmErr(i32),
    NtProtectVmErr(i32),
    InvalidUtf8String,
    MismatchedArch,
    MismatchedLoader,
    NoEntryPoint,
    UnsupportedDotNetExecutable,
    InvalidProcDescString,
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::convert::From<core::str::Utf8Error> for Error {
    fn from(_: core::str::Utf8Error) -> Self {
        Error::InvalidUtf8String
    }
}

impl core::convert::From<core::num::ParseIntError> for Error {
    fn from(_: core::num::ParseIntError) -> Self {
        Error::InvalidProcDescString
    }
}
