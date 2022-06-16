#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelInvoiceMsg {
    /// Hash corresponding to the (hold) invoice to cancel.
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelInvoiceResp {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHoldInvoiceRequest {
    ///
    ///An optional memo to attach along with the invoice. Used for record keeping
    ///purposes for the invoice's creator, and will also be set in the description
    ///field of the encoded payment request if the description_hash field is not
    ///being used.
    #[prost(string, tag = "1")]
    pub memo: ::prost::alloc::string::String,
    /// The hash of the preimage
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The value of this invoice in satoshis
    ///
    ///The fields value and value_msat are mutually exclusive.
    #[prost(int64, tag = "3")]
    pub value: i64,
    ///
    ///The value of this invoice in millisatoshis
    ///
    ///The fields value and value_msat are mutually exclusive.
    #[prost(int64, tag = "10")]
    pub value_msat: i64,
    ///
    ///Hash (SHA-256) of a description of the payment. Used if the description of
    ///payment (memo) is too long to naturally fit within the description field
    ///of an encoded payment request.
    #[prost(bytes = "vec", tag = "4")]
    pub description_hash: ::prost::alloc::vec::Vec<u8>,
    /// Payment request expiry time in seconds. Default is 3600 (1 hour).
    #[prost(int64, tag = "5")]
    pub expiry: i64,
    /// Fallback on-chain address.
    #[prost(string, tag = "6")]
    pub fallback_addr: ::prost::alloc::string::String,
    /// Delta to use for the time-lock of the CLTV extended to the final hop.
    #[prost(uint64, tag = "7")]
    pub cltv_expiry: u64,
    ///
    ///Route hints that can each be individually used to assist in reaching the
    ///invoice's destination.
    #[prost(message, repeated, tag = "8")]
    pub route_hints: ::prost::alloc::vec::Vec<super::lnrpc::RouteHint>,
    /// Whether this invoice should include routing hints for private channels.
    #[prost(bool, tag = "9")]
    pub private: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHoldInvoiceResp {
    ///
    ///A bare-bones invoice for a payment within the Lightning Network. With the
    ///details of the invoice, the sender has all the data necessary to send a
    ///payment to the recipient.
    #[prost(string, tag = "1")]
    pub payment_request: ::prost::alloc::string::String,
    ///
    ///The "add" index of this invoice. Each newly created invoice will increment
    ///this index making it monotonically increasing. Callers to the
    ///SubscribeInvoices call can use this to instantly get notified of all added
    ///invoices with an add_index greater than this one.
    #[prost(uint64, tag = "2")]
    pub add_index: u64,
    ///
    ///The payment address of the generated invoice. This value should be used
    ///in all payments for this invoice as we require it for end to end
    ///security.
    #[prost(bytes = "vec", tag = "3")]
    pub payment_addr: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettleInvoiceMsg {
    /// Externally discovered pre-image that should be used to settle the hold
    /// invoice.
    #[prost(bytes = "vec", tag = "1")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettleInvoiceResp {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeSingleInvoiceRequest {
    /// Hash corresponding to the (hold) invoice to subscribe to.
    #[prost(bytes = "vec", tag = "2")]
    pub r_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupInvoiceMsg {
    #[prost(enumeration = "LookupModifier", tag = "4")]
    pub lookup_modifier: i32,
    #[prost(oneof = "lookup_invoice_msg::InvoiceRef", tags = "1, 2, 3")]
    pub invoice_ref: ::core::option::Option<lookup_invoice_msg::InvoiceRef>,
}
/// Nested message and enum types in `LookupInvoiceMsg`.
pub mod lookup_invoice_msg {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InvoiceRef {
        #[prost(bytes, tag = "1")]
        PaymentHash(::prost::alloc::vec::Vec<u8>),
        #[prost(bytes, tag = "2")]
        PaymentAddr(::prost::alloc::vec::Vec<u8>),
        #[prost(bytes, tag = "3")]
        SetId(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LookupModifier {
    /// The default look up modifier, no look up behavior is changed.
    Default = 0,
    ///
    ///Indicates that when a look up is done based on a set_id, then only that set
    ///of HTLCs related to that set ID should be returned.
    HtlcSetOnly = 1,
    ///
    ///Indicates that when a look up is done using a payment_addr, then no HTLCs
    ///related to the payment_addr should be returned. This is useful when one
    ///wants to be able to obtain the set of associated setIDs with a given
    ///invoice, then look up the sub-invoices "projected" by that set ID.
    HtlcSetBlank = 2,
}
#[doc = r" Generated client implementations."]
pub mod invoices_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Invoices is a service that can be used to create, accept, settle and cancel"]
    #[doc = " invoices."]
    #[derive(Debug, Clone)]
    pub struct InvoicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InvoicesClient<tonic::transport::Channel> {
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
    impl<T> InvoicesClient<T>
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
        ) -> InvoicesClient<InterceptedService<T, F>>
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
            InvoicesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "SubscribeSingleInvoice returns a uni-directional stream (server -> client)"]
        #[doc = "to notify the client of state transitions of the specified invoice."]
        #[doc = "Initially the current invoice state is always sent out."]
        pub async fn subscribe_single_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeSingleInvoiceRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::lnrpc::Invoice>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/invoicesrpc.Invoices/SubscribeSingleInvoice",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = "CancelInvoice cancels a currently open invoice. If the invoice is already"]
        #[doc = "canceled, this call will succeed. If the invoice is already settled, it will"]
        #[doc = "fail."]
        pub async fn cancel_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelInvoiceMsg>,
        ) -> Result<tonic::Response<super::CancelInvoiceResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/invoicesrpc.Invoices/CancelInvoice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "AddHoldInvoice creates a hold invoice. It ties the invoice to the hash"]
        #[doc = "supplied in the request."]
        pub async fn add_hold_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::AddHoldInvoiceRequest>,
        ) -> Result<tonic::Response<super::AddHoldInvoiceResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/invoicesrpc.Invoices/AddHoldInvoice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SettleInvoice settles an accepted invoice. If the invoice is already"]
        #[doc = "settled, this call will succeed."]
        pub async fn settle_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::SettleInvoiceMsg>,
        ) -> Result<tonic::Response<super::SettleInvoiceResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/invoicesrpc.Invoices/SettleInvoice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "LookupInvoiceV2 attempts to look up at invoice. An invoice can be refrenced"]
        #[doc = "using either its payment hash, payment address, or set ID."]
        pub async fn lookup_invoice_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupInvoiceMsg>,
        ) -> Result<tonic::Response<super::super::lnrpc::Invoice>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/invoicesrpc.Invoices/LookupInvoiceV2");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod invoices_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with InvoicesServer."]
    #[async_trait]
    pub trait Invoices: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SubscribeSingleInvoice method."]
        type SubscribeSingleInvoiceStream: futures_core::Stream<Item = Result<super::super::lnrpc::Invoice, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "SubscribeSingleInvoice returns a uni-directional stream (server -> client)"]
        #[doc = "to notify the client of state transitions of the specified invoice."]
        #[doc = "Initially the current invoice state is always sent out."]
        async fn subscribe_single_invoice(
            &self,
            request: tonic::Request<super::SubscribeSingleInvoiceRequest>,
        ) -> Result<tonic::Response<Self::SubscribeSingleInvoiceStream>, tonic::Status>;
        #[doc = ""]
        #[doc = "CancelInvoice cancels a currently open invoice. If the invoice is already"]
        #[doc = "canceled, this call will succeed. If the invoice is already settled, it will"]
        #[doc = "fail."]
        async fn cancel_invoice(
            &self,
            request: tonic::Request<super::CancelInvoiceMsg>,
        ) -> Result<tonic::Response<super::CancelInvoiceResp>, tonic::Status>;
        #[doc = ""]
        #[doc = "AddHoldInvoice creates a hold invoice. It ties the invoice to the hash"]
        #[doc = "supplied in the request."]
        async fn add_hold_invoice(
            &self,
            request: tonic::Request<super::AddHoldInvoiceRequest>,
        ) -> Result<tonic::Response<super::AddHoldInvoiceResp>, tonic::Status>;
        #[doc = ""]
        #[doc = "SettleInvoice settles an accepted invoice. If the invoice is already"]
        #[doc = "settled, this call will succeed."]
        async fn settle_invoice(
            &self,
            request: tonic::Request<super::SettleInvoiceMsg>,
        ) -> Result<tonic::Response<super::SettleInvoiceResp>, tonic::Status>;
        #[doc = ""]
        #[doc = "LookupInvoiceV2 attempts to look up at invoice. An invoice can be refrenced"]
        #[doc = "using either its payment hash, payment address, or set ID."]
        async fn lookup_invoice_v2(
            &self,
            request: tonic::Request<super::LookupInvoiceMsg>,
        ) -> Result<tonic::Response<super::super::lnrpc::Invoice>, tonic::Status>;
    }
    #[doc = " Invoices is a service that can be used to create, accept, settle and cancel"]
    #[doc = " invoices."]
    #[derive(Debug)]
    pub struct InvoicesServer<T: Invoices> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Invoices> InvoicesServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InvoicesServer<T>
    where
        T: Invoices,
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
                "/invoicesrpc.Invoices/SubscribeSingleInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSingleInvoiceSvc<T: Invoices>(pub Arc<T>);
                    impl<T: Invoices>
                        tonic::server::ServerStreamingService<super::SubscribeSingleInvoiceRequest>
                        for SubscribeSingleInvoiceSvc<T>
                    {
                        type Response = super::super::lnrpc::Invoice;
                        type ResponseStream = T::SubscribeSingleInvoiceStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeSingleInvoiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_single_invoice(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeSingleInvoiceSvc(inner);
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
                "/invoicesrpc.Invoices/CancelInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct CancelInvoiceSvc<T: Invoices>(pub Arc<T>);
                    impl<T: Invoices> tonic::server::UnaryService<super::CancelInvoiceMsg> for CancelInvoiceSvc<T> {
                        type Response = super::CancelInvoiceResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelInvoiceMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel_invoice(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelInvoiceSvc(inner);
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
                "/invoicesrpc.Invoices/AddHoldInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct AddHoldInvoiceSvc<T: Invoices>(pub Arc<T>);
                    impl<T: Invoices> tonic::server::UnaryService<super::AddHoldInvoiceRequest>
                        for AddHoldInvoiceSvc<T>
                    {
                        type Response = super::AddHoldInvoiceResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddHoldInvoiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_hold_invoice(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddHoldInvoiceSvc(inner);
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
                "/invoicesrpc.Invoices/SettleInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct SettleInvoiceSvc<T: Invoices>(pub Arc<T>);
                    impl<T: Invoices> tonic::server::UnaryService<super::SettleInvoiceMsg> for SettleInvoiceSvc<T> {
                        type Response = super::SettleInvoiceResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SettleInvoiceMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).settle_invoice(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SettleInvoiceSvc(inner);
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
                "/invoicesrpc.Invoices/LookupInvoiceV2" => {
                    #[allow(non_camel_case_types)]
                    struct LookupInvoiceV2Svc<T: Invoices>(pub Arc<T>);
                    impl<T: Invoices> tonic::server::UnaryService<super::LookupInvoiceMsg> for LookupInvoiceV2Svc<T> {
                        type Response = super::super::lnrpc::Invoice;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LookupInvoiceMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).lookup_invoice_v2(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LookupInvoiceV2Svc(inner);
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
    impl<T: Invoices> Clone for InvoicesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Invoices> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Invoices> tonic::transport::NamedService for InvoicesServer<T> {
        const NAME: &'static str = "invoicesrpc.Invoices";
    }
}
