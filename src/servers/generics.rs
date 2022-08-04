use super::*;

#[inline]
pub(super) fn _open<Server: ServerImpl>(path: &Path, access: AccessMode) -> Result<Server, NamedPipeWinError> {
        unsafe {
                match UnsafeServer::create(
                        path,
                        access.into(),
                        PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
                        1,
                        DEFAULT_BUFSIZE,
                        DEFAULT_BUFSIZE,
                        NMPWAIT_WAIT_FOREVER,
                        SECURITY_ATTRIBUTES::default(),
                ) {
                        Ok(server) => Ok(Server::from_inner(server)),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(r#"{}::open("{}") failed with"#, ::std::any::type_name::<Server>(), path.display()),
                        )),
                }
        }
}

#[inline]
pub(super) fn _close<Server: ServerImpl>(s: Server) -> Result<(), (Server, NamedPipeWinError)> {
        unsafe {
                let us = s.into_inner();
                if let Err((us, error)) = us.close() {
                        let path = us.path.display().to_string();
                        return Err((
                                Server::from_inner(us),
                                NamedPipeWinError::from_inner(
                                        error,
                                        format!(r#"{} {{path: "{}",..}} close() failed with"#, ::std::any::type_name::<Server>(), path,),
                                ),
                        ));
                }
                Ok(())
        }
}

#[inline]
pub(super) fn _connect<Server: ServerImpl, ConnectedServer: ConnectedServerImpl>(s: &Server) -> Result<ConnectedServer, NamedPipeWinError> {
        unsafe {
                let us = s.as_inner();
                match us.connect() {
                        Ok(ucs) => Ok(ConnectedServer::from_inner(ucs)),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} connect() failed with"#,
                                        ::std::any::type_name::<Server>(),
                                        us.path.display()
                                ),
                        )),
                }
        }
}

#[inline]
pub(super) fn _disconnect<ConnectedServer: ConnectedServerImpl>(cs: ConnectedServer) -> Result<(), (ConnectedServer, NamedPipeWinError)> {
        unsafe {
                let ucs = cs.into_inner();
                if let Err((ucs, error)) = ucs.disconnect() {
                        let path = ucs.path.display().to_string();
                        return Err((
                                ConnectedServer::from_inner(ucs),
                                NamedPipeWinError::from_inner(
                                        error,
                                        format!(
                                                r#"{} {{path: "{}",..}} disconnect() failed with"#,
                                                ::std::any::type_name::<ConnectedServer>(),
                                                path,
                                        ),
                                ),
                        ));
                }
                Ok(())
        }
}

#[inline]
pub(super) fn _read<ConnectedServer: ConnectedServerImpl>(cs: &mut ConnectedServer, buf: &mut [u8]) -> Result<u32, NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_mut_inner();
                match ucs.read(buf) {
                        Ok(count) => Ok(count as u32),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} read() failed with"#,
                                        ::std::any::type_name::<ConnectedServer>(),
                                        ucs.path.display()
                                ),
                        )),
                }
        }
}

#[inline]
pub(super) fn _write<ConnectedServer: ConnectedServerImpl>(cs: &mut ConnectedServer, buf: &[u8]) -> Result<u32, NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_mut_inner();
                match ucs.write(buf) {
                        Ok(count) => Ok(count as u32),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} write() failed with"#,
                                        ::std::any::type_name::<ConnectedServer>(),
                                        ucs.path.display()
                                ),
                        )),
                }
        }
}

#[inline]
pub(super) fn _flush<ConnectedServer: ConnectedServerImpl>(cs: &mut ConnectedServer) -> Result<(), NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_mut_inner();
                if let Err(error) = ucs.flush() {
                        return Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} flush() failed with"#,
                                        ::std::any::type_name::<ConnectedServer>(),
                                        ucs.path.display()
                                ),
                        ));
                }
                Ok(())
        }
}

#[inline]
pub(super) fn _set_stdin<ConnectedServer: ConnectedServerImpl>(cs: &ConnectedServer) -> Result<OldStdinHandle, NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_inner();
                match ucs.set_stdin() {
                        Ok(old_stdin) => Ok(old_stdin),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} set_stdin() failed with"#,
                                        ::std::any::type_name::<ConnectedServer>(),
                                        ucs.path.display(),
                                ),
                        )),
                }
        }
}

#[inline]
pub(super) fn _set_stdout<ConnectedServer: ConnectedServerImpl>(cs: &ConnectedServer) -> Result<OldStdoutHandle, NamedPipeWinError> {
        unsafe {
                let ucs = cs.as_inner();
                match ucs.set_stdout() {
                        Ok(old_stdout) => Ok(old_stdout),
                        Err(error) => Err(NamedPipeWinError::from_inner(
                                error,
                                format!(
                                        r#"{} {{path: "{}",..}} set_stdout() failed with"#,
                                        ::std::any::type_name::<ConnectedServer>(),
                                        ucs.path.display(),
                                ),
                        )),
                }
        }
}
