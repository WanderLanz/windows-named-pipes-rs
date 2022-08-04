use super::*;

mod generics;
use generics::*;
pub mod unsafes;
use unsafes::*;

def_client_struct! {
        ACCESS_MODE = ClientAccessMode::Duplex,
        /// Client for a dual direction pipe (client ↔ server).
        DuplexClient
        impl Read {
                fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { Read::read(&mut self.0, buf) }
        }
        impl Write {
                fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Write::write(&mut self.0, buf) }
                fn flush(&mut self) -> io::Result<()> { Write::flush(&mut self.0) }
        }
        impl {
                /// open duplex named pipe with message read mode and default security token
                pub fn open(path: &Path) -> Result<Self,NamedPipeWinError> {
                        _open(path, ClientAccessMode::Duplex, ReadMode::Message, None)
                }
                #[doc = "read from pipe into buffer, returning number of bytes read"]
                pub fn read(&mut self, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> { _read(self,buf) }
                #[doc = "Set the named pipe as current process's stdin pipe.<br>Returns old stdin handle upon success<br><br>Experimental, may be removed in future. rust stdlib stdin/stdout/stderr are not designed for named pipes in mind, so UB may occur."]
                pub unsafe fn set_stdin(&self) -> Result<OldStdinHandle, NamedPipeWinError> { _set_stdin(self) }
                #[doc = "write into pipe from buffer, returning number of bytes written"]
                pub fn write(&mut self, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> { _write(self,buf) }
                #[doc = "flush the I/O buffers and if wait mode, wait until all messages are received"]
                pub fn flush(&mut self) -> Result<(), NamedPipeWinError> { _flush(self) }
                #[doc = "Set the named pipe as current process's stdout pipe.<br>Returns old stdout handle upon success<br><br>Experimental, may be removed in future. rust stdlib stdin/stdout/stderr are not designed for named pipes in mind, so UB may occur."]
                pub unsafe fn set_stdout(&self) -> Result<OldStdoutHandle, NamedPipeWinError> { _set_stdout(self) }
        }
}

def_client_struct! {
        ACCESS_MODE = ClientAccessMode::Inbound,
        /// Client for an inbound pipe (client → server).
        InboundClient
        impl Read {
                fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { Read::read(&mut self.0, buf) }
        }
        impl Write {
                fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Write::write(&mut self.0, buf) }
                fn flush(&mut self) -> io::Result<()> { Write::flush(&mut self.0) }
        }
        impl {
                /// open inbound named pipe with message read mode and default security token
                pub fn open(path: &Path) -> Result<Self,NamedPipeWinError> {
                        _open(path, ClientAccessMode::Inbound, ReadMode::Byte /* NOTE: no-op with write-only */, None)
                }
                #[doc = "read from pipe into buffer, returning number of bytes read"]
                pub fn read(&mut self, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> { _read(self,buf) }
                #[doc = "Set the named pipe as current process's stdin pipe.<br>Returns old stdin handle upon success<br><br>Experimental, may be removed in future. rust stdlib stdin/stdout/stderr are not designed for named pipes in mind, so UB may occur."]
                pub unsafe fn set_stdin(&self) -> Result<OldStdinHandle, NamedPipeWinError> { _set_stdin(self) }
                #[doc = "write into pipe from buffer, returning number of bytes written"]
                pub fn write(&mut self, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> { _write(self,buf) }
                #[doc = "flush the I/O buffers and if wait mode, wait until all messages are received"]
                pub fn flush(&mut self) -> Result<(), NamedPipeWinError> { _flush(self) }
                #[doc = "Set the named pipe as current process's stdout pipe.<br>Returns old stdout handle upon success<br><br>Experimental, may be removed in future. rust stdlib stdin/stdout/stderr are not designed for named pipes in mind, so UB may occur."]
                pub unsafe fn set_stdout(&self) -> Result<OldStdoutHandle, NamedPipeWinError> { _set_stdout(self) }
        }
}

def_client_struct! {
        ACCESS_MODE = ClientAccessMode::Outbound,
        /// Client for an outbound pipe (client ← server).
        OutboundClient
        impl Read {
                fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { Read::read(&mut self.0, buf) }
        }
        impl Write {
                fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Write::write(&mut self.0, buf) }
                fn flush(&mut self) -> io::Result<()> { Write::flush(&mut self.0) }
        }
        impl {
                /// open outbound named pipe with message read mode and default security token
                pub fn open(path: &Path) -> Result<Self,NamedPipeWinError> {
                        _open(path, ClientAccessMode::Outbound, ReadMode::Message, None)
                }
                #[doc = "read from pipe into buffer, returning number of bytes read"]
                pub fn read(&mut self, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> { _read(self,buf) }
                #[doc = "Set the named pipe as current process's stdin pipe.<br>Returns old stdin handle upon success<br><br>Experimental, may be removed in future. rust stdlib stdin/stdout/stderr are not designed for named pipes in mind, so UB may occur."]
                pub unsafe fn set_stdin(&self) -> Result<OldStdinHandle, NamedPipeWinError> { _set_stdin(self) }
                #[doc = "write into pipe from buffer, returning number of bytes written"]
                pub fn write(&mut self, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> { _write(self,buf) }
                #[doc = "flush the I/O buffers and if wait mode, wait until all messages are received"]
                pub fn flush(&mut self) -> Result<(), NamedPipeWinError> { _flush(self) }
                #[doc = "Set the named pipe as current process's stdout pipe.<br>Returns old stdout handle upon success<br><br>Experimental, may be removed in future. rust stdlib stdin/stdout/stderr are not designed for named pipes in mind, so UB may occur."]
                pub unsafe fn set_stdout(&self) -> Result<OldStdoutHandle, NamedPipeWinError> { _set_stdout(self) }
        }
}
