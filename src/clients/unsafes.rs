use super::*;

/// Win32 named pipe client
pub struct UnsafeClient {
        pub(crate) handle: UnsafeHandle,
        pub(crate) path: PathBuf,
        pub(crate) info: PipeInfo,
}
impl AsRawHandle for UnsafeClient {
        fn as_raw_handle(&self) -> RawHandle { self.handle.as_raw_handle() }
}
impl IntoRawHandle for UnsafeClient {
        fn into_raw_handle(self) -> RawHandle { UnsafeHandle::from(self).0 as RawHandle }
}
impl AsHandle for UnsafeClient {
        fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.handle.as_raw_handle()) } }
}
impl From<UnsafeClient> for UnsafeHandle {
        fn from(that: UnsafeClient) -> Self {
                unsafe {
                        let mut that = ManuallyDrop::new(that);
                        ::std::ptr::drop_in_place(&mut that.path);
                        ::std::ptr::drop_in_place(&mut that.info);
                        that.handle
                }
        }
}
impl From<UnsafeClient> for OwnedHandle {
        fn from(that: UnsafeClient) -> Self { unsafe { FromRawHandle::from_raw_handle(that.into_raw_handle()) } }
}
impl From<UnsafeClient> for fs::File {
        fn from(that: UnsafeClient) -> Self { unsafe { FromRawHandle::from_raw_handle(that.into_raw_handle()) } }
}
impl Read for UnsafeClient {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { Read::read(&mut &self.handle, buf) }
}
impl Write for UnsafeClient {
        #[inline]
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Write::write(&mut &self.handle, buf) }
        fn flush(&mut self) -> io::Result<()> { Write::flush(&mut &self.handle) }
}
impl Drop for UnsafeClient {
        fn drop(&mut self) {
                unsafe {
                        let h = self.as_win_handle();
                        let _ = FlushFileBuffers(h);
                        let _ = CloseHandle(h);
                }
        }
}
impl UnsafeClient {
        pub unsafe fn name(&self) -> &str { &self.path.file_name().unwrap().to_str().unwrap() }
        pub unsafe fn host(&self) -> &str { &self.path.parent().unwrap().parent().unwrap().to_str().unwrap() }
        #[inline]
        pub fn as_win_handle(&self) -> HANDLE { self.handle.as_win_handle() }
        #[inline]
        pub(crate) unsafe fn from_inner(handle: UnsafeHandle, path: PathBuf, info: PipeInfo) -> Self { Self { handle, path, info } }
        #[inline]
        pub unsafe fn set_stdout(&self) -> win::core::Result<OldStdoutHandle> { self.handle.set_stdout() }
        #[inline]
        pub unsafe fn set_stdin(&self) -> win::core::Result<OldStdinHandle> { self.handle.set_stdin() }

        /// open a client connection and handle to the given named pipe path
        pub unsafe fn open(path: &Path, access: ClientAccessMode, read_mode: ReadMode, token: Option<ImpersonationLevel>) -> win::core::Result<Self> {
                let handle = UnsafeHandle::open(path, access, token)?;
                if read_mode == ReadMode::Message {
                        if !SetNamedPipeHandleState(handle.as_win_handle(), &read_mode.into(), ::std::ptr::null(), ::std::ptr::null()).as_bool() {
                                let _ = CloseHandle(handle.as_win_handle());
                                return Err(WinError::from_win32());
                        }
                }
                let info = match handle.get_info() {
                        Ok(info) => info,
                        Err(err) => {
                                let _ = CloseHandle(handle.as_win_handle());
                                return Err(err);
                        }
                };
                Ok(Self {
                        handle,
                        path: path.to_owned(),
                        info,
                })
        }
        /// close the client connection and handle, safely dropping handle while capturing error
        pub fn close(self) -> Result<(), (Self, WinError)> {
                if let Err(err) = self.handle.close() {
                        return Err((self, err));
                }
                let _ = UnsafeHandle::from(self); // already closed
                Ok(())
        }

        /// connect to the named pipe as client, then write to and read from pipe
        /// # Input
        /// - `path`: path of the pipe to connect to
        /// - `write_buf`: data to write to the pipe
        /// - `read_buf`: buffer to read into from the pipe
        /// # Return
        /// number of bytes read
        pub unsafe fn call(path: &Path, write_buf: &mut [u8], read_buf: &mut [u8]) -> win::core::Result<u32> {
                let path = cvt_path(path)?;
                if write_buf.len() > u32::MAX as usize {
                        return Err(WinError::new(
                                win::core::HRESULT(-1),
                                win::w!("write buffer length larger than u32").to_owned(),
                        ));
                }
                let noutbuffersize = ::core::cmp::min(read_buf.len(), u32::MAX as _) as u32;
                let lpbytesread = &mut 0u32;
                if CallNamedPipeA(
                        path,
                        write_buf.as_mut_ptr().cast(),
                        write_buf.len() as _,
                        read_buf.as_mut_ptr().cast(),
                        noutbuffersize,
                        lpbytesread,
                        NMPWAIT_USE_DEFAULT_WAIT,
                )
                .as_bool()
                {
                        Ok(*lpbytesread)
                } else {
                        Err(WinError::from_win32())
                }
        }
        #[inline]
        pub fn read(&mut self, buf: &mut [u8]) -> win::core::Result<u32> { self.handle.read(buf) }
        #[inline]
        pub fn write(&mut self, buf: &[u8]) -> win::core::Result<u32> { self.handle.write(buf) }
        #[inline]
        pub fn flush(&mut self) -> win::core::Result<()> { self.handle.flush() }
}
