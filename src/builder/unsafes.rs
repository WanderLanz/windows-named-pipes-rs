use super::*;

/// emulate [`OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) for Named Pipe Server
#[derive(Debug, PartialEq, Eq)]
pub struct UnsafeOpenOptions {
        /// Pipe direction.
        pub(crate) access_mode: AccessMode,
        /// Byte stream or message stream.
        pub(crate) type_mode: TypeMode,
        /// How read operations are handled. Forced Byte Mode on Byte Type.
        pub(crate) read_mode: ReadMode, // INFO: must be Byte when pipe type is Byte
        // overlapped: bool, // NOT IMPLEMENTED: Win32 API Asynchronous I/O, let rust async handle sync as async
        /// Blocks writes until message is received on receiving side and no buffering is performed on byte type.
        pub(crate) write_through: bool, // INFO: always true if pipe type is Message
        /// Fails if pipe already exists.
        pub(crate) create_new: bool, // CreateNamedPipe opens or creates if it doesn't exist, unless create_new is true
        /// Blocking mode, not for Async I/O.
        pub(crate) wait_mode: WaitMode,
        /// accept or automatically reject remote clients.
        pub(crate) remote_clients: bool,
}
impl UnsafeOpenOptions {
        pub fn access(&mut self, access_mode: AccessMode) -> &mut Self {
                self.access_mode = access_mode;
                self
        }
        pub fn pipe_type(&mut self, pipe_type: TypeMode) -> &mut Self {
                self.type_mode = pipe_type;
                self
        }
        pub fn readmode(&mut self, readmode: ReadMode) -> &mut Self {
                self.read_mode = readmode;
                self
        }
        //pub fn overlapped(&mut self, overlapped: Overlapped) -> &mut Self { self.overlapped = overlapped; self }
        //pub fn no_buffering(&mut self, no_buffering: bool) -> &mut Self { self.no_buffering = no_buffering; self }
        pub fn write_through(&mut self, write_through: bool) -> &mut Self {
                self.write_through = write_through;
                self
        }
        pub fn create_new(&mut self, create_new: bool) -> &mut Self {
                self.create_new = create_new;
                self
        }
        pub fn no_wait(&mut self, wait_mode: WaitMode) -> &mut Self {
                self.wait_mode = wait_mode;
                self
        }
        pub fn remote_clients(&mut self, remote_clients: bool) -> &mut Self {
                self.remote_clients = remote_clients;
                self
        }
        pub fn new() -> Self { Self::default() }
        pub fn get_open_mode(&self) -> FILE_FLAGS_AND_ATTRIBUTES {
                let pre = self.access_mode.into();
                match (self.create_new, self.write_through /*, self.overlapped */) {
                        (true, true) => pre | FILE_FLAG_FIRST_PIPE_INSTANCE | FILE_FLAG_WRITE_THROUGH,
                        (true, false) => pre | FILE_FLAG_FIRST_PIPE_INSTANCE,
                        (false, true) => pre | FILE_FLAG_WRITE_THROUGH,
                        (false, false) => pre,
                }
        }
        pub fn get_pipe_mode(&self) -> NAMED_PIPE_MODE {
                let pre = self.type_mode.into() | self.read_mode.into() | self.wait_mode.into();
                if self.remote_clients {
                        pre | PIPE_ACCEPT_REMOTE_CLIENTS
                } else {
                        pre | PIPE_REJECT_REMOTE_CLIENTS
                }
        }
}
impl Default for UnsafeOpenOptions {
        fn default() -> Self {
                Self {
                        access_mode: AccessMode::Duplex,
                        type_mode: TypeMode::Message,
                        read_mode: ReadMode::Message,
                        // overlapped: false,
                        write_through: false,
                        create_new: false,
                        wait_mode: WaitMode::Wait,
                        remote_clients: false,
                }
        }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnsafeServerBuilder {
        pub(crate) options: UnsafeOpenOptions,
        pub(crate) max_instances: u32,
        /// outbound buffer size (server -> client)
        pub(crate) buffer_size_out: u32,
        /// inbound buffer size (client -> server)
        pub(crate) buffer_size_in: u32,
        /// default timeout for IO and connection requests
        pub(crate) timeout: u32,
        pub(crate) security: SECURITY_ATTRIBUTES, // *const windows::Win32::Security::SECURITY_ATTRIBUTES
}

impl Default for UnsafeServerBuilder {
        fn default() -> Self {
                Self {
                        options: UnsafeOpenOptions::default(),
                        max_instances: 1, // 1 client at a time
                        buffer_size_out: DEFAULT_BUFSIZE,
                        buffer_size_in: DEFAULT_BUFSIZE,
                        timeout: NMPWAIT_WAIT_FOREVER,
                        security: SECURITY_ATTRIBUTES::default(),
                }
        }
}

impl UnsafeServerBuilder {
        pub fn access_mode(&mut self, access_mode: AccessMode) -> &mut Self {
                self.options.access_mode = access_mode;
                self
        }
        pub fn type_mode(&mut self, type_mode: TypeMode) -> &mut Self {
                self.options.type_mode = type_mode;
                self
        }
        pub fn read_mode(&mut self, read_mode: ReadMode) -> &mut Self {
                self.options.read_mode = read_mode;
                self
        }
        //pub fn overlapped(&mut self, overlapped: Overlapped) -> &mut Self { self.overlapped = overlapped; self }
        pub fn write_through(&mut self, write_through: bool) -> &mut Self {
                self.options.write_through = write_through;
                self
        }
        pub fn create_new(&mut self, create_new: bool) -> &mut Self {
                self.options.create_new = create_new;
                self
        }
        pub fn wait_mode(&mut self, wait_mode: WaitMode) -> &mut Self {
                self.options.wait_mode = wait_mode;
                self
        }
        pub fn remote_clients(&mut self, remote_clients: bool) -> &mut Self {
                self.options.remote_clients = remote_clients;
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
        pub unsafe fn open(&self, path: &Path) -> ::windows::core::Result<UnsafeHandle> {
                UnsafeHandle::create(
                        path,
                        self.options.get_open_mode(),
                        self.options.get_pipe_mode(),
                        self.max_instances,
                        self.buffer_size_out,
                        self.buffer_size_in,
                        self.timeout,
                        self.security,
                )
        }
}
