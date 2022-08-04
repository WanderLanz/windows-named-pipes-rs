use super::*;

pub struct UnsafeServer {
        pub(crate) handle: UnsafeHandle,
        pub(crate) path: ::std::path::PathBuf,
        pub(crate) info: PipeInfo,
}
impl AsRawHandle for UnsafeServer {
        #[inline]
        fn as_raw_handle(&self) -> RawHandle { self.handle.as_raw_handle() }
}
impl IntoRawHandle for UnsafeServer {
        fn into_raw_handle(self) -> RawHandle { UnsafeHandle::from(self).0 as RawHandle }
}
impl AsHandle for UnsafeServer {
        #[inline]
        fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.handle.as_raw_handle()) } }
}
impl From<UnsafeServer> for UnsafeHandle {
        fn from(that: UnsafeServer) -> Self {
                unsafe {
                        let mut that = ::std::mem::ManuallyDrop::new(that);
                        ::std::ptr::drop_in_place(&mut that.path);
                        ::std::ptr::drop_in_place(&mut that.info);
                        that.handle
                }
        }
}
impl From<UnsafeServer> for OwnedHandle {
        fn from(that: UnsafeServer) -> Self { unsafe { FromRawHandle::from_raw_handle(that.into_raw_handle()) } }
}
impl From<UnsafeServer> for fs::File {
        fn from(that: UnsafeServer) -> Self { unsafe { FromRawHandle::from_raw_handle(that.into_raw_handle()) } }
}
impl Drop for UnsafeServer {
        fn drop(&mut self) {
                unsafe {
                        let _ = CloseHandle(self.handle.as_win_handle());
                }
        }
}
impl UnsafeServer {
        pub unsafe fn name(&self) -> &str { &self.path.file_name().unwrap().to_str().unwrap() }
        pub unsafe fn host(&self) -> &str { &self.path.parent().unwrap().parent().unwrap().to_str().unwrap() }
        #[inline]
        pub fn as_win_handle(&self) -> HANDLE { self.handle.as_win_handle() }
        pub(crate) unsafe fn from_inner(handle: UnsafeHandle, path: PathBuf, info: PipeInfo) -> Self { Self { handle, path, info } }
        /// create a new named pipe server with the given path or open an exisiting path
        pub unsafe fn create(
                path: &Path,
                dwopenmode: FILE_FLAGS_AND_ATTRIBUTES,
                dwpipemode: NAMED_PIPE_MODE,
                nmaxinstances: u32,
                noutbuffersize: u32,
                ninbuffersize: u32,
                ndefaulttimeout: u32,
                lpsecurityattributes: SECURITY_ATTRIBUTES,
        ) -> ::windows::core::Result<Self> {
                let handle = UnsafeHandle::create(
                        path,
                        dwopenmode,
                        dwpipemode,
                        nmaxinstances,
                        noutbuffersize,
                        ninbuffersize,
                        ndefaulttimeout,
                        lpsecurityattributes,
                )?;
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

        /// close the server's pipe handle, capturing error if it fails and wrapping with given msg
        pub fn close(self) -> Result<(), (Self, WinError)> {
                if let Err(err) = self.handle.close() {
                        return Err((self, err));
                }
                let _ = UnsafeHandle::from(self); // already closed
                Ok(())
        }
        /// wait for client to connect to the pipe and get IO enabled server.
        pub unsafe fn connect(&self) -> ::windows::core::Result<UnsafeConnectedServer> {
                if let Err(err) = self.handle.connect() {
                        return Err(err);
                }
                Ok(UnsafeConnectedServer {
                        handle: self.handle.clone(),
                        path: self.path.clone(),
                        info: self.info.clone(),
                })
        }
}

pub struct UnsafeConnectedServer {
        pub(crate) handle: crate::handle::UnsafeHandle,
        pub(crate) path: ::std::path::PathBuf,
        pub(crate) info: crate::handle::PipeInfo,
}
impl AsRawHandle for UnsafeConnectedServer {
        #[inline]
        fn as_raw_handle(&self) -> RawHandle { self.handle.as_raw_handle() }
}
impl IntoRawHandle for UnsafeConnectedServer {
        fn into_raw_handle(self) -> RawHandle { UnsafeHandle::from(self).0 as RawHandle }
}
impl AsHandle for UnsafeConnectedServer {
        #[inline]
        fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.handle.as_raw_handle()) } }
}
impl From<UnsafeConnectedServer> for UnsafeHandle {
        fn from(that: UnsafeConnectedServer) -> Self {
                unsafe {
                        let mut that = ::std::mem::ManuallyDrop::new(that);
                        ::std::ptr::drop_in_place(&mut that.path);
                        ::std::ptr::drop_in_place(&mut that.info);
                        that.handle
                }
        }
}
impl From<UnsafeConnectedServer> for OwnedHandle {
        fn from(that: UnsafeConnectedServer) -> Self { unsafe { FromRawHandle::from_raw_handle(that.into_raw_handle()) } }
}
impl From<UnsafeConnectedServer> for fs::File {
        fn from(that: UnsafeConnectedServer) -> Self { unsafe { FromRawHandle::from_raw_handle(that.into_raw_handle()) } }
}
impl ::std::io::Read for UnsafeConnectedServer {
        #[inline]
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> { ::std::io::Read::read(&mut &self.handle, buf) }
}
impl ::std::io::Write for UnsafeConnectedServer {
        #[inline]
        fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> { ::std::io::Write::write(&mut &self.handle, buf) }
        #[inline]
        fn flush(&mut self) -> ::std::io::Result<()> { ::std::io::Write::flush(&mut &self.handle) }
}
impl Drop for UnsafeConnectedServer {
        fn drop(&mut self) {
                unsafe {
                        let h = self.as_win_handle();
                        let _ = FlushFileBuffers(h);
                        let _ = DisconnectNamedPipe(h);
                        // let non-connected server instance close handle
                }
        }
}
impl UnsafeConnectedServer {
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
        // REVIEW: any UB with the drop?
        pub fn disconnect(self) -> Result<(), (Self, WinError)> {
                if let Err(err) = self.handle.disconnect() {
                        return Err((self, err));
                }
                let _ = UnsafeHandle::from(self); // no need to disconnect again.
                Ok(())
        }
        #[inline]
        pub fn read(&mut self, buf: &mut [u8]) -> win::core::Result<u32> { self.handle.read(buf) }
        #[inline]
        pub fn write(&mut self, buf: &[u8]) -> win::core::Result<u32> { self.handle.write(buf) }
        #[inline]
        pub fn flush(&mut self) -> win::core::Result<()> { self.handle.flush() }
}
