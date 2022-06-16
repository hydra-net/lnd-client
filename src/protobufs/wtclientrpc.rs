#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTowerRequest {
    /// The identifying public key of the watchtower to add.
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    /// A network address the watchtower is reachable over.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTowerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTowerRequest {
    /// The identifying public key of the watchtower to remove.
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    ///
    ///If set, then the record for this address will be removed, indicating that is
    ///is stale. Otherwise, the watchtower will no longer be used for future
    ///session negotiations and backups.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTowerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTowerInfoRequest {
    /// The identifying public key of the watchtower to retrieve information for.
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    /// Whether we should include sessions with the watchtower in the response.
    #[prost(bool, tag = "2")]
    pub include_sessions: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TowerSession {
    ///
    ///The total number of successful backups that have been made to the
    ///watchtower session.
    #[prost(uint32, tag = "1")]
    pub num_backups: u32,
    ///
    ///The total number of backups in the session that are currently pending to be
    ///acknowledged by the watchtower.
    #[prost(uint32, tag = "2")]
    pub num_pending_backups: u32,
    /// The maximum number of backups allowed by the watchtower session.
    #[prost(uint32, tag = "3")]
    pub max_backups: u32,
    ///
    ///Deprecated, use sweep_sat_per_vbyte.
    ///The fee rate, in satoshis per vbyte, that will be used by the watchtower for
    ///the justice transaction in the event of a channel breach.
    #[deprecated]
    #[prost(uint32, tag = "4")]
    pub sweep_sat_per_byte: u32,
    ///
    ///The fee rate, in satoshis per vbyte, that will be used by the watchtower for
    ///the justice transaction in the event of a channel breach.
    #[prost(uint32, tag = "5")]
    pub sweep_sat_per_vbyte: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tower {
    /// The identifying public key of the watchtower.
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    /// The list of addresses the watchtower is reachable over.
    #[prost(string, repeated, tag = "2")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether the watchtower is currently a candidate for new sessions.
    #[prost(bool, tag = "3")]
    pub active_session_candidate: bool,
    /// The number of sessions that have been negotiated with the watchtower.
    #[prost(uint32, tag = "4")]
    pub num_sessions: u32,
    /// The list of sessions that have been negotiated with the watchtower.
    #[prost(message, repeated, tag = "5")]
    pub sessions: ::prost::alloc::vec::Vec<TowerSession>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTowersRequest {
    /// Whether we should include sessions with the watchtower in the response.
    #[prost(bool, tag = "1")]
    pub include_sessions: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTowersResponse {
    /// The list of watchtowers available for new backups.
    #[prost(message, repeated, tag = "1")]
    pub towers: ::prost::alloc::vec::Vec<Tower>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatsResponse {
    ///
    ///The total number of backups made to all active and exhausted watchtower
    ///sessions.
    #[prost(uint32, tag = "1")]
    pub num_backups: u32,
    ///
    ///The total number of backups that are pending to be acknowledged by all
    ///active and exhausted watchtower sessions.
    #[prost(uint32, tag = "2")]
    pub num_pending_backups: u32,
    ///
    ///The total number of backups that all active and exhausted watchtower
    ///sessions have failed to acknowledge.
    #[prost(uint32, tag = "3")]
    pub num_failed_backups: u32,
    /// The total number of new sessions made to watchtowers.
    #[prost(uint32, tag = "4")]
    pub num_sessions_acquired: u32,
    /// The total number of watchtower sessions that have been exhausted.
    #[prost(uint32, tag = "5")]
    pub num_sessions_exhausted: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyRequest {
    ///
    ///The client type from which to retrieve the active offering policy.
    #[prost(enumeration = "PolicyType", tag = "1")]
    pub policy_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyResponse {
    ///
    ///The maximum number of updates each session we negotiate with watchtowers
    ///should allow.
    #[prost(uint32, tag = "1")]
    pub max_updates: u32,
    ///
    ///Deprecated, use sweep_sat_per_vbyte.
    ///The fee rate, in satoshis per vbyte, that will be used by watchtowers for
    ///justice transactions in response to channel breaches.
    #[deprecated]
    #[prost(uint32, tag = "2")]
    pub sweep_sat_per_byte: u32,
    ///
    ///The fee rate, in satoshis per vbyte, that will be used by watchtowers for
    ///justice transactions in response to channel breaches.
    #[prost(uint32, tag = "3")]
    pub sweep_sat_per_vbyte: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolicyType {
    /// Selects the policy from the legacy tower client.
    Legacy = 0,
    /// Selects the policy from the anchor tower client.
    Anchor = 1,
}
#[doc = r" Generated client implementations."]
pub mod watchtower_client_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " WatchtowerClient is a service that grants access to the watchtower client"]
    #[doc = " functionality of the daemon."]
    #[derive(Debug, Clone)]
    pub struct WatchtowerClientClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WatchtowerClientClient<tonic::transport::Channel> {
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
    impl<T> WatchtowerClientClient<T>
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
        ) -> WatchtowerClientClient<InterceptedService<T, F>>
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
            WatchtowerClientClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "AddTower adds a new watchtower reachable at the given address and"]
        #[doc = "considers it for new sessions. If the watchtower already exists, then"]
        #[doc = "any new addresses included will be considered when dialing it for"]
        #[doc = "session negotiations and backups."]
        pub async fn add_tower(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTowerRequest>,
        ) -> Result<tonic::Response<super::AddTowerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/wtclientrpc.WatchtowerClient/AddTower");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "RemoveTower removes a watchtower from being considered for future session"]
        #[doc = "negotiations and from being used for any subsequent backups until it's added"]
        #[doc = "again. If an address is provided, then this RPC only serves as a way of"]
        #[doc = "removing the address from the watchtower instead."]
        pub async fn remove_tower(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveTowerRequest>,
        ) -> Result<tonic::Response<super::RemoveTowerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/wtclientrpc.WatchtowerClient/RemoveTower");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ListTowers returns the list of watchtowers registered with the client."]
        pub async fn list_towers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTowersRequest>,
        ) -> Result<tonic::Response<super::ListTowersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/wtclientrpc.WatchtowerClient/ListTowers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetTowerInfo retrieves information for a registered watchtower."]
        pub async fn get_tower_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTowerInfoRequest>,
        ) -> Result<tonic::Response<super::Tower>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/wtclientrpc.WatchtowerClient/GetTowerInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stats returns the in-memory statistics of the client since startup."]
        pub async fn stats(
            &mut self,
            request: impl tonic::IntoRequest<super::StatsRequest>,
        ) -> Result<tonic::Response<super::StatsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/wtclientrpc.WatchtowerClient/Stats");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Policy returns the active watchtower client policy configuration."]
        pub async fn policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::PolicyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/wtclientrpc.WatchtowerClient/Policy");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod watchtower_client_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WatchtowerClientServer."]
    #[async_trait]
    pub trait WatchtowerClient: Send + Sync + 'static {
        #[doc = ""]
        #[doc = "AddTower adds a new watchtower reachable at the given address and"]
        #[doc = "considers it for new sessions. If the watchtower already exists, then"]
        #[doc = "any new addresses included will be considered when dialing it for"]
        #[doc = "session negotiations and backups."]
        async fn add_tower(
            &self,
            request: tonic::Request<super::AddTowerRequest>,
        ) -> Result<tonic::Response<super::AddTowerResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "RemoveTower removes a watchtower from being considered for future session"]
        #[doc = "negotiations and from being used for any subsequent backups until it's added"]
        #[doc = "again. If an address is provided, then this RPC only serves as a way of"]
        #[doc = "removing the address from the watchtower instead."]
        async fn remove_tower(
            &self,
            request: tonic::Request<super::RemoveTowerRequest>,
        ) -> Result<tonic::Response<super::RemoveTowerResponse>, tonic::Status>;
        #[doc = " ListTowers returns the list of watchtowers registered with the client."]
        async fn list_towers(
            &self,
            request: tonic::Request<super::ListTowersRequest>,
        ) -> Result<tonic::Response<super::ListTowersResponse>, tonic::Status>;
        #[doc = " GetTowerInfo retrieves information for a registered watchtower."]
        async fn get_tower_info(
            &self,
            request: tonic::Request<super::GetTowerInfoRequest>,
        ) -> Result<tonic::Response<super::Tower>, tonic::Status>;
        #[doc = " Stats returns the in-memory statistics of the client since startup."]
        async fn stats(
            &self,
            request: tonic::Request<super::StatsRequest>,
        ) -> Result<tonic::Response<super::StatsResponse>, tonic::Status>;
        #[doc = " Policy returns the active watchtower client policy configuration."]
        async fn policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::PolicyResponse>, tonic::Status>;
    }
    #[doc = " WatchtowerClient is a service that grants access to the watchtower client"]
    #[doc = " functionality of the daemon."]
    #[derive(Debug)]
    pub struct WatchtowerClientServer<T: WatchtowerClient> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WatchtowerClient> WatchtowerClientServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WatchtowerClientServer<T>
    where
        T: WatchtowerClient,
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
                "/wtclientrpc.WatchtowerClient/AddTower" => {
                    #[allow(non_camel_case_types)]
                    struct AddTowerSvc<T: WatchtowerClient>(pub Arc<T>);
                    impl<T: WatchtowerClient> tonic::server::UnaryService<super::AddTowerRequest> for AddTowerSvc<T> {
                        type Response = super::AddTowerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddTowerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_tower(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddTowerSvc(inner);
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
                "/wtclientrpc.WatchtowerClient/RemoveTower" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveTowerSvc<T: WatchtowerClient>(pub Arc<T>);
                    impl<T: WatchtowerClient> tonic::server::UnaryService<super::RemoveTowerRequest>
                        for RemoveTowerSvc<T>
                    {
                        type Response = super::RemoveTowerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveTowerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_tower(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveTowerSvc(inner);
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
                "/wtclientrpc.WatchtowerClient/ListTowers" => {
                    #[allow(non_camel_case_types)]
                    struct ListTowersSvc<T: WatchtowerClient>(pub Arc<T>);
                    impl<T: WatchtowerClient> tonic::server::UnaryService<super::ListTowersRequest>
                        for ListTowersSvc<T>
                    {
                        type Response = super::ListTowersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTowersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_towers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTowersSvc(inner);
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
                "/wtclientrpc.WatchtowerClient/GetTowerInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetTowerInfoSvc<T: WatchtowerClient>(pub Arc<T>);
                    impl<T: WatchtowerClient>
                        tonic::server::UnaryService<super::GetTowerInfoRequest>
                        for GetTowerInfoSvc<T>
                    {
                        type Response = super::Tower;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTowerInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_tower_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTowerInfoSvc(inner);
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
                "/wtclientrpc.WatchtowerClient/Stats" => {
                    #[allow(non_camel_case_types)]
                    struct StatsSvc<T: WatchtowerClient>(pub Arc<T>);
                    impl<T: WatchtowerClient> tonic::server::UnaryService<super::StatsRequest> for StatsSvc<T> {
                        type Response = super::StatsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stats(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatsSvc(inner);
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
                "/wtclientrpc.WatchtowerClient/Policy" => {
                    #[allow(non_camel_case_types)]
                    struct PolicySvc<T: WatchtowerClient>(pub Arc<T>);
                    impl<T: WatchtowerClient> tonic::server::UnaryService<super::PolicyRequest> for PolicySvc<T> {
                        type Response = super::PolicyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PolicySvc(inner);
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
    impl<T: WatchtowerClient> Clone for WatchtowerClientServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WatchtowerClient> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WatchtowerClient> tonic::transport::NamedService for WatchtowerClientServer<T> {
        const NAME: &'static str = "wtclientrpc.WatchtowerClient";
    }
}
