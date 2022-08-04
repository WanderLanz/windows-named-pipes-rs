/// define a Server and ConnectedServer struct with common methods and traits
macro_rules! def_server_structs {
        (
                ACCESS_MODE = $access_mode:expr,
                $(#[$server_metas:meta])*
                $server:ident
                $(impl {$($server_impl_inner:item)*})?,
                $(#[$c_server_metas:meta])*
                $c_server:ident
                $(impl $c_server_trait:ident {$($c_server_trait_inner:item)+})+
                impl {$($c_server_impl_inner:item)+}
        ) => {
                $(#[$server_metas])*
                #[repr(transparent)]
                pub struct $server(pub(crate) UnsafeServer);
                impl ServerImpl for $server { const ACCESS_MODE: AccessMode = $access_mode; }
                impl UnsafeWrapper for $server {
                        type Inner = UnsafeServer;
                        unsafe fn as_inner(&self) -> &Self::Inner { &self.0 }
                        unsafe fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
                        unsafe fn from_inner(inner: Self::Inner) -> Self { Self(inner) }
                        unsafe fn into_inner(self) -> Self::Inner { self.0 }
                }
                impl AsRawHandle for $server {
                        fn as_raw_handle(&self) -> RawHandle { self.0.as_raw_handle() }
                }
                impl AsHandle for $server {
                        fn as_handle(&self) -> BorrowedHandle<'_> { self.0.as_handle() }
                }
                impl $server {
                        #[doc = "Get the full path of the pipe."]
                        pub fn path(&self) -> &Path { &self.0.path }
                        #[doc = "Get the name of the pipe"]
                        pub fn name(&self) -> &str { unsafe { self.0.name() } }
                        #[doc = "Get the host of the pipe"]
                        pub fn host(&self) -> &str { unsafe { self.0.host() } }
                        #[doc = "The server handle as a windows.rs [`HANDLE`](::windows::Win32::Foundation::HANDLE)"]
                        pub fn as_win_handle(&self) -> HANDLE { self.0.as_win_handle() }
                        /// create a new named pipe server with the given path or open an exisiting pipe
                        /// # Behavior
                        /// opens a pipe with Message type, Message read mode, Wait mode, Wait forever, Reject remote clients, DEFAULT_BUFSIZE, default security attributes, and max instances of 1.
                        pub fn open(path: &Path) -> Result<Self, NamedPipeWinError> { _open(path, Self::ACCESS_MODE) }
                        #[doc = r#"Open server with the given name with local path, equivalent to `open("\\.\pipe\name")`."#]
                        pub fn with_name(name: &str) -> Result<Self, NamedPipeWinError> {
                                let mut path = r"\\.\pipe\".to_string();
                                path.push_str(name);
                                Self::open(&PathBuf::from(path))
                        }
                        #[doc = r#"Open server with the given name and host, equivalent to `open("\\host\pipe\name")`."#]
                        pub fn with_host_name(host_name: &str, name: &str) -> Result<Self, NamedPipeWinError> {
                                let mut path = r"\\".to_string();
                                path.push_str(host_name);
                                path.push_str(r"\pipe\");
                                path.push_str(name);
                                Self::open(&PathBuf::from(path))
                        }
                        #[doc = "wait for client to connect to the pipe and get IO enabled server."]
                        pub fn connect(&mut self) -> Result<$c_server, NamedPipeWinError> { _connect(self) }
                        #[doc = "Close the server's pipe handle manually before drop, for handling any drop error."]
                        pub fn close(self) -> Result<(), (Self, NamedPipeWinError)> { _close(self) }
                        #[doc = "Create a builder to specify additional/custom properties of the server."]
                        pub fn builder() -> ServerBuilder::<$server> { ServerBuilder::new() }
                        $($($server_impl_inner)*)?
                }
                $(#[$c_server_metas])*
                #[repr(transparent)]
                pub struct $c_server(pub(crate) UnsafeConnectedServer);
                impl ConnectedServerImpl for $c_server {}
                impl UnsafeWrapper for $c_server {
                        type Inner = UnsafeConnectedServer;
                        unsafe fn as_inner(&self) -> &Self::Inner { &self.0 }
                        unsafe fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
                        unsafe fn from_inner(inner: Self::Inner) -> Self { Self(inner) }
                        unsafe fn into_inner(self) -> Self::Inner { self.0 }
                }
                impl AsRawHandle for $c_server {
                        fn as_raw_handle(&self) -> RawHandle { self.0.as_raw_handle() }
                }
                impl AsHandle for $c_server {
                        fn as_handle(&self) -> BorrowedHandle<'_> { self.0.as_handle() }
                }
                $(impl $c_server_trait for $c_server {$($c_server_trait_inner)+})+
                impl $c_server {
                        #[doc = "Get the full path of the pipe."]
                        pub fn path(&self) -> &Path { &self.0.path }
                        #[doc = "Get the name of the pipe"]
                        pub fn name(&self) -> &str { unsafe { self.0.name() } }
                        #[doc = "Get the host of the pipe"]
                        pub fn host(&self) -> &str { unsafe { self.0.host() } }
                        #[doc = "The server handle as a windows.rs [`HANDLE`](::windows::Win32::Foundation::HANDLE)"]
                        pub fn as_win_handle(&self) -> HANDLE { self.0.as_win_handle() }
                        #[doc = "Disconnect the server's pipe handle manually before drop, for handling any drop error.<br><br>Does not flush."]
                        pub fn disconnect(self) -> Result<(), (Self, NamedPipeWinError)> { _disconnect(self) }
                        $($c_server_impl_inner)+
                }
        };
}

/// define a Client struct with common methods and traits.
macro_rules! def_client_struct {
        (
                ACCESS_MODE = $access_mode:expr,
                $(#[$metas:meta])*
                $client:ident
                $(impl $trait:ident {$($trait_inner:item)+})+
                impl {$($impl_inner:item)+}
        ) => {
                $(#[$metas])*
                #[repr(transparent)]
                pub struct $client(pub(crate) UnsafeClient);
                impl ClientImpl for $client { const ACCESS_MODE: ClientAccessMode = $access_mode; }
                impl UnsafeWrapper for $client {
                        type Inner = UnsafeClient;
                        unsafe fn as_inner(&self) -> &Self::Inner { &self.0 }
                        unsafe fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
                        unsafe fn from_inner(inner: Self::Inner) -> Self { Self(inner) }
                        unsafe fn into_inner(self) -> Self::Inner { self.0 }
                }
                impl AsRawHandle for $client { fn as_raw_handle(&self) -> RawHandle { self.0.as_raw_handle() } }
                impl AsHandle for $client { fn as_handle(&self) -> BorrowedHandle<'_> { self.0.as_handle() } }
                $(impl $trait for $client {$($trait_inner)+})+
                impl $client {
                        #[doc = "Get the full path of the pipe."]
                        pub fn path(&self) -> &Path { &self.0.path }
                        #[doc = "Get the name of the pipe"]
                        pub fn name(&self) -> &str { unsafe { self.0.name() } }
                        #[doc = "Get the host of the pipe"]
                        pub fn host(&self) -> &str { unsafe { self.0.host() } }
                        #[doc = "The client handle as a windows.rs [`HANDLE`](::windows::Win32::Foundation::HANDLE)"]
                        pub fn as_win_handle(&self) -> HANDLE { self.0.as_win_handle() }
                        #[doc = r#"Open client end of pipe with the given name with local machine, equivalent to `open("\\.\pipe\name")`."#]
                        pub fn with_name(name: &str) -> Result<Self, NamedPipeWinError> {
                                let mut path = r"\\.\pipe\".to_string();
                                path.push_str(name);
                                Self::open(&PathBuf::from(path))
                        }
                        #[doc = r#"Open client end of pipe with the given name and host, equivalent to `open("\\host\pipe\name")`."#]
                        pub fn with_host_name(host_name: &str, name: &str) -> Result<Self, NamedPipeWinError> {
                                let mut path = r"\\".to_string();
                                path.push_str(host_name);
                                path.push_str(r"\pipe\");
                                path.push_str(name);
                                Self::open(&PathBuf::from(path))
                        }
                        #[doc = "Close the client's pipe handle manually before drop, for handling any drop error."]
                        pub fn close(self) -> Result<(), (Self, NamedPipeWinError)> { _close(self) }
                        #[doc = "Create a builder to specify additional/custom properties of the client."]
                        pub fn builder() -> ClientBuilder::<$client> { ClientBuilder::new() }
                        $($impl_inner)+
                }
        };
}
