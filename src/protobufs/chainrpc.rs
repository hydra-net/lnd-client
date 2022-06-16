#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfRequest {
    ///
    ///The transaction hash for which we should request a confirmation notification
    ///for. If set to a hash of all zeros, then the confirmation notification will
    ///be requested for the script instead.
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    ///
    ///An output script within a transaction with the hash above which will be used
    ///by light clients to match block filters. If the transaction hash is set to a
    ///hash of all zeros, then a confirmation notification will be requested for
    ///this script instead.
    #[prost(bytes = "vec", tag = "2")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The number of desired confirmations the transaction/output script should
    ///reach before dispatching a confirmation notification.
    #[prost(uint32, tag = "3")]
    pub num_confs: u32,
    ///
    ///The earliest height in the chain for which the transaction/output script
    ///could have been included in a block. This should in most cases be set to the
    ///broadcast height of the transaction/output script.
    #[prost(uint32, tag = "4")]
    pub height_hint: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfDetails {
    /// The raw bytes of the confirmed transaction.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_tx: ::prost::alloc::vec::Vec<u8>,
    /// The hash of the block in which the confirmed transaction was included in.
    #[prost(bytes = "vec", tag = "2")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    /// The height of the block in which the confirmed transaction was included
    /// in.
    #[prost(uint32, tag = "3")]
    pub block_height: u32,
    /// The index of the confirmed transaction within the transaction.
    #[prost(uint32, tag = "4")]
    pub tx_index: u32,
}
/// TODO(wilmer): need to know how the client will use this first.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reorg {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfEvent {
    #[prost(oneof = "conf_event::Event", tags = "1, 2")]
    pub event: ::core::option::Option<conf_event::Event>,
}
/// Nested message and enum types in `ConfEvent`.
pub mod conf_event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        ///
        ///An event that includes the confirmation details of the request
        ///(txid/ouput script).
        #[prost(message, tag = "1")]
        Conf(super::ConfDetails),
        ///
        ///An event send when the transaction of the request is reorged out of the
        ///chain.
        #[prost(message, tag = "2")]
        Reorg(super::Reorg),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Outpoint {
    /// The hash of the transaction.
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// The index of the output within the transaction.
    #[prost(uint32, tag = "2")]
    pub index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendRequest {
    ///
    ///The outpoint for which we should request a spend notification for. If set to
    ///a zero outpoint, then the spend notification will be requested for the
    ///script instead. A zero or nil outpoint is not supported for Taproot spends
    ///because the output script cannot reliably be computed from the witness alone
    ///and the spent output script is not always available in the rescan context.
    ///So an outpoint must _always_ be specified when registering a spend
    ///notification for a Taproot output.
    #[prost(message, optional, tag = "1")]
    pub outpoint: ::core::option::Option<Outpoint>,
    ///
    ///The output script for the outpoint above. This will be used by light clients
    ///to match block filters. If the outpoint is set to a zero outpoint, then a
    ///spend notification will be requested for this script instead.
    #[prost(bytes = "vec", tag = "2")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The earliest height in the chain for which the outpoint/output script could
    ///have been spent. This should in most cases be set to the broadcast height of
    ///the outpoint/output script.
    #[prost(uint32, tag = "3")]
    pub height_hint: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendDetails {
    /// The outpoint was that spent.
    #[prost(message, optional, tag = "1")]
    pub spending_outpoint: ::core::option::Option<Outpoint>,
    /// The raw bytes of the spending transaction.
    #[prost(bytes = "vec", tag = "2")]
    pub raw_spending_tx: ::prost::alloc::vec::Vec<u8>,
    /// The hash of the spending transaction.
    #[prost(bytes = "vec", tag = "3")]
    pub spending_tx_hash: ::prost::alloc::vec::Vec<u8>,
    /// The input of the spending transaction that fulfilled the spend request.
    #[prost(uint32, tag = "4")]
    pub spending_input_index: u32,
    /// The height at which the spending transaction was included in a block.
    #[prost(uint32, tag = "5")]
    pub spending_height: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendEvent {
    #[prost(oneof = "spend_event::Event", tags = "1, 2")]
    pub event: ::core::option::Option<spend_event::Event>,
}
/// Nested message and enum types in `SpendEvent`.
pub mod spend_event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        ///
        ///An event that includes the details of the spending transaction of the
        ///request (outpoint/output script).
        #[prost(message, tag = "1")]
        Spend(super::SpendDetails),
        ///
        ///An event sent when the spending transaction of the request was
        ///reorged out of the chain.
        #[prost(message, tag = "2")]
        Reorg(super::Reorg),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockEpoch {
    /// The hash of the block.
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// The height of the block.
    #[prost(uint32, tag = "2")]
    pub height: u32,
}
#[doc = r" Generated client implementations."]
pub mod chain_notifier_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " ChainNotifier is a service that can be used to get information about the"]
    #[doc = " chain backend by registering notifiers for chain events."]
    #[derive(Debug, Clone)]
    pub struct ChainNotifierClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChainNotifierClient<tonic::transport::Channel> {
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
    impl<T> ChainNotifierClient<T>
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
        ) -> ChainNotifierClient<InterceptedService<T, F>>
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
            ChainNotifierClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "RegisterConfirmationsNtfn is a synchronous response-streaming RPC that"]
        #[doc = "registers an intent for a client to be notified once a confirmation request"]
        #[doc = "has reached its required number of confirmations on-chain."]
        #[doc = ""]
        #[doc = "A confirmation request must have a valid output script. It is also possible"]
        #[doc = "to give a transaction ID. If the transaction ID is not set, a notification"]
        #[doc = "is sent once the output script confirms. If the transaction ID is also set,"]
        #[doc = "a notification is sent once the output script confirms in the given"]
        #[doc = "transaction."]
        pub async fn register_confirmations_ntfn(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ConfEvent>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chainrpc.ChainNotifier/RegisterConfirmationsNtfn",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = "RegisterSpendNtfn is a synchronous response-streaming RPC that registers an"]
        #[doc = "intent for a client to be notification once a spend request has been spent"]
        #[doc = "by a transaction that has confirmed on-chain."]
        #[doc = ""]
        #[doc = "A client can specify whether the spend request should be for a particular"]
        #[doc = "outpoint  or for an output script by specifying a zero outpoint."]
        pub async fn register_spend_ntfn(
            &mut self,
            request: impl tonic::IntoRequest<super::SpendRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SpendEvent>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/chainrpc.ChainNotifier/RegisterSpendNtfn");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = "RegisterBlockEpochNtfn is a synchronous response-streaming RPC that"]
        #[doc = "registers an intent for a client to be notified of blocks in the chain. The"]
        #[doc = "stream will return a hash and height tuple of a block for each new/stale"]
        #[doc = "block in the chain. It is the client's responsibility to determine whether"]
        #[doc = "the tuple returned is for a new or stale block in the chain."]
        #[doc = ""]
        #[doc = "A client can also request a historical backlog of blocks from a particular"]
        #[doc = "point. This allows clients to be idempotent by ensuring that they do not"]
        #[doc = "missing processing a single block within the chain."]
        pub async fn register_block_epoch_ntfn(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockEpoch>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::BlockEpoch>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/chainrpc.ChainNotifier/RegisterBlockEpochNtfn",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod chain_notifier_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ChainNotifierServer."]
    #[async_trait]
    pub trait ChainNotifier: Send + Sync + 'static {
        #[doc = "Server streaming response type for the RegisterConfirmationsNtfn method."]
        type RegisterConfirmationsNtfnStream: futures_core::Stream<Item = Result<super::ConfEvent, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "RegisterConfirmationsNtfn is a synchronous response-streaming RPC that"]
        #[doc = "registers an intent for a client to be notified once a confirmation request"]
        #[doc = "has reached its required number of confirmations on-chain."]
        #[doc = ""]
        #[doc = "A confirmation request must have a valid output script. It is also possible"]
        #[doc = "to give a transaction ID. If the transaction ID is not set, a notification"]
        #[doc = "is sent once the output script confirms. If the transaction ID is also set,"]
        #[doc = "a notification is sent once the output script confirms in the given"]
        #[doc = "transaction."]
        async fn register_confirmations_ntfn(
            &self,
            request: tonic::Request<super::ConfRequest>,
        ) -> Result<tonic::Response<Self::RegisterConfirmationsNtfnStream>, tonic::Status>;
        #[doc = "Server streaming response type for the RegisterSpendNtfn method."]
        type RegisterSpendNtfnStream: futures_core::Stream<Item = Result<super::SpendEvent, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "RegisterSpendNtfn is a synchronous response-streaming RPC that registers an"]
        #[doc = "intent for a client to be notification once a spend request has been spent"]
        #[doc = "by a transaction that has confirmed on-chain."]
        #[doc = ""]
        #[doc = "A client can specify whether the spend request should be for a particular"]
        #[doc = "outpoint  or for an output script by specifying a zero outpoint."]
        async fn register_spend_ntfn(
            &self,
            request: tonic::Request<super::SpendRequest>,
        ) -> Result<tonic::Response<Self::RegisterSpendNtfnStream>, tonic::Status>;
        #[doc = "Server streaming response type for the RegisterBlockEpochNtfn method."]
        type RegisterBlockEpochNtfnStream: futures_core::Stream<Item = Result<super::BlockEpoch, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "RegisterBlockEpochNtfn is a synchronous response-streaming RPC that"]
        #[doc = "registers an intent for a client to be notified of blocks in the chain. The"]
        #[doc = "stream will return a hash and height tuple of a block for each new/stale"]
        #[doc = "block in the chain. It is the client's responsibility to determine whether"]
        #[doc = "the tuple returned is for a new or stale block in the chain."]
        #[doc = ""]
        #[doc = "A client can also request a historical backlog of blocks from a particular"]
        #[doc = "point. This allows clients to be idempotent by ensuring that they do not"]
        #[doc = "missing processing a single block within the chain."]
        async fn register_block_epoch_ntfn(
            &self,
            request: tonic::Request<super::BlockEpoch>,
        ) -> Result<tonic::Response<Self::RegisterBlockEpochNtfnStream>, tonic::Status>;
    }
    #[doc = " ChainNotifier is a service that can be used to get information about the"]
    #[doc = " chain backend by registering notifiers for chain events."]
    #[derive(Debug)]
    pub struct ChainNotifierServer<T: ChainNotifier> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ChainNotifier> ChainNotifierServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChainNotifierServer<T>
    where
        T: ChainNotifier,
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
                "/chainrpc.ChainNotifier/RegisterConfirmationsNtfn" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterConfirmationsNtfnSvc<T: ChainNotifier>(pub Arc<T>);
                    impl<T: ChainNotifier> tonic::server::ServerStreamingService<super::ConfRequest>
                        for RegisterConfirmationsNtfnSvc<T>
                    {
                        type Response = super::ConfEvent;
                        type ResponseStream = T::RegisterConfirmationsNtfnStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).register_confirmations_ntfn(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterConfirmationsNtfnSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chainrpc.ChainNotifier/RegisterSpendNtfn" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterSpendNtfnSvc<T: ChainNotifier>(pub Arc<T>);
                    impl<T: ChainNotifier>
                        tonic::server::ServerStreamingService<super::SpendRequest>
                        for RegisterSpendNtfnSvc<T>
                    {
                        type Response = super::SpendEvent;
                        type ResponseStream = T::RegisterSpendNtfnStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpendRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).register_spend_ntfn(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterSpendNtfnSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chainrpc.ChainNotifier/RegisterBlockEpochNtfn" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterBlockEpochNtfnSvc<T: ChainNotifier>(pub Arc<T>);
                    impl<T: ChainNotifier> tonic::server::ServerStreamingService<super::BlockEpoch>
                        for RegisterBlockEpochNtfnSvc<T>
                    {
                        type Response = super::BlockEpoch;
                        type ResponseStream = T::RegisterBlockEpochNtfnStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockEpoch>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).register_block_epoch_ntfn(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterBlockEpochNtfnSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: ChainNotifier> Clone for ChainNotifierServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ChainNotifier> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ChainNotifier> tonic::transport::NamedService for ChainNotifierServer<T> {
        const NAME: &'static str = "chainrpc.ChainNotifier";
    }
}
