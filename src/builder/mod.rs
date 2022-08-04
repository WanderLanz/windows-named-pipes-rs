use std::marker::PhantomData;

use crate::{clients::unsafes::UnsafeClient, servers::unsafes::UnsafeServer};

use super::*;

mod modes;
pub use modes::*;
// mod unsafes;
// pub use unsafes::*;

// Not stable yet.

pub struct ServerBuilder<Server: ServerImpl> {
        pub(crate) __server: PhantomData<Server>,
        /// Byte stream or message stream.
        pub(crate) type_mode: TypeMode,

        /// How read operations are handled. Forced Byte Mode on Byte Type.
        pub(crate) read_mode: ReadMode, // INFO: must be Byte when pipe type is Byte

        // overlapped: bool, // NOT IMPLEMENTED: Win32 API Asynchronous I/O, let rust async handle sync as async
        /// Fails if pipe already exists.
        pub(crate) create_new: bool, // CreateNamedPipe opens or creates if it doesn't exist, unless create_new is true

        /// accept or automatically reject remote clients.
        pub(crate) remote_clients: bool,

        pub(crate) max_instances: u32,

        /// outbound buffer size (server -> client)
        pub(crate) buffer_size_out: u32,

        /// inbound buffer size (client -> server)
        pub(crate) buffer_size_in: u32,

        /// default timeout for IO and connection requests
        pub(crate) timeout: u32,

        pub(crate) security: SECURITY_ATTRIBUTES, // *const windows::Win32::Security::SECURITY_ATTRIBUTES
}
impl<Server: ServerImpl> ServerBuilder<Server> {
        pub fn type_mode(&mut self, type_mode: TypeMode) -> &mut Self {
                self.type_mode = type_mode;
                self
        }
        pub fn read_mode(&mut self, read_mode: ReadMode) -> &mut Self {
                self.read_mode = read_mode;
                self
        }
        pub fn create_new(&mut self, create_new: bool) -> &mut Self {
                self.create_new = create_new;
                self
        }
        pub fn remote_clients(&mut self, remote_clients: bool) -> &mut Self {
                self.remote_clients = remote_clients;
                self
        }
        pub fn max_instances(&mut self, max_instances: u32) -> &mut Self {
                self.max_instances = max_instances;
                self
        }
        pub fn buffersize_out(&mut self, buffersize_out: u32) -> &mut Self {
                self.buffer_size_out = buffersize_out;
                self
        }
        pub fn buffersize_in(&mut self, buffersize_in: u32) -> &mut Self {
                self.buffer_size_in = buffersize_in;
                self
        }
        pub fn timeout(&mut self, timeout: u32) -> &mut Self {
                self.timeout = timeout;
                self
        }
        // NOT IMPLEMENTED: safe security attributes wrapper
        pub fn security(&mut self, security: SECURITY_ATTRIBUTES) -> &mut Self {
                self.security = security;
                self
        }
        pub fn new() -> Self { Self::default() }
        pub fn open(&self, path: &Path) -> Result<Server, NamedPipeWinError> {
                unsafe {
                        match UnsafeServer::create(
                                path,
                                self.get_open_mode(),
                                self.get_pipe_mode(),
                                self.max_instances,
                                self.buffer_size_out,
                                self.buffer_size_in,
                                self.timeout,
                                self.security,
                        ) {
                                Ok(inner) => Ok(Server::from_inner(inner)),
                                Err(inner) => {
                                        return Err(NamedPipeWinError::from_inner(
                                                inner,
                                                format!(
                                                        r#"windows_named_pipes::ServerBuilder::<{}>::open("{}") failed with"#,
                                                        ::std::any::type_name::<Server>(),
                                                        path.display(),
                                                ),
                                        ))
                                }
                        }
                }
        }
        fn get_open_mode(&self) -> FILE_FLAGS_AND_ATTRIBUTES {
                let pre = Server::ACCESS_MODE.into();
                if self.create_new {
                        pre | FILE_FLAG_FIRST_PIPE_INSTANCE | FILE_FLAG_WRITE_THROUGH
                } else {
                        pre | FILE_FLAG_WRITE_THROUGH
                }
        }
        fn get_pipe_mode(&self) -> NAMED_PIPE_MODE {
                let pre = self.type_mode.into() | self.read_mode.into();
                if self.remote_clients {
                        pre | PIPE_ACCEPT_REMOTE_CLIENTS | PIPE_WAIT
                } else {
                        pre | PIPE_REJECT_REMOTE_CLIENTS | PIPE_WAIT
                }
        }
}

impl<Server: ServerImpl> Default for ServerBuilder<Server> {
        fn default() -> Self {
                // REVIEW: good defaults?
                ServerBuilder {
                        __server: PhantomData::<Server>,
                        type_mode: TypeMode::Message,
                        read_mode: ReadMode::Message,
                        create_new: false,
                        remote_clients: false,
                        max_instances: 1,
                        buffer_size_out: DEFAULT_BUFSIZE,
                        buffer_size_in: DEFAULT_BUFSIZE,
                        timeout: NMPWAIT_WAIT_FOREVER,
                        security: SECURITY_ATTRIBUTES::default(),
                }
        }
}

pub struct ClientBuilder<Client: ClientImpl> {
        pub(crate) __client: PhantomData<Client>,

        /// How read operations are handled. Forced Byte Mode on Byte Type.
        pub(crate) read_mode: ReadMode, // INFO: must be Byte when pipe type is Byte

        pub(crate) security_token: Option<ImpersonationLevel>,
}
impl<Client: ClientImpl> ClientBuilder<Client> {
        pub fn read_mode(&mut self, read_mode: ReadMode) -> &mut Self {
                self.read_mode = read_mode;
                self
        }
        // NOT IMPLEMENTED: safe security attributes wrapper
        pub fn security(&mut self, security_token: ImpersonationLevel) -> &mut Self {
                self.security_token = Some(security_token);
                self
        }
        pub fn new() -> Self { Self::default() }
        pub fn open(&self, path: &Path) -> Result<Client, NamedPipeWinError> {
                unsafe {
                        match UnsafeClient::open(path, Client::ACCESS_MODE, self.read_mode, self.security_token) {
                                Ok(inner) => Ok(Client::from_inner(inner)),
                                Err(inner) => Err(NamedPipeWinError::from_inner(
                                        inner,
                                        format!(
                                                r#"windows_named_pipes::ClientBuilder::<{}>::open("{}") failed with"#,
                                                ::std::any::type_name::<Client>(),
                                                path.display(),
                                        ),
                                )),
                        }
                }
        }
}

impl<Client: ClientImpl> Default for ClientBuilder<Client> {
        fn default() -> Self {
                // REVIEW: good defaults?
                ClientBuilder {
                        __client: PhantomData::<Client>,
                        read_mode: ReadMode::Message,
                        security_token: None,
                }
        }
}
