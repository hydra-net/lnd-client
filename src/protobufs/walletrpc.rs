#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUnspentRequest {
    /// The minimum number of confirmations to be included.
    #[prost(int32, tag = "1")]
    pub min_confs: i32,
    /// The maximum number of confirmations to be included.
    #[prost(int32, tag = "2")]
    pub max_confs: i32,
    /// An optional filter to only include outputs belonging to an account.
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
    ///
    ///When min_confs and max_confs are zero, setting false implicitly
    ///overrides max_confs to be MaxInt32, otherwise max_confs remains
    ///zero. An error is returned if the value is true and both min_confs
    ///and max_confs are non-zero. (default: false)
    #[prost(bool, tag = "4")]
    pub unconfirmed_only: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUnspentResponse {
    /// A list of utxos satisfying the specified number of confirmations.
    #[prost(message, repeated, tag = "1")]
    pub utxos: ::prost::alloc::vec::Vec<super::lnrpc::Utxo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseOutputRequest {
    ///
    ///An ID of 32 random bytes that must be unique for each distinct application
    ///using this RPC which will be used to bound the output lease to.
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// The identifying outpoint of the output being leased.
    #[prost(message, optional, tag = "2")]
    pub outpoint: ::core::option::Option<super::lnrpc::OutPoint>,
    /// The time in seconds before the lock expires. If set to zero, the default
    /// lock duration is used.
    #[prost(uint64, tag = "3")]
    pub expiration_seconds: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseOutputResponse {
    ///
    ///The absolute expiration of the output lease represented as a unix timestamp.
    #[prost(uint64, tag = "1")]
    pub expiration: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseOutputRequest {
    /// The unique ID that was used to lock the output.
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// The identifying outpoint of the output being released.
    #[prost(message, optional, tag = "2")]
    pub outpoint: ::core::option::Option<super::lnrpc::OutPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseOutputResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyReq {
    ///
    ///Is the key finger print of the root pubkey that this request is targeting.
    ///This allows the WalletKit to possibly serve out keys for multiple HD chains
    ///via public derivation.
    #[prost(int32, tag = "1")]
    pub key_finger_print: i32,
    ///
    ///The target key family to derive a key from. In other contexts, this is
    ///known as the "account".
    #[prost(int32, tag = "2")]
    pub key_family: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddrRequest {
    ///
    ///The name of the account to retrieve the next address of. If empty, the
    ///default wallet account is used.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    ///
    ///The type of address to derive.
    #[prost(enumeration = "AddressType", tag = "2")]
    pub r#type: i32,
    ///
    ///Whether a change address should be derived.
    #[prost(bool, tag = "3")]
    pub change: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddrResponse {
    ///
    ///The address encoded using a bech32 format.
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// The name used to identify the account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    ///The type of addresses the account supports.
    ///AddressType                       | External Branch | Internal Branch
    ///---------------------------------------------------------------------
    ///WITNESS_PUBKEY_HASH               | P2WPKH          | P2WPKH
    ///NESTED_WITNESS_PUBKEY_HASH        | NP2WPKH         | NP2WPKH
    ///HYBRID_NESTED_WITNESS_PUBKEY_HASH | NP2WPKH         | P2WPKH
    #[prost(enumeration = "AddressType", tag = "2")]
    pub address_type: i32,
    ///
    ///The public key backing the account that all keys are derived from
    ///represented as an extended key. This will always be empty for the default
    ///imported account in which single public keys are imported into.
    #[prost(string, tag = "3")]
    pub extended_public_key: ::prost::alloc::string::String,
    ///
    ///The fingerprint of the root key from which the account public key was
    ///derived from. This will always be zero for the default imported account in
    ///which single public keys are imported into. The bytes are in big-endian
    ///order.
    #[prost(bytes = "vec", tag = "4")]
    pub master_key_fingerprint: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The derivation path corresponding to the account public key. This will
    ///always be empty for the default imported account in which single public keys
    ///are imported into.
    #[prost(string, tag = "5")]
    pub derivation_path: ::prost::alloc::string::String,
    ///
    ///The number of keys derived from the external branch of the account public
    ///key. This will always be zero for the default imported account in which
    ///single public keys are imported into.
    #[prost(uint32, tag = "6")]
    pub external_key_count: u32,
    ///
    ///The number of keys derived from the internal branch of the account public
    ///key. This will always be zero for the default imported account in which
    ///single public keys are imported into.
    #[prost(uint32, tag = "7")]
    pub internal_key_count: u32,
    /// Whether the wallet stores private keys for the account.
    #[prost(bool, tag = "8")]
    pub watch_only: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsRequest {
    /// An optional filter to only return accounts matching this name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An optional filter to only return accounts matching this address type.
    #[prost(enumeration = "AddressType", tag = "2")]
    pub address_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAccountRequest {
    /// A name to identify the account with.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    ///A public key that corresponds to a wallet account represented as an extended
    ///key. It must conform to a derivation path of the form
    ///m/purpose'/coin_type'/account'.
    #[prost(string, tag = "2")]
    pub extended_public_key: ::prost::alloc::string::String,
    ///
    ///The fingerprint of the root key (also known as the key with derivation path
    ///m/) from which the account public key was derived from. This may be required
    ///by some hardware wallets for proper identification and signing. The bytes
    ///must be in big-endian order.
    #[prost(bytes = "vec", tag = "3")]
    pub master_key_fingerprint: ::prost::alloc::vec::Vec<u8>,
    ///
    ///An address type is only required when the extended account public key has a
    ///legacy version (xpub, tpub, etc.), such that the wallet cannot detect what
    ///address scheme it belongs to.
    #[prost(enumeration = "AddressType", tag = "4")]
    pub address_type: i32,
    ///
    ///Whether a dry run should be attempted when importing the account. This
    ///serves as a way to confirm whether the account is being imported correctly
    ///by returning the first N addresses for the external and internal branches of
    ///the account. If these addresses match as expected, then it should be safe to
    ///import the account as is.
    #[prost(bool, tag = "5")]
    pub dry_run: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAccountResponse {
    /// The details of the imported account.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<Account>,
    ///
    ///The first N addresses that belong to the external branch of the account.
    ///The external branch is typically used for external non-change addresses.
    ///These are only returned if a dry run was specified within the request.
    #[prost(string, repeated, tag = "2")]
    pub dry_run_external_addrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    ///The first N addresses that belong to the internal branch of the account.
    ///The internal branch is typically used for change addresses. These are only
    ///returned if a dry run was specified within the request.
    #[prost(string, repeated, tag = "3")]
    pub dry_run_internal_addrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportPublicKeyRequest {
    /// A compressed public key represented as raw bytes.
    #[prost(bytes = "vec", tag = "1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    /// The type of address that will be generated from the public key.
    #[prost(enumeration = "AddressType", tag = "2")]
    pub address_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportPublicKeyResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    ///
    ///The raw serialized transaction.
    #[prost(bytes = "vec", tag = "1")]
    pub tx_hex: ::prost::alloc::vec::Vec<u8>,
    ///
    ///An optional label to save with the transaction. Limited to 500 characters.
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishResponse {
    ///
    ///If blank, then no error occurred and the transaction was successfully
    ///published. If not the empty string, then a string representation of the
    ///broadcast error.
    ///
    ///TODO(roasbeef): map to a proper enum type
    #[prost(string, tag = "1")]
    pub publish_error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOutputsRequest {
    ///
    ///The number of satoshis per kilo weight that should be used when crafting
    ///this transaction.
    #[prost(int64, tag = "1")]
    pub sat_per_kw: i64,
    ///
    ///A slice of the outputs that should be created in the transaction produced.
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<super::signrpc::TxOut>,
    /// An optional label for the transaction, limited to 500 characters.
    #[prost(string, tag = "3")]
    pub label: ::prost::alloc::string::String,
    /// The minimum number of confirmations each one of your outputs used for
    /// the transaction must satisfy.
    #[prost(int32, tag = "4")]
    pub min_confs: i32,
    /// Whether unconfirmed outputs should be used as inputs for the transaction.
    #[prost(bool, tag = "5")]
    pub spend_unconfirmed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOutputsResponse {
    ///
    ///The serialized transaction sent out on the network.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_tx: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateFeeRequest {
    ///
    ///The number of confirmations to shoot for when estimating the fee.
    #[prost(int32, tag = "1")]
    pub conf_target: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateFeeResponse {
    ///
    ///The amount of satoshis per kw that should be used in order to reach the
    ///confirmation target in the request.
    #[prost(int64, tag = "1")]
    pub sat_per_kw: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingSweep {
    /// The outpoint of the output we're attempting to sweep.
    #[prost(message, optional, tag = "1")]
    pub outpoint: ::core::option::Option<super::lnrpc::OutPoint>,
    /// The witness type of the output we're attempting to sweep.
    #[prost(enumeration = "WitnessType", tag = "2")]
    pub witness_type: i32,
    /// The value of the output we're attempting to sweep.
    #[prost(uint32, tag = "3")]
    pub amount_sat: u32,
    ///
    ///Deprecated, use sat_per_vbyte.
    ///The fee rate we'll use to sweep the output, expressed in sat/vbyte. The fee
    ///rate is only determined once a sweeping transaction for the output is
    ///created, so it's possible for this to be 0 before this.
    #[deprecated]
    #[prost(uint32, tag = "4")]
    pub sat_per_byte: u32,
    /// The number of broadcast attempts we've made to sweep the output.
    #[prost(uint32, tag = "5")]
    pub broadcast_attempts: u32,
    ///
    ///The next height of the chain at which we'll attempt to broadcast the
    ///sweep transaction of the output.
    #[prost(uint32, tag = "6")]
    pub next_broadcast_height: u32,
    /// The requested confirmation target for this output.
    #[prost(uint32, tag = "8")]
    pub requested_conf_target: u32,
    /// Deprecated, use requested_sat_per_vbyte.
    /// The requested fee rate, expressed in sat/vbyte, for this output.
    #[deprecated]
    #[prost(uint32, tag = "9")]
    pub requested_sat_per_byte: u32,
    ///
    ///The fee rate we'll use to sweep the output, expressed in sat/vbyte. The fee
    ///rate is only determined once a sweeping transaction for the output is
    ///created, so it's possible for this to be 0 before this.
    #[prost(uint64, tag = "10")]
    pub sat_per_vbyte: u64,
    /// The requested fee rate, expressed in sat/vbyte, for this output.
    #[prost(uint64, tag = "11")]
    pub requested_sat_per_vbyte: u64,
    ///
    ///Whether this input must be force-swept. This means that it is swept even
    ///if it has a negative yield.
    #[prost(bool, tag = "7")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingSweepsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingSweepsResponse {
    ///
    ///The set of outputs currently being swept by lnd's central batching engine.
    #[prost(message, repeated, tag = "1")]
    pub pending_sweeps: ::prost::alloc::vec::Vec<PendingSweep>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BumpFeeRequest {
    /// The input we're attempting to bump the fee of.
    #[prost(message, optional, tag = "1")]
    pub outpoint: ::core::option::Option<super::lnrpc::OutPoint>,
    /// The target number of blocks that the input should be spent within.
    #[prost(uint32, tag = "2")]
    pub target_conf: u32,
    ///
    ///Deprecated, use sat_per_vbyte.
    ///The fee rate, expressed in sat/vbyte, that should be used to spend the input
    ///with.
    #[deprecated]
    #[prost(uint32, tag = "3")]
    pub sat_per_byte: u32,
    ///
    ///Whether this input must be force-swept. This means that it is swept even
    ///if it has a negative yield.
    #[prost(bool, tag = "4")]
    pub force: bool,
    ///
    ///The fee rate, expressed in sat/vbyte, that should be used to spend the input
    ///with.
    #[prost(uint64, tag = "5")]
    pub sat_per_vbyte: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BumpFeeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSweepsRequest {
    ///
    ///Retrieve the full sweep transaction details. If false, only the sweep txids
    ///will be returned. Note that some sweeps that LND publishes will have been
    ///replaced-by-fee, so will not be included in this output.
    #[prost(bool, tag = "1")]
    pub verbose: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSweepsResponse {
    #[prost(oneof = "list_sweeps_response::Sweeps", tags = "1, 2")]
    pub sweeps: ::core::option::Option<list_sweeps_response::Sweeps>,
}
/// Nested message and enum types in `ListSweepsResponse`.
pub mod list_sweeps_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionIDs {
        ///
        ///Reversed, hex-encoded string representing the transaction ids of the
        ///sweeps that our node has broadcast. Note that these transactions may
        ///not have confirmed yet, we record sweeps on broadcast, not confirmation.
        #[prost(string, repeated, tag = "1")]
        pub transaction_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sweeps {
        #[prost(message, tag = "1")]
        TransactionDetails(super::super::lnrpc::TransactionDetails),
        #[prost(message, tag = "2")]
        TransactionIds(TransactionIDs),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelTransactionRequest {
    /// The txid of the transaction to label.
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    /// The label to add to the transaction, limited to 500 characters.
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// Whether to overwrite the existing label, if it is present.
    #[prost(bool, tag = "3")]
    pub overwrite: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelTransactionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundPsbtRequest {
    ///
    ///The name of the account to fund the PSBT with. If empty, the default wallet
    ///account is used.
    #[prost(string, tag = "5")]
    pub account: ::prost::alloc::string::String,
    /// The minimum number of confirmations each one of your outputs used for
    /// the transaction must satisfy.
    #[prost(int32, tag = "6")]
    pub min_confs: i32,
    /// Whether unconfirmed outputs should be used as inputs for the transaction.
    #[prost(bool, tag = "7")]
    pub spend_unconfirmed: bool,
    #[prost(oneof = "fund_psbt_request::Template", tags = "1, 2")]
    pub template: ::core::option::Option<fund_psbt_request::Template>,
    #[prost(oneof = "fund_psbt_request::Fees", tags = "3, 4")]
    pub fees: ::core::option::Option<fund_psbt_request::Fees>,
}
/// Nested message and enum types in `FundPsbtRequest`.
pub mod fund_psbt_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Template {
        ///
        ///Use an existing PSBT packet as the template for the funded PSBT.
        ///
        ///The packet must contain at least one non-dust output. If one or more
        ///inputs are specified, no coin selection is performed. In that case every
        ///input must be an UTXO known to the wallet that has not been locked
        ///before. The sum of all inputs must be sufficiently greater than the sum
        ///of all outputs to pay a miner fee with the specified fee rate. A change
        ///output is added to the PSBT if necessary.
        #[prost(bytes, tag = "1")]
        Psbt(::prost::alloc::vec::Vec<u8>),
        ///
        ///Use the outputs and optional inputs from this raw template.
        #[prost(message, tag = "2")]
        Raw(super::TxTemplate),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Fees {
        ///
        ///The target number of blocks that the transaction should be confirmed in.
        #[prost(uint32, tag = "3")]
        TargetConf(u32),
        ///
        ///The fee rate, expressed in sat/vbyte, that should be used to spend the
        ///input with.
        #[prost(uint64, tag = "4")]
        SatPerVbyte(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundPsbtResponse {
    ///
    ///The funded but not yet signed PSBT packet.
    #[prost(bytes = "vec", tag = "1")]
    pub funded_psbt: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The index of the added change output or -1 if no change was left over.
    #[prost(int32, tag = "2")]
    pub change_output_index: i32,
    ///
    ///The list of lock leases that were acquired for the inputs in the funded PSBT
    ///packet.
    #[prost(message, repeated, tag = "3")]
    pub locked_utxos: ::prost::alloc::vec::Vec<UtxoLease>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxTemplate {
    ///
    ///An optional list of inputs to use. Every input must be an UTXO known to the
    ///wallet that has not been locked before. The sum of all inputs must be
    ///sufficiently greater than the sum of all outputs to pay a miner fee with the
    ///fee rate specified in the parent message.
    ///
    ///If no inputs are specified, coin selection will be performed instead and
    ///inputs of sufficient value will be added to the resulting PSBT.
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<super::lnrpc::OutPoint>,
    ///
    ///A map of all addresses and the amounts to send to in the funded PSBT.
    #[prost(map = "string, uint64", tag = "2")]
    pub outputs: ::std::collections::HashMap<::prost::alloc::string::String, u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoLease {
    ///
    ///A 32 byte random ID that identifies the lease.
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// The identifying outpoint of the output being leased.
    #[prost(message, optional, tag = "2")]
    pub outpoint: ::core::option::Option<super::lnrpc::OutPoint>,
    ///
    ///The absolute expiration of the output lease represented as a unix timestamp.
    #[prost(uint64, tag = "3")]
    pub expiration: u64,
    ///
    ///The public key script of the leased output.
    #[prost(bytes = "vec", tag = "4")]
    pub pk_script: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The value of the leased output in satoshis.
    #[prost(uint64, tag = "5")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignPsbtRequest {
    ///
    ///The PSBT that should be signed. The PSBT must contain all required inputs,
    ///outputs, UTXO data and custom fields required to identify the signing key.
    #[prost(bytes = "vec", tag = "1")]
    pub funded_psbt: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignPsbtResponse {
    /// The signed transaction in PSBT format.
    #[prost(bytes = "vec", tag = "1")]
    pub signed_psbt: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizePsbtRequest {
    ///
    ///A PSBT that should be signed and finalized. The PSBT must contain all
    ///required inputs, outputs, UTXO data and partial signatures of all other
    ///signers.
    #[prost(bytes = "vec", tag = "1")]
    pub funded_psbt: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The name of the account to finalize the PSBT with. If empty, the default
    ///wallet account is used.
    #[prost(string, tag = "5")]
    pub account: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizePsbtResponse {
    /// The fully signed and finalized transaction in PSBT format.
    #[prost(bytes = "vec", tag = "1")]
    pub signed_psbt: ::prost::alloc::vec::Vec<u8>,
    /// The fully signed and finalized transaction in the raw wire format.
    #[prost(bytes = "vec", tag = "2")]
    pub raw_final_tx: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLeasesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLeasesResponse {
    /// The list of currently leased utxos.
    #[prost(message, repeated, tag = "1")]
    pub locked_utxos: ::prost::alloc::vec::Vec<UtxoLease>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddressType {
    Unknown = 0,
    WitnessPubkeyHash = 1,
    NestedWitnessPubkeyHash = 2,
    HybridNestedWitnessPubkeyHash = 3,
    TaprootPubkey = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WitnessType {
    UnknownWitness = 0,
    ///
    ///A witness that allows us to spend the output of a commitment transaction
    ///after a relative lock-time lockout.
    CommitmentTimeLock = 1,
    ///
    ///A witness that allows us to spend a settled no-delay output immediately on a
    ///counterparty's commitment transaction.
    CommitmentNoDelay = 2,
    ///
    ///A witness that allows us to sweep the settled output of a malicious
    ///counterparty's who broadcasts a revoked commitment transaction.
    CommitmentRevoke = 3,
    ///
    ///A witness that allows us to sweep an HTLC which we offered to the remote
    ///party in the case that they broadcast a revoked commitment state.
    HtlcOfferedRevoke = 4,
    ///
    ///A witness that allows us to sweep an HTLC output sent to us in the case that
    ///the remote party broadcasts a revoked commitment state.
    HtlcAcceptedRevoke = 5,
    ///
    ///A witness that allows us to sweep an HTLC output that we extended to a
    ///party, but was never fulfilled.  This HTLC output isn't directly on the
    ///commitment transaction, but is the result of a confirmed second-level HTLC
    ///transaction. As a result, we can only spend this after a CSV delay.
    HtlcOfferedTimeoutSecondLevel = 6,
    ///
    ///A witness that allows us to sweep an HTLC output that was offered to us, and
    ///for which we have a payment preimage. This HTLC output isn't directly on our
    ///commitment transaction, but is the result of confirmed second-level HTLC
    ///transaction. As a result, we can only spend this after a CSV delay.
    HtlcAcceptedSuccessSecondLevel = 7,
    ///
    ///A witness that allows us to sweep an HTLC that we offered to the remote
    ///party which lies in the commitment transaction of the remote party. We can
    ///spend this output after the absolute CLTV timeout of the HTLC as passed.
    HtlcOfferedRemoteTimeout = 8,
    ///
    ///A witness that allows us to sweep an HTLC that was offered to us by the
    ///remote party. We use this witness in the case that the remote party goes to
    ///chain, and we know the pre-image to the HTLC. We can sweep this without any
    ///additional timeout.
    HtlcAcceptedRemoteSuccess = 9,
    ///
    ///A witness that allows us to sweep an HTLC from the remote party's commitment
    ///transaction in the case that the broadcast a revoked commitment, but then
    ///also immediately attempt to go to the second level to claim the HTLC.
    HtlcSecondLevelRevoke = 10,
    ///
    ///A witness type that allows us to spend a regular p2wkh output that's sent to
    ///an output which is under complete control of the backing wallet.
    WitnessKeyHash = 11,
    ///
    ///A witness type that allows us to sweep an output that sends to a nested P2SH
    ///script that pays to a key solely under our control.
    NestedWitnessKeyHash = 12,
    ///
    ///A witness type that allows us to spend our anchor on the commitment
    ///transaction.
    CommitmentAnchor = 13,
}
#[doc = r" Generated client implementations."]
pub mod wallet_kit_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " WalletKit is a service that gives access to the core functionalities of the"]
    #[doc = " daemon's wallet."]
    #[derive(Debug, Clone)]
    pub struct WalletKitClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletKitClient<tonic::transport::Channel> {
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
    impl<T> WalletKitClient<T>
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
        ) -> WalletKitClient<InterceptedService<T, F>>
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
            WalletKitClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "ListUnspent returns a list of all utxos spendable by the wallet with a"]
        #[doc = "number of confirmations between the specified minimum and maximum. By"]
        #[doc = "default, all utxos are listed. To list only the unconfirmed utxos, set"]
        #[doc = "the unconfirmed_only to true."]
        pub async fn list_unspent(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUnspentRequest>,
        ) -> Result<tonic::Response<super::ListUnspentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ListUnspent");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "LeaseOutput locks an output to the given ID, preventing it from being"]
        #[doc = "available for any future coin selection attempts. The absolute time of the"]
        #[doc = "lock's expiration is returned. The expiration of the lock can be extended by"]
        #[doc = "successive invocations of this RPC. Outputs can be unlocked before their"]
        #[doc = "expiration through `ReleaseOutput`."]
        pub async fn lease_output(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaseOutputRequest>,
        ) -> Result<tonic::Response<super::LeaseOutputResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/LeaseOutput");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ReleaseOutput unlocks an output, allowing it to be available for coin"]
        #[doc = "selection if it remains unspent. The ID should match the one used to"]
        #[doc = "originally lock the output."]
        pub async fn release_output(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseOutputRequest>,
        ) -> Result<tonic::Response<super::ReleaseOutputResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ReleaseOutput");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ListLeases lists all currently locked utxos."]
        pub async fn list_leases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLeasesRequest>,
        ) -> Result<tonic::Response<super::ListLeasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ListLeases");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DeriveNextKey attempts to derive the *next* key within the key family"]
        #[doc = "(account in BIP43) specified. This method should return the next external"]
        #[doc = "child within this branch."]
        pub async fn derive_next_key(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyReq>,
        ) -> Result<tonic::Response<super::super::signrpc::KeyDescriptor>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/DeriveNextKey");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DeriveKey attempts to derive an arbitrary key specified by the passed"]
        #[doc = "KeyLocator."]
        pub async fn derive_key(
            &mut self,
            request: impl tonic::IntoRequest<super::super::signrpc::KeyLocator>,
        ) -> Result<tonic::Response<super::super::signrpc::KeyDescriptor>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/DeriveKey");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "NextAddr returns the next unused address within the wallet."]
        pub async fn next_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::AddrRequest>,
        ) -> Result<tonic::Response<super::AddrResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/NextAddr");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ListAccounts retrieves all accounts belonging to the wallet by default. A"]
        #[doc = "name and key scope filter can be provided to filter through all of the"]
        #[doc = "wallet accounts and return only those matching."]
        pub async fn list_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountsRequest>,
        ) -> Result<tonic::Response<super::ListAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ListAccounts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ImportAccount imports an account backed by an account extended public key."]
        #[doc = "The master key fingerprint denotes the fingerprint of the root key"]
        #[doc = "corresponding to the account public key (also known as the key with"]
        #[doc = "derivation path m/). This may be required by some hardware wallets for"]
        #[doc = "proper identification and signing."]
        #[doc = ""]
        #[doc = "The address type can usually be inferred from the key's version, but may be"]
        #[doc = "required for certain keys to map them into the proper scope."]
        #[doc = ""]
        #[doc = "For BIP-0044 keys, an address type must be specified as we intend to not"]
        #[doc = "support importing BIP-0044 keys into the wallet using the legacy"]
        #[doc = "pay-to-pubkey-hash (P2PKH) scheme. A nested witness address type will force"]
        #[doc = "the standard BIP-0049 derivation scheme, while a witness address type will"]
        #[doc = "force the standard BIP-0084 derivation scheme."]
        #[doc = ""]
        #[doc = "For BIP-0049 keys, an address type must also be specified to make a"]
        #[doc = "distinction between the standard BIP-0049 address schema (nested witness"]
        #[doc = "pubkeys everywhere) and our own BIP-0049Plus address schema (nested pubkeys"]
        #[doc = "externally, witness pubkeys internally)."]
        #[doc = ""]
        #[doc = "NOTE: Events (deposits/spends) for keys derived from an account will only be"]
        #[doc = "detected by lnd if they happen after the import. Rescans to detect past"]
        #[doc = "events will be supported later on."]
        pub async fn import_account(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportAccountRequest>,
        ) -> Result<tonic::Response<super::ImportAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ImportAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ImportPublicKey imports a public key as watch-only into the wallet."]
        #[doc = ""]
        #[doc = "NOTE: Events (deposits/spends) for a key will only be detected by lnd if"]
        #[doc = "they happen after the import. Rescans to detect past events will be"]
        #[doc = "supported later on."]
        pub async fn import_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportPublicKeyRequest>,
        ) -> Result<tonic::Response<super::ImportPublicKeyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ImportPublicKey");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "PublishTransaction attempts to publish the passed transaction to the"]
        #[doc = "network. Once this returns without an error, the wallet will continually"]
        #[doc = "attempt to re-broadcast the transaction on start up, until it enters the"]
        #[doc = "chain."]
        pub async fn publish_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> Result<tonic::Response<super::PublishResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/PublishTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SendOutputs is similar to the existing sendmany call in Bitcoind, and"]
        #[doc = "allows the caller to create a transaction that sends to several outputs at"]
        #[doc = "once. This is ideal when wanting to batch create a set of transactions."]
        pub async fn send_outputs(
            &mut self,
            request: impl tonic::IntoRequest<super::SendOutputsRequest>,
        ) -> Result<tonic::Response<super::SendOutputsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/SendOutputs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "EstimateFee attempts to query the internal fee estimator of the wallet to"]
        #[doc = "determine the fee (in sat/kw) to attach to a transaction in order to"]
        #[doc = "achieve the confirmation target."]
        pub async fn estimate_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::EstimateFeeRequest>,
        ) -> Result<tonic::Response<super::EstimateFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/EstimateFee");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "PendingSweeps returns lists of on-chain outputs that lnd is currently"]
        #[doc = "attempting to sweep within its central batching engine. Outputs with similar"]
        #[doc = "fee rates are batched together in order to sweep them within a single"]
        #[doc = "transaction."]
        #[doc = ""]
        #[doc = "NOTE: Some of the fields within PendingSweepsRequest are not guaranteed to"]
        #[doc = "remain supported. This is an advanced API that depends on the internals of"]
        #[doc = "the UtxoSweeper, so things may change."]
        pub async fn pending_sweeps(
            &mut self,
            request: impl tonic::IntoRequest<super::PendingSweepsRequest>,
        ) -> Result<tonic::Response<super::PendingSweepsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/PendingSweeps");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "BumpFee bumps the fee of an arbitrary input within a transaction. This RPC"]
        #[doc = "takes a different approach than bitcoind's bumpfee command. lnd has a"]
        #[doc = "central batching engine in which inputs with similar fee rates are batched"]
        #[doc = "together to save on transaction fees. Due to this, we cannot rely on"]
        #[doc = "bumping the fee on a specific transaction, since transactions can change at"]
        #[doc = "any point with the addition of new inputs. The list of inputs that"]
        #[doc = "currently exist within lnd's central batching engine can be retrieved"]
        #[doc = "through the PendingSweeps RPC."]
        #[doc = ""]
        #[doc = "When bumping the fee of an input that currently exists within lnd's central"]
        #[doc = "batching engine, a higher fee transaction will be created that replaces the"]
        #[doc = "lower fee transaction through the Replace-By-Fee (RBF) policy. If it"]
        #[doc = ""]
        #[doc = "This RPC also serves useful when wanting to perform a Child-Pays-For-Parent"]
        #[doc = "(CPFP), where the child transaction pays for its parent's fee. This can be"]
        #[doc = "done by specifying an outpoint within the low fee transaction that is under"]
        #[doc = "the control of the wallet."]
        #[doc = ""]
        #[doc = "The fee preference can be expressed either as a specific fee rate or a delta"]
        #[doc = "of blocks in which the output should be swept on-chain within. If a fee"]
        #[doc = "preference is not explicitly specified, then an error is returned."]
        #[doc = ""]
        #[doc = "Note that this RPC currently doesn't perform any validation checks on the"]
        #[doc = "fee preference being provided. For now, the responsibility of ensuring that"]
        #[doc = "the new fee preference is sufficient is delegated to the user."]
        pub async fn bump_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::BumpFeeRequest>,
        ) -> Result<tonic::Response<super::BumpFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/BumpFee");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ListSweeps returns a list of the sweep transactions our node has produced."]
        #[doc = "Note that these sweeps may not be confirmed yet, as we record sweeps on"]
        #[doc = "broadcast, not confirmation."]
        pub async fn list_sweeps(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSweepsRequest>,
        ) -> Result<tonic::Response<super::ListSweepsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/ListSweeps");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "LabelTransaction adds a label to a transaction. If the transaction already"]
        #[doc = "has a label the call will fail unless the overwrite bool is set. This will"]
        #[doc = "overwrite the exiting transaction label. Labels must not be empty, and"]
        #[doc = "cannot exceed 500 characters."]
        pub async fn label_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::LabelTransactionRequest>,
        ) -> Result<tonic::Response<super::LabelTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/LabelTransaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "FundPsbt creates a fully populated PSBT that contains enough inputs to fund"]
        #[doc = "the outputs specified in the template. There are two ways of specifying a"]
        #[doc = "template: Either by passing in a PSBT with at least one output declared or"]
        #[doc = "by passing in a raw TxTemplate message."]
        #[doc = ""]
        #[doc = "If there are no inputs specified in the template, coin selection is"]
        #[doc = "performed automatically. If the template does contain any inputs, it is"]
        #[doc = "assumed that full coin selection happened externally and no additional"]
        #[doc = "inputs are added. If the specified inputs aren't enough to fund the outputs"]
        #[doc = "with the given fee rate, an error is returned."]
        #[doc = ""]
        #[doc = "After either selecting or verifying the inputs, all input UTXOs are locked"]
        #[doc = "with an internal app ID."]
        #[doc = ""]
        #[doc = "NOTE: If this method returns without an error, it is the caller's"]
        #[doc = "responsibility to either spend the locked UTXOs (by finalizing and then"]
        #[doc = "publishing the transaction) or to unlock/release the locked UTXOs in case of"]
        #[doc = "an error on the caller's side."]
        pub async fn fund_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::FundPsbtRequest>,
        ) -> Result<tonic::Response<super::FundPsbtResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/FundPsbt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SignPsbt expects a partial transaction with all inputs and outputs fully"]
        #[doc = "declared and tries to sign all unsigned inputs that have all required fields"]
        #[doc = "(UTXO information, BIP32 derivation information, witness or sig scripts)"]
        #[doc = "set."]
        #[doc = "If no error is returned, the PSBT is ready to be given to the next signer or"]
        #[doc = "to be finalized if lnd was the last signer."]
        #[doc = ""]
        #[doc = "NOTE: This RPC only signs inputs (and only those it can sign), it does not"]
        #[doc = "perform any other tasks (such as coin selection, UTXO locking or"]
        #[doc = "input/output/fee value validation, PSBT finalization). Any input that is"]
        #[doc = "incomplete will be skipped."]
        pub async fn sign_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::SignPsbtRequest>,
        ) -> Result<tonic::Response<super::SignPsbtResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/SignPsbt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "FinalizePsbt expects a partial transaction with all inputs and outputs fully"]
        #[doc = "declared and tries to sign all inputs that belong to the wallet. Lnd must be"]
        #[doc = "the last signer of the transaction. That means, if there are any unsigned"]
        #[doc = "non-witness inputs or inputs without UTXO information attached or inputs"]
        #[doc = "without witness data that do not belong to lnd's wallet, this method will"]
        #[doc = "fail. If no error is returned, the PSBT is ready to be extracted and the"]
        #[doc = "final TX within to be broadcast."]
        #[doc = ""]
        #[doc = "NOTE: This method does NOT publish the transaction once finalized. It is the"]
        #[doc = "caller's responsibility to either publish the transaction on success or"]
        #[doc = "unlock/release any locked UTXOs in case of an error in this method."]
        pub async fn finalize_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizePsbtRequest>,
        ) -> Result<tonic::Response<super::FinalizePsbtResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/walletrpc.WalletKit/FinalizePsbt");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod wallet_kit_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WalletKitServer."]
    #[async_trait]
    pub trait WalletKit: Send + Sync + 'static {
        #[doc = ""]
        #[doc = "ListUnspent returns a list of all utxos spendable by the wallet with a"]
        #[doc = "number of confirmations between the specified minimum and maximum. By"]
        #[doc = "default, all utxos are listed. To list only the unconfirmed utxos, set"]
        #[doc = "the unconfirmed_only to true."]
        async fn list_unspent(
            &self,
            request: tonic::Request<super::ListUnspentRequest>,
        ) -> Result<tonic::Response<super::ListUnspentResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "LeaseOutput locks an output to the given ID, preventing it from being"]
        #[doc = "available for any future coin selection attempts. The absolute time of the"]
        #[doc = "lock's expiration is returned. The expiration of the lock can be extended by"]
        #[doc = "successive invocations of this RPC. Outputs can be unlocked before their"]
        #[doc = "expiration through `ReleaseOutput`."]
        async fn lease_output(
            &self,
            request: tonic::Request<super::LeaseOutputRequest>,
        ) -> Result<tonic::Response<super::LeaseOutputResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "ReleaseOutput unlocks an output, allowing it to be available for coin"]
        #[doc = "selection if it remains unspent. The ID should match the one used to"]
        #[doc = "originally lock the output."]
        async fn release_output(
            &self,
            request: tonic::Request<super::ReleaseOutputRequest>,
        ) -> Result<tonic::Response<super::ReleaseOutputResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "ListLeases lists all currently locked utxos."]
        async fn list_leases(
            &self,
            request: tonic::Request<super::ListLeasesRequest>,
        ) -> Result<tonic::Response<super::ListLeasesResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "DeriveNextKey attempts to derive the *next* key within the key family"]
        #[doc = "(account in BIP43) specified. This method should return the next external"]
        #[doc = "child within this branch."]
        async fn derive_next_key(
            &self,
            request: tonic::Request<super::KeyReq>,
        ) -> Result<tonic::Response<super::super::signrpc::KeyDescriptor>, tonic::Status>;
        #[doc = ""]
        #[doc = "DeriveKey attempts to derive an arbitrary key specified by the passed"]
        #[doc = "KeyLocator."]
        async fn derive_key(
            &self,
            request: tonic::Request<super::super::signrpc::KeyLocator>,
        ) -> Result<tonic::Response<super::super::signrpc::KeyDescriptor>, tonic::Status>;
        #[doc = ""]
        #[doc = "NextAddr returns the next unused address within the wallet."]
        async fn next_addr(
            &self,
            request: tonic::Request<super::AddrRequest>,
        ) -> Result<tonic::Response<super::AddrResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "ListAccounts retrieves all accounts belonging to the wallet by default. A"]
        #[doc = "name and key scope filter can be provided to filter through all of the"]
        #[doc = "wallet accounts and return only those matching."]
        async fn list_accounts(
            &self,
            request: tonic::Request<super::ListAccountsRequest>,
        ) -> Result<tonic::Response<super::ListAccountsResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "ImportAccount imports an account backed by an account extended public key."]
        #[doc = "The master key fingerprint denotes the fingerprint of the root key"]
        #[doc = "corresponding to the account public key (also known as the key with"]
        #[doc = "derivation path m/). This may be required by some hardware wallets for"]
        #[doc = "proper identification and signing."]
        #[doc = ""]
        #[doc = "The address type can usually be inferred from the key's version, but may be"]
        #[doc = "required for certain keys to map them into the proper scope."]
        #[doc = ""]
        #[doc = "For BIP-0044 keys, an address type must be specified as we intend to not"]
        #[doc = "support importing BIP-0044 keys into the wallet using the legacy"]
        #[doc = "pay-to-pubkey-hash (P2PKH) scheme. A nested witness address type will force"]
        #[doc = "the standard BIP-0049 derivation scheme, while a witness address type will"]
        #[doc = "force the standard BIP-0084 derivation scheme."]
        #[doc = ""]
        #[doc = "For BIP-0049 keys, an address type must also be specified to make a"]
        #[doc = "distinction between the standard BIP-0049 address schema (nested witness"]
        #[doc = "pubkeys everywhere) and our own BIP-0049Plus address schema (nested pubkeys"]
        #[doc = "externally, witness pubkeys internally)."]
        #[doc = ""]
        #[doc = "NOTE: Events (deposits/spends) for keys derived from an account will only be"]
        #[doc = "detected by lnd if they happen after the import. Rescans to detect past"]
        #[doc = "events will be supported later on."]
        async fn import_account(
            &self,
            request: tonic::Request<super::ImportAccountRequest>,
        ) -> Result<tonic::Response<super::ImportAccountResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "ImportPublicKey imports a public key as watch-only into the wallet."]
        #[doc = ""]
        #[doc = "NOTE: Events (deposits/spends) for a key will only be detected by lnd if"]
        #[doc = "they happen after the import. Rescans to detect past events will be"]
        #[doc = "supported later on."]
        async fn import_public_key(
            &self,
            request: tonic::Request<super::ImportPublicKeyRequest>,
        ) -> Result<tonic::Response<super::ImportPublicKeyResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "PublishTransaction attempts to publish the passed transaction to the"]
        #[doc = "network. Once this returns without an error, the wallet will continually"]
        #[doc = "attempt to re-broadcast the transaction on start up, until it enters the"]
        #[doc = "chain."]
        async fn publish_transaction(
            &self,
            request: tonic::Request<super::Transaction>,
        ) -> Result<tonic::Response<super::PublishResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "SendOutputs is similar to the existing sendmany call in Bitcoind, and"]
        #[doc = "allows the caller to create a transaction that sends to several outputs at"]
        #[doc = "once. This is ideal when wanting to batch create a set of transactions."]
        async fn send_outputs(
            &self,
            request: tonic::Request<super::SendOutputsRequest>,
        ) -> Result<tonic::Response<super::SendOutputsResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "EstimateFee attempts to query the internal fee estimator of the wallet to"]
        #[doc = "determine the fee (in sat/kw) to attach to a transaction in order to"]
        #[doc = "achieve the confirmation target."]
        async fn estimate_fee(
            &self,
            request: tonic::Request<super::EstimateFeeRequest>,
        ) -> Result<tonic::Response<super::EstimateFeeResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "PendingSweeps returns lists of on-chain outputs that lnd is currently"]
        #[doc = "attempting to sweep within its central batching engine. Outputs with similar"]
        #[doc = "fee rates are batched together in order to sweep them within a single"]
        #[doc = "transaction."]
        #[doc = ""]
        #[doc = "NOTE: Some of the fields within PendingSweepsRequest are not guaranteed to"]
        #[doc = "remain supported. This is an advanced API that depends on the internals of"]
        #[doc = "the UtxoSweeper, so things may change."]
        async fn pending_sweeps(
            &self,
            request: tonic::Request<super::PendingSweepsRequest>,
        ) -> Result<tonic::Response<super::PendingSweepsResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "BumpFee bumps the fee of an arbitrary input within a transaction. This RPC"]
        #[doc = "takes a different approach than bitcoind's bumpfee command. lnd has a"]
        #[doc = "central batching engine in which inputs with similar fee rates are batched"]
        #[doc = "together to save on transaction fees. Due to this, we cannot rely on"]
        #[doc = "bumping the fee on a specific transaction, since transactions can change at"]
        #[doc = "any point with the addition of new inputs. The list of inputs that"]
        #[doc = "currently exist within lnd's central batching engine can be retrieved"]
        #[doc = "through the PendingSweeps RPC."]
        #[doc = ""]
        #[doc = "When bumping the fee of an input that currently exists within lnd's central"]
        #[doc = "batching engine, a higher fee transaction will be created that replaces the"]
        #[doc = "lower fee transaction through the Replace-By-Fee (RBF) policy. If it"]
        #[doc = ""]
        #[doc = "This RPC also serves useful when wanting to perform a Child-Pays-For-Parent"]
        #[doc = "(CPFP), where the child transaction pays for its parent's fee. This can be"]
        #[doc = "done by specifying an outpoint within the low fee transaction that is under"]
        #[doc = "the control of the wallet."]
        #[doc = ""]
        #[doc = "The fee preference can be expressed either as a specific fee rate or a delta"]
        #[doc = "of blocks in which the output should be swept on-chain within. If a fee"]
        #[doc = "preference is not explicitly specified, then an error is returned."]
        #[doc = ""]
        #[doc = "Note that this RPC currently doesn't perform any validation checks on the"]
        #[doc = "fee preference being provided. For now, the responsibility of ensuring that"]
        #[doc = "the new fee preference is sufficient is delegated to the user."]
        async fn bump_fee(
            &self,
            request: tonic::Request<super::BumpFeeRequest>,
        ) -> Result<tonic::Response<super::BumpFeeResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "ListSweeps returns a list of the sweep transactions our node has produced."]
        #[doc = "Note that these sweeps may not be confirmed yet, as we record sweeps on"]
        #[doc = "broadcast, not confirmation."]
        async fn list_sweeps(
            &self,
            request: tonic::Request<super::ListSweepsRequest>,
        ) -> Result<tonic::Response<super::ListSweepsResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "LabelTransaction adds a label to a transaction. If the transaction already"]
        #[doc = "has a label the call will fail unless the overwrite bool is set. This will"]
        #[doc = "overwrite the exiting transaction label. Labels must not be empty, and"]
        #[doc = "cannot exceed 500 characters."]
        async fn label_transaction(
            &self,
            request: tonic::Request<super::LabelTransactionRequest>,
        ) -> Result<tonic::Response<super::LabelTransactionResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "FundPsbt creates a fully populated PSBT that contains enough inputs to fund"]
        #[doc = "the outputs specified in the template. There are two ways of specifying a"]
        #[doc = "template: Either by passing in a PSBT with at least one output declared or"]
        #[doc = "by passing in a raw TxTemplate message."]
        #[doc = ""]
        #[doc = "If there are no inputs specified in the template, coin selection is"]
        #[doc = "performed automatically. If the template does contain any inputs, it is"]
        #[doc = "assumed that full coin selection happened externally and no additional"]
        #[doc = "inputs are added. If the specified inputs aren't enough to fund the outputs"]
        #[doc = "with the given fee rate, an error is returned."]
        #[doc = ""]
        #[doc = "After either selecting or verifying the inputs, all input UTXOs are locked"]
        #[doc = "with an internal app ID."]
        #[doc = ""]
        #[doc = "NOTE: If this method returns without an error, it is the caller's"]
        #[doc = "responsibility to either spend the locked UTXOs (by finalizing and then"]
        #[doc = "publishing the transaction) or to unlock/release the locked UTXOs in case of"]
        #[doc = "an error on the caller's side."]
        async fn fund_psbt(
            &self,
            request: tonic::Request<super::FundPsbtRequest>,
        ) -> Result<tonic::Response<super::FundPsbtResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "SignPsbt expects a partial transaction with all inputs and outputs fully"]
        #[doc = "declared and tries to sign all unsigned inputs that have all required fields"]
        #[doc = "(UTXO information, BIP32 derivation information, witness or sig scripts)"]
        #[doc = "set."]
        #[doc = "If no error is returned, the PSBT is ready to be given to the next signer or"]
        #[doc = "to be finalized if lnd was the last signer."]
        #[doc = ""]
        #[doc = "NOTE: This RPC only signs inputs (and only those it can sign), it does not"]
        #[doc = "perform any other tasks (such as coin selection, UTXO locking or"]
        #[doc = "input/output/fee value validation, PSBT finalization). Any input that is"]
        #[doc = "incomplete will be skipped."]
        async fn sign_psbt(
            &self,
            request: tonic::Request<super::SignPsbtRequest>,
        ) -> Result<tonic::Response<super::SignPsbtResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "FinalizePsbt expects a partial transaction with all inputs and outputs fully"]
        #[doc = "declared and tries to sign all inputs that belong to the wallet. Lnd must be"]
        #[doc = "the last signer of the transaction. That means, if there are any unsigned"]
        #[doc = "non-witness inputs or inputs without UTXO information attached or inputs"]
        #[doc = "without witness data that do not belong to lnd's wallet, this method will"]
        #[doc = "fail. If no error is returned, the PSBT is ready to be extracted and the"]
        #[doc = "final TX within to be broadcast."]
        #[doc = ""]
        #[doc = "NOTE: This method does NOT publish the transaction once finalized. It is the"]
        #[doc = "caller's responsibility to either publish the transaction on success or"]
        #[doc = "unlock/release any locked UTXOs in case of an error in this method."]
        async fn finalize_psbt(
            &self,
            request: tonic::Request<super::FinalizePsbtRequest>,
        ) -> Result<tonic::Response<super::FinalizePsbtResponse>, tonic::Status>;
    }
    #[doc = " WalletKit is a service that gives access to the core functionalities of the"]
    #[doc = " daemon's wallet."]
    #[derive(Debug)]
    pub struct WalletKitServer<T: WalletKit> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WalletKit> WalletKitServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WalletKitServer<T>
    where
        T: WalletKit,
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
                "/walletrpc.WalletKit/ListUnspent" => {
                    #[allow(non_camel_case_types)]
                    struct ListUnspentSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::ListUnspentRequest> for ListUnspentSvc<T> {
                        type Response = super::ListUnspentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUnspentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_unspent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListUnspentSvc(inner);
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
                "/walletrpc.WalletKit/LeaseOutput" => {
                    #[allow(non_camel_case_types)]
                    struct LeaseOutputSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::LeaseOutputRequest> for LeaseOutputSvc<T> {
                        type Response = super::LeaseOutputResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeaseOutputRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).lease_output(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LeaseOutputSvc(inner);
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
                "/walletrpc.WalletKit/ReleaseOutput" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseOutputSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::ReleaseOutputRequest>
                        for ReleaseOutputSvc<T>
                    {
                        type Response = super::ReleaseOutputResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReleaseOutputRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).release_output(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReleaseOutputSvc(inner);
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
                "/walletrpc.WalletKit/ListLeases" => {
                    #[allow(non_camel_case_types)]
                    struct ListLeasesSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::ListLeasesRequest> for ListLeasesSvc<T> {
                        type Response = super::ListLeasesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListLeasesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_leases(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListLeasesSvc(inner);
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
                "/walletrpc.WalletKit/DeriveNextKey" => {
                    #[allow(non_camel_case_types)]
                    struct DeriveNextKeySvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::KeyReq> for DeriveNextKeySvc<T> {
                        type Response = super::super::signrpc::KeyDescriptor;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::KeyReq>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).derive_next_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeriveNextKeySvc(inner);
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
                "/walletrpc.WalletKit/DeriveKey" => {
                    #[allow(non_camel_case_types)]
                    struct DeriveKeySvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit>
                        tonic::server::UnaryService<super::super::signrpc::KeyLocator>
                        for DeriveKeySvc<T>
                    {
                        type Response = super::super::signrpc::KeyDescriptor;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::signrpc::KeyLocator>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).derive_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeriveKeySvc(inner);
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
                "/walletrpc.WalletKit/NextAddr" => {
                    #[allow(non_camel_case_types)]
                    struct NextAddrSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::AddrRequest> for NextAddrSvc<T> {
                        type Response = super::AddrResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddrRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).next_addr(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NextAddrSvc(inner);
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
                "/walletrpc.WalletKit/ListAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct ListAccountsSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::ListAccountsRequest> for ListAccountsSvc<T> {
                        type Response = super::ListAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAccountsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAccountsSvc(inner);
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
                "/walletrpc.WalletKit/ImportAccount" => {
                    #[allow(non_camel_case_types)]
                    struct ImportAccountSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::ImportAccountRequest>
                        for ImportAccountSvc<T>
                    {
                        type Response = super::ImportAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).import_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ImportAccountSvc(inner);
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
                "/walletrpc.WalletKit/ImportPublicKey" => {
                    #[allow(non_camel_case_types)]
                    struct ImportPublicKeySvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::ImportPublicKeyRequest>
                        for ImportPublicKeySvc<T>
                    {
                        type Response = super::ImportPublicKeyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportPublicKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).import_public_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ImportPublicKeySvc(inner);
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
                "/walletrpc.WalletKit/PublishTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct PublishTransactionSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::Transaction> for PublishTransactionSvc<T> {
                        type Response = super::PublishResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Transaction>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).publish_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PublishTransactionSvc(inner);
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
                "/walletrpc.WalletKit/SendOutputs" => {
                    #[allow(non_camel_case_types)]
                    struct SendOutputsSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::SendOutputsRequest> for SendOutputsSvc<T> {
                        type Response = super::SendOutputsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendOutputsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_outputs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendOutputsSvc(inner);
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
                "/walletrpc.WalletKit/EstimateFee" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateFeeSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::EstimateFeeRequest> for EstimateFeeSvc<T> {
                        type Response = super::EstimateFeeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EstimateFeeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).estimate_fee(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EstimateFeeSvc(inner);
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
                "/walletrpc.WalletKit/PendingSweeps" => {
                    #[allow(non_camel_case_types)]
                    struct PendingSweepsSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::PendingSweepsRequest>
                        for PendingSweepsSvc<T>
                    {
                        type Response = super::PendingSweepsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PendingSweepsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pending_sweeps(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PendingSweepsSvc(inner);
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
                "/walletrpc.WalletKit/BumpFee" => {
                    #[allow(non_camel_case_types)]
                    struct BumpFeeSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::BumpFeeRequest> for BumpFeeSvc<T> {
                        type Response = super::BumpFeeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BumpFeeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).bump_fee(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BumpFeeSvc(inner);
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
                "/walletrpc.WalletKit/ListSweeps" => {
                    #[allow(non_camel_case_types)]
                    struct ListSweepsSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::ListSweepsRequest> for ListSweepsSvc<T> {
                        type Response = super::ListSweepsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSweepsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_sweeps(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSweepsSvc(inner);
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
                "/walletrpc.WalletKit/LabelTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct LabelTransactionSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::LabelTransactionRequest>
                        for LabelTransactionSvc<T>
                    {
                        type Response = super::LabelTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LabelTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).label_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LabelTransactionSvc(inner);
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
                "/walletrpc.WalletKit/FundPsbt" => {
                    #[allow(non_camel_case_types)]
                    struct FundPsbtSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::FundPsbtRequest> for FundPsbtSvc<T> {
                        type Response = super::FundPsbtResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FundPsbtRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fund_psbt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundPsbtSvc(inner);
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
                "/walletrpc.WalletKit/SignPsbt" => {
                    #[allow(non_camel_case_types)]
                    struct SignPsbtSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::SignPsbtRequest> for SignPsbtSvc<T> {
                        type Response = super::SignPsbtResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignPsbtRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sign_psbt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignPsbtSvc(inner);
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
                "/walletrpc.WalletKit/FinalizePsbt" => {
                    #[allow(non_camel_case_types)]
                    struct FinalizePsbtSvc<T: WalletKit>(pub Arc<T>);
                    impl<T: WalletKit> tonic::server::UnaryService<super::FinalizePsbtRequest> for FinalizePsbtSvc<T> {
                        type Response = super::FinalizePsbtResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinalizePsbtRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).finalize_psbt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinalizePsbtSvc(inner);
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
    impl<T: WalletKit> Clone for WalletKitServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WalletKit> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WalletKit> tonic::transport::NamedService for WalletKitServer<T> {
        const NAME: &'static str = "walletrpc.WalletKit";
    }
}
