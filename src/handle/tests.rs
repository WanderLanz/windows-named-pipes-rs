#![cfg(test)]
use super::super::*;
use super::*;
use io::{Read, Write};
use std::{
        ffi::OsString,
        sync::{Arc, Mutex},
        thread::sleep,
};
#[test]
fn test_handle_io_out() {
        let zeroout = [0u8; 10];
        let ohandle: UnsafeHandle = unsafe { OwnedHandle::from_raw_handle(io::stdout().as_raw_handle()).into() };
        assert_eq!(ohandle.write(&zeroout).unwrap(), 10);
}
#[test]
fn test_handle_io_in() {
        unsafe {
                let f = fs::File::options().read(true).open(r".\src\handle\test_read.txt").unwrap();
                let mut ibuf = vec![0u8; 25];
                let ihandle: UnsafeHandle = OwnedHandle::from_raw_handle(f.as_raw_handle()).into();
                let res = ihandle.read(&mut ibuf).unwrap();
                assert_eq!(res, 11);
                assert!(&ibuf[..11] == b"handle test");
        }
}
