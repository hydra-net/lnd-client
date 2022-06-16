#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyLocator {
    /// The family of key being identified.
    #[prost(int32, tag = "1")]
    pub key_family: i32,
    /// The precise index of the key being identified.
    #[prost(int32, tag = "2")]
    pub key_index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyDescriptor {
    ///
    ///The raw bytes of the public key in the key pair being identified. Either
    ///this or the KeyLocator must be specified.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_key_bytes: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The key locator that identifies which private key to use for signing.
    ///Either this or the raw bytes of the target public key must be specified.
    #[prost(message, optional, tag = "2")]
    pub key_loc: ::core::option::Option<KeyLocator>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOut {
    /// The value of the output being spent.
    #[prost(int64, tag = "1")]
    pub value: i64,
    /// The script of the output being spent.
    #[prost(bytes = "vec", tag = "2")]
    pub pk_script: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignDescriptor {
    ///
    ///A descriptor that precisely describes *which* key to use for signing. This
    ///may provide the raw public key directly, or require the Signer to re-derive
    ///the key according to the populated derivation path.
    ///
    ///Note that if the key descriptor was obtained through walletrpc.DeriveKey,
    ///then the key locator MUST always be provided, since the derived keys are not
    ///persisted unlike with DeriveNextKey.
    #[prost(message, optional, tag = "1")]
    pub key_desc: ::core::option::Option<KeyDescriptor>,
    ///
    ///A scalar value that will be added to the private key corresponding to the
    ///above public key to obtain the private key to be used to sign this input.
    ///This value is typically derived via the following computation:
    ///
    /// derivedKey = privkey + sha256(perCommitmentPoint || pubKey) mod N
    #[prost(bytes = "vec", tag = "2")]
    pub single_tweak: ::prost::alloc::vec::Vec<u8>,
    ///
    ///A private key that will be used in combination with its corresponding
    ///private key to derive the private key that is to be used to sign the target
    ///input. Within the Lightning protocol, this value is typically the
    ///commitment secret from a previously revoked commitment transaction. This
    ///value is in combination with two hash values, and the original private key
    ///to derive the private key to be used when signing.
    ///
    /// k = (privKey*sha256(pubKey || tweakPub) +
    ///tweakPriv*sha256(tweakPub || pubKey)) mod N
    #[prost(bytes = "vec", tag = "3")]
    pub double_tweak: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The 32 byte input to the taproot tweak derivation that is used to derive
    ///the output key from an internal key: outputKey = internalKey +
    ///tagged_hash("tapTweak", internalKey || tapTweak).
    ///
    ///When doing a BIP 86 spend, this field can be an empty byte slice.
    ///
    ///When doing a normal key path spend, with the output key committing to an
    ///actual script root, then this field should be: the tapscript root hash.
    #[prost(bytes = "vec", tag = "10")]
    pub tap_tweak: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The full script required to properly redeem the output. This field will
    ///only be populated if a p2tr, p2wsh or a p2sh output is being signed. If a
    ///taproot script path spend is being attempted, then this should be the raw
    ///leaf script.
    #[prost(bytes = "vec", tag = "4")]
    pub witness_script: ::prost::alloc::vec::Vec<u8>,
    ///
    ///A description of the output being spent. The value and script MUST be
    ///provided.
    #[prost(message, optional, tag = "5")]
    pub output: ::core::option::Option<TxOut>,
    ///
    ///The target sighash type that should be used when generating the final
    ///sighash, and signature.
    #[prost(uint32, tag = "7")]
    pub sighash: u32,
    ///
    ///The target input within the transaction that should be signed.
    #[prost(int32, tag = "8")]
    pub input_index: i32,
    ///
    ///The sign method specifies how the input should be signed. Depending on the
    ///method, either the tap_tweak, witness_script or both need to be specified.
    ///Defaults to SegWit v0 signing to be backward compatible with older RPC
    ///clients.
    #[prost(enumeration = "SignMethod", tag = "9")]
    pub sign_method: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignReq {
    /// The raw bytes of the transaction to be signed.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_tx_bytes: ::prost::alloc::vec::Vec<u8>,
    /// A set of sign descriptors, for each input to be signed.
    #[prost(message, repeated, tag = "2")]
    pub sign_descs: ::prost::alloc::vec::Vec<SignDescriptor>,
    ///
    ///The full list of UTXO information for each of the inputs being spent. This
    ///is required when spending one or more taproot (SegWit v1) outputs.
    #[prost(message, repeated, tag = "3")]
    pub prev_outputs: ::prost::alloc::vec::Vec<TxOut>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignResp {
    ///
    ///A set of signatures realized in a fixed 64-byte format ordered in ascending
    ///input order.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub raw_sigs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputScript {
    /// The serializes witness stack for the specified input.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub witness: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    ///
    ///The optional sig script for the specified witness that will only be set if
    ///the input specified is a nested p2sh witness program.
    #[prost(bytes = "vec", tag = "2")]
    pub sig_script: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputScriptResp {
    /// The set of fully valid input scripts requested.
    #[prost(message, repeated, tag = "1")]
    pub input_scripts: ::prost::alloc::vec::Vec<InputScript>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageReq {
    /// The message to be signed.
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// The key locator that identifies which key to use for signing.
    #[prost(message, optional, tag = "2")]
    pub key_loc: ::core::option::Option<KeyLocator>,
    /// Double-SHA256 hash instead of just the default single round.
    #[prost(bool, tag = "3")]
    pub double_hash: bool,
    ///
    ///Use the compact (pubkey recoverable) format instead of the raw lnwire
    ///format.
    #[prost(bool, tag = "4")]
    pub compact_sig: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageResp {
    ///
    ///The signature for the given message in the fixed-size LN wire format.
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMessageReq {
    /// The message over which the signature is to be verified.
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The fixed-size LN wire encoded signature to be verified over the given
    ///message.
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// The public key the signature has to be valid for.
    #[prost(bytes = "vec", tag = "3")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMessageResp {
    /// Whether the signature was valid over the given message.
    #[prost(bool, tag = "1")]
    pub valid: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedKeyRequest {
    /// The ephemeral public key to use for the DH key derivation.
    #[prost(bytes = "vec", tag = "1")]
    pub ephemeral_pubkey: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Deprecated. The optional key locator of the local key that should be used.
    ///If this parameter is not set then the node's identity private key will be
    ///used.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub key_loc: ::core::option::Option<KeyLocator>,
    ///
    ///A key descriptor describes the key used for performing ECDH. Either a key
    ///locator or a raw public key is expected, if neither is supplied, defaults to
    ///the node's identity private key.
    #[prost(message, optional, tag = "3")]
    pub key_desc: ::core::option::Option<KeyDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedKeyResponse {
    /// The shared public key, hashed with sha256.
    #[prost(bytes = "vec", tag = "1")]
    pub shared_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TweakDesc {
    ///
    ///Tweak is the 32-byte value that will modify the public key.
    #[prost(bytes = "vec", tag = "1")]
    pub tweak: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Specifies if the target key should be converted to an x-only public key
    ///before tweaking. If true, then the public key will be mapped to an x-only
    ///key before the tweaking operation is applied.
    #[prost(bool, tag = "2")]
    pub is_x_only: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaprootTweakDesc {
    ///
    ///The root hash of the tapscript tree if a script path is committed to. If
    ///the MuSig2 key put on chain doesn't also commit to a script path (BIP-0086
    ///key spend only), then this needs to be empty and the key_spend_only field
    ///below must be set to true. This is required because gRPC cannot
    ///differentiate between a zero-size byte slice and a nil byte slice (both
    ///would be serialized the same way). So the extra boolean is required.
    #[prost(bytes = "vec", tag = "1")]
    pub script_root: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Indicates that the above script_root is expected to be empty because this
    ///is a BIP-0086 key spend only commitment where only the internal key is
    ///committed to instead of also including a script root hash.
    #[prost(bool, tag = "2")]
    pub key_spend_only: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2CombineKeysRequest {
    ///
    ///A list of all public keys (serialized in 32-byte x-only format!)
    ///participating in the signing session. The list will always be sorted
    ///lexicographically internally. This must include the local key which is
    ///described by the above key_loc.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub all_signer_pubkeys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    ///
    ///A series of optional generic tweaks to be applied to the the aggregated
    ///public key.
    #[prost(message, repeated, tag = "2")]
    pub tweaks: ::prost::alloc::vec::Vec<TweakDesc>,
    ///
    ///An optional taproot specific tweak that must be specified if the MuSig2
    ///combined key will be used as the main taproot key of a taproot output
    ///on-chain.
    #[prost(message, optional, tag = "3")]
    pub taproot_tweak: ::core::option::Option<TaprootTweakDesc>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2CombineKeysResponse {
    ///
    ///The combined public key (in the 32-byte x-only format) with all tweaks
    ///applied to it. If a taproot tweak is specified, this corresponds to the
    ///taproot key that can be put into the on-chain output.
    #[prost(bytes = "vec", tag = "1")]
    pub combined_key: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The raw combined public key (in the 32-byte x-only format) before any tweaks
    ///are applied to it. If a taproot tweak is specified, this corresponds to the
    ///internal key that needs to be put into the witness if the script spend path
    ///is used.
    #[prost(bytes = "vec", tag = "2")]
    pub taproot_internal_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2SessionRequest {
    ///
    ///The key locator that identifies which key to use for signing.
    #[prost(message, optional, tag = "1")]
    pub key_loc: ::core::option::Option<KeyLocator>,
    ///
    ///A list of all public keys (serialized in 32-byte x-only format!)
    ///participating in the signing session. The list will always be sorted
    ///lexicographically internally. This must include the local key which is
    ///described by the above key_loc.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub all_signer_pubkeys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    ///
    ///An optional list of all public nonces of other signing participants that
    ///might already be known.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub other_signer_public_nonces: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    ///
    ///A series of optional generic tweaks to be applied to the the aggregated
    ///public key.
    #[prost(message, repeated, tag = "4")]
    pub tweaks: ::prost::alloc::vec::Vec<TweakDesc>,
    ///
    ///An optional taproot specific tweak that must be specified if the MuSig2
    ///combined key will be used as the main taproot key of a taproot output
    ///on-chain.
    #[prost(message, optional, tag = "5")]
    pub taproot_tweak: ::core::option::Option<TaprootTweakDesc>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2SessionResponse {
    ///
    ///The unique ID that represents this signing session. A session can be used
    ///for producing a signature a single time. If the signing fails for any
    ///reason, a new session with the same participants needs to be created.
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The combined public key (in the 32-byte x-only format) with all tweaks
    ///applied to it. If a taproot tweak is specified, this corresponds to the
    ///taproot key that can be put into the on-chain output.
    #[prost(bytes = "vec", tag = "2")]
    pub combined_key: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The raw combined public key (in the 32-byte x-only format) before any tweaks
    ///are applied to it. If a taproot tweak is specified, this corresponds to the
    ///internal key that needs to be put into the witness if the script spend path
    ///is used.
    #[prost(bytes = "vec", tag = "3")]
    pub taproot_internal_key: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The two public nonces the local signer uses, combined into a single value
    ///of 66 bytes. Can be split into the two 33-byte points to get the individual
    ///nonces.
    #[prost(bytes = "vec", tag = "4")]
    pub local_public_nonces: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Indicates whether all nonces required to start the signing process are known
    ///now.
    #[prost(bool, tag = "5")]
    pub have_all_nonces: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2RegisterNoncesRequest {
    ///
    ///The unique ID of the signing session those nonces should be registered with.
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///A list of all public nonces of other signing participants that should be
    ///registered.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub other_signer_public_nonces: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2RegisterNoncesResponse {
    ///
    ///Indicates whether all nonces required to start the signing process are known
    ///now.
    #[prost(bool, tag = "1")]
    pub have_all_nonces: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2SignRequest {
    ///
    ///The unique ID of the signing session to use for signing.
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The 32-byte SHA256 digest of the message to sign.
    #[prost(bytes = "vec", tag = "2")]
    pub message_digest: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Cleanup indicates that after signing, the session state can be cleaned up,
    ///since another participant is going to be responsible for combining the
    ///partial signatures.
    #[prost(bool, tag = "3")]
    pub cleanup: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2SignResponse {
    ///
    ///The partial signature created by the local signer.
    #[prost(bytes = "vec", tag = "1")]
    pub local_partial_signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2CombineSigRequest {
    ///
    ///The unique ID of the signing session to combine the signatures for.
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The list of all other participants' partial signatures to add to the current
    ///session.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub other_partial_signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2CombineSigResponse {
    ///
    ///Indicates whether all partial signatures required to create a final, full
    ///signature are known yet. If this is true, then the final_signature field is
    ///set, otherwise it is empty.
    #[prost(bool, tag = "1")]
    pub have_all_signatures: bool,
    ///
    ///The final, full signature that is valid for the combined public key.
    #[prost(bytes = "vec", tag = "2")]
    pub final_signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2CleanupRequest {
    ///
    ///The unique ID of the signing session that should be removed/cleaned up.
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuSig2CleanupResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignMethod {
    ///
    ///Specifies that a SegWit v0 (p2wkh, np2wkh, p2wsh) input script should be
    ///signed.
    WitnessV0 = 0,
    ///
    ///Specifies that a SegWit v1 (p2tr) input should be signed by using the
    ///BIP0086 method (commit to internal key only).
    TaprootKeySpendBip0086 = 1,
    ///
    ///Specifies that a SegWit v1 (p2tr) input should be signed by using a given
    ///taproot hash to commit to in addition to the internal key.
    TaprootKeySpend = 2,
    ///
    ///Specifies that a SegWit v1 (p2tr) input should be spent using the script
    ///path and that a specific leaf script should be signed for.
    TaprootScriptSpend = 3,
}
#[doc = r" Generated client implementations."]
pub mod signer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Signer is a service that gives access to the signing functionality of the"]
    #[doc = " daemon's wallet."]
    #[derive(Debug, Clone)]
    pub struct SignerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SignerClient<tonic::transport::Channel> {
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
    impl<T> SignerClient<T>
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
        ) -> SignerClient<InterceptedService<T, F>>
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
            SignerClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "SignOutputRaw is a method that can be used to generated a signature for a"]
        #[doc = "set of inputs/outputs to a transaction. Each request specifies details"]
        #[doc = "concerning how the outputs should be signed, which keys they should be"]
        #[doc = "signed with, and also any optional tweaks. The return value is a fixed"]
        #[doc = "64-byte signature (the same format as we use on the wire in Lightning)."]
        #[doc = ""]
        #[doc = "If we are  unable to sign using the specified keys, then an error will be"]
        #[doc = "returned."]
        pub async fn sign_output_raw(
            &mut self,
            request: impl tonic::IntoRequest<super::SignReq>,
        ) -> Result<tonic::Response<super::SignResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/SignOutputRaw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ComputeInputScript generates a complete InputIndex for the passed"]
        #[doc = "transaction with the signature as defined within the passed SignDescriptor."]
        #[doc = "This method should be capable of generating the proper input script for"]
        #[doc = "both regular p2wkh output and p2wkh outputs nested within a regular p2sh"]
        #[doc = "output."]
        #[doc = ""]
        #[doc = "Note that when using this method to sign inputs belonging to the wallet,"]
        #[doc = "the only items of the SignDescriptor that need to be populated are pkScript"]
        #[doc = "in the TxOut field, the value in that same field, and finally the input"]
        #[doc = "index."]
        pub async fn compute_input_script(
            &mut self,
            request: impl tonic::IntoRequest<super::SignReq>,
        ) -> Result<tonic::Response<super::InputScriptResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/ComputeInputScript");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SignMessage signs a message with the key specified in the key locator. The"]
        #[doc = "returned signature is fixed-size LN wire format encoded."]
        #[doc = ""]
        #[doc = "The main difference to SignMessage in the main RPC is that a specific key is"]
        #[doc = "used to sign the message instead of the node identity private key."]
        pub async fn sign_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessageReq>,
        ) -> Result<tonic::Response<super::SignMessageResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/SignMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "VerifyMessage verifies a signature over a message using the public key"]
        #[doc = "provided. The signature must be fixed-size LN wire format encoded."]
        #[doc = ""]
        #[doc = "The main difference to VerifyMessage in the main RPC is that the public key"]
        #[doc = "used to sign the message does not have to be a node known to the network."]
        pub async fn verify_message(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMessageReq>,
        ) -> Result<tonic::Response<super::VerifyMessageResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/VerifyMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DeriveSharedKey returns a shared secret key by performing Diffie-Hellman key"]
        #[doc = "derivation between the ephemeral public key in the request and the node's"]
        #[doc = "key specified in the key_desc parameter. Either a key locator or a raw"]
        #[doc = "public key is expected in the key_desc, if neither is supplied, defaults to"]
        #[doc = "the node's identity private key:"]
        #[doc = "P_shared = privKeyNode * ephemeralPubkey"]
        #[doc = "The resulting shared public key is serialized in the compressed format and"]
        #[doc = "hashed with sha256, resulting in the final key length of 256bit."]
        pub async fn derive_shared_key(
            &mut self,
            request: impl tonic::IntoRequest<super::SharedKeyRequest>,
        ) -> Result<tonic::Response<super::SharedKeyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/DeriveSharedKey");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "MuSig2CombineKeys (experimental!) is a stateless helper RPC that can be used"]
        #[doc = "to calculate the combined MuSig2 public key from a list of all participating"]
        #[doc = "signers' public keys. This RPC is completely stateless and deterministic and"]
        #[doc = "does not create any signing session. It can be used to determine the Taproot"]
        #[doc = "public key that should be put in an on-chain output once all public keys are"]
        #[doc = "known. A signing session is only needed later when that output should be"]
        #[doc = "_spent_ again."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        pub async fn mu_sig2_combine_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::MuSig2CombineKeysRequest>,
        ) -> Result<tonic::Response<super::MuSig2CombineKeysResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/MuSig2CombineKeys");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "MuSig2CreateSession (experimental!) creates a new MuSig2 signing session"]
        #[doc = "using the local key identified by the key locator. The complete list of all"]
        #[doc = "public keys of all signing parties must be provided, including the public"]
        #[doc = "key of the local signing key. If nonces of other parties are already known,"]
        #[doc = "they can be submitted as well to reduce the number of RPC calls necessary"]
        #[doc = "later on."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        pub async fn mu_sig2_create_session(
            &mut self,
            request: impl tonic::IntoRequest<super::MuSig2SessionRequest>,
        ) -> Result<tonic::Response<super::MuSig2SessionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/MuSig2CreateSession");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "MuSig2RegisterNonces (experimental!) registers one or more public nonces of"]
        #[doc = "other signing participants for a session identified by its ID. This RPC can"]
        #[doc = "be called multiple times until all nonces are registered."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        pub async fn mu_sig2_register_nonces(
            &mut self,
            request: impl tonic::IntoRequest<super::MuSig2RegisterNoncesRequest>,
        ) -> Result<tonic::Response<super::MuSig2RegisterNoncesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/MuSig2RegisterNonces");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "MuSig2Sign (experimental!) creates a partial signature using the local"]
        #[doc = "signing key that was specified when the session was created. This can only"]
        #[doc = "be called when all public nonces of all participants are known and have been"]
        #[doc = "registered with the session. If this node isn't responsible for combining"]
        #[doc = "all the partial signatures, then the cleanup flag should be set, indicating"]
        #[doc = "that the session can be removed from memory once the signature was produced."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        pub async fn mu_sig2_sign(
            &mut self,
            request: impl tonic::IntoRequest<super::MuSig2SignRequest>,
        ) -> Result<tonic::Response<super::MuSig2SignResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/MuSig2Sign");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "MuSig2CombineSig (experimental!) combines the given partial signature(s)"]
        #[doc = "with the local one, if it already exists. Once a partial signature of all"]
        #[doc = "participants is registered, the final signature will be combined and"]
        #[doc = "returned."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        pub async fn mu_sig2_combine_sig(
            &mut self,
            request: impl tonic::IntoRequest<super::MuSig2CombineSigRequest>,
        ) -> Result<tonic::Response<super::MuSig2CombineSigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/MuSig2CombineSig");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "MuSig2Cleanup (experimental!) allows a caller to clean up a session early in"]
        #[doc = "cases where it's obvious that the signing session won't succeed and the"]
        #[doc = "resources can be released."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        pub async fn mu_sig2_cleanup(
            &mut self,
            request: impl tonic::IntoRequest<super::MuSig2CleanupRequest>,
        ) -> Result<tonic::Response<super::MuSig2CleanupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/signrpc.Signer/MuSig2Cleanup");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod signer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SignerServer."]
    #[async_trait]
    pub trait Signer: Send + Sync + 'static {
        #[doc = ""]
        #[doc = "SignOutputRaw is a method that can be used to generated a signature for a"]
        #[doc = "set of inputs/outputs to a transaction. Each request specifies details"]
        #[doc = "concerning how the outputs should be signed, which keys they should be"]
        #[doc = "signed with, and also any optional tweaks. The return value is a fixed"]
        #[doc = "64-byte signature (the same format as we use on the wire in Lightning)."]
        #[doc = ""]
        #[doc = "If we are  unable to sign using the specified keys, then an error will be"]
        #[doc = "returned."]
        async fn sign_output_raw(
            &self,
            request: tonic::Request<super::SignReq>,
        ) -> Result<tonic::Response<super::SignResp>, tonic::Status>;
        #[doc = ""]
        #[doc = "ComputeInputScript generates a complete InputIndex for the passed"]
        #[doc = "transaction with the signature as defined within the passed SignDescriptor."]
        #[doc = "This method should be capable of generating the proper input script for"]
        #[doc = "both regular p2wkh output and p2wkh outputs nested within a regular p2sh"]
        #[doc = "output."]
        #[doc = ""]
        #[doc = "Note that when using this method to sign inputs belonging to the wallet,"]
        #[doc = "the only items of the SignDescriptor that need to be populated are pkScript"]
        #[doc = "in the TxOut field, the value in that same field, and finally the input"]
        #[doc = "index."]
        async fn compute_input_script(
            &self,
            request: tonic::Request<super::SignReq>,
        ) -> Result<tonic::Response<super::InputScriptResp>, tonic::Status>;
        #[doc = ""]
        #[doc = "SignMessage signs a message with the key specified in the key locator. The"]
        #[doc = "returned signature is fixed-size LN wire format encoded."]
        #[doc = ""]
        #[doc = "The main difference to SignMessage in the main RPC is that a specific key is"]
        #[doc = "used to sign the message instead of the node identity private key."]
        async fn sign_message(
            &self,
            request: tonic::Request<super::SignMessageReq>,
        ) -> Result<tonic::Response<super::SignMessageResp>, tonic::Status>;
        #[doc = ""]
        #[doc = "VerifyMessage verifies a signature over a message using the public key"]
        #[doc = "provided. The signature must be fixed-size LN wire format encoded."]
        #[doc = ""]
        #[doc = "The main difference to VerifyMessage in the main RPC is that the public key"]
        #[doc = "used to sign the message does not have to be a node known to the network."]
        async fn verify_message(
            &self,
            request: tonic::Request<super::VerifyMessageReq>,
        ) -> Result<tonic::Response<super::VerifyMessageResp>, tonic::Status>;
        #[doc = ""]
        #[doc = "DeriveSharedKey returns a shared secret key by performing Diffie-Hellman key"]
        #[doc = "derivation between the ephemeral public key in the request and the node's"]
        #[doc = "key specified in the key_desc parameter. Either a key locator or a raw"]
        #[doc = "public key is expected in the key_desc, if neither is supplied, defaults to"]
        #[doc = "the node's identity private key:"]
        #[doc = "P_shared = privKeyNode * ephemeralPubkey"]
        #[doc = "The resulting shared public key is serialized in the compressed format and"]
        #[doc = "hashed with sha256, resulting in the final key length of 256bit."]
        async fn derive_shared_key(
            &self,
            request: tonic::Request<super::SharedKeyRequest>,
        ) -> Result<tonic::Response<super::SharedKeyResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "MuSig2CombineKeys (experimental!) is a stateless helper RPC that can be used"]
        #[doc = "to calculate the combined MuSig2 public key from a list of all participating"]
        #[doc = "signers' public keys. This RPC is completely stateless and deterministic and"]
        #[doc = "does not create any signing session. It can be used to determine the Taproot"]
        #[doc = "public key that should be put in an on-chain output once all public keys are"]
        #[doc = "known. A signing session is only needed later when that output should be"]
        #[doc = "_spent_ again."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        async fn mu_sig2_combine_keys(
            &self,
            request: tonic::Request<super::MuSig2CombineKeysRequest>,
        ) -> Result<tonic::Response<super::MuSig2CombineKeysResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "MuSig2CreateSession (experimental!) creates a new MuSig2 signing session"]
        #[doc = "using the local key identified by the key locator. The complete list of all"]
        #[doc = "public keys of all signing parties must be provided, including the public"]
        #[doc = "key of the local signing key. If nonces of other parties are already known,"]
        #[doc = "they can be submitted as well to reduce the number of RPC calls necessary"]
        #[doc = "later on."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        async fn mu_sig2_create_session(
            &self,
            request: tonic::Request<super::MuSig2SessionRequest>,
        ) -> Result<tonic::Response<super::MuSig2SessionResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "MuSig2RegisterNonces (experimental!) registers one or more public nonces of"]
        #[doc = "other signing participants for a session identified by its ID. This RPC can"]
        #[doc = "be called multiple times until all nonces are registered."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        async fn mu_sig2_register_nonces(
            &self,
            request: tonic::Request<super::MuSig2RegisterNoncesRequest>,
        ) -> Result<tonic::Response<super::MuSig2RegisterNoncesResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "MuSig2Sign (experimental!) creates a partial signature using the local"]
        #[doc = "signing key that was specified when the session was created. This can only"]
        #[doc = "be called when all public nonces of all participants are known and have been"]
        #[doc = "registered with the session. If this node isn't responsible for combining"]
        #[doc = "all the partial signatures, then the cleanup flag should be set, indicating"]
        #[doc = "that the session can be removed from memory once the signature was produced."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        async fn mu_sig2_sign(
            &self,
            request: tonic::Request<super::MuSig2SignRequest>,
        ) -> Result<tonic::Response<super::MuSig2SignResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "MuSig2CombineSig (experimental!) combines the given partial signature(s)"]
        #[doc = "with the local one, if it already exists. Once a partial signature of all"]
        #[doc = "participants is registered, the final signature will be combined and"]
        #[doc = "returned."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        async fn mu_sig2_combine_sig(
            &self,
            request: tonic::Request<super::MuSig2CombineSigRequest>,
        ) -> Result<tonic::Response<super::MuSig2CombineSigResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "MuSig2Cleanup (experimental!) allows a caller to clean up a session early in"]
        #[doc = "cases where it's obvious that the signing session won't succeed and the"]
        #[doc = "resources can be released."]
        #[doc = ""]
        #[doc = "NOTE: The MuSig2 BIP is not final yet and therefore this API must be"]
        #[doc = "considered to be HIGHLY EXPERIMENTAL and subject to change in upcoming"]
        #[doc = "releases. Backward compatibility is not guaranteed!"]
        async fn mu_sig2_cleanup(
            &self,
            request: tonic::Request<super::MuSig2CleanupRequest>,
        ) -> Result<tonic::Response<super::MuSig2CleanupResponse>, tonic::Status>;
    }
    #[doc = " Signer is a service that gives access to the signing functionality of the"]
    #[doc = " daemon's wallet."]
    #[derive(Debug)]
    pub struct SignerServer<T: Signer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Signer> SignerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SignerServer<T>
    where
        T: Signer,
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
                "/signrpc.Signer/SignOutputRaw" => {
                    #[allow(non_camel_case_types)]
                    struct SignOutputRawSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::SignReq> for SignOutputRawSvc<T> {
                        type Response = super::SignResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sign_output_raw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignOutputRawSvc(inner);
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
                "/signrpc.Signer/ComputeInputScript" => {
                    #[allow(non_camel_case_types)]
                    struct ComputeInputScriptSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::SignReq> for ComputeInputScriptSvc<T> {
                        type Response = super::InputScriptResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).compute_input_script(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ComputeInputScriptSvc(inner);
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
                "/signrpc.Signer/SignMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SignMessageSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::SignMessageReq> for SignMessageSvc<T> {
                        type Response = super::SignMessageResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignMessageReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sign_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignMessageSvc(inner);
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
                "/signrpc.Signer/VerifyMessage" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyMessageSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::VerifyMessageReq> for VerifyMessageSvc<T> {
                        type Response = super::VerifyMessageResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyMessageReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).verify_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VerifyMessageSvc(inner);
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
                "/signrpc.Signer/DeriveSharedKey" => {
                    #[allow(non_camel_case_types)]
                    struct DeriveSharedKeySvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::SharedKeyRequest> for DeriveSharedKeySvc<T> {
                        type Response = super::SharedKeyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SharedKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).derive_shared_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeriveSharedKeySvc(inner);
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
                "/signrpc.Signer/MuSig2CombineKeys" => {
                    #[allow(non_camel_case_types)]
                    struct MuSig2CombineKeysSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::MuSig2CombineKeysRequest>
                        for MuSig2CombineKeysSvc<T>
                    {
                        type Response = super::MuSig2CombineKeysResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MuSig2CombineKeysRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mu_sig2_combine_keys(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MuSig2CombineKeysSvc(inner);
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
                "/signrpc.Signer/MuSig2CreateSession" => {
                    #[allow(non_camel_case_types)]
                    struct MuSig2CreateSessionSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::MuSig2SessionRequest>
                        for MuSig2CreateSessionSvc<T>
                    {
                        type Response = super::MuSig2SessionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MuSig2SessionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mu_sig2_create_session(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MuSig2CreateSessionSvc(inner);
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
                "/signrpc.Signer/MuSig2RegisterNonces" => {
                    #[allow(non_camel_case_types)]
                    struct MuSig2RegisterNoncesSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::MuSig2RegisterNoncesRequest>
                        for MuSig2RegisterNoncesSvc<T>
                    {
                        type Response = super::MuSig2RegisterNoncesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MuSig2RegisterNoncesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).mu_sig2_register_nonces(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MuSig2RegisterNoncesSvc(inner);
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
                "/signrpc.Signer/MuSig2Sign" => {
                    #[allow(non_camel_case_types)]
                    struct MuSig2SignSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::MuSig2SignRequest> for MuSig2SignSvc<T> {
                        type Response = super::MuSig2SignResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MuSig2SignRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mu_sig2_sign(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MuSig2SignSvc(inner);
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
                "/signrpc.Signer/MuSig2CombineSig" => {
                    #[allow(non_camel_case_types)]
                    struct MuSig2CombineSigSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::MuSig2CombineSigRequest>
                        for MuSig2CombineSigSvc<T>
                    {
                        type Response = super::MuSig2CombineSigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MuSig2CombineSigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mu_sig2_combine_sig(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MuSig2CombineSigSvc(inner);
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
                "/signrpc.Signer/MuSig2Cleanup" => {
                    #[allow(non_camel_case_types)]
                    struct MuSig2CleanupSvc<T: Signer>(pub Arc<T>);
                    impl<T: Signer> tonic::server::UnaryService<super::MuSig2CleanupRequest> for MuSig2CleanupSvc<T> {
                        type Response = super::MuSig2CleanupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MuSig2CleanupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mu_sig2_cleanup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MuSig2CleanupSvc(inner);
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
    impl<T: Signer> Clone for SignerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Signer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Signer> tonic::transport::NamedService for SignerServer<T> {
        const NAME: &'static str = "signrpc.Signer";
    }
}
