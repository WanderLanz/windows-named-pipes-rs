#![allow(dead_code)]
use super::*;
use crate::clients::unsafes::*;
use crate::servers::unsafes::*;

pub trait UnsafeWrapper {
        type Inner;
        unsafe fn from_inner(inner: Self::Inner) -> Self;
        unsafe fn into_inner(self) -> Self::Inner;
        unsafe fn as_inner(&self) -> &Self::Inner;
        unsafe fn as_mut_inner(&mut self) -> &mut Self::Inner;
}
pub trait ServerImpl: UnsafeWrapper<Inner = UnsafeServer> {
        const ACCESS_MODE: AccessMode;
}
pub(crate) trait ConnectedServerImpl: UnsafeWrapper<Inner = UnsafeConnectedServer> {}
pub trait ClientImpl: UnsafeWrapper<Inner = UnsafeClient> {
        const ACCESS_MODE: ClientAccessMode;
}
