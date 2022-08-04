use super::*;
mod tests;

/// Raw Named Pipe handle for Windows.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct UnsafeHandle(pub isize);
impl AsRawHandle for UnsafeHandle {
        fn as_raw_handle(&self) -> RawHandle { self.0 as RawHandle }
}
impl IntoRawHandle for UnsafeHandle {
        fn into_raw_handle(self) -> RawHandle { self.0 as RawHandle }
}
impl FromRawHandle for UnsafeHandle {
        unsafe fn from_raw_handle(that: RawHandle) -> Self { Self(that as isize) }
}
impl From<OwnedHandle> for UnsafeHandle {
        fn from(that: OwnedHandle) -> Self { Self(that.into_raw_handle() as isize) }
}
impl From<UnsafeHandle> for OwnedHandle {
        fn from(that: UnsafeHandle) -> Self { unsafe { FromRawHandle::from_raw_handle(that.0 as RawHandle) } }
}
impl From<fs::File> for UnsafeHandle {
        fn from(f: fs::File) -> Self { Self(f.into_raw_handle() as isize) }
}
impl From<UnsafeHandle> for fs::File {
        fn from(that: UnsafeHandle) -> Self { unsafe { FromRawHandle::from_raw_handle(that.0 as RawHandle) } }
}
impl From<HANDLE> for UnsafeHandle {
        fn from(h: HANDLE) -> Self { Self(h.0) }
}
impl From<UnsafeHandle> for HANDLE {
        fn from(h: UnsafeHandle) -> Self { HANDLE(h.0) }
}
impl UnsafeHandle {
        pub(crate) fn as_win_handle(&self) -> HANDLE { HANDLE(self.0) }
        pub(crate) fn as_inner(&self) -> isize { self.0 }
        pub(crate) fn into_inner(self) -> isize { self.0 }
        pub(crate) fn from_inner(inner: isize) -> Self { Self(inner) }
        /// Fallback. quickly do a function as a file
        pub(crate) fn as_file<R, F: FnOnce(&fs::File) -> R>(&self, f: F) -> R {
                // REVIEW
                let fi = ::std::mem::ManuallyDrop::new(unsafe { fs::File::from_raw_handle(self.as_raw_handle()) });
                f(&fi)
        }
        pub fn read(&self, buf: &mut [u8]) -> ::windows::core::Result<u32> {
                let lpnumberofbytesread = &mut 0u32;
                let nnumberofbytestoread = ::core::cmp::min(buf.len(), u32::MAX as usize) as u32;
                if unsafe {
                        ReadFile(
                                self.as_win_handle(),
                                buf.as_mut_ptr().cast(),
                                nnumberofbytestoread,
                                lpnumberofbytesread,
                                ::core::ptr::null_mut(),
                        )
                }
                .as_bool()
                {
                        Ok((*lpnumberofbytesread) as _)
                } else {
                        Err(WinError::from_win32())
                }
        }
        #[inline]
        pub fn is_read_vectored(&self) -> bool { false }
        pub fn write(&self, buf: &[u8]) -> ::windows::core::Result<u32> {
                if buf.len() > u32::MAX as usize {
                        return Err(WinError::new(
                                ::windows::core::HRESULT(-1),
                                ::windows::w!("write buffer length larger than u32").to_owned(),
                        ));
                }
                let lpnumberofbyteswritten = &mut 0u32;
                // let nnumberofbytestowrite = ::core::cmp::min(buf.len(), u32::MAX as usize) as u32;
                if unsafe {
                        WriteFile(
                                self.as_win_handle(),
                                buf.as_ptr().cast(),
                                buf.len() as _,
                                lpnumberofbyteswritten,
                                ::core::ptr::null_mut(),
                        )
                }
                .as_bool()
                {
                        Ok((*lpnumberofbyteswritten) as _)
                } else {
                        Err(WinError::from_win32())
                }
        }
        // NOT IMPLEMENTED: Overlapped read/write (async)
        #[inline]
        pub fn is_write_vectored(&self) -> bool { false }
        // REVIEW: Does WriteFile guarantee a flush to pipe or should we make it so it does?
        pub fn flush(&self) -> ::windows::core::Result<()> {
                if unsafe { FlushFileBuffers(self.as_win_handle()).as_bool() } {
                        Ok(())
                } else {
                        Err(WinError::from_win32())
                }
        }
        /// Set pipe as stdout. Returns previous stdout handle if successful.
        pub fn set_stdout(&self) -> ::windows::core::Result<OldStdoutHandle> {
                use windows::Win32::System::Console::{SetStdHandle, STD_OUTPUT_HANDLE};
                let old_handle = HANDLE(io::stdout().as_raw_handle() as isize);
                if unsafe { SetStdHandle(STD_OUTPUT_HANDLE, self.as_win_handle()) }.as_bool() {
                        Ok(OldStdoutHandle(old_handle))
                } else {
                        Err(WinError::from_win32())
                }
        }
        /// Set pipe as stdin. Returns previous stdin handle if successful.
        pub fn set_stdin(&self) -> ::windows::core::Result<OldStdinHandle> {
                use windows::Win32::System::Console::{SetStdHandle, STD_INPUT_HANDLE};
                let old_handle = HANDLE(io::stdin().as_raw_handle() as isize);
                if unsafe { SetStdHandle(STD_INPUT_HANDLE, self.as_win_handle()) }.as_bool() {
                        Ok(OldStdinHandle(old_handle))
                } else {
                        Err(WinError::from_win32())
                }
        }
        pub fn create(
                path: &Path,
                dwopenmode: FILE_FLAGS_AND_ATTRIBUTES,
                dwpipemode: NAMED_PIPE_MODE,
                nmaxinstances: u32,
                noutbuffersize: u32,
                ninbuffersize: u32,
                ndefaulttimeout: u32,
                lpsecurityattributes: SECURITY_ATTRIBUTES,
        ) -> ::windows::core::Result<UnsafeHandle> {
                unsafe {
                        let path = cvt_path(path)?;
                        Ok(UnsafeHandle::from(CreateNamedPipeA(
                                path,
                                dwopenmode,
                                dwpipemode,
                                nmaxinstances,
                                noutbuffersize,
                                ninbuffersize,
                                ndefaulttimeout,
                                &lpsecurityattributes,
                        )?))
                }
        }
        pub fn connect(&self) -> ::windows::core::Result<()> {
                if unsafe { ConnectNamedPipe(self.as_win_handle(), ::core::ptr::null_mut()).as_bool() } {
                        Ok(())
                } else {
                        Err(WinError::from_win32())
                }
        }
        pub fn disconnect(&self) -> ::windows::core::Result<()> {
                if unsafe { DisconnectNamedPipe(self.as_win_handle()).as_bool() } {
                        Ok(())
                } else {
                        Err(WinError::from_win32())
                }
        }
        pub fn open(path: &Path, access: ClientAccessMode, token: Option<ImpersonationLevel>) -> ::windows::core::Result<UnsafeHandle> {
                unsafe {
                        let lpfilename = cvt_path(path)?;
                        Ok(UnsafeHandle::from(CreateFileA(
                                lpfilename,
                                match access {
                                        ClientAccessMode::Duplex => FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                                        ClientAccessMode::Outbound => FILE_GENERIC_READ,
                                        ClientAccessMode::Inbound => FILE_GENERIC_WRITE,
                                },
                                FILE_SHARE_NONE,
                                &SECURITY_ATTRIBUTES::default(),
                                OPEN_EXISTING,
                                if let Some(t) = token { t.into() } else { FILE_FLAGS_AND_ATTRIBUTES(0) },
                                HANDLE(0),
                        )?))
                }
        }
        pub fn close(self) -> ::windows::core::Result<()> {
                if unsafe { CloseHandle(self.as_win_handle()).as_bool() } {
                        Ok(())
                } else {
                        Err(WinError::from_win32())
                }
        }
        pub fn get_info(&self) -> ::windows::core::Result<PipeInfo> {
                unsafe {
                        let mut info = PipeInfo::default();
                        if GetNamedPipeInfo(
                                self.as_win_handle(),
                                &mut info.flags.0,
                                &mut info.out_size,
                                &mut info.in_size,
                                &mut info.max_instances,
                        )
                        .as_bool()
                        {
                                Ok(info)
                        } else {
                                Err(WinError::from_win32())
                        }
                }
        }
}
impl<'a> io::Read for &'a UnsafeHandle {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                match UnsafeHandle::read(*self, buf) {
                        Ok(w) => Ok(w as _),
                        Err(e) => Err(::std::io::Error::from_raw_os_error(e.code().0)),
                }
        }
}
impl<'a> io::Write for &'a UnsafeHandle {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                match UnsafeHandle::write(*self, buf) {
                        Ok(w) => Ok(w as _),
                        Err(e) => Err(::std::io::Error::from_raw_os_error(e.code().0)),
                }
        }
        fn flush(&mut self) -> io::Result<()> {
                match UnsafeHandle::flush(*self) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(::std::io::Error::from_raw_os_error(e.code().0)),
                }
        }
}

