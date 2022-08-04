use std::io;
use windows::core::Error as WinError;
/// simply appends information about the high level lib error to the lower level windows.rs error.
#[derive(Debug)]
pub struct NamedPipeWinError {
        inner: WinError,
        msg: String,
}
impl std::fmt::Display for NamedPipeWinError {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { core::write!(f, "{}: {}", self.msg, self.inner.message()) }
}
impl std::error::Error for NamedPipeWinError {
        #[inline]
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { Some(&self.inner) }
        #[inline]
        fn cause(&self) -> Option<&dyn std::error::Error> { self.source() }
}
impl NamedPipeWinError {
        #[inline]
        pub fn from_win32(msg: String) -> Self {
                NamedPipeWinError {
                        inner: WinError::from_win32(),
                        msg,
                }
        }
        #[inline]
        pub fn from_inner(inner: WinError, msg: String) -> Self { NamedPipeWinError { inner, msg } }
        #[inline]
        pub fn from_msg(msg: String) -> Self {
                NamedPipeWinError {
                        inner: unsafe { ::std::mem::zeroed() },
                        msg,
                }
        }
}
/// simply appends information about the high level lib error to the lower level io error.
#[derive(Debug)]
pub struct NamedPipeIoError {
        inner: io::Error,
        msg: String,
}
impl std::fmt::Display for NamedPipeIoError {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { core::write!(f, "{}: {}", self.msg, self.inner.to_string()) }
}
impl std::error::Error for NamedPipeIoError {
        #[inline]
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { Some(&self.inner) }
        #[inline]
        fn cause(&self) -> Option<&dyn std::error::Error> { self.source() }
}
impl NamedPipeIoError {
        #[inline]
        pub fn from_inner(inner: io::Error, msg: String) -> Self { Self { inner, msg } }
        #[inline]
        pub fn last_os_error<S>(msg: String) -> Self {
                NamedPipeIoError {
                        inner: io::Error::last_os_error(),
                        msg,
                }
        }
        #[inline]
        pub fn from_raw_os_error(rawerr: i32, msg: String) -> Self {
                NamedPipeIoError {
                        inner: io::Error::from_raw_os_error(rawerr),
                        msg,
                }
        }
}
