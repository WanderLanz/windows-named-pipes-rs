use super::*;

#[inline]
pub(crate) fn _open<Client: ClientImpl>(
        path: &Path,
        access: ClientAccessMode,
        read_mode: ReadMode,
        token: Option<ImpersonationLevel>,
) -> Result<Client, NamedPipeWinError> {
        unsafe {
                match UnsafeClient::open(path, access, read_mode, token) {
                        Ok(server) => Ok(Client::from_inner(server)),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(r#"{}::open("{}") failed with"#, ::std::any::type_name::<Client>(), path.display()),
                        )),
                }
        }
}

#[inline]
pub(crate) fn _close<Client: ClientImpl>(c: Client) -> Result<(), (Client, NamedPipeWinError)> {
        unsafe {
                let uc = c.into_inner();
                if let Err((uc, error)) = uc.close() {
                        let path = uc.path.display().to_string();
                        return Err((
                                Client::from_inner(uc),
                                NamedPipeWinError::from_inner(
                                        error,
                                        format!(r#"{} {{path: "{}",..}} close() failed with"#, ::std::any::type_name::<Client>(), path,),
                                ),
                        ));
                }
                Ok(())
        }
}

#[inline]
pub(super) fn _read<Client: ClientImpl>(c: &mut Client, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> {
        unsafe {
                let uc = c.as_mut_inner();
                match uc.read(buf) {
                        Ok(count) => Ok(count as u32),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} read() failed with"#,
                                        ::std::any::type_name::<Client>(),
                                        uc.path.display()
                                ),
                        )),
                }
        }
}

#[inline]
pub(super) fn _write<Client: ClientImpl>(cs: &mut Client, buf: &[u8]) -> Result<u32, NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_mut_inner();
                match ucs.write(buf) {
                        Ok(count) => Ok(count as u32),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} write() failed with"#,
                                        ::std::any::type_name::<Client>(),
                                        ucs.path.display()
                                ),
                        )),
                }
        }
}

#[inline]
pub(super) fn _flush<Client: ClientImpl>(cs: &mut Client) -> Result<(), NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_mut_inner();
                if let Err(error) = ucs.flush() {
                        return Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} flush() failed with"#,
                                        ::std::any::type_name::<Client>(),
                                        ucs.path.display()
                                ),
                        ));
                }
                Ok(())
        }
}

#[inline]
pub(super) fn _set_stdin<Client: ClientImpl>(cs: &Client) -> Result<OldStdinHandle, NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_inner();
                match ucs.set_stdin() {
                        Ok(old_stdin) => Ok(old_stdin),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} set_stdin() failed with"#,
                                        ::std::any::type_name::<Client>(),
                                        ucs.path.display(),
                                ),
                        )),
                }
        }
}

#[inline]
pub(super) fn _set_stdout<Client: ClientImpl>(cs: &Client) -> Result<OldStdoutHandle, NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_inner();
                match ucs.set_stdout() {
                        Ok(old_stdout) => Ok(old_stdout),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} set_stdout() failed with"#,
                                        ::std::any::type_name::<Client>(),
                                        ucs.path.display(),
                                ),
                        )),
                }
        }
}
