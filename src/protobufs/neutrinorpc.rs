#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    /// Indicates whether the neutrino backend is active or not.
    #[prost(bool, tag = "1")]
    pub active: bool,
    /// Is fully synced.
    #[prost(bool, tag = "2")]
    pub synced: bool,
    /// Best block height.
    #[prost(int32, tag = "3")]
    pub block_height: i32,
    /// Best block hash.
    #[prost(string, tag = "4")]
    pub block_hash: ::prost::alloc::string::String,
    /// Connected peers.
    #[prost(string, repeated, tag = "5")]
    pub peers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPeerRequest {
    /// Peer to add.
    #[prost(string, tag = "1")]
    pub peer_addrs: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPeerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectPeerRequest {
    /// Peer to disconnect.
    #[prost(string, tag = "1")]
    pub peer_addrs: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectPeerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsBannedRequest {
    /// Peer to lookup.
    #[prost(string, tag = "1")]
    pub peer_addrs: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsBannedResponse {
    #[prost(bool, tag = "1")]
    pub banned: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockHeaderRequest {
    /// Block hash in hex notation.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockHeaderResponse {
    /// The block hash (same as provided).
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// The number of confirmations.
    #[prost(int64, tag = "2")]
    pub confirmations: i64,
    /// The block size excluding witness data.
    #[prost(int64, tag = "3")]
    pub stripped_size: i64,
    /// The block size (bytes).
    #[prost(int64, tag = "4")]
    pub size: i64,
    /// The block weight as defined in BIP 141.
    #[prost(int64, tag = "5")]
    pub weight: i64,
    /// The block height or index.
    #[prost(int32, tag = "6")]
    pub height: i32,
    /// The block version.
    #[prost(int32, tag = "7")]
    pub version: i32,
    /// The block version.
    #[prost(string, tag = "8")]
    pub version_hex: ::prost::alloc::string::String,
    /// The merkle root.
    #[prost(string, tag = "9")]
    pub merkleroot: ::prost::alloc::string::String,
    /// The block time in seconds since epoch (Jan 1 1970 GMT).
    #[prost(int64, tag = "10")]
    pub time: i64,
    /// The nonce.
    #[prost(uint32, tag = "11")]
    pub nonce: u32,
    /// The bits in hex notation.
    #[prost(string, tag = "12")]
    pub bits: ::prost::alloc::string::String,
    /// The number of transactions in the block.
    #[prost(int32, tag = "13")]
    pub ntx: i32,
    /// The hash of the previous block.
    #[prost(string, tag = "14")]
    pub previous_block_hash: ::prost::alloc::string::String,
    /// The raw hex of the block.
    #[prost(bytes = "vec", tag = "15")]
    pub raw_hex: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockRequest {
    /// Block hash in hex notation.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockResponse {
    /// The block hash (same as provided).
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// The number of confirmations.
    #[prost(int64, tag = "2")]
    pub confirmations: i64,
    /// The block size excluding witness data.
    #[prost(int64, tag = "3")]
    pub stripped_size: i64,
    /// The block size (bytes).
    #[prost(int64, tag = "4")]
    pub size: i64,
    /// The block weight as defined in BIP 141.
    #[prost(int64, tag = "5")]
    pub weight: i64,
    /// The block height or index.
    #[prost(int32, tag = "6")]
    pub height: i32,
    /// The block version.
    #[prost(int32, tag = "7")]
    pub version: i32,
    /// The block version.
    #[prost(string, tag = "8")]
    pub version_hex: ::prost::alloc::string::String,
    /// The merkle root.
    #[prost(string, tag = "9")]
    pub merkleroot: ::prost::alloc::string::String,
    /// List of transaction ids.
    #[prost(string, repeated, tag = "10")]
    pub tx: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The block time in seconds since epoch (Jan 1 1970 GMT).
    #[prost(int64, tag = "11")]
    pub time: i64,
    /// The nonce.
    #[prost(uint32, tag = "12")]
    pub nonce: u32,
    /// The bits in hex notation.
    #[prost(string, tag = "13")]
    pub bits: ::prost::alloc::string::String,
    /// The number of transactions in the block.
    #[prost(int32, tag = "14")]
    pub ntx: i32,
    /// The hash of the previous block.
    #[prost(string, tag = "15")]
    pub previous_block_hash: ::prost::alloc::string::String,
    /// The raw hex of the block.
    #[prost(bytes = "vec", tag = "16")]
    pub raw_hex: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCFilterRequest {
    /// Block hash in hex notation.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCFilterResponse {
    /// GCS filter.
    #[prost(bytes = "vec", tag = "1")]
    pub filter: ::prost::alloc::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod neutrino_kit_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " NeutrinoKit is a service that can be used to get information about the"]
    #[doc = " current state of the neutrino instance, fetch blocks and add/remove peers."]
    #[derive(Debug, Clone)]
    pub struct NeutrinoKitClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NeutrinoKitClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> NeutrinoKitClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NeutrinoKitClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            NeutrinoKitClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = ""]
        #[doc = "Status returns the status of the light client neutrino instance,"]
        #[doc = "along with height and hash of the best block, and a list of connected"]
        #[doc = "peers."]
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutrinorpc.NeutrinoKit/Status");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "AddPeer adds a new peer that has already been connected to the server."]
        pub async fn add_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPeerRequest>,
        ) -> Result<tonic::Response<super::AddPeerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutrinorpc.NeutrinoKit/AddPeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DisconnectPeer disconnects a peer by target address. Both outbound and"]
        #[doc = "inbound nodes will be searched for the target node. An error message will"]
        #[doc = "be returned if the peer was not found."]
        pub async fn disconnect_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::DisconnectPeerRequest>,
        ) -> Result<tonic::Response<super::DisconnectPeerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/neutrinorpc.NeutrinoKit/DisconnectPeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "IsBanned returns true if the peer is banned, otherwise false."]
        pub async fn is_banned(
            &mut self,
            request: impl tonic::IntoRequest<super::IsBannedRequest>,
        ) -> Result<tonic::Response<super::IsBannedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutrinorpc.NeutrinoKit/IsBanned");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "GetBlockHeader returns a block header with a particular block hash."]
        pub async fn get_block_header(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockHeaderRequest>,
        ) -> Result<tonic::Response<super::GetBlockHeaderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/neutrinorpc.NeutrinoKit/GetBlockHeader");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "GetBlock returns a block with a particular block hash."]
        pub async fn get_block(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockRequest>,
        ) -> Result<tonic::Response<super::GetBlockResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutrinorpc.NeutrinoKit/GetBlock");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "GetCFilter returns a compact filter from a block."]
        pub async fn get_c_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCFilterRequest>,
        ) -> Result<tonic::Response<super::GetCFilterResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutrinorpc.NeutrinoKit/GetCFilter");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod neutrino_kit_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with NeutrinoKitServer."]
    #[async_trait]
    pub trait NeutrinoKit: Send + Sync + 'static {
        #[doc = ""]
        #[doc = "Status returns the status of the light client neutrino instance,"]
        #[doc = "along with height and hash of the best block, and a list of connected"]
        #[doc = "peers."]
        async fn status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> Result<tonic::Response<super::StatusResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "AddPeer adds a new peer that has already been connected to the server."]
        async fn add_peer(
            &self,
            request: tonic::Request<super::AddPeerRequest>,
        ) -> Result<tonic::Response<super::AddPeerResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "DisconnectPeer disconnects a peer by target address. Both outbound and"]
        #[doc = "inbound nodes will be searched for the target node. An error message will"]
        #[doc = "be returned if the peer was not found."]
        async fn disconnect_peer(
            &self,
            request: tonic::Request<super::DisconnectPeerRequest>,
        ) -> Result<tonic::Response<super::DisconnectPeerResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "IsBanned returns true if the peer is banned, otherwise false."]
        async fn is_banned(
            &self,
            request: tonic::Request<super::IsBannedRequest>,
        ) -> Result<tonic::Response<super::IsBannedResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "GetBlockHeader returns a block header with a particular block hash."]
        async fn get_block_header(
            &self,
            request: tonic::Request<super::GetBlockHeaderRequest>,
        ) -> Result<tonic::Response<super::GetBlockHeaderResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "GetBlock returns a block with a particular block hash."]
        async fn get_block(
            &self,
            request: tonic::Request<super::GetBlockRequest>,
        ) -> Result<tonic::Response<super::GetBlockResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "GetCFilter returns a compact filter from a block."]
        async fn get_c_filter(
            &self,
            request: tonic::Request<super::GetCFilterRequest>,
        ) -> Result<tonic::Response<super::GetCFilterResponse>, tonic::Status>;
    }
    #[doc = " NeutrinoKit is a service that can be used to get information about the"]
    #[doc = " current state of the neutrino instance, fetch blocks and add/remove peers."]
    #[derive(Debug)]
    pub struct NeutrinoKitServer<T: NeutrinoKit> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: NeutrinoKit> NeutrinoKitServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NeutrinoKitServer<T>
    where
        T: NeutrinoKit,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/neutrinorpc.NeutrinoKit/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: NeutrinoKit>(pub Arc<T>);
                    impl<T: NeutrinoKit> tonic::server::UnaryService<super::StatusRequest> for StatusSvc<T> {
                        type Response = super::StatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/neutrinorpc.NeutrinoKit/AddPeer" => {
                    #[allow(non_camel_case_types)]
                    struct AddPeerSvc<T: NeutrinoKit>(pub Arc<T>);
                    impl<T: NeutrinoKit> tonic::server::UnaryService<super::AddPeerRequest> for AddPeerSvc<T> {
                        type Response = super::AddPeerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_peer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/neutrinorpc.NeutrinoKit/DisconnectPeer" => {
                    #[allow(non_camel_case_types)]
                    struct DisconnectPeerSvc<T: NeutrinoKit>(pub Arc<T>);
                    impl<T: NeutrinoKit> tonic::server::UnaryService<super::DisconnectPeerRequest>
                        for DisconnectPeerSvc<T>
                    {
                        type Response = super::DisconnectPeerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisconnectPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).disconnect_peer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DisconnectPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/neutrinorpc.NeutrinoKit/IsBanned" => {
                    #[allow(non_camel_case_types)]
                    struct IsBannedSvc<T: NeutrinoKit>(pub Arc<T>);
                    impl<T: NeutrinoKit> tonic::server::UnaryService<super::IsBannedRequest> for IsBannedSvc<T> {
                        type Response = super::IsBannedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsBannedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_banned(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsBannedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/neutrinorpc.NeutrinoKit/GetBlockHeader" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockHeaderSvc<T: NeutrinoKit>(pub Arc<T>);
                    impl<T: NeutrinoKit> tonic::server::UnaryService<super::GetBlockHeaderRequest>
                        for GetBlockHeaderSvc<T>
                    {
                        type Response = super::GetBlockHeaderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockHeaderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_header(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockHeaderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/neutrinorpc.NeutrinoKit/GetBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockSvc<T: NeutrinoKit>(pub Arc<T>);
                    impl<T: NeutrinoKit> tonic::server::UnaryService<super::GetBlockRequest> for GetBlockSvc<T> {
                        type Response = super::GetBlockResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/neutrinorpc.NeutrinoKit/GetCFilter" => {
                    #[allow(non_camel_case_types)]
                    struct GetCFilterSvc<T: NeutrinoKit>(pub Arc<T>);
                    impl<T: NeutrinoKit> tonic::server::UnaryService<super::GetCFilterRequest> for GetCFilterSvc<T> {
                        type Response = super::GetCFilterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCFilterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_c_filter(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCFilterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: NeutrinoKit> Clone for NeutrinoKitServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: NeutrinoKit> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: NeutrinoKit> tonic::transport::NamedService for NeutrinoKitServer<T> {
        const NAME: &'static str = "neutrinorpc.NeutrinoKit";
    }
}
