#![allow(dead_code)]
use super::*;

mod generics;
use generics::*;
pub mod unsafes;
use unsafes::*;

// Using macro for repetitive code, expand macros once stable.

def_server_structs!(
        ACCESS_MODE = AccessMode::Duplex,
        /// Dual direction server (server ↔ client).
        /// <br>Must `connect()` to a client before using I/O.
        DuplexServer,
        /// Dual direction connected server (server ↔ client)
        ConnectedDuplexServer
        impl Read {
                fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { Read::read(&mut self.0, buf) }
        }
        impl Write {
                fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Write::write(&mut self.0, buf) }
                fn flush(&mut self) -> io::Result<()> { Write::flush(&mut self.0) }
        }
        impl {
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
);

def_server_structs!(
        ACCESS_MODE = AccessMode::Inbound,
        /// Inbound server (server ← client).
        /// <br>Must `connect()` to a client before using I/O.
        InboundServer,
        /// Inbound connected server (server ← client)
        ConnectedInboundServer
        impl Read {
                fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { Read::read(&mut self.0, buf) }
        }
        impl {
                #[doc = "read from pipe into buffer, returning number of bytes read"]
                pub fn read(&mut self, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> { _read(self,buf) }
                #[doc = "Set the named pipe as current process's stdin pipe.<br>Returns old stdin handle upon success<br><br>Experimental, may be removed in future. rust stdlib stdin/stdout/stderr are not designed for named pipes in mind, so UB may occur."]
                pub unsafe fn set_stdin(&self) -> Result<OldStdinHandle, NamedPipeWinError> { _set_stdin(self) }
        }
);

def_server_structs!(
        ACCESS_MODE = AccessMode::Outbound,
        /// Outbound server (server → client).
        /// <br>Must `connect()` to a client before using I/O.
        OutboundServer,
        /// Outbound connected server (server → client)
        ConnectedOutboundServer
        impl Write {
                fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Write::write(&mut self.0, buf) }
                fn flush(&mut self) -> io::Result<()> { Write::flush(&mut self.0) }
        }
        impl {
                #[doc = "write into pipe from buffer, returning number of bytes written"]
                pub fn write(&mut self, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> { _write(self,buf) }
                #[doc = "flush the I/O buffers and if wait mode, wait until all messages are received"]
                pub fn flush(&mut self) -> Result<(), NamedPipeWinError> { _flush(self) }
                #[doc = "Set the named pipe as current process's stdout pipe.<br>Returns old stdout handle upon success<br><br>Experimental, may be removed in future. rust stdlib stdin/stdout/stderr are not designed for named pipes in mind, so UB may occur."]
                pub unsafe fn set_stdout(&self) -> Result<OldStdoutHandle, NamedPipeWinError> { _set_stdout(self) }
        }
);
