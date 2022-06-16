#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    /// Indicates whether the autopilot is active or not.
    #[prost(bool, tag = "1")]
    pub active: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyStatusRequest {
    /// Whether the autopilot agent should be enabled or not.
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyStatusResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryScoresRequest {
    #[prost(string, repeated, tag = "1")]
    pub pubkeys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If set, we will ignore the local channel state when calculating scores.
    #[prost(bool, tag = "2")]
    pub ignore_local_state: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryScoresResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<query_scores_response::HeuristicResult>,
}
/// Nested message and enum types in `QueryScoresResponse`.
pub mod query_scores_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeuristicResult {
        #[prost(string, tag = "1")]
        pub heuristic: ::prost::alloc::string::String,
        #[prost(map = "string, double", tag = "2")]
        pub scores: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetScoresRequest {
    /// The name of the heuristic to provide scores to.
    #[prost(string, tag = "1")]
    pub heuristic: ::prost::alloc::string::String,
    ///
    ///A map from hex-encoded public keys to scores. Scores must be in the range
    ///[0.0, 1.0].
    #[prost(map = "string, double", tag = "2")]
    pub scores: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetScoresResponse {}
#[doc = r" Generated client implementations."]
pub mod autopilot_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Autopilot is a service that can be used to get information about the current"]
    #[doc = " state of the daemon's autopilot agent, and also supply it with information"]
    #[doc = " that can be used when deciding where to open channels."]
    #[derive(Debug, Clone)]
    pub struct AutopilotClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AutopilotClient<tonic::transport::Channel> {
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
    impl<T> AutopilotClient<T>
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
        ) -> AutopilotClient<InterceptedService<T, F>>
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
            AutopilotClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "Status returns whether the daemon's autopilot agent is active."]
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
            let path = http::uri::PathAndQuery::from_static("/autopilotrpc.Autopilot/Status");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ModifyStatus is used to modify the status of the autopilot agent, like"]
        #[doc = "enabling or disabling it."]
        pub async fn modify_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyStatusRequest>,
        ) -> Result<tonic::Response<super::ModifyStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/autopilotrpc.Autopilot/ModifyStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "QueryScores queries all available autopilot heuristics, in addition to any"]
        #[doc = "active combination of these heruristics, for the scores they would give to"]
        #[doc = "the given nodes."]
        pub async fn query_scores(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryScoresRequest>,
        ) -> Result<tonic::Response<super::QueryScoresResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/autopilotrpc.Autopilot/QueryScores");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SetScores attempts to set the scores used by the running autopilot agent,"]
        #[doc = "if the external scoring heuristic is enabled."]
        pub async fn set_scores(
            &mut self,
            request: impl tonic::IntoRequest<super::SetScoresRequest>,
        ) -> Result<tonic::Response<super::SetScoresResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/autopilotrpc.Autopilot/SetScores");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod autopilot_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AutopilotServer."]
    #[async_trait]
    pub trait Autopilot: Send + Sync + 'static {
        #[doc = ""]
        #[doc = "Status returns whether the daemon's autopilot agent is active."]
        async fn status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> Result<tonic::Response<super::StatusResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "ModifyStatus is used to modify the status of the autopilot agent, like"]
        #[doc = "enabling or disabling it."]
        async fn modify_status(
            &self,
            request: tonic::Request<super::ModifyStatusRequest>,
        ) -> Result<tonic::Response<super::ModifyStatusResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "QueryScores queries all available autopilot heuristics, in addition to any"]
        #[doc = "active combination of these heruristics, for the scores they would give to"]
        #[doc = "the given nodes."]
        async fn query_scores(
            &self,
            request: tonic::Request<super::QueryScoresRequest>,
        ) -> Result<tonic::Response<super::QueryScoresResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "SetScores attempts to set the scores used by the running autopilot agent,"]
        #[doc = "if the external scoring heuristic is enabled."]
        async fn set_scores(
            &self,
            request: tonic::Request<super::SetScoresRequest>,
        ) -> Result<tonic::Response<super::SetScoresResponse>, tonic::Status>;
    }
    #[doc = " Autopilot is a service that can be used to get information about the current"]
    #[doc = " state of the daemon's autopilot agent, and also supply it with information"]
    #[doc = " that can be used when deciding where to open channels."]
    #[derive(Debug)]
    pub struct AutopilotServer<T: Autopilot> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Autopilot> AutopilotServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AutopilotServer<T>
    where
        T: Autopilot,
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
                "/autopilotrpc.Autopilot/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: Autopilot>(pub Arc<T>);
                    impl<T: Autopilot> tonic::server::UnaryService<super::StatusRequest> for StatusSvc<T> {
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
                "/autopilotrpc.Autopilot/ModifyStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyStatusSvc<T: Autopilot>(pub Arc<T>);
                    impl<T: Autopilot> tonic::server::UnaryService<super::ModifyStatusRequest> for ModifyStatusSvc<T> {
                        type Response = super::ModifyStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModifyStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModifyStatusSvc(inner);
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
                "/autopilotrpc.Autopilot/QueryScores" => {
                    #[allow(non_camel_case_types)]
                    struct QueryScoresSvc<T: Autopilot>(pub Arc<T>);
                    impl<T: Autopilot> tonic::server::UnaryService<super::QueryScoresRequest> for QueryScoresSvc<T> {
                        type Response = super::QueryScoresResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryScoresRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_scores(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryScoresSvc(inner);
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
                "/autopilotrpc.Autopilot/SetScores" => {
                    #[allow(non_camel_case_types)]
                    struct SetScoresSvc<T: Autopilot>(pub Arc<T>);
                    impl<T: Autopilot> tonic::server::UnaryService<super::SetScoresRequest> for SetScoresSvc<T> {
                        type Response = super::SetScoresResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetScoresRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_scores(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetScoresSvc(inner);
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
    impl<T: Autopilot> Clone for AutopilotServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Autopilot> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Autopilot> tonic::transport::NamedService for AutopilotServer<T> {
        const NAME: &'static str = "autopilotrpc.Autopilot";
    }
}
