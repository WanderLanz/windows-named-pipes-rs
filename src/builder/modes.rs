use win::Win32::Storage::FileSystem::{
        FILE_ACCESS_FLAGS, SECURITY_ANONYMOUS, SECURITY_DELEGATION, SECURITY_EFFECTIVE_ONLY, SECURITY_IDENTIFICATION, SECURITY_IMPERSONATION,
        SECURITY_SQOS_PRESENT,
};

use super::*;

/// Direction of the pipe.
/// <br>`CreateNamedPipe::dwOpenMode`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum AccessMode {
        Inbound = PIPE_ACCESS_INBOUND.0,
        Outbound = PIPE_ACCESS_OUTBOUND.0,
        Duplex = PIPE_ACCESS_DUPLEX.0,
}
impl AccessMode {
        pub fn into(self) -> FILE_FLAGS_AND_ATTRIBUTES { FILE_FLAGS_AND_ATTRIBUTES(self as u32) }
        pub fn from(ffaa: FILE_FLAGS_AND_ATTRIBUTES) -> Self {
                let inbound = ffaa.0 & PIPE_ACCESS_INBOUND.0 != 0;
                let outbound = ffaa.0 & PIPE_ACCESS_OUTBOUND.0 != 0;
                match (inbound, outbound) {
                        (true, false) => AccessMode::Inbound,
                        (false, true) => AccessMode::Outbound,
                        (true, true) => AccessMode::Duplex,
                        _ => AccessMode::Duplex,
                }
        }
}

/// Byte stream or message stream.
///  <br>`CreateNamedPipe::dwPipeMode`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum TypeMode {
        /// pipe acts as byte stream
        Byte = PIPE_TYPE_BYTE.0,
        /// pipe acts as message stream
        Message = PIPE_TYPE_MESSAGE.0,
}
impl TypeMode {
        pub fn into(self) -> NAMED_PIPE_MODE { NAMED_PIPE_MODE(self as u32) }
        pub fn from(nmpm: NAMED_PIPE_MODE) -> Self {
                if nmpm.0 & PIPE_TYPE_MESSAGE.0 != 0 {
                        Self::Message
                } else {
                        Self::Byte
                }
        }
}

/// How read operations are handled. Forced Byte Mode on Byte Type
/// <br>`CreateNamedPipe::dwPipeMode`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum ReadMode {
        /// pipe acts as byte stream on read
        Byte = PIPE_READMODE_BYTE.0,
        /// pipe acts as message stream on read
        Message = PIPE_READMODE_MESSAGE.0,
}
impl ReadMode {
        pub fn into(self) -> NAMED_PIPE_MODE { NAMED_PIPE_MODE(self as u32) }
        pub fn from(nmpm: NAMED_PIPE_MODE) -> Self {
                if nmpm.0 & PIPE_READMODE_MESSAGE.0 != 0 {
                        Self::Message
                } else {
                        Self::Byte
                }
        }
}

/// Blocking mode, not for Async I/O.
/// <br>`CreateNamedPipe::dwPipeMode`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum WaitMode {
        /// blocking
        Wait = PIPE_WAIT.0,
        /// non-blocking
        NoWait = PIPE_NOWAIT.0,
}
impl WaitMode {
        pub fn into(self) -> NAMED_PIPE_MODE { NAMED_PIPE_MODE(self as u32) }
        pub fn from(nmpm: NAMED_PIPE_MODE) -> Self {
                if nmpm.0 & PIPE_NOWAIT.0 != 0 {
                        Self::NoWait
                } else {
                        Self::Wait
                }
        }
}

/// Same as AccessMode, except translates to FILE_GENERIC_READ and/or FILE_GENERIC_WRITE.
/// <br>`CreateFile::dwDesiredAccess`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum ClientAccessMode {
        Inbound = FILE_GENERIC_WRITE.0,
        Outbound = FILE_GENERIC_READ.0,
        Duplex = FILE_GENERIC_WRITE.0 | FILE_GENERIC_READ.0,
}
impl ClientAccessMode {
        pub fn into(self) -> FILE_ACCESS_FLAGS { FILE_ACCESS_FLAGS(self as u32) }
        pub fn from(faf: FILE_ACCESS_FLAGS) -> Self {
                let write = faf.0 & FILE_GENERIC_WRITE.0 == FILE_GENERIC_WRITE.0;
                let read = faf.0 & FILE_GENERIC_READ.0 == FILE_GENERIC_READ.0;
                match (read, write) {
                        (true, false) => ClientAccessMode::Outbound,
                        (false, true) => ClientAccessMode::Inbound,
                        (true, true) => ClientAccessMode::Duplex,
                        _ => ClientAccessMode::Duplex,
                }
        }
}

/// Ask the server to restrict [impersonation] to level provided, ignored with remote pipes.
/// <br>`CreateFile::dwFlagsAndAttributes`
///
/// [impersonation]: https://docs.microsoft.com/en-us/windows/win32/secauthz/impersonation-levels
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum ImpersonationLevel {
        /// The server cannot impersonate or identify the client.
        Anonymous = SECURITY_ANONYMOUS.0,
        /// The server can get the identity and privileges of the client, but cannot impersonate the client.
        Identification = SECURITY_IDENTIFICATION.0,
        /// The server can impersonate the client's security context on the local system.
        Impersonation = SECURITY_IMPERSONATION.0,
        /// The server can impersonate the client's security context on remote systems.
        Delegation = SECURITY_DELEGATION.0,
}
impl ImpersonationLevel {
        pub fn into(self) -> FILE_FLAGS_AND_ATTRIBUTES { FILE_FLAGS_AND_ATTRIBUTES(self as u32) | SECURITY_SQOS_PRESENT | SECURITY_EFFECTIVE_ONLY }
        pub fn from(ffaa: FILE_FLAGS_AND_ATTRIBUTES) -> Self {
                let ide = ffaa.0 & SECURITY_IDENTIFICATION.0 == SECURITY_IDENTIFICATION.0;
                let imp = ffaa.0 & SECURITY_IMPERSONATION.0 == SECURITY_IMPERSONATION.0;
                let del = ffaa.0 & SECURITY_DELEGATION.0 == SECURITY_DELEGATION.0;
                match (ide, imp, del) {
                        (true, false, false) => ImpersonationLevel::Identification,
                        (false, true, false) => ImpersonationLevel::Impersonation,
                        (false, false, true) => ImpersonationLevel::Delegation,
                        _ => ImpersonationLevel::Anonymous,
                }
        }
}
