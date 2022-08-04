//! Completely irresponsible, unprofessional, amateur library for Win32 Named Pipes with windows.rs.
//! Seriously, use at your own risk and be sure to double check before using anything!
//!
//! Mostly just a quick and dirty wrapper around the Win32 API for learning purposes.
//!
//! For reference: Inbound Named Pipes are cient-to-server, Outbound Named Pipes are server-to-client, and Duplex Named Pipes are both.
//!
//! License: MIT License (see LICENSE file)
//! Copyright (c) 2022, Lance W.
//!
//! Dependencies' Licenses:
//! - windows-rs: [Apache 2.0](https://github.com/microsoft/windows-rs/blob/master/license-apache-2.0) & [MIT License](https://github.com/microsoft/windows-rs/blob/master/license-mit)

// annihilate namespace cleanliness for convenience and readability
use ::std::{
        fs,
        io::{self, Read, Write},
        mem::ManuallyDrop,
        os::windows::io::*,
        path::{Path, PathBuf},
};
use ::windows::{
        self as win,
        core::{Error as WinError, HRESULT, PCSTR},
        Win32::{
                Foundation::{CloseHandle, HANDLE, MAX_PATH},
                Security::SECURITY_ATTRIBUTES,
                Storage::FileSystem::{
                        CreateFileA, FlushFileBuffers, ReadFile, WriteFile, FILE_FLAGS_AND_ATTRIBUTES, FILE_FLAG_FIRST_PIPE_INSTANCE,
                        FILE_FLAG_WRITE_THROUGH, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_NONE, OPEN_EXISTING, PIPE_ACCESS_DUPLEX,
                        PIPE_ACCESS_INBOUND, PIPE_ACCESS_OUTBOUND,
                },
                System::Pipes::*,
        },
};

pub const DEFAULT_BUFSIZE: u32 = 512;
/// Maximum guaranteed number of bytes for a transaction (read and write in one call)
pub const MAX_TRANSACTION: u32 = 65535;
/// Prefix for local named pipes, within the same UWP application
pub const LOCAL_PREFIX: &str = r"\\.\pipe\LOCAL\";
/// Prefix for named pipes on the local machine.
pub const DEFAULT_PREFIX: &str = r"\\.\pipe\";

const ERROR_INVALID_NAME: HRESULT = HRESULT(123);

mod tests;

#[macro_use]
mod macros;

mod errors;
pub use errors::*;

mod generics;
use generics::*;

mod handle;
use handle::*;
pub use handle::{OldStdinHandle, OldStdoutHandle}; // need to expose in order for handle mod to stay non-pub

mod unsafes;
use unsafes::*;

pub mod servers;
pub use servers::*;

pub mod clients;
pub use clients::*;

pub mod builder;
use builder::*;