#[repr(transparent)]
pub struct OldStdinHandle(HANDLE);
impl OldStdinHandle {
        pub fn set_stdin(self) -> Result<(), WinError> {
                use windows::Win32::System::Console::{SetStdHandle, STD_INPUT_HANDLE};
                if unsafe { SetStdHandle(STD_INPUT_HANDLE, self.0) }.as_bool() {
                        Ok(())
                } else {
                        Err(WinError::from_win32())
                }
        }
}

#[repr(transparent)]
pub struct OldStdoutHandle(HANDLE);
impl OldStdoutHandle {
        pub fn set_stdout(self) -> Result<(), WinError> {
                use windows::Win32::System::Console::{SetStdHandle, STD_OUTPUT_HANDLE};
                if unsafe { SetStdHandle(STD_OUTPUT_HANDLE, self.0) }.as_bool() {
                        Ok(())
                } else {
                        Err(WinError::from_win32())
                }
        }
}

#[derive(Default, Clone)]
pub struct PipeInfoFlags(pub NAMED_PIPE_MODE);
impl PipeInfoFlags {
        pub fn server(&self) -> bool { self.0 .0 & PIPE_SERVER_END.0 != 0 }
        pub fn client(&self) -> bool { self.0 .0 & PIPE_SERVER_END.0 == 0 }
        pub fn byte(&self) -> bool { self.0 .0 & PIPE_TYPE_MESSAGE.0 == 0 }
        pub fn message(&self) -> bool { self.0 .0 & PIPE_TYPE_MESSAGE.0 != 0 }
}
#[derive(Default, Clone)]
pub struct PipeInfo {
        pub flags: PipeInfoFlags,
        pub in_size: u32,
        pub out_size: u32,
        pub max_instances: u32,
}
