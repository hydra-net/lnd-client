#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCustomMessagesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomMessage {
    /// Peer from which the message originates
    #[prost(bytes = "vec", tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<u8>,
    /// Message type. This value will be in the custom range (>= 32768).
    #[prost(uint32, tag = "2")]
    pub r#type: u32,
    /// Raw message data
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCustomMessageRequest {
    /// Peer to send the message to
    #[prost(bytes = "vec", tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<u8>,
    /// Message type. This value needs to be in the custom range (>= 32768).
    #[prost(uint32, tag = "2")]
    pub r#type: u32,
    /// Raw message data.
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCustomMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Utxo {
    /// The type of address
    #[prost(enumeration = "AddressType", tag = "1")]
    pub address_type: i32,
    /// The address
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// The value of the unspent coin in satoshis
    #[prost(int64, tag = "3")]
    pub amount_sat: i64,
    /// The pkscript in hex
    #[prost(string, tag = "4")]
    pub pk_script: ::prost::alloc::string::String,
    /// The outpoint in format txid:n
    #[prost(message, optional, tag = "5")]
    pub outpoint: ::core::option::Option<OutPoint>,
    /// The number of confirmations for the Utxo
    #[prost(int64, tag = "6")]
    pub confirmations: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputDetail {
    /// The type of the output
    #[prost(enumeration = "OutputScriptType", tag = "1")]
    pub output_type: i32,
    /// The address
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// The pkscript in hex
    #[prost(string, tag = "3")]
    pub pk_script: ::prost::alloc::string::String,
    /// The output index used in the raw transaction
    #[prost(int64, tag = "4")]
    pub output_index: i64,
    /// The value of the output coin in satoshis
    #[prost(int64, tag = "5")]
    pub amount: i64,
    /// Denotes if the output is controlled by the internal wallet
    #[prost(bool, tag = "6")]
    pub is_our_address: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// The transaction hash
    #[prost(string, tag = "1")]
    pub tx_hash: ::prost::alloc::string::String,
    /// The transaction amount, denominated in satoshis
    #[prost(int64, tag = "2")]
    pub amount: i64,
    /// The number of confirmations
    #[prost(int32, tag = "3")]
    pub num_confirmations: i32,
    /// The hash of the block this transaction was included in
    #[prost(string, tag = "4")]
    pub block_hash: ::prost::alloc::string::String,
    /// The height of the block this transaction was included in
    #[prost(int32, tag = "5")]
    pub block_height: i32,
    /// Timestamp of this transaction
    #[prost(int64, tag = "6")]
    pub time_stamp: i64,
    /// Fees paid for this transaction
    #[prost(int64, tag = "7")]
    pub total_fees: i64,
    /// Addresses that received funds for this transaction. Deprecated as it is
    /// now incorporated in the output_details field.
    #[deprecated]
    #[prost(string, repeated, tag = "8")]
    pub dest_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Outputs that received funds for this transaction
    #[prost(message, repeated, tag = "11")]
    pub output_details: ::prost::alloc::vec::Vec<OutputDetail>,
    /// The raw transaction hex.
    #[prost(string, tag = "9")]
    pub raw_tx_hex: ::prost::alloc::string::String,
    /// A label that was optionally set on transaction broadcast.
    #[prost(string, tag = "10")]
    pub label: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsRequest {
    ///
    ///The height from which to list transactions, inclusive. If this value is
    ///greater than end_height, transactions will be read in reverse.
    #[prost(int32, tag = "1")]
    pub start_height: i32,
    ///
    ///The height until which to list transactions, inclusive. To include
    ///unconfirmed transactions, this value should be set to -1, which will
    ///return transactions from start_height until the current chain tip and
    ///unconfirmed transactions. If no end_height is provided, the call will
    ///default to this option.
    #[prost(int32, tag = "2")]
    pub end_height: i32,
    /// An optional filter to only include transactions relevant to an account.
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionDetails {
    /// The list of transactions relevant to the wallet.
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeLimit {
    #[prost(oneof = "fee_limit::Limit", tags = "1, 3, 2")]
    pub limit: ::core::option::Option<fee_limit::Limit>,
}
/// Nested message and enum types in `FeeLimit`.
pub mod fee_limit {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Limit {
        ///
        ///The fee limit expressed as a fixed amount of satoshis.
        ///
        ///The fields fixed and fixed_msat are mutually exclusive.
        #[prost(int64, tag = "1")]
        Fixed(i64),
        ///
        ///The fee limit expressed as a fixed amount of millisatoshis.
        ///
        ///The fields fixed and fixed_msat are mutually exclusive.
        #[prost(int64, tag = "3")]
        FixedMsat(i64),
        /// The fee limit expressed as a percentage of the payment amount.
        #[prost(int64, tag = "2")]
        Percent(i64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendRequest {
    ///
    ///The identity pubkey of the payment recipient. When using REST, this field
    ///must be encoded as base64.
    #[prost(bytes = "vec", tag = "1")]
    pub dest: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The hex-encoded identity pubkey of the payment recipient. Deprecated now
    ///that the REST gateway supports base64 encoding of bytes fields.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub dest_string: ::prost::alloc::string::String,
    ///
    ///The amount to send expressed in satoshis.
    ///
    ///The fields amt and amt_msat are mutually exclusive.
    #[prost(int64, tag = "3")]
    pub amt: i64,
    ///
    ///The amount to send expressed in millisatoshis.
    ///
    ///The fields amt and amt_msat are mutually exclusive.
    #[prost(int64, tag = "12")]
    pub amt_msat: i64,
    ///
    ///The hash to use within the payment's HTLC. When using REST, this field
    ///must be encoded as base64.
    #[prost(bytes = "vec", tag = "4")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The hex-encoded hash to use within the payment's HTLC. Deprecated now
    ///that the REST gateway supports base64 encoding of bytes fields.
    #[deprecated]
    #[prost(string, tag = "5")]
    pub payment_hash_string: ::prost::alloc::string::String,
    ///
    ///A bare-bones invoice for a payment within the Lightning Network. With the
    ///details of the invoice, the sender has all the data necessary to send a
    ///payment to the recipient.
    #[prost(string, tag = "6")]
    pub payment_request: ::prost::alloc::string::String,
    ///
    ///The CLTV delta from the current height that should be used to set the
    ///timelock for the final hop.
    #[prost(int32, tag = "7")]
    pub final_cltv_delta: i32,
    ///
    ///The maximum number of satoshis that will be paid as a fee of the payment.
    ///This value can be represented either as a percentage of the amount being
    ///sent, or as a fixed amount of the maximum fee the user is willing the pay to
    ///send the payment. If not specified, lnd will use a default value of 100%
    ///fees for small amounts (<=1k sat) or 5% fees for larger amounts.
    #[prost(message, optional, tag = "8")]
    pub fee_limit: ::core::option::Option<FeeLimit>,
    ///
    ///The channel id of the channel that must be taken to the first hop. If zero,
    ///any channel may be used.
    #[prost(uint64, tag = "9")]
    pub outgoing_chan_id: u64,
    ///
    ///The pubkey of the last hop of the route. If empty, any hop may be used.
    #[prost(bytes = "vec", tag = "13")]
    pub last_hop_pubkey: ::prost::alloc::vec::Vec<u8>,
    ///
    ///An optional maximum total time lock for the route. This should not exceed
    ///lnd's `--max-cltv-expiry` setting. If zero, then the value of
    ///`--max-cltv-expiry` is enforced.
    #[prost(uint32, tag = "10")]
    pub cltv_limit: u32,
    ///
    ///An optional field that can be used to pass an arbitrary set of TLV records
    ///to a peer which understands the new records. This can be used to pass
    ///application specific data during the payment attempt. Record types are
    ///required to be in the custom range >= 65536. When using REST, the values
    ///must be encoded as base64.
    #[prost(map = "uint64, bytes", tag = "11")]
    pub dest_custom_records: ::std::collections::HashMap<u64, ::prost::alloc::vec::Vec<u8>>,
    /// If set, circular payments to self are permitted.
    #[prost(bool, tag = "14")]
    pub allow_self_payment: bool,
    ///
    ///Features assumed to be supported by the final node. All transitive feature
    ///dependencies must also be set properly. For a given feature bit pair, either
    ///optional or remote may be set, but not both. If this field is nil or empty,
    ///the router will try to load destination features from the graph as a
    ///fallback.
    #[prost(enumeration = "FeatureBit", repeated, tag = "15")]
    pub dest_features: ::prost::alloc::vec::Vec<i32>,
    ///
    ///The payment address of the generated invoice.
    #[prost(bytes = "vec", tag = "16")]
    pub payment_addr: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    #[prost(string, tag = "1")]
    pub payment_error: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub payment_preimage: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub payment_route: ::core::option::Option<Route>,
    #[prost(bytes = "vec", tag = "4")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendToRouteRequest {
    ///
    ///The payment hash to use for the HTLC. When using REST, this field must be
    ///encoded as base64.
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    ///
    ///An optional hex-encoded payment hash to be used for the HTLC. Deprecated now
    ///that the REST gateway supports base64 encoding of bytes fields.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub payment_hash_string: ::prost::alloc::string::String,
    /// Route that should be used to attempt to complete the payment.
    #[prost(message, optional, tag = "4")]
    pub route: ::core::option::Option<Route>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAcceptRequest {
    /// The pubkey of the node that wishes to open an inbound channel.
    #[prost(bytes = "vec", tag = "1")]
    pub node_pubkey: ::prost::alloc::vec::Vec<u8>,
    /// The hash of the genesis block that the proposed channel resides in.
    #[prost(bytes = "vec", tag = "2")]
    pub chain_hash: ::prost::alloc::vec::Vec<u8>,
    /// The pending channel id.
    #[prost(bytes = "vec", tag = "3")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
    /// The funding amount in satoshis that initiator wishes to use in the
    /// channel.
    #[prost(uint64, tag = "4")]
    pub funding_amt: u64,
    /// The push amount of the proposed channel in millisatoshis.
    #[prost(uint64, tag = "5")]
    pub push_amt: u64,
    /// The dust limit of the initiator's commitment tx.
    #[prost(uint64, tag = "6")]
    pub dust_limit: u64,
    /// The maximum amount of coins in millisatoshis that can be pending in this
    /// channel.
    #[prost(uint64, tag = "7")]
    pub max_value_in_flight: u64,
    /// The minimum amount of satoshis the initiator requires us to have at all
    /// times.
    #[prost(uint64, tag = "8")]
    pub channel_reserve: u64,
    /// The smallest HTLC in millisatoshis that the initiator will accept.
    #[prost(uint64, tag = "9")]
    pub min_htlc: u64,
    /// The initial fee rate that the initiator suggests for both commitment
    /// transactions.
    #[prost(uint64, tag = "10")]
    pub fee_per_kw: u64,
    ///
    ///The number of blocks to use for the relative time lock in the pay-to-self
    ///output of both commitment transactions.
    #[prost(uint32, tag = "11")]
    pub csv_delay: u32,
    /// The total number of incoming HTLC's that the initiator will accept.
    #[prost(uint32, tag = "12")]
    pub max_accepted_htlcs: u32,
    /// A bit-field which the initiator uses to specify proposed channel
    /// behavior.
    #[prost(uint32, tag = "13")]
    pub channel_flags: u32,
    /// The commitment type the initiator wishes to use for the proposed channel.
    #[prost(enumeration = "CommitmentType", tag = "14")]
    pub commitment_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelAcceptResponse {
    /// Whether or not the client accepts the channel.
    #[prost(bool, tag = "1")]
    pub accept: bool,
    /// The pending channel id to which this response applies.
    #[prost(bytes = "vec", tag = "2")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///An optional error to send the initiating party to indicate why the channel
    ///was rejected. This field *should not* contain sensitive information, it will
    ///be sent to the initiating party. This field should only be set if accept is
    ///false, the channel will be rejected if an error is set with accept=true
    ///because the meaning of this response is ambiguous. Limited to 500
    ///characters.
    #[prost(string, tag = "3")]
    pub error: ::prost::alloc::string::String,
    ///
    ///The upfront shutdown address to use if the initiating peer supports option
    ///upfront shutdown script (see ListPeers for the features supported). Note
    ///that the channel open will fail if this value is set for a peer that does
    ///not support this feature bit.
    #[prost(string, tag = "4")]
    pub upfront_shutdown: ::prost::alloc::string::String,
    ///
    ///The csv delay (in blocks) that we require for the remote party.
    #[prost(uint32, tag = "5")]
    pub csv_delay: u32,
    ///
    ///The reserve amount in satoshis that we require the remote peer to adhere to.
    ///We require that the remote peer always have some reserve amount allocated to
    ///them so that there is always a disincentive to broadcast old state (if they
    ///hold 0 sats on their side of the channel, there is nothing to lose).
    #[prost(uint64, tag = "6")]
    pub reserve_sat: u64,
    ///
    ///The maximum amount of funds in millisatoshis that we allow the remote peer
    ///to have in outstanding htlcs.
    #[prost(uint64, tag = "7")]
    pub in_flight_max_msat: u64,
    ///
    ///The maximum number of htlcs that the remote peer can offer us.
    #[prost(uint32, tag = "8")]
    pub max_htlc_count: u32,
    ///
    ///The minimum value in millisatoshis for incoming htlcs on the channel.
    #[prost(uint64, tag = "9")]
    pub min_htlc_in: u64,
    ///
    ///The number of confirmations we require before we consider the channel open.
    #[prost(uint32, tag = "10")]
    pub min_accept_depth: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPoint {
    /// The index of the output of the funding transaction
    #[prost(uint32, tag = "3")]
    pub output_index: u32,
    #[prost(oneof = "channel_point::FundingTxid", tags = "1, 2")]
    pub funding_txid: ::core::option::Option<channel_point::FundingTxid>,
}
/// Nested message and enum types in `ChannelPoint`.
pub mod channel_point {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FundingTxid {
        ///
        ///Txid of the funding transaction. When using REST, this field must be
        ///encoded as base64.
        #[prost(bytes, tag = "1")]
        FundingTxidBytes(::prost::alloc::vec::Vec<u8>),
        ///
        ///Hex-encoded string representing the byte-reversed hash of the funding
        ///transaction.
        #[prost(string, tag = "2")]
        FundingTxidStr(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutPoint {
    /// Raw bytes representing the transaction id.
    #[prost(bytes = "vec", tag = "1")]
    pub txid_bytes: ::prost::alloc::vec::Vec<u8>,
    /// Reversed, hex-encoded string representing the transaction id.
    #[prost(string, tag = "2")]
    pub txid_str: ::prost::alloc::string::String,
    /// The index of the output on the transaction.
    #[prost(uint32, tag = "3")]
    pub output_index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightningAddress {
    /// The identity pubkey of the Lightning node.
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
    /// The network location of the lightning node, e.g. `69.69.69.69:1337` or
    /// `localhost:10011`.
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateFeeRequest {
    /// The map from addresses to amounts for the transaction.
    #[prost(map = "string, int64", tag = "1")]
    pub addr_to_amount: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// The target number of blocks that this transaction should be confirmed
    /// by.
    #[prost(int32, tag = "2")]
    pub target_conf: i32,
    /// The minimum number of confirmations each one of your outputs used for
    /// the transaction must satisfy.
    #[prost(int32, tag = "3")]
    pub min_confs: i32,
    /// Whether unconfirmed outputs should be used as inputs for the transaction.
    #[prost(bool, tag = "4")]
    pub spend_unconfirmed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateFeeResponse {
    /// The total fee in satoshis.
    #[prost(int64, tag = "1")]
    pub fee_sat: i64,
    /// Deprecated, use sat_per_vbyte.
    /// The fee rate in satoshi/vbyte.
    #[deprecated]
    #[prost(int64, tag = "2")]
    pub feerate_sat_per_byte: i64,
    /// The fee rate in satoshi/vbyte.
    #[prost(uint64, tag = "3")]
    pub sat_per_vbyte: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendManyRequest {
    /// The map from addresses to amounts
    #[prost(map = "string, int64", tag = "1")]
    pub addr_to_amount: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// The target number of blocks that this transaction should be confirmed
    /// by.
    #[prost(int32, tag = "3")]
    pub target_conf: i32,
    /// A manual fee rate set in sat/vbyte that should be used when crafting the
    /// transaction.
    #[prost(uint64, tag = "4")]
    pub sat_per_vbyte: u64,
    /// Deprecated, use sat_per_vbyte.
    /// A manual fee rate set in sat/vbyte that should be used when crafting the
    /// transaction.
    #[deprecated]
    #[prost(int64, tag = "5")]
    pub sat_per_byte: i64,
    /// An optional label for the transaction, limited to 500 characters.
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    /// The minimum number of confirmations each one of your outputs used for
    /// the transaction must satisfy.
    #[prost(int32, tag = "7")]
    pub min_confs: i32,
    /// Whether unconfirmed outputs should be used as inputs for the transaction.
    #[prost(bool, tag = "8")]
    pub spend_unconfirmed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendManyResponse {
    /// The id of the transaction
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCoinsRequest {
    /// The address to send coins to
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    /// The amount in satoshis to send
    #[prost(int64, tag = "2")]
    pub amount: i64,
    /// The target number of blocks that this transaction should be confirmed
    /// by.
    #[prost(int32, tag = "3")]
    pub target_conf: i32,
    /// A manual fee rate set in sat/vbyte that should be used when crafting the
    /// transaction.
    #[prost(uint64, tag = "4")]
    pub sat_per_vbyte: u64,
    /// Deprecated, use sat_per_vbyte.
    /// A manual fee rate set in sat/vbyte that should be used when crafting the
    /// transaction.
    #[deprecated]
    #[prost(int64, tag = "5")]
    pub sat_per_byte: i64,
    ///
    ///If set, then the amount field will be ignored, and lnd will attempt to
    ///send all the coins under control of the internal wallet to the specified
    ///address.
    #[prost(bool, tag = "6")]
    pub send_all: bool,
    /// An optional label for the transaction, limited to 500 characters.
    #[prost(string, tag = "7")]
    pub label: ::prost::alloc::string::String,
    /// The minimum number of confirmations each one of your outputs used for
    /// the transaction must satisfy.
    #[prost(int32, tag = "8")]
    pub min_confs: i32,
    /// Whether unconfirmed outputs should be used as inputs for the transaction.
    #[prost(bool, tag = "9")]
    pub spend_unconfirmed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCoinsResponse {
    /// The transaction ID of the transaction
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
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
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUnspentResponse {
    /// A list of utxos
    #[prost(message, repeated, tag = "1")]
    pub utxos: ::prost::alloc::vec::Vec<Utxo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAddressRequest {
    /// The type of address to generate.
    #[prost(enumeration = "AddressType", tag = "1")]
    pub r#type: i32,
    ///
    ///The name of the account to generate a new address for. If empty, the
    ///default wallet account is used.
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAddressResponse {
    /// The newly generated wallet address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageRequest {
    ///
    ///The message to be signed. When using REST, this field must be encoded as
    ///base64.
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Instead of the default double-SHA256 hashing of the message before signing,
    ///only use one round of hashing instead.
    #[prost(bool, tag = "2")]
    pub single_hash: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessageResponse {
    /// The signature for the given message
    #[prost(string, tag = "1")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMessageRequest {
    ///
    ///The message over which the signature is to be verified. When using REST,
    ///this field must be encoded as base64.
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// The signature to be verified over the given message
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMessageResponse {
    /// Whether the signature was valid over the given message
    #[prost(bool, tag = "1")]
    pub valid: bool,
    /// The pubkey recovered from the signature
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectPeerRequest {
    ///
    ///Lightning address of the peer to connect to.
    #[prost(message, optional, tag = "1")]
    pub addr: ::core::option::Option<LightningAddress>,
    ///
    ///If set, the daemon will attempt to persistently connect to the target
    ///peer. Otherwise, the call will be synchronous.
    #[prost(bool, tag = "2")]
    pub perm: bool,
    ///
    ///The connection timeout value (in seconds) for this request. It won't affect
    ///other requests.
    #[prost(uint64, tag = "3")]
    pub timeout: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectPeerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectPeerRequest {
    /// The pubkey of the node to disconnect from
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectPeerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Htlc {
    #[prost(bool, tag = "1")]
    pub incoming: bool,
    #[prost(int64, tag = "2")]
    pub amount: i64,
    #[prost(bytes = "vec", tag = "3")]
    pub hash_lock: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "4")]
    pub expiration_height: u32,
    /// Index identifying the htlc on the channel.
    #[prost(uint64, tag = "5")]
    pub htlc_index: u64,
    /// If this HTLC is involved in a forwarding operation, this field indicates
    /// the forwarding channel. For an outgoing htlc, it is the incoming channel.
    /// For an incoming htlc, it is the outgoing channel. When the htlc
    /// originates from this node or this node is the final destination,
    /// forwarding_channel will be zero. The forwarding channel will also be zero
    /// for htlcs that need to be forwarded but don't have a forwarding decision
    /// persisted yet.
    #[prost(uint64, tag = "6")]
    pub forwarding_channel: u64,
    /// Index identifying the htlc on the forwarding channel.
    #[prost(uint64, tag = "7")]
    pub forwarding_htlc_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelConstraints {
    ///
    ///The CSV delay expressed in relative blocks. If the channel is force closed,
    ///we will need to wait for this many blocks before we can regain our funds.
    #[prost(uint32, tag = "1")]
    pub csv_delay: u32,
    /// The minimum satoshis this node is required to reserve in its balance.
    #[prost(uint64, tag = "2")]
    pub chan_reserve_sat: u64,
    /// The dust limit (in satoshis) of the initiator's commitment tx.
    #[prost(uint64, tag = "3")]
    pub dust_limit_sat: u64,
    /// The maximum amount of coins in millisatoshis that can be pending in this
    /// channel.
    #[prost(uint64, tag = "4")]
    pub max_pending_amt_msat: u64,
    /// The smallest HTLC in millisatoshis that the initiator will accept.
    #[prost(uint64, tag = "5")]
    pub min_htlc_msat: u64,
    /// The total number of incoming HTLC's that the initiator will accept.
    #[prost(uint32, tag = "6")]
    pub max_accepted_htlcs: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// Whether this channel is active or not
    #[prost(bool, tag = "1")]
    pub active: bool,
    /// The identity pubkey of the remote node
    #[prost(string, tag = "2")]
    pub remote_pubkey: ::prost::alloc::string::String,
    ///
    ///The outpoint (txid:index) of the funding transaction. With this value, Bob
    ///will be able to generate a signature for Alice's version of the commitment
    ///transaction.
    #[prost(string, tag = "3")]
    pub channel_point: ::prost::alloc::string::String,
    ///
    ///The unique channel ID for the channel. The first 3 bytes are the block
    ///height, the next 3 the index within the block, and the last 2 bytes are the
    ///output index for the channel.
    #[prost(uint64, tag = "4")]
    pub chan_id: u64,
    /// The total amount of funds held in this channel
    #[prost(int64, tag = "5")]
    pub capacity: i64,
    /// This node's current balance in this channel
    #[prost(int64, tag = "6")]
    pub local_balance: i64,
    /// The counterparty's current balance in this channel
    #[prost(int64, tag = "7")]
    pub remote_balance: i64,
    ///
    ///The amount calculated to be paid in fees for the current set of commitment
    ///transactions. The fee amount is persisted with the channel in order to
    ///allow the fee amount to be removed and recalculated with each channel state
    ///update, including updates that happen after a system restart.
    #[prost(int64, tag = "8")]
    pub commit_fee: i64,
    /// The weight of the commitment transaction
    #[prost(int64, tag = "9")]
    pub commit_weight: i64,
    ///
    ///The required number of satoshis per kilo-weight that the requester will pay
    ///at all times, for both the funding transaction and commitment transaction.
    ///This value can later be updated once the channel is open.
    #[prost(int64, tag = "10")]
    pub fee_per_kw: i64,
    /// The unsettled balance in this channel
    #[prost(int64, tag = "11")]
    pub unsettled_balance: i64,
    ///
    ///The total number of satoshis we've sent within this channel.
    #[prost(int64, tag = "12")]
    pub total_satoshis_sent: i64,
    ///
    ///The total number of satoshis we've received within this channel.
    #[prost(int64, tag = "13")]
    pub total_satoshis_received: i64,
    ///
    ///The total number of updates conducted within this channel.
    #[prost(uint64, tag = "14")]
    pub num_updates: u64,
    ///
    ///The list of active, uncleared HTLCs currently pending within the channel.
    #[prost(message, repeated, tag = "15")]
    pub pending_htlcs: ::prost::alloc::vec::Vec<Htlc>,
    ///
    ///Deprecated. The CSV delay expressed in relative blocks. If the channel is
    ///force closed, we will need to wait for this many blocks before we can regain
    ///our funds.
    #[deprecated]
    #[prost(uint32, tag = "16")]
    pub csv_delay: u32,
    /// Whether this channel is advertised to the network or not.
    #[prost(bool, tag = "17")]
    pub private: bool,
    /// True if we were the ones that created the channel.
    #[prost(bool, tag = "18")]
    pub initiator: bool,
    /// A set of flags showing the current state of the channel.
    #[prost(string, tag = "19")]
    pub chan_status_flags: ::prost::alloc::string::String,
    /// Deprecated. The minimum satoshis this node is required to reserve in its
    /// balance.
    #[deprecated]
    #[prost(int64, tag = "20")]
    pub local_chan_reserve_sat: i64,
    ///
    ///Deprecated. The minimum satoshis the other node is required to reserve in
    ///its balance.
    #[deprecated]
    #[prost(int64, tag = "21")]
    pub remote_chan_reserve_sat: i64,
    /// Deprecated. Use commitment_type.
    #[deprecated]
    #[prost(bool, tag = "22")]
    pub static_remote_key: bool,
    /// The commitment type used by this channel.
    #[prost(enumeration = "CommitmentType", tag = "26")]
    pub commitment_type: i32,
    ///
    ///The number of seconds that the channel has been monitored by the channel
    ///scoring system. Scores are currently not persisted, so this value may be
    ///less than the lifetime of the channel \[EXPERIMENTAL\].
    #[prost(int64, tag = "23")]
    pub lifetime: i64,
    ///
    ///The number of seconds that the remote peer has been observed as being online
    ///by the channel scoring system over the lifetime of the channel
    ///\[EXPERIMENTAL\].
    #[prost(int64, tag = "24")]
    pub uptime: i64,
    ///
    ///Close address is the address that we will enforce payout to on cooperative
    ///close if the channel was opened utilizing option upfront shutdown. This
    ///value can be set on channel open by setting close_address in an open channel
    ///request. If this value is not set, you can still choose a payout address by
    ///cooperatively closing with the delivery_address field set.
    #[prost(string, tag = "25")]
    pub close_address: ::prost::alloc::string::String,
    ///
    ///The amount that the initiator of the channel optionally pushed to the remote
    ///party on channel open. This amount will be zero if the channel initiator did
    ///not push any funds to the remote peer. If the initiator field is true, we
    ///pushed this amount to our peer, if it is false, the remote peer pushed this
    ///amount to us.
    #[prost(uint64, tag = "27")]
    pub push_amount_sat: u64,
    ///
    ///This uint32 indicates if this channel is to be considered 'frozen'. A
    ///frozen channel doest not allow a cooperative channel close by the
    ///initiator. The thaw_height is the height that this restriction stops
    ///applying to the channel. This field is optional, not setting it or using a
    ///value of zero will mean the channel has no additional restrictions. The
    ///height can be interpreted in two ways: as a relative height if the value is
    ///less than 500,000, or as an absolute height otherwise.
    #[prost(uint32, tag = "28")]
    pub thaw_height: u32,
    /// List constraints for the local node.
    #[prost(message, optional, tag = "29")]
    pub local_constraints: ::core::option::Option<ChannelConstraints>,
    /// List constraints for the remote node.
    #[prost(message, optional, tag = "30")]
    pub remote_constraints: ::core::option::Option<ChannelConstraints>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsRequest {
    #[prost(bool, tag = "1")]
    pub active_only: bool,
    #[prost(bool, tag = "2")]
    pub inactive_only: bool,
    #[prost(bool, tag = "3")]
    pub public_only: bool,
    #[prost(bool, tag = "4")]
    pub private_only: bool,
    ///
    ///Filters the response for channels with a target peer's pubkey. If peer is
    ///empty, all channels will be returned.
    #[prost(bytes = "vec", tag = "5")]
    pub peer: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsResponse {
    /// The list of active channels
    #[prost(message, repeated, tag = "11")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCloseSummary {
    /// The outpoint (txid:index) of the funding transaction.
    #[prost(string, tag = "1")]
    pub channel_point: ::prost::alloc::string::String,
    ///  The unique channel ID for the channel.
    #[prost(uint64, tag = "2")]
    pub chan_id: u64,
    /// The hash of the genesis block that this channel resides within.
    #[prost(string, tag = "3")]
    pub chain_hash: ::prost::alloc::string::String,
    /// The txid of the transaction which ultimately closed this channel.
    #[prost(string, tag = "4")]
    pub closing_tx_hash: ::prost::alloc::string::String,
    /// Public key of the remote peer that we formerly had a channel with.
    #[prost(string, tag = "5")]
    pub remote_pubkey: ::prost::alloc::string::String,
    /// Total capacity of the channel.
    #[prost(int64, tag = "6")]
    pub capacity: i64,
    /// Height at which the funding transaction was spent.
    #[prost(uint32, tag = "7")]
    pub close_height: u32,
    /// Settled balance at the time of channel closure
    #[prost(int64, tag = "8")]
    pub settled_balance: i64,
    /// The sum of all the time-locked outputs at the time of channel closure
    #[prost(int64, tag = "9")]
    pub time_locked_balance: i64,
    /// Details on how the channel was closed.
    #[prost(enumeration = "channel_close_summary::ClosureType", tag = "10")]
    pub close_type: i32,
    ///
    ///Open initiator is the party that initiated opening the channel. Note that
    ///this value may be unknown if the channel was closed before we migrated to
    ///store open channel information after close.
    #[prost(enumeration = "Initiator", tag = "11")]
    pub open_initiator: i32,
    ///
    ///Close initiator indicates which party initiated the close. This value will
    ///be unknown for channels that were cooperatively closed before we started
    ///tracking cooperative close initiators. Note that this indicates which party
    ///initiated a close, and it is possible for both to initiate cooperative or
    ///force closes, although only one party's close will be confirmed on chain.
    #[prost(enumeration = "Initiator", tag = "12")]
    pub close_initiator: i32,
    #[prost(message, repeated, tag = "13")]
    pub resolutions: ::prost::alloc::vec::Vec<Resolution>,
}
/// Nested message and enum types in `ChannelCloseSummary`.
pub mod channel_close_summary {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClosureType {
        CooperativeClose = 0,
        LocalForceClose = 1,
        RemoteForceClose = 2,
        BreachClose = 3,
        FundingCanceled = 4,
        Abandoned = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resolution {
    /// The type of output we are resolving.
    #[prost(enumeration = "ResolutionType", tag = "1")]
    pub resolution_type: i32,
    /// The outcome of our on chain action that resolved the outpoint.
    #[prost(enumeration = "ResolutionOutcome", tag = "2")]
    pub outcome: i32,
    /// The outpoint that was spent by the resolution.
    #[prost(message, optional, tag = "3")]
    pub outpoint: ::core::option::Option<OutPoint>,
    /// The amount that was claimed by the resolution.
    #[prost(uint64, tag = "4")]
    pub amount_sat: u64,
    /// The hex-encoded transaction ID of the sweep transaction that spent the
    /// output.
    #[prost(string, tag = "5")]
    pub sweep_txid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosedChannelsRequest {
    #[prost(bool, tag = "1")]
    pub cooperative: bool,
    #[prost(bool, tag = "2")]
    pub local_force: bool,
    #[prost(bool, tag = "3")]
    pub remote_force: bool,
    #[prost(bool, tag = "4")]
    pub breach: bool,
    #[prost(bool, tag = "5")]
    pub funding_canceled: bool,
    #[prost(bool, tag = "6")]
    pub abandoned: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosedChannelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<ChannelCloseSummary>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    /// The identity pubkey of the peer
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
    /// Network address of the peer; eg `127.0.0.1:10011`
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    /// Bytes of data transmitted to this peer
    #[prost(uint64, tag = "4")]
    pub bytes_sent: u64,
    /// Bytes of data transmitted from this peer
    #[prost(uint64, tag = "5")]
    pub bytes_recv: u64,
    /// Satoshis sent to this peer
    #[prost(int64, tag = "6")]
    pub sat_sent: i64,
    /// Satoshis received from this peer
    #[prost(int64, tag = "7")]
    pub sat_recv: i64,
    /// A channel is inbound if the counterparty initiated the channel
    #[prost(bool, tag = "8")]
    pub inbound: bool,
    /// Ping time to this peer
    #[prost(int64, tag = "9")]
    pub ping_time: i64,
    /// The type of sync we are currently performing with this peer.
    #[prost(enumeration = "peer::SyncType", tag = "10")]
    pub sync_type: i32,
    /// Features advertised by the remote peer in their init message.
    #[prost(map = "uint32, message", tag = "11")]
    pub features: ::std::collections::HashMap<u32, Feature>,
    ///
    ///The latest errors received from our peer with timestamps, limited to the 10
    ///most recent errors. These errors are tracked across peer connections, but
    ///are not persisted across lnd restarts. Note that these errors are only
    ///stored for peers that we have channels open with, to prevent peers from
    ///spamming us with errors at no cost.
    #[prost(message, repeated, tag = "12")]
    pub errors: ::prost::alloc::vec::Vec<TimestampedError>,
    ///
    ///The number of times we have recorded this peer going offline or coming
    ///online, recorded across restarts. Note that this value is decreased over
    ///time if the peer has not recently flapped, so that we can forgive peers
    ///with historically high flap counts.
    #[prost(int32, tag = "13")]
    pub flap_count: i32,
    ///
    ///The timestamp of the last flap we observed for this peer. If this value is
    ///zero, we have not observed any flaps for this peer.
    #[prost(int64, tag = "14")]
    pub last_flap_ns: i64,
    ///
    ///The last ping payload the peer has sent to us.
    #[prost(bytes = "vec", tag = "15")]
    pub last_ping_payload: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Peer`.
pub mod peer {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SyncType {
        ///
        ///Denotes that we cannot determine the peer's current sync type.
        UnknownSync = 0,
        ///
        ///Denotes that we are actively receiving new graph updates from the peer.
        ActiveSync = 1,
        ///
        ///Denotes that we are not receiving new graph updates from the peer.
        PassiveSync = 2,
        ///
        ///Denotes that this peer is pinned into an active sync.
        PinnedSync = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampedError {
    /// The unix timestamp in seconds when the error occurred.
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// The string representation of the error sent by our peer.
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeersRequest {
    ///
    ///If true, only the last error that our peer sent us will be returned with
    ///the peer's information, rather than the full set of historic errors we have
    ///stored.
    #[prost(bool, tag = "1")]
    pub latest_error: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeersResponse {
    /// The list of currently connected peers
    #[prost(message, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerEventSubscription {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerEvent {
    /// The identity pubkey of the peer.
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
    #[prost(enumeration = "peer_event::EventType", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `PeerEvent`.
pub mod peer_event {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        PeerOnline = 0,
        PeerOffline = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoResponse {
    /// The version of the LND software that the node is running.
    #[prost(string, tag = "14")]
    pub version: ::prost::alloc::string::String,
    /// The SHA1 commit hash that the daemon is compiled with.
    #[prost(string, tag = "20")]
    pub commit_hash: ::prost::alloc::string::String,
    /// The identity pubkey of the current node.
    #[prost(string, tag = "1")]
    pub identity_pubkey: ::prost::alloc::string::String,
    /// If applicable, the alias of the current node, e.g. "bob"
    #[prost(string, tag = "2")]
    pub alias: ::prost::alloc::string::String,
    /// The color of the current node in hex code format
    #[prost(string, tag = "17")]
    pub color: ::prost::alloc::string::String,
    /// Number of pending channels
    #[prost(uint32, tag = "3")]
    pub num_pending_channels: u32,
    /// Number of active channels
    #[prost(uint32, tag = "4")]
    pub num_active_channels: u32,
    /// Number of inactive channels
    #[prost(uint32, tag = "15")]
    pub num_inactive_channels: u32,
    /// Number of peers
    #[prost(uint32, tag = "5")]
    pub num_peers: u32,
    /// The node's current view of the height of the best block
    #[prost(uint32, tag = "6")]
    pub block_height: u32,
    /// The node's current view of the hash of the best block
    #[prost(string, tag = "8")]
    pub block_hash: ::prost::alloc::string::String,
    /// Timestamp of the block best known to the wallet
    #[prost(int64, tag = "13")]
    pub best_header_timestamp: i64,
    /// Whether the wallet's view is synced to the main chain
    #[prost(bool, tag = "9")]
    pub synced_to_chain: bool,
    /// Whether we consider ourselves synced with the public channel graph.
    #[prost(bool, tag = "18")]
    pub synced_to_graph: bool,
    ///
    ///Whether the current node is connected to testnet. This field is
    ///deprecated and the network field should be used instead
    #[deprecated]
    #[prost(bool, tag = "10")]
    pub testnet: bool,
    /// A list of active chains the node is connected to
    #[prost(message, repeated, tag = "16")]
    pub chains: ::prost::alloc::vec::Vec<Chain>,
    /// The URIs of the current node.
    #[prost(string, repeated, tag = "12")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    ///Features that our node has advertised in our init message, node
    ///announcements and invoices.
    #[prost(map = "uint32, message", tag = "19")]
    pub features: ::std::collections::HashMap<u32, Feature>,
    ///
    ///Indicates whether the HTLC interceptor API is in always-on mode.
    #[prost(bool, tag = "21")]
    pub require_htlc_interceptor: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecoveryInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecoveryInfoResponse {
    /// Whether the wallet is in recovery mode
    #[prost(bool, tag = "1")]
    pub recovery_mode: bool,
    /// Whether the wallet recovery progress is finished
    #[prost(bool, tag = "2")]
    pub recovery_finished: bool,
    /// The recovery progress, ranging from 0 to 1.
    #[prost(double, tag = "3")]
    pub progress: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chain {
    /// The blockchain the node is on (eg bitcoin, litecoin)
    #[prost(string, tag = "1")]
    pub chain: ::prost::alloc::string::String,
    /// The network the node is on (eg regtest, testnet, mainnet)
    #[prost(string, tag = "2")]
    pub network: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmationUpdate {
    #[prost(bytes = "vec", tag = "1")]
    pub block_sha: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag = "2")]
    pub block_height: i32,
    #[prost(uint32, tag = "3")]
    pub num_confs_left: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOpenUpdate {
    #[prost(message, optional, tag = "1")]
    pub channel_point: ::core::option::Option<ChannelPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCloseUpdate {
    #[prost(bytes = "vec", tag = "1")]
    pub closing_txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseChannelRequest {
    ///
    ///The outpoint (txid:index) of the funding transaction. With this value, Bob
    ///will be able to generate a signature for Alice's version of the commitment
    ///transaction.
    #[prost(message, optional, tag = "1")]
    pub channel_point: ::core::option::Option<ChannelPoint>,
    /// If true, then the channel will be closed forcibly. This means the
    /// current commitment transaction will be signed and broadcast.
    #[prost(bool, tag = "2")]
    pub force: bool,
    /// The target number of blocks that the closure transaction should be
    /// confirmed by.
    #[prost(int32, tag = "3")]
    pub target_conf: i32,
    /// Deprecated, use sat_per_vbyte.
    /// A manual fee rate set in sat/vbyte that should be used when crafting the
    /// closure transaction.
    #[deprecated]
    #[prost(int64, tag = "4")]
    pub sat_per_byte: i64,
    ///
    ///An optional address to send funds to in the case of a cooperative close.
    ///If the channel was opened with an upfront shutdown script and this field
    ///is set, the request to close will fail because the channel must pay out
    ///to the upfront shutdown addresss.
    #[prost(string, tag = "5")]
    pub delivery_address: ::prost::alloc::string::String,
    /// A manual fee rate set in sat/vbyte that should be used when crafting the
    /// closure transaction.
    #[prost(uint64, tag = "6")]
    pub sat_per_vbyte: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseStatusUpdate {
    #[prost(oneof = "close_status_update::Update", tags = "1, 3")]
    pub update: ::core::option::Option<close_status_update::Update>,
}
/// Nested message and enum types in `CloseStatusUpdate`.
pub mod close_status_update {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        #[prost(message, tag = "1")]
        ClosePending(super::PendingUpdate),
        #[prost(message, tag = "3")]
        ChanClose(super::ChannelCloseUpdate),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingUpdate {
    #[prost(bytes = "vec", tag = "1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub output_index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyForPsbtFunding {
    ///
    ///The P2WSH address of the channel funding multisig address that the below
    ///specified amount in satoshis needs to be sent to.
    #[prost(string, tag = "1")]
    pub funding_address: ::prost::alloc::string::String,
    ///
    ///The exact amount in satoshis that needs to be sent to the above address to
    ///fund the pending channel.
    #[prost(int64, tag = "2")]
    pub funding_amount: i64,
    ///
    ///A raw PSBT that contains the pending channel output. If a base PSBT was
    ///provided in the PsbtShim, this is the base PSBT with one additional output.
    ///If no base PSBT was specified, this is an otherwise empty PSBT with exactly
    ///one output.
    #[prost(bytes = "vec", tag = "3")]
    pub psbt: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOpenChannelRequest {
    /// The list of channels to open.
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<BatchOpenChannel>,
    /// The target number of blocks that the funding transaction should be
    /// confirmed by.
    #[prost(int32, tag = "2")]
    pub target_conf: i32,
    /// A manual fee rate set in sat/vByte that should be used when crafting the
    /// funding transaction.
    #[prost(int64, tag = "3")]
    pub sat_per_vbyte: i64,
    /// The minimum number of confirmations each one of your outputs used for
    /// the funding transaction must satisfy.
    #[prost(int32, tag = "4")]
    pub min_confs: i32,
    /// Whether unconfirmed outputs should be used as inputs for the funding
    /// transaction.
    #[prost(bool, tag = "5")]
    pub spend_unconfirmed: bool,
    /// An optional label for the batch transaction, limited to 500 characters.
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOpenChannel {
    /// The pubkey of the node to open a channel with. When using REST, this
    /// field must be encoded as base64.
    #[prost(bytes = "vec", tag = "1")]
    pub node_pubkey: ::prost::alloc::vec::Vec<u8>,
    /// The number of satoshis the wallet should commit to the channel.
    #[prost(int64, tag = "2")]
    pub local_funding_amount: i64,
    /// The number of satoshis to push to the remote side as part of the initial
    /// commitment state.
    #[prost(int64, tag = "3")]
    pub push_sat: i64,
    /// Whether this channel should be private, not announced to the greater
    /// network.
    #[prost(bool, tag = "4")]
    pub private: bool,
    /// The minimum value in millisatoshi we will require for incoming HTLCs on
    /// the channel.
    #[prost(int64, tag = "5")]
    pub min_htlc_msat: i64,
    /// The delay we require on the remote's commitment transaction. If this is
    /// not set, it will be scaled automatically with the channel size.
    #[prost(uint32, tag = "6")]
    pub remote_csv_delay: u32,
    ///
    ///Close address is an optional address which specifies the address to which
    ///funds should be paid out to upon cooperative close. This field may only be
    ///set if the peer supports the option upfront feature bit (call listpeers
    ///to check). The remote peer will only accept cooperative closes to this
    ///address if it is set.
    ///
    ///Note: If this value is set on channel creation, you will *not* be able to
    ///cooperatively close out to a different address.
    #[prost(string, tag = "7")]
    pub close_address: ::prost::alloc::string::String,
    ///
    ///An optional, unique identifier of 32 random bytes that will be used as the
    ///pending channel ID to identify the channel while it is in the pre-pending
    ///state.
    #[prost(bytes = "vec", tag = "8")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The explicit commitment type to use. Note this field will only be used if
    ///the remote peer supports explicit channel negotiation.
    #[prost(enumeration = "CommitmentType", tag = "9")]
    pub commitment_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOpenChannelResponse {
    #[prost(message, repeated, tag = "1")]
    pub pending_channels: ::prost::alloc::vec::Vec<PendingUpdate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenChannelRequest {
    /// A manual fee rate set in sat/vbyte that should be used when crafting the
    /// funding transaction.
    #[prost(uint64, tag = "1")]
    pub sat_per_vbyte: u64,
    ///
    ///The pubkey of the node to open a channel with. When using REST, this field
    ///must be encoded as base64.
    #[prost(bytes = "vec", tag = "2")]
    pub node_pubkey: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The hex encoded pubkey of the node to open a channel with. Deprecated now
    ///that the REST gateway supports base64 encoding of bytes fields.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub node_pubkey_string: ::prost::alloc::string::String,
    /// The number of satoshis the wallet should commit to the channel
    #[prost(int64, tag = "4")]
    pub local_funding_amount: i64,
    /// The number of satoshis to push to the remote side as part of the initial
    /// commitment state
    #[prost(int64, tag = "5")]
    pub push_sat: i64,
    /// The target number of blocks that the funding transaction should be
    /// confirmed by.
    #[prost(int32, tag = "6")]
    pub target_conf: i32,
    /// Deprecated, use sat_per_vbyte.
    /// A manual fee rate set in sat/vbyte that should be used when crafting the
    /// funding transaction.
    #[deprecated]
    #[prost(int64, tag = "7")]
    pub sat_per_byte: i64,
    /// Whether this channel should be private, not announced to the greater
    /// network.
    #[prost(bool, tag = "8")]
    pub private: bool,
    /// The minimum value in millisatoshi we will require for incoming HTLCs on
    /// the channel.
    #[prost(int64, tag = "9")]
    pub min_htlc_msat: i64,
    /// The delay we require on the remote's commitment transaction. If this is
    /// not set, it will be scaled automatically with the channel size.
    #[prost(uint32, tag = "10")]
    pub remote_csv_delay: u32,
    /// The minimum number of confirmations each one of your outputs used for
    /// the funding transaction must satisfy.
    #[prost(int32, tag = "11")]
    pub min_confs: i32,
    /// Whether unconfirmed outputs should be used as inputs for the funding
    /// transaction.
    #[prost(bool, tag = "12")]
    pub spend_unconfirmed: bool,
    ///
    ///Close address is an optional address which specifies the address to which
    ///funds should be paid out to upon cooperative close. This field may only be
    ///set if the peer supports the option upfront feature bit (call listpeers
    ///to check). The remote peer will only accept cooperative closes to this
    ///address if it is set.
    ///
    ///Note: If this value is set on channel creation, you will *not* be able to
    ///cooperatively close out to a different address.
    #[prost(string, tag = "13")]
    pub close_address: ::prost::alloc::string::String,
    ///
    ///Funding shims are an optional argument that allow the caller to intercept
    ///certain funding functionality. For example, a shim can be provided to use a
    ///particular key for the commitment key (ideally cold) rather than use one
    ///that is generated by the wallet as normal, or signal that signing will be
    ///carried out in an interactive manner (PSBT based).
    #[prost(message, optional, tag = "14")]
    pub funding_shim: ::core::option::Option<FundingShim>,
    ///
    ///The maximum amount of coins in millisatoshi that can be pending within
    ///the channel. It only applies to the remote party.
    #[prost(uint64, tag = "15")]
    pub remote_max_value_in_flight_msat: u64,
    ///
    ///The maximum number of concurrent HTLCs we will allow the remote party to add
    ///to the commitment transaction.
    #[prost(uint32, tag = "16")]
    pub remote_max_htlcs: u32,
    ///
    ///Max local csv is the maximum csv delay we will allow for our own commitment
    ///transaction.
    #[prost(uint32, tag = "17")]
    pub max_local_csv: u32,
    ///
    ///The explicit commitment type to use. Note this field will only be used if
    ///the remote peer supports explicit channel negotiation.
    #[prost(enumeration = "CommitmentType", tag = "18")]
    pub commitment_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenStatusUpdate {
    ///
    ///The pending channel ID of the created channel. This value may be used to
    ///further the funding flow manually via the FundingStateStep method.
    #[prost(bytes = "vec", tag = "4")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "open_status_update::Update", tags = "1, 3, 5")]
    pub update: ::core::option::Option<open_status_update::Update>,
}
/// Nested message and enum types in `OpenStatusUpdate`.
pub mod open_status_update {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        ///
        ///Signals that the channel is now fully negotiated and the funding
        ///transaction published.
        #[prost(message, tag = "1")]
        ChanPending(super::PendingUpdate),
        ///
        ///Signals that the channel's funding transaction has now reached the
        ///required number of confirmations on chain and can be used.
        #[prost(message, tag = "3")]
        ChanOpen(super::ChannelOpenUpdate),
        ///
        ///Signals that the funding process has been suspended and the construction
        ///of a PSBT that funds the channel PK script is now required.
        #[prost(message, tag = "5")]
        PsbtFund(super::ReadyForPsbtFunding),
    }
}
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
    ///The raw bytes of the key being identified.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_key_bytes: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The key locator that identifies which key to use for signing.
    #[prost(message, optional, tag = "2")]
    pub key_loc: ::core::option::Option<KeyLocator>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChanPointShim {
    ///
    ///The size of the pre-crafted output to be used as the channel point for this
    ///channel funding.
    #[prost(int64, tag = "1")]
    pub amt: i64,
    /// The target channel point to refrence in created commitment transactions.
    #[prost(message, optional, tag = "2")]
    pub chan_point: ::core::option::Option<ChannelPoint>,
    /// Our local key to use when creating the multi-sig output.
    #[prost(message, optional, tag = "3")]
    pub local_key: ::core::option::Option<KeyDescriptor>,
    /// The key of the remote party to use when creating the multi-sig output.
    #[prost(bytes = "vec", tag = "4")]
    pub remote_key: ::prost::alloc::vec::Vec<u8>,
    ///
    ///If non-zero, then this will be used as the pending channel ID on the wire
    ///protocol to initate the funding request. This is an optional field, and
    ///should only be set if the responder is already expecting a specific pending
    ///channel ID.
    #[prost(bytes = "vec", tag = "5")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///This uint32 indicates if this channel is to be considered 'frozen'. A frozen
    ///channel does not allow a cooperative channel close by the initiator. The
    ///thaw_height is the height that this restriction stops applying to the
    ///channel. The height can be interpreted in two ways: as a relative height if
    ///the value is less than 500,000, or as an absolute height otherwise.
    #[prost(uint32, tag = "6")]
    pub thaw_height: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PsbtShim {
    ///
    ///A unique identifier of 32 random bytes that will be used as the pending
    ///channel ID to identify the PSBT state machine when interacting with it and
    ///on the wire protocol to initiate the funding request.
    #[prost(bytes = "vec", tag = "1")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///An optional base PSBT the new channel output will be added to. If this is
    ///non-empty, it must be a binary serialized PSBT.
    #[prost(bytes = "vec", tag = "2")]
    pub base_psbt: ::prost::alloc::vec::Vec<u8>,
    ///
    ///If a channel should be part of a batch (multiple channel openings in one
    ///transaction), it can be dangerous if the whole batch transaction is
    ///published too early before all channel opening negotiations are completed.
    ///This flag prevents this particular channel from broadcasting the transaction
    ///after the negotiation with the remote peer. In a batch of channel openings
    ///this flag should be set to true for every channel but the very last.
    #[prost(bool, tag = "3")]
    pub no_publish: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingShim {
    #[prost(oneof = "funding_shim::Shim", tags = "1, 2")]
    pub shim: ::core::option::Option<funding_shim::Shim>,
}
/// Nested message and enum types in `FundingShim`.
pub mod funding_shim {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Shim {
        ///
        ///A channel shim where the channel point was fully constructed outside
        ///of lnd's wallet and the transaction might already be published.
        #[prost(message, tag = "1")]
        ChanPointShim(super::ChanPointShim),
        ///
        ///A channel shim that uses a PSBT to fund and sign the channel funding
        ///transaction.
        #[prost(message, tag = "2")]
        PsbtShim(super::PsbtShim),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingShimCancel {
    /// The pending channel ID of the channel to cancel the funding shim for.
    #[prost(bytes = "vec", tag = "1")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingPsbtVerify {
    ///
    ///The funded but not yet signed PSBT that sends the exact channel capacity
    ///amount to the PK script returned in the open channel message in a previous
    ///step.
    #[prost(bytes = "vec", tag = "1")]
    pub funded_psbt: ::prost::alloc::vec::Vec<u8>,
    /// The pending channel ID of the channel to get the PSBT for.
    #[prost(bytes = "vec", tag = "2")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Can only be used if the no_publish flag was set to true in the OpenChannel
    ///call meaning that the caller is solely responsible for publishing the final
    ///funding transaction. If skip_finalize is set to true then lnd will not wait
    ///for a FundingPsbtFinalize state step and instead assumes that a transaction
    ///with the same TXID as the passed in PSBT will eventually confirm.
    ///IT IS ABSOLUTELY IMPERATIVE that the TXID of the transaction that is
    ///eventually published does have the _same TXID_ as the verified PSBT. That
    ///means no inputs or outputs can change, only signatures can be added. If the
    ///TXID changes between this call and the publish step then the channel will
    ///never be created and the funds will be in limbo.
    #[prost(bool, tag = "3")]
    pub skip_finalize: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingPsbtFinalize {
    ///
    ///The funded PSBT that contains all witness data to send the exact channel
    ///capacity amount to the PK script returned in the open channel message in a
    ///previous step. Cannot be set at the same time as final_raw_tx.
    #[prost(bytes = "vec", tag = "1")]
    pub signed_psbt: ::prost::alloc::vec::Vec<u8>,
    /// The pending channel ID of the channel to get the PSBT for.
    #[prost(bytes = "vec", tag = "2")]
    pub pending_chan_id: ::prost::alloc::vec::Vec<u8>,
    ///
    ///As an alternative to the signed PSBT with all witness data, the final raw
    ///wire format transaction can also be specified directly. Cannot be set at the
    ///same time as signed_psbt.
    #[prost(bytes = "vec", tag = "3")]
    pub final_raw_tx: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingTransitionMsg {
    #[prost(oneof = "funding_transition_msg::Trigger", tags = "1, 2, 3, 4")]
    pub trigger: ::core::option::Option<funding_transition_msg::Trigger>,
}
/// Nested message and enum types in `FundingTransitionMsg`.
pub mod funding_transition_msg {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        ///
        ///The funding shim to register. This should be used before any
        ///channel funding has began by the remote party, as it is intended as a
        ///preparatory step for the full channel funding.
        #[prost(message, tag = "1")]
        ShimRegister(super::FundingShim),
        /// Used to cancel an existing registered funding shim.
        #[prost(message, tag = "2")]
        ShimCancel(super::FundingShimCancel),
        ///
        ///Used to continue a funding flow that was initiated to be executed
        ///through a PSBT. This step verifies that the PSBT contains the correct
        ///outputs to fund the channel.
        #[prost(message, tag = "3")]
        PsbtVerify(super::FundingPsbtVerify),
        ///
        ///Used to continue a funding flow that was initiated to be executed
        ///through a PSBT. This step finalizes the funded and signed PSBT, finishes
        ///negotiation with the peer and finally publishes the resulting funding
        ///transaction.
        #[prost(message, tag = "4")]
        PsbtFinalize(super::FundingPsbtFinalize),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingStateStepResp {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingHtlc {
    /// The direction within the channel that the htlc was sent
    #[prost(bool, tag = "1")]
    pub incoming: bool,
    /// The total value of the htlc
    #[prost(int64, tag = "2")]
    pub amount: i64,
    /// The final output to be swept back to the user's wallet
    #[prost(string, tag = "3")]
    pub outpoint: ::prost::alloc::string::String,
    /// The next block height at which we can spend the current stage
    #[prost(uint32, tag = "4")]
    pub maturity_height: u32,
    ///
    ///The number of blocks remaining until the current stage can be swept.
    ///Negative values indicate how many blocks have passed since becoming
    ///mature.
    #[prost(int32, tag = "5")]
    pub blocks_til_maturity: i32,
    /// Indicates whether the htlc is in its first or second stage of recovery
    #[prost(uint32, tag = "6")]
    pub stage: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingChannelsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingChannelsResponse {
    /// The balance in satoshis encumbered in pending channels
    #[prost(int64, tag = "1")]
    pub total_limbo_balance: i64,
    /// Channels pending opening
    #[prost(message, repeated, tag = "2")]
    pub pending_open_channels:
        ::prost::alloc::vec::Vec<pending_channels_response::PendingOpenChannel>,
    ///
    ///Deprecated: Channels pending closing previously contained cooperatively
    ///closed channels with a single confirmation. These channels are now
    ///considered closed from the time we see them on chain.
    #[deprecated]
    #[prost(message, repeated, tag = "3")]
    pub pending_closing_channels:
        ::prost::alloc::vec::Vec<pending_channels_response::ClosedChannel>,
    /// Channels pending force closing
    #[prost(message, repeated, tag = "4")]
    pub pending_force_closing_channels:
        ::prost::alloc::vec::Vec<pending_channels_response::ForceClosedChannel>,
    /// Channels waiting for closing tx to confirm
    #[prost(message, repeated, tag = "5")]
    pub waiting_close_channels:
        ::prost::alloc::vec::Vec<pending_channels_response::WaitingCloseChannel>,
}
/// Nested message and enum types in `PendingChannelsResponse`.
pub mod pending_channels_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PendingChannel {
        #[prost(string, tag = "1")]
        pub remote_node_pub: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub channel_point: ::prost::alloc::string::String,
        #[prost(int64, tag = "3")]
        pub capacity: i64,
        #[prost(int64, tag = "4")]
        pub local_balance: i64,
        #[prost(int64, tag = "5")]
        pub remote_balance: i64,
        /// The minimum satoshis this node is required to reserve in its
        /// balance.
        #[prost(int64, tag = "6")]
        pub local_chan_reserve_sat: i64,
        ///
        ///The minimum satoshis the other node is required to reserve in its
        ///balance.
        #[prost(int64, tag = "7")]
        pub remote_chan_reserve_sat: i64,
        /// The party that initiated opening the channel.
        #[prost(enumeration = "super::Initiator", tag = "8")]
        pub initiator: i32,
        /// The commitment type used by this channel.
        #[prost(enumeration = "super::CommitmentType", tag = "9")]
        pub commitment_type: i32,
        /// Total number of forwarding packages created in this channel.
        #[prost(int64, tag = "10")]
        pub num_forwarding_packages: i64,
        /// A set of flags showing the current state of the channel.
        #[prost(string, tag = "11")]
        pub chan_status_flags: ::prost::alloc::string::String,
        /// Whether this channel is advertised to the network or not.
        #[prost(bool, tag = "12")]
        pub private: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PendingOpenChannel {
        /// The pending channel
        #[prost(message, optional, tag = "1")]
        pub channel: ::core::option::Option<PendingChannel>,
        ///
        ///The amount calculated to be paid in fees for the current set of
        ///commitment transactions. The fee amount is persisted with the channel
        ///in order to allow the fee amount to be removed and recalculated with
        ///each channel state update, including updates that happen after a system
        ///restart.
        #[prost(int64, tag = "4")]
        pub commit_fee: i64,
        /// The weight of the commitment transaction
        #[prost(int64, tag = "5")]
        pub commit_weight: i64,
        ///
        ///The required number of satoshis per kilo-weight that the requester will
        ///pay at all times, for both the funding transaction and commitment
        ///transaction. This value can later be updated once the channel is open.
        #[prost(int64, tag = "6")]
        pub fee_per_kw: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WaitingCloseChannel {
        /// The pending channel waiting for closing tx to confirm
        #[prost(message, optional, tag = "1")]
        pub channel: ::core::option::Option<PendingChannel>,
        /// The balance in satoshis encumbered in this channel
        #[prost(int64, tag = "2")]
        pub limbo_balance: i64,
        ///
        ///A list of valid commitment transactions. Any of these can confirm at
        ///this point.
        #[prost(message, optional, tag = "3")]
        pub commitments: ::core::option::Option<Commitments>,
        /// The transaction id of the closing transaction
        #[prost(string, tag = "4")]
        pub closing_txid: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Commitments {
        /// Hash of the local version of the commitment tx.
        #[prost(string, tag = "1")]
        pub local_txid: ::prost::alloc::string::String,
        /// Hash of the remote version of the commitment tx.
        #[prost(string, tag = "2")]
        pub remote_txid: ::prost::alloc::string::String,
        /// Hash of the remote pending version of the commitment tx.
        #[prost(string, tag = "3")]
        pub remote_pending_txid: ::prost::alloc::string::String,
        ///
        ///The amount in satoshis calculated to be paid in fees for the local
        ///commitment.
        #[prost(uint64, tag = "4")]
        pub local_commit_fee_sat: u64,
        ///
        ///The amount in satoshis calculated to be paid in fees for the remote
        ///commitment.
        #[prost(uint64, tag = "5")]
        pub remote_commit_fee_sat: u64,
        ///
        ///The amount in satoshis calculated to be paid in fees for the remote
        ///pending commitment.
        #[prost(uint64, tag = "6")]
        pub remote_pending_commit_fee_sat: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClosedChannel {
        /// The pending channel to be closed
        #[prost(message, optional, tag = "1")]
        pub channel: ::core::option::Option<PendingChannel>,
        /// The transaction id of the closing transaction
        #[prost(string, tag = "2")]
        pub closing_txid: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ForceClosedChannel {
        /// The pending channel to be force closed
        #[prost(message, optional, tag = "1")]
        pub channel: ::core::option::Option<PendingChannel>,
        /// The transaction id of the closing transaction
        #[prost(string, tag = "2")]
        pub closing_txid: ::prost::alloc::string::String,
        /// The balance in satoshis encumbered in this pending channel
        #[prost(int64, tag = "3")]
        pub limbo_balance: i64,
        /// The height at which funds can be swept into the wallet
        #[prost(uint32, tag = "4")]
        pub maturity_height: u32,
        ///
        ///Remaining # of blocks until the commitment output can be swept.
        ///Negative values indicate how many blocks have passed since becoming
        ///mature.
        #[prost(int32, tag = "5")]
        pub blocks_til_maturity: i32,
        /// The total value of funds successfully recovered from this channel
        #[prost(int64, tag = "6")]
        pub recovered_balance: i64,
        #[prost(message, repeated, tag = "8")]
        pub pending_htlcs: ::prost::alloc::vec::Vec<super::PendingHtlc>,
        #[prost(enumeration = "force_closed_channel::AnchorState", tag = "9")]
        pub anchor: i32,
    }
    /// Nested message and enum types in `ForceClosedChannel`.
    pub mod force_closed_channel {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum AnchorState {
            Limbo = 0,
            Recovered = 1,
            Lost = 2,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEventSubscription {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEventUpdate {
    #[prost(enumeration = "channel_event_update::UpdateType", tag = "5")]
    pub r#type: i32,
    #[prost(oneof = "channel_event_update::Channel", tags = "1, 2, 3, 4, 6, 7")]
    pub channel: ::core::option::Option<channel_event_update::Channel>,
}
/// Nested message and enum types in `ChannelEventUpdate`.
pub mod channel_event_update {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UpdateType {
        OpenChannel = 0,
        ClosedChannel = 1,
        ActiveChannel = 2,
        InactiveChannel = 3,
        PendingOpenChannel = 4,
        FullyResolvedChannel = 5,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Channel {
        #[prost(message, tag = "1")]
        OpenChannel(super::Channel),
        #[prost(message, tag = "2")]
        ClosedChannel(super::ChannelCloseSummary),
        #[prost(message, tag = "3")]
        ActiveChannel(super::ChannelPoint),
        #[prost(message, tag = "4")]
        InactiveChannel(super::ChannelPoint),
        #[prost(message, tag = "6")]
        PendingOpenChannel(super::PendingUpdate),
        #[prost(message, tag = "7")]
        FullyResolvedChannel(super::ChannelPoint),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletAccountBalance {
    /// The confirmed balance of the account (with >= 1 confirmations).
    #[prost(int64, tag = "1")]
    pub confirmed_balance: i64,
    /// The unconfirmed balance of the account (with 0 confirmations).
    #[prost(int64, tag = "2")]
    pub unconfirmed_balance: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletBalanceRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletBalanceResponse {
    /// The balance of the wallet
    #[prost(int64, tag = "1")]
    pub total_balance: i64,
    /// The confirmed balance of a wallet(with >= 1 confirmations)
    #[prost(int64, tag = "2")]
    pub confirmed_balance: i64,
    /// The unconfirmed balance of a wallet(with 0 confirmations)
    #[prost(int64, tag = "3")]
    pub unconfirmed_balance: i64,
    /// The total amount of wallet UTXOs held in outputs that are locked for
    /// other usage.
    #[prost(int64, tag = "5")]
    pub locked_balance: i64,
    /// A mapping of each wallet account's name to its balance.
    #[prost(map = "string, message", tag = "4")]
    pub account_balance:
        ::std::collections::HashMap<::prost::alloc::string::String, WalletAccountBalance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amount {
    /// Value denominated in satoshis.
    #[prost(uint64, tag = "1")]
    pub sat: u64,
    /// Value denominated in milli-satoshis.
    #[prost(uint64, tag = "2")]
    pub msat: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBalanceRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBalanceResponse {
    /// Deprecated. Sum of channels balances denominated in satoshis
    #[deprecated]
    #[prost(int64, tag = "1")]
    pub balance: i64,
    /// Deprecated. Sum of channels pending balances denominated in satoshis
    #[deprecated]
    #[prost(int64, tag = "2")]
    pub pending_open_balance: i64,
    /// Sum of channels local balances.
    #[prost(message, optional, tag = "3")]
    pub local_balance: ::core::option::Option<Amount>,
    /// Sum of channels remote balances.
    #[prost(message, optional, tag = "4")]
    pub remote_balance: ::core::option::Option<Amount>,
    /// Sum of channels local unsettled balances.
    #[prost(message, optional, tag = "5")]
    pub unsettled_local_balance: ::core::option::Option<Amount>,
    /// Sum of channels remote unsettled balances.
    #[prost(message, optional, tag = "6")]
    pub unsettled_remote_balance: ::core::option::Option<Amount>,
    /// Sum of channels pending local balances.
    #[prost(message, optional, tag = "7")]
    pub pending_open_local_balance: ::core::option::Option<Amount>,
    /// Sum of channels pending remote balances.
    #[prost(message, optional, tag = "8")]
    pub pending_open_remote_balance: ::core::option::Option<Amount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRoutesRequest {
    /// The 33-byte hex-encoded public key for the payment destination
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
    ///
    ///The amount to send expressed in satoshis.
    ///
    ///The fields amt and amt_msat are mutually exclusive.
    #[prost(int64, tag = "2")]
    pub amt: i64,
    ///
    ///The amount to send expressed in millisatoshis.
    ///
    ///The fields amt and amt_msat are mutually exclusive.
    #[prost(int64, tag = "12")]
    pub amt_msat: i64,
    ///
    ///An optional CLTV delta from the current height that should be used for the
    ///timelock of the final hop. Note that unlike SendPayment, QueryRoutes does
    ///not add any additional block padding on top of final_ctlv_delta. This
    ///padding of a few blocks needs to be added manually or otherwise failures may
    ///happen when a block comes in while the payment is in flight.
    #[prost(int32, tag = "4")]
    pub final_cltv_delta: i32,
    ///
    ///The maximum number of satoshis that will be paid as a fee of the payment.
    ///This value can be represented either as a percentage of the amount being
    ///sent, or as a fixed amount of the maximum fee the user is willing the pay to
    ///send the payment. If not specified, lnd will use a default value of 100%
    ///fees for small amounts (<=1k sat) or 5% fees for larger amounts.
    #[prost(message, optional, tag = "5")]
    pub fee_limit: ::core::option::Option<FeeLimit>,
    ///
    ///A list of nodes to ignore during path finding. When using REST, these fields
    ///must be encoded as base64.
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub ignored_nodes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    ///
    ///Deprecated. A list of edges to ignore during path finding.
    #[deprecated]
    #[prost(message, repeated, tag = "7")]
    pub ignored_edges: ::prost::alloc::vec::Vec<EdgeLocator>,
    ///
    ///The source node where the request route should originated from. If empty,
    ///self is assumed.
    #[prost(string, tag = "8")]
    pub source_pub_key: ::prost::alloc::string::String,
    ///
    ///If set to true, edge probabilities from mission control will be used to get
    ///the optimal route.
    #[prost(bool, tag = "9")]
    pub use_mission_control: bool,
    ///
    ///A list of directed node pairs that will be ignored during path finding.
    #[prost(message, repeated, tag = "10")]
    pub ignored_pairs: ::prost::alloc::vec::Vec<NodePair>,
    ///
    ///An optional maximum total time lock for the route. If the source is empty or
    ///ourselves, this should not exceed lnd's `--max-cltv-expiry` setting. If
    ///zero, then the value of `--max-cltv-expiry` is used as the limit.
    #[prost(uint32, tag = "11")]
    pub cltv_limit: u32,
    ///
    ///An optional field that can be used to pass an arbitrary set of TLV records
    ///to a peer which understands the new records. This can be used to pass
    ///application specific data during the payment attempt. If the destination
    ///does not support the specified records, an error will be returned.
    ///Record types are required to be in the custom range >= 65536. When using
    ///REST, the values must be encoded as base64.
    #[prost(map = "uint64, bytes", tag = "13")]
    pub dest_custom_records: ::std::collections::HashMap<u64, ::prost::alloc::vec::Vec<u8>>,
    ///
    ///The channel id of the channel that must be taken to the first hop. If zero,
    ///any channel may be used.
    #[prost(uint64, tag = "14")]
    pub outgoing_chan_id: u64,
    ///
    ///The pubkey of the last hop of the route. If empty, any hop may be used.
    #[prost(bytes = "vec", tag = "15")]
    pub last_hop_pubkey: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Optional route hints to reach the destination through private channels.
    #[prost(message, repeated, tag = "16")]
    pub route_hints: ::prost::alloc::vec::Vec<RouteHint>,
    ///
    ///Features assumed to be supported by the final node. All transitive feature
    ///dependencies must also be set properly. For a given feature bit pair, either
    ///optional or remote may be set, but not both. If this field is nil or empty,
    ///the router will try to load destination features from the graph as a
    ///fallback.
    #[prost(enumeration = "FeatureBit", repeated, tag = "17")]
    pub dest_features: ::prost::alloc::vec::Vec<i32>,
    ///
    ///The time preference for this payment. Set to -1 to optimize for fees
    ///only, to 1 to optimize for reliability only or a value inbetween for a mix.
    #[prost(double, tag = "18")]
    pub time_pref: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePair {
    ///
    ///The sending node of the pair. When using REST, this field must be encoded as
    ///base64.
    #[prost(bytes = "vec", tag = "1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The receiving node of the pair. When using REST, this field must be encoded
    ///as base64.
    #[prost(bytes = "vec", tag = "2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EdgeLocator {
    /// The short channel id of this edge.
    #[prost(uint64, tag = "1")]
    pub channel_id: u64,
    ///
    ///The direction of this edge. If direction_reverse is false, the direction
    ///of this edge is from the channel endpoint with the lexicographically smaller
    ///pub key to the endpoint with the larger pub key. If direction_reverse is
    ///is true, the edge goes the other way.
    #[prost(bool, tag = "2")]
    pub direction_reverse: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRoutesResponse {
    ///
    ///The route that results from the path finding operation. This is still a
    ///repeated field to retain backwards compatibility.
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    ///
    ///The success probability of the returned route based on the current mission
    ///control state. \[EXPERIMENTAL\]
    #[prost(double, tag = "2")]
    pub success_prob: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hop {
    ///
    ///The unique channel ID for the channel. The first 3 bytes are the block
    ///height, the next 3 the index within the block, and the last 2 bytes are the
    ///output index for the channel.
    #[prost(uint64, tag = "1")]
    pub chan_id: u64,
    #[deprecated]
    #[prost(int64, tag = "2")]
    pub chan_capacity: i64,
    #[deprecated]
    #[prost(int64, tag = "3")]
    pub amt_to_forward: i64,
    #[deprecated]
    #[prost(int64, tag = "4")]
    pub fee: i64,
    #[prost(uint32, tag = "5")]
    pub expiry: u32,
    #[prost(int64, tag = "6")]
    pub amt_to_forward_msat: i64,
    #[prost(int64, tag = "7")]
    pub fee_msat: i64,
    ///
    ///An optional public key of the hop. If the public key is given, the payment
    ///can be executed without relying on a copy of the channel graph.
    #[prost(string, tag = "8")]
    pub pub_key: ::prost::alloc::string::String,
    ///
    ///If set to true, then this hop will be encoded using the new variable length
    ///TLV format. Note that if any custom tlv_records below are specified, then
    ///this field MUST be set to true for them to be encoded properly.
    #[deprecated]
    #[prost(bool, tag = "9")]
    pub tlv_payload: bool,
    ///
    ///An optional TLV record that signals the use of an MPP payment. If present,
    ///the receiver will enforce that the same mpp_record is included in the final
    ///hop payload of all non-zero payments in the HTLC set. If empty, a regular
    ///single-shot payment is or was attempted.
    #[prost(message, optional, tag = "10")]
    pub mpp_record: ::core::option::Option<MppRecord>,
    ///
    ///An optional TLV record that signals the use of an AMP payment. If present,
    ///the receiver will treat all received payments including the same
    ///(payment_addr, set_id) pair  as being part of one logical payment. The
    ///payment will be settled by XORing the root_share's together and deriving the
    ///child hashes and preimages according to BOLT XX. Must be used in conjunction
    ///with mpp_record.
    #[prost(message, optional, tag = "12")]
    pub amp_record: ::core::option::Option<AmpRecord>,
    ///
    ///An optional set of key-value TLV records. This is useful within the context
    ///of the SendToRoute call as it allows callers to specify arbitrary K-V pairs
    ///to drop off at each hop within the onion.
    #[prost(map = "uint64, bytes", tag = "11")]
    pub custom_records: ::std::collections::HashMap<u64, ::prost::alloc::vec::Vec<u8>>,
    /// The payment metadata to send along with the payment to the payee.
    #[prost(bytes = "vec", tag = "13")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MppRecord {
    ///
    ///A unique, random identifier used to authenticate the sender as the intended
    ///payer of a multi-path payment. The payment_addr must be the same for all
    ///subpayments, and match the payment_addr provided in the receiver's invoice.
    ///The same payment_addr must be used on all subpayments.
    #[prost(bytes = "vec", tag = "11")]
    pub payment_addr: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The total amount in milli-satoshis being sent as part of a larger multi-path
    ///payment. The caller is responsible for ensuring subpayments to the same node
    ///and payment_hash sum exactly to total_amt_msat. The same
    ///total_amt_msat must be used on all subpayments.
    #[prost(int64, tag = "10")]
    pub total_amt_msat: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmpRecord {
    #[prost(bytes = "vec", tag = "1")]
    pub root_share: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub set_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub child_index: u32,
}
///
///A path through the channel graph which runs over one or more channels in
///succession. This struct carries all the information required to craft the
///Sphinx onion packet, and send the payment along the first hop in the path. A
///route is only selected as valid if all the channels have sufficient capacity to
///carry the initial payment amount after fees are accounted for.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    ///
    ///The cumulative (final) time lock across the entire route. This is the CLTV
    ///value that should be extended to the first hop in the route. All other hops
    ///will decrement the time-lock as advertised, leaving enough time for all
    ///hops to wait for or present the payment preimage to complete the payment.
    #[prost(uint32, tag = "1")]
    pub total_time_lock: u32,
    ///
    ///The sum of the fees paid at each hop within the final route. In the case
    ///of a one-hop payment, this value will be zero as we don't need to pay a fee
    ///to ourselves.
    #[deprecated]
    #[prost(int64, tag = "2")]
    pub total_fees: i64,
    ///
    ///The total amount of funds required to complete a payment over this route.
    ///This value includes the cumulative fees at each hop. As a result, the HTLC
    ///extended to the first-hop in the route will need to have at least this many
    ///satoshis, otherwise the route will fail at an intermediate node due to an
    ///insufficient amount of fees.
    #[deprecated]
    #[prost(int64, tag = "3")]
    pub total_amt: i64,
    ///
    ///Contains details concerning the specific forwarding details at each hop.
    #[prost(message, repeated, tag = "4")]
    pub hops: ::prost::alloc::vec::Vec<Hop>,
    ///
    ///The total fees in millisatoshis.
    #[prost(int64, tag = "5")]
    pub total_fees_msat: i64,
    ///
    ///The total amount in millisatoshis.
    #[prost(int64, tag = "6")]
    pub total_amt_msat: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfoRequest {
    /// The 33-byte hex-encoded compressed public of the target node
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
    /// If true, will include all known channels associated with the node.
    #[prost(bool, tag = "2")]
    pub include_channels: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    ///
    ///An individual vertex/node within the channel graph. A node is
    ///connected to other nodes by one or more channel edges emanating from it. As
    ///the graph is directed, a node will also have an incoming edge attached to
    ///it for each outgoing edge.
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<LightningNode>,
    /// The total number of channels for the node.
    #[prost(uint32, tag = "2")]
    pub num_channels: u32,
    /// The sum of all channels capacity for the node, denominated in satoshis.
    #[prost(int64, tag = "3")]
    pub total_capacity: i64,
    /// A list of all public channels for the node.
    #[prost(message, repeated, tag = "4")]
    pub channels: ::prost::alloc::vec::Vec<ChannelEdge>,
}
///
///An individual vertex/node within the channel graph. A node is
///connected to other nodes by one or more channel edges emanating from it. As the
///graph is directed, a node will also have an incoming edge attached to it for
///each outgoing edge.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightningNode {
    #[prost(uint32, tag = "1")]
    pub last_update: u32,
    #[prost(string, tag = "2")]
    pub pub_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub alias: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub addresses: ::prost::alloc::vec::Vec<NodeAddress>,
    #[prost(string, tag = "5")]
    pub color: ::prost::alloc::string::String,
    #[prost(map = "uint32, message", tag = "6")]
    pub features: ::std::collections::HashMap<u32, Feature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeAddress {
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub addr: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingPolicy {
    #[prost(uint32, tag = "1")]
    pub time_lock_delta: u32,
    #[prost(int64, tag = "2")]
    pub min_htlc: i64,
    #[prost(int64, tag = "3")]
    pub fee_base_msat: i64,
    #[prost(int64, tag = "4")]
    pub fee_rate_milli_msat: i64,
    #[prost(bool, tag = "5")]
    pub disabled: bool,
    #[prost(uint64, tag = "6")]
    pub max_htlc_msat: u64,
    #[prost(uint32, tag = "7")]
    pub last_update: u32,
}
///
///A fully authenticated channel along with all its unique attributes.
///Once an authenticated channel announcement has been processed on the network,
///then an instance of ChannelEdgeInfo encapsulating the channels attributes is
///stored. The other portions relevant to routing policy of a channel are stored
///within a ChannelEdgePolicy for each direction of the channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEdge {
    ///
    ///The unique channel ID for the channel. The first 3 bytes are the block
    ///height, the next 3 the index within the block, and the last 2 bytes are the
    ///output index for the channel.
    #[prost(uint64, tag = "1")]
    pub channel_id: u64,
    #[prost(string, tag = "2")]
    pub chan_point: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(uint32, tag = "3")]
    pub last_update: u32,
    #[prost(string, tag = "4")]
    pub node1_pub: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub node2_pub: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub capacity: i64,
    #[prost(message, optional, tag = "7")]
    pub node1_policy: ::core::option::Option<RoutingPolicy>,
    #[prost(message, optional, tag = "8")]
    pub node2_policy: ::core::option::Option<RoutingPolicy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGraphRequest {
    ///
    ///Whether unannounced channels are included in the response or not. If set,
    ///unannounced channels are included. Unannounced channels are both private
    ///channels, and public channels that are not yet announced to the network.
    #[prost(bool, tag = "1")]
    pub include_unannounced: bool,
}
/// Returns a new instance of the directed channel graph.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGraph {
    /// The list of `LightningNode`s in this channel graph
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<LightningNode>,
    /// The list of `ChannelEdge`s in this channel graph
    #[prost(message, repeated, tag = "2")]
    pub edges: ::prost::alloc::vec::Vec<ChannelEdge>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeMetricsRequest {
    /// The requested node metrics.
    #[prost(enumeration = "NodeMetricType", repeated, tag = "1")]
    pub types: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeMetricsResponse {
    ///
    ///Betweenness centrality is the sum of the ratio of shortest paths that pass
    ///through the node for each pair of nodes in the graph (not counting paths
    ///starting or ending at this node).
    ///Map of node pubkey to betweenness centrality of the node. Normalized
    ///values are in the \[0,1\] closed interval.
    #[prost(map = "string, message", tag = "1")]
    pub betweenness_centrality:
        ::std::collections::HashMap<::prost::alloc::string::String, FloatMetric>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatMetric {
    /// Arbitrary float value.
    #[prost(double, tag = "1")]
    pub value: f64,
    /// The value normalized to \[0,1\] or \[-1,1\].
    #[prost(double, tag = "2")]
    pub normalized_value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChanInfoRequest {
    ///
    ///The unique channel ID for the channel. The first 3 bytes are the block
    ///height, the next 3 the index within the block, and the last 2 bytes are the
    ///output index for the channel.
    #[prost(uint64, tag = "1")]
    pub chan_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInfo {
    #[prost(uint32, tag = "1")]
    pub graph_diameter: u32,
    #[prost(double, tag = "2")]
    pub avg_out_degree: f64,
    #[prost(uint32, tag = "3")]
    pub max_out_degree: u32,
    #[prost(uint32, tag = "4")]
    pub num_nodes: u32,
    #[prost(uint32, tag = "5")]
    pub num_channels: u32,
    #[prost(int64, tag = "6")]
    pub total_network_capacity: i64,
    #[prost(double, tag = "7")]
    pub avg_channel_size: f64,
    #[prost(int64, tag = "8")]
    pub min_channel_size: i64,
    #[prost(int64, tag = "9")]
    pub max_channel_size: i64,
    #[prost(int64, tag = "10")]
    pub median_channel_size_sat: i64,
    /// The number of edges marked as zombies.
    #[prost(uint64, tag = "11")]
    pub num_zombie_chans: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphTopologySubscription {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphTopologyUpdate {
    #[prost(message, repeated, tag = "1")]
    pub node_updates: ::prost::alloc::vec::Vec<NodeUpdate>,
    #[prost(message, repeated, tag = "2")]
    pub channel_updates: ::prost::alloc::vec::Vec<ChannelEdgeUpdate>,
    #[prost(message, repeated, tag = "3")]
    pub closed_chans: ::prost::alloc::vec::Vec<ClosedChannelUpdate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeUpdate {
    ///
    ///Deprecated, use node_addresses.
    #[deprecated]
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub identity_key: ::prost::alloc::string::String,
    ///
    ///Deprecated, use features.
    #[deprecated]
    #[prost(bytes = "vec", tag = "3")]
    pub global_features: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub alias: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub color: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "7")]
    pub node_addresses: ::prost::alloc::vec::Vec<NodeAddress>,
    ///
    ///Features that the node has advertised in the init message, node
    ///announcements and invoices.
    #[prost(map = "uint32, message", tag = "6")]
    pub features: ::std::collections::HashMap<u32, Feature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEdgeUpdate {
    ///
    ///The unique channel ID for the channel. The first 3 bytes are the block
    ///height, the next 3 the index within the block, and the last 2 bytes are the
    ///output index for the channel.
    #[prost(uint64, tag = "1")]
    pub chan_id: u64,
    #[prost(message, optional, tag = "2")]
    pub chan_point: ::core::option::Option<ChannelPoint>,
    #[prost(int64, tag = "3")]
    pub capacity: i64,
    #[prost(message, optional, tag = "4")]
    pub routing_policy: ::core::option::Option<RoutingPolicy>,
    #[prost(string, tag = "5")]
    pub advertising_node: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub connecting_node: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosedChannelUpdate {
    ///
    ///The unique channel ID for the channel. The first 3 bytes are the block
    ///height, the next 3 the index within the block, and the last 2 bytes are the
    ///output index for the channel.
    #[prost(uint64, tag = "1")]
    pub chan_id: u64,
    #[prost(int64, tag = "2")]
    pub capacity: i64,
    #[prost(uint32, tag = "3")]
    pub closed_height: u32,
    #[prost(message, optional, tag = "4")]
    pub chan_point: ::core::option::Option<ChannelPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HopHint {
    /// The public key of the node at the start of the channel.
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    /// The unique identifier of the channel.
    #[prost(uint64, tag = "2")]
    pub chan_id: u64,
    /// The base fee of the channel denominated in millisatoshis.
    #[prost(uint32, tag = "3")]
    pub fee_base_msat: u32,
    ///
    ///The fee rate of the channel for sending one satoshi across it denominated in
    ///millionths of a satoshi.
    #[prost(uint32, tag = "4")]
    pub fee_proportional_millionths: u32,
    /// The time-lock delta of the channel.
    #[prost(uint32, tag = "5")]
    pub cltv_expiry_delta: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetId {
    #[prost(bytes = "vec", tag = "1")]
    pub set_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteHint {
    ///
    ///A list of hop hints that when chained together can assist in reaching a
    ///specific destination.
    #[prost(message, repeated, tag = "1")]
    pub hop_hints: ::prost::alloc::vec::Vec<HopHint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmpInvoiceState {
    /// The state the HTLCs associated with this setID are in.
    #[prost(enumeration = "InvoiceHtlcState", tag = "1")]
    pub state: i32,
    /// The settle index of this HTLC set, if the invoice state is settled.
    #[prost(uint64, tag = "2")]
    pub settle_index: u64,
    /// The time this HTLC set was settled expressed in unix epoch.
    #[prost(int64, tag = "3")]
    pub settle_time: i64,
    /// The total amount paid for the sub-invoice expressed in milli satoshis.
    #[prost(int64, tag = "5")]
    pub amt_paid_msat: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoice {
    ///
    ///An optional memo to attach along with the invoice. Used for record keeping
    ///purposes for the invoice's creator, and will also be set in the description
    ///field of the encoded payment request if the description_hash field is not
    ///being used.
    #[prost(string, tag = "1")]
    pub memo: ::prost::alloc::string::String,
    ///
    ///The hex-encoded preimage (32 byte) which will allow settling an incoming
    ///HTLC payable to this preimage. When using REST, this field must be encoded
    ///as base64.
    #[prost(bytes = "vec", tag = "3")]
    pub r_preimage: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The hash of the preimage. When using REST, this field must be encoded as
    ///base64.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(bytes = "vec", tag = "4")]
    pub r_hash: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The value of this invoice in satoshis
    ///
    ///The fields value and value_msat are mutually exclusive.
    #[prost(int64, tag = "5")]
    pub value: i64,
    ///
    ///The value of this invoice in millisatoshis
    ///
    ///The fields value and value_msat are mutually exclusive.
    #[prost(int64, tag = "23")]
    pub value_msat: i64,
    /// Whether this invoice has been fulfilled
    #[deprecated]
    #[prost(bool, tag = "6")]
    pub settled: bool,
    ///
    ///When this invoice was created.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(int64, tag = "7")]
    pub creation_date: i64,
    ///
    ///When this invoice was settled.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(int64, tag = "8")]
    pub settle_date: i64,
    ///
    ///A bare-bones invoice for a payment within the Lightning Network. With the
    ///details of the invoice, the sender has all the data necessary to send a
    ///payment to the recipient.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(string, tag = "9")]
    pub payment_request: ::prost::alloc::string::String,
    ///
    ///Hash (SHA-256) of a description of the payment. Used if the description of
    ///payment (memo) is too long to naturally fit within the description field
    ///of an encoded payment request. When using REST, this field must be encoded
    ///as base64.
    #[prost(bytes = "vec", tag = "10")]
    pub description_hash: ::prost::alloc::vec::Vec<u8>,
    /// Payment request expiry time in seconds. Default is 3600 (1 hour).
    #[prost(int64, tag = "11")]
    pub expiry: i64,
    /// Fallback on-chain address.
    #[prost(string, tag = "12")]
    pub fallback_addr: ::prost::alloc::string::String,
    /// Delta to use for the time-lock of the CLTV extended to the final hop.
    #[prost(uint64, tag = "13")]
    pub cltv_expiry: u64,
    ///
    ///Route hints that can each be individually used to assist in reaching the
    ///invoice's destination.
    #[prost(message, repeated, tag = "14")]
    pub route_hints: ::prost::alloc::vec::Vec<RouteHint>,
    /// Whether this invoice should include routing hints for private channels.
    #[prost(bool, tag = "15")]
    pub private: bool,
    ///
    ///The "add" index of this invoice. Each newly created invoice will increment
    ///this index making it monotonically increasing. Callers to the
    ///SubscribeInvoices call can use this to instantly get notified of all added
    ///invoices with an add_index greater than this one.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(uint64, tag = "16")]
    pub add_index: u64,
    ///
    ///The "settle" index of this invoice. Each newly settled invoice will
    ///increment this index making it monotonically increasing. Callers to the
    ///SubscribeInvoices call can use this to instantly get notified of all
    ///settled invoices with an settle_index greater than this one.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(uint64, tag = "17")]
    pub settle_index: u64,
    /// Deprecated, use amt_paid_sat or amt_paid_msat.
    #[deprecated]
    #[prost(int64, tag = "18")]
    pub amt_paid: i64,
    ///
    ///The amount that was accepted for this invoice, in satoshis. This will ONLY
    ///be set if this invoice has been settled. We provide this field as if the
    ///invoice was created with a zero value, then we need to record what amount
    ///was ultimately accepted. Additionally, it's possible that the sender paid
    ///MORE that was specified in the original invoice. So we'll record that here
    ///as well.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(int64, tag = "19")]
    pub amt_paid_sat: i64,
    ///
    ///The amount that was accepted for this invoice, in millisatoshis. This will
    ///ONLY be set if this invoice has been settled. We provide this field as if
    ///the invoice was created with a zero value, then we need to record what
    ///amount was ultimately accepted. Additionally, it's possible that the sender
    ///paid MORE that was specified in the original invoice. So we'll record that
    ///here as well.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(int64, tag = "20")]
    pub amt_paid_msat: i64,
    ///
    ///The state the invoice is in.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(enumeration = "invoice::InvoiceState", tag = "21")]
    pub state: i32,
    ///
    ///List of HTLCs paying to this invoice \[EXPERIMENTAL\].
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(message, repeated, tag = "22")]
    pub htlcs: ::prost::alloc::vec::Vec<InvoiceHtlc>,
    ///
    ///List of features advertised on the invoice.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(map = "uint32, message", tag = "24")]
    pub features: ::std::collections::HashMap<u32, Feature>,
    ///
    ///Indicates if this invoice was a spontaneous payment that arrived via keysend
    ///\[EXPERIMENTAL\].
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(bool, tag = "25")]
    pub is_keysend: bool,
    ///
    ///The payment address of this invoice. This value will be used in MPP
    ///payments, and also for newer invoices that always require the MPP payload
    ///for added end-to-end security.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(bytes = "vec", tag = "26")]
    pub payment_addr: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Signals whether or not this is an AMP invoice.
    #[prost(bool, tag = "27")]
    pub is_amp: bool,
    ///
    ///\[EXPERIMENTAL\]:
    ///
    ///Maps a 32-byte hex-encoded set ID to the sub-invoice AMP state for the
    ///given set ID. This field is always populated for AMP invoices, and can be
    ///used along side LookupInvoice to obtain the HTLC information related to a
    ///given sub-invoice.
    ///Note: Output only, don't specify for creating an invoice.
    #[prost(map = "string, message", tag = "28")]
    pub amp_invoice_state:
        ::std::collections::HashMap<::prost::alloc::string::String, AmpInvoiceState>,
}
/// Nested message and enum types in `Invoice`.
pub mod invoice {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InvoiceState {
        Open = 0,
        Settled = 1,
        Canceled = 2,
        Accepted = 3,
    }
}
/// Details of an HTLC that paid to an invoice
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceHtlc {
    /// Short channel id over which the htlc was received.
    #[prost(uint64, tag = "1")]
    pub chan_id: u64,
    /// Index identifying the htlc on the channel.
    #[prost(uint64, tag = "2")]
    pub htlc_index: u64,
    /// The amount of the htlc in msat.
    #[prost(uint64, tag = "3")]
    pub amt_msat: u64,
    /// Block height at which this htlc was accepted.
    #[prost(int32, tag = "4")]
    pub accept_height: i32,
    /// Time at which this htlc was accepted.
    #[prost(int64, tag = "5")]
    pub accept_time: i64,
    /// Time at which this htlc was settled or canceled.
    #[prost(int64, tag = "6")]
    pub resolve_time: i64,
    /// Block height at which this htlc expires.
    #[prost(int32, tag = "7")]
    pub expiry_height: i32,
    /// Current state the htlc is in.
    #[prost(enumeration = "InvoiceHtlcState", tag = "8")]
    pub state: i32,
    /// Custom tlv records.
    #[prost(map = "uint64, bytes", tag = "9")]
    pub custom_records: ::std::collections::HashMap<u64, ::prost::alloc::vec::Vec<u8>>,
    /// The total amount of the mpp payment in msat.
    #[prost(uint64, tag = "10")]
    pub mpp_total_amt_msat: u64,
    /// Details relevant to AMP HTLCs, only populated if this is an AMP HTLC.
    #[prost(message, optional, tag = "11")]
    pub amp: ::core::option::Option<Amp>,
}
/// Details specific to AMP HTLCs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amp {
    /// An n-of-n secret share of the root seed from which child payment hashes
    /// and preimages are derived.
    #[prost(bytes = "vec", tag = "1")]
    pub root_share: ::prost::alloc::vec::Vec<u8>,
    /// An identifier for the HTLC set that this HTLC belongs to.
    #[prost(bytes = "vec", tag = "2")]
    pub set_id: ::prost::alloc::vec::Vec<u8>,
    /// A nonce used to randomize the child preimage and child hash from a given
    /// root_share.
    #[prost(uint32, tag = "3")]
    pub child_index: u32,
    /// The payment hash of the AMP HTLC.
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// The preimage used to settle this AMP htlc. This field will only be
    /// populated if the invoice is in InvoiceState_ACCEPTED or
    /// InvoiceState_SETTLED.
    #[prost(bytes = "vec", tag = "5")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddInvoiceResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub r_hash: ::prost::alloc::vec::Vec<u8>,
    ///
    ///A bare-bones invoice for a payment within the Lightning Network. With the
    ///details of the invoice, the sender has all the data necessary to send a
    ///payment to the recipient.
    #[prost(string, tag = "2")]
    pub payment_request: ::prost::alloc::string::String,
    ///
    ///The "add" index of this invoice. Each newly created invoice will increment
    ///this index making it monotonically increasing. Callers to the
    ///SubscribeInvoices call can use this to instantly get notified of all added
    ///invoices with an add_index greater than this one.
    #[prost(uint64, tag = "16")]
    pub add_index: u64,
    ///
    ///The payment address of the generated invoice. This value should be used
    ///in all payments for this invoice as we require it for end to end
    ///security.
    #[prost(bytes = "vec", tag = "17")]
    pub payment_addr: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentHash {
    ///
    ///The hex-encoded payment hash of the invoice to be looked up. The passed
    ///payment hash must be exactly 32 bytes, otherwise an error is returned.
    ///Deprecated now that the REST gateway supports base64 encoding of bytes
    ///fields.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub r_hash_str: ::prost::alloc::string::String,
    ///
    ///The payment hash of the invoice to be looked up. When using REST, this field
    ///must be encoded as base64.
    #[prost(bytes = "vec", tag = "2")]
    pub r_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInvoiceRequest {
    ///
    ///If set, only invoices that are not settled and not canceled will be returned
    ///in the response.
    #[prost(bool, tag = "1")]
    pub pending_only: bool,
    ///
    ///The index of an invoice that will be used as either the start or end of a
    ///query to determine which invoices should be returned in the response.
    #[prost(uint64, tag = "4")]
    pub index_offset: u64,
    /// The max number of invoices to return in the response to this query.
    #[prost(uint64, tag = "5")]
    pub num_max_invoices: u64,
    ///
    ///If set, the invoices returned will result from seeking backwards from the
    ///specified index offset. This can be used to paginate backwards.
    #[prost(bool, tag = "6")]
    pub reversed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInvoiceResponse {
    ///
    ///A list of invoices from the time slice of the time series specified in the
    ///request.
    #[prost(message, repeated, tag = "1")]
    pub invoices: ::prost::alloc::vec::Vec<Invoice>,
    ///
    ///The index of the last item in the set of returned invoices. This can be used
    ///to seek further, pagination style.
    #[prost(uint64, tag = "2")]
    pub last_index_offset: u64,
    ///
    ///The index of the last item in the set of returned invoices. This can be used
    ///to seek backwards, pagination style.
    #[prost(uint64, tag = "3")]
    pub first_index_offset: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceSubscription {
    ///
    ///If specified (non-zero), then we'll first start by sending out
    ///notifications for all added indexes with an add_index greater than this
    ///value. This allows callers to catch up on any events they missed while they
    ///weren't connected to the streaming RPC.
    #[prost(uint64, tag = "1")]
    pub add_index: u64,
    ///
    ///If specified (non-zero), then we'll first start by sending out
    ///notifications for all settled indexes with an settle_index greater than
    ///this value. This allows callers to catch up on any events they missed while
    ///they weren't connected to the streaming RPC.
    #[prost(uint64, tag = "2")]
    pub settle_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payment {
    /// The payment hash
    #[prost(string, tag = "1")]
    pub payment_hash: ::prost::alloc::string::String,
    /// Deprecated, use value_sat or value_msat.
    #[deprecated]
    #[prost(int64, tag = "2")]
    pub value: i64,
    /// Deprecated, use creation_time_ns
    #[deprecated]
    #[prost(int64, tag = "3")]
    pub creation_date: i64,
    /// Deprecated, use fee_sat or fee_msat.
    #[deprecated]
    #[prost(int64, tag = "5")]
    pub fee: i64,
    /// The payment preimage
    #[prost(string, tag = "6")]
    pub payment_preimage: ::prost::alloc::string::String,
    /// The value of the payment in satoshis
    #[prost(int64, tag = "7")]
    pub value_sat: i64,
    /// The value of the payment in milli-satoshis
    #[prost(int64, tag = "8")]
    pub value_msat: i64,
    /// The optional payment request being fulfilled.
    #[prost(string, tag = "9")]
    pub payment_request: ::prost::alloc::string::String,
    /// The status of the payment.
    #[prost(enumeration = "payment::PaymentStatus", tag = "10")]
    pub status: i32,
    ///  The fee paid for this payment in satoshis
    #[prost(int64, tag = "11")]
    pub fee_sat: i64,
    ///  The fee paid for this payment in milli-satoshis
    #[prost(int64, tag = "12")]
    pub fee_msat: i64,
    /// The time in UNIX nanoseconds at which the payment was created.
    #[prost(int64, tag = "13")]
    pub creation_time_ns: i64,
    /// The HTLCs made in attempt to settle the payment.
    #[prost(message, repeated, tag = "14")]
    pub htlcs: ::prost::alloc::vec::Vec<HtlcAttempt>,
    ///
    ///The creation index of this payment. Each payment can be uniquely identified
    ///by this index, which may not strictly increment by 1 for payments made in
    ///older versions of lnd.
    #[prost(uint64, tag = "15")]
    pub payment_index: u64,
    #[prost(enumeration = "PaymentFailureReason", tag = "16")]
    pub failure_reason: i32,
}
/// Nested message and enum types in `Payment`.
pub mod payment {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PaymentStatus {
        Unknown = 0,
        InFlight = 1,
        Succeeded = 2,
        Failed = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HtlcAttempt {
    /// The unique ID that is used for this attempt.
    #[prost(uint64, tag = "7")]
    pub attempt_id: u64,
    /// The status of the HTLC.
    #[prost(enumeration = "htlc_attempt::HtlcStatus", tag = "1")]
    pub status: i32,
    /// The route taken by this HTLC.
    #[prost(message, optional, tag = "2")]
    pub route: ::core::option::Option<Route>,
    /// The time in UNIX nanoseconds at which this HTLC was sent.
    #[prost(int64, tag = "3")]
    pub attempt_time_ns: i64,
    ///
    ///The time in UNIX nanoseconds at which this HTLC was settled or failed.
    ///This value will not be set if the HTLC is still IN_FLIGHT.
    #[prost(int64, tag = "4")]
    pub resolve_time_ns: i64,
    /// Detailed htlc failure info.
    #[prost(message, optional, tag = "5")]
    pub failure: ::core::option::Option<Failure>,
    /// The preimage that was used to settle the HTLC.
    #[prost(bytes = "vec", tag = "6")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `HTLCAttempt`.
pub mod htlc_attempt {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HtlcStatus {
        InFlight = 0,
        Succeeded = 1,
        Failed = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPaymentsRequest {
    ///
    ///If true, then return payments that have not yet fully completed. This means
    ///that pending payments, as well as failed payments will show up if this
    ///field is set to true. This flag doesn't change the meaning of the indices,
    ///which are tied to individual payments.
    #[prost(bool, tag = "1")]
    pub include_incomplete: bool,
    ///
    ///The index of a payment that will be used as either the start or end of a
    ///query to determine which payments should be returned in the response. The
    ///index_offset is exclusive. In the case of a zero index_offset, the query
    ///will start with the oldest payment when paginating forwards, or will end
    ///with the most recent payment when paginating backwards.
    #[prost(uint64, tag = "2")]
    pub index_offset: u64,
    /// The maximal number of payments returned in the response to this query.
    #[prost(uint64, tag = "3")]
    pub max_payments: u64,
    ///
    ///If set, the payments returned will result from seeking backwards from the
    ///specified index offset. This can be used to paginate backwards. The order
    ///of the returned payments is always oldest first (ascending index order).
    #[prost(bool, tag = "4")]
    pub reversed: bool,
    ///
    ///If set, all payments (complete and incomplete, independent of the
    ///max_payments parameter) will be counted. Note that setting this to true will
    ///increase the run time of the call significantly on systems that have a lot
    ///of payments, as all of them have to be iterated through to be counted.
    #[prost(bool, tag = "5")]
    pub count_total_payments: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPaymentsResponse {
    /// The list of payments
    #[prost(message, repeated, tag = "1")]
    pub payments: ::prost::alloc::vec::Vec<Payment>,
    ///
    ///The index of the first item in the set of returned payments. This can be
    ///used as the index_offset to continue seeking backwards in the next request.
    #[prost(uint64, tag = "2")]
    pub first_index_offset: u64,
    ///
    ///The index of the last item in the set of returned payments. This can be used
    ///as the index_offset to continue seeking forwards in the next request.
    #[prost(uint64, tag = "3")]
    pub last_index_offset: u64,
    ///
    ///Will only be set if count_total_payments in the request was set. Represents
    ///the total number of payments (complete and incomplete, independent of the
    ///number of payments requested in the query) currently present in the payments
    ///database.
    #[prost(uint64, tag = "4")]
    pub total_num_payments: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePaymentRequest {
    /// Payment hash to delete.
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    ///
    ///Only delete failed HTLCs from the payment, not the payment itself.
    #[prost(bool, tag = "2")]
    pub failed_htlcs_only: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAllPaymentsRequest {
    /// Only delete failed payments.
    #[prost(bool, tag = "1")]
    pub failed_payments_only: bool,
    ///
    ///Only delete failed HTLCs from payments, not the payment itself.
    #[prost(bool, tag = "2")]
    pub failed_htlcs_only: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePaymentResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAllPaymentsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbandonChannelRequest {
    #[prost(message, optional, tag = "1")]
    pub channel_point: ::core::option::Option<ChannelPoint>,
    #[prost(bool, tag = "2")]
    pub pending_funding_shim_only: bool,
    ///
    ///Override the requirement for being in dev mode by setting this to true and
    ///confirming the user knows what they are doing and this is a potential foot
    ///gun to lose funds if used on active channels.
    #[prost(bool, tag = "3")]
    pub i_know_what_i_am_doing: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbandonChannelResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugLevelRequest {
    #[prost(bool, tag = "1")]
    pub show: bool,
    #[prost(string, tag = "2")]
    pub level_spec: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugLevelResponse {
    #[prost(string, tag = "1")]
    pub sub_systems: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayReqString {
    /// The payment request string to be decoded
    #[prost(string, tag = "1")]
    pub pay_req: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayReq {
    #[prost(string, tag = "1")]
    pub destination: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub payment_hash: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub num_satoshis: i64,
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    #[prost(int64, tag = "5")]
    pub expiry: i64,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub description_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub fallback_addr: ::prost::alloc::string::String,
    #[prost(int64, tag = "9")]
    pub cltv_expiry: i64,
    #[prost(message, repeated, tag = "10")]
    pub route_hints: ::prost::alloc::vec::Vec<RouteHint>,
    #[prost(bytes = "vec", tag = "11")]
    pub payment_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "12")]
    pub num_msat: i64,
    #[prost(map = "uint32, message", tag = "13")]
    pub features: ::std::collections::HashMap<u32, Feature>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_required: bool,
    #[prost(bool, tag = "4")]
    pub is_known: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeReportRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFeeReport {
    /// The short channel id that this fee report belongs to.
    #[prost(uint64, tag = "5")]
    pub chan_id: u64,
    /// The channel that this fee report belongs to.
    #[prost(string, tag = "1")]
    pub channel_point: ::prost::alloc::string::String,
    /// The base fee charged regardless of the number of milli-satoshis sent.
    #[prost(int64, tag = "2")]
    pub base_fee_msat: i64,
    /// The amount charged per milli-satoshis transferred expressed in
    /// millionths of a satoshi.
    #[prost(int64, tag = "3")]
    pub fee_per_mil: i64,
    /// The effective fee rate in milli-satoshis. Computed by dividing the
    /// fee_per_mil value by 1 million.
    #[prost(double, tag = "4")]
    pub fee_rate: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeReportResponse {
    /// An array of channel fee reports which describes the current fee schedule
    /// for each channel.
    #[prost(message, repeated, tag = "1")]
    pub channel_fees: ::prost::alloc::vec::Vec<ChannelFeeReport>,
    /// The total amount of fee revenue (in satoshis) the switch has collected
    /// over the past 24 hrs.
    #[prost(uint64, tag = "2")]
    pub day_fee_sum: u64,
    /// The total amount of fee revenue (in satoshis) the switch has collected
    /// over the past 1 week.
    #[prost(uint64, tag = "3")]
    pub week_fee_sum: u64,
    /// The total amount of fee revenue (in satoshis) the switch has collected
    /// over the past 1 month.
    #[prost(uint64, tag = "4")]
    pub month_fee_sum: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyUpdateRequest {
    /// The base fee charged regardless of the number of milli-satoshis sent.
    #[prost(int64, tag = "3")]
    pub base_fee_msat: i64,
    /// The effective fee rate in milli-satoshis. The precision of this value
    /// goes up to 6 decimal places, so 1e-6.
    #[prost(double, tag = "4")]
    pub fee_rate: f64,
    /// The effective fee rate in micro-satoshis (parts per million).
    #[prost(uint32, tag = "9")]
    pub fee_rate_ppm: u32,
    /// The required timelock delta for HTLCs forwarded over the channel.
    #[prost(uint32, tag = "5")]
    pub time_lock_delta: u32,
    /// If set, the maximum HTLC size in milli-satoshis. If unset, the maximum
    /// HTLC will be unchanged.
    #[prost(uint64, tag = "6")]
    pub max_htlc_msat: u64,
    /// The minimum HTLC size in milli-satoshis. Only applied if
    /// min_htlc_msat_specified is true.
    #[prost(uint64, tag = "7")]
    pub min_htlc_msat: u64,
    /// If true, min_htlc_msat is applied.
    #[prost(bool, tag = "8")]
    pub min_htlc_msat_specified: bool,
    #[prost(oneof = "policy_update_request::Scope", tags = "1, 2")]
    pub scope: ::core::option::Option<policy_update_request::Scope>,
}
/// Nested message and enum types in `PolicyUpdateRequest`.
pub mod policy_update_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        /// If set, then this update applies to all currently active channels.
        #[prost(bool, tag = "1")]
        Global(bool),
        /// If set, this update will target a specific channel.
        #[prost(message, tag = "2")]
        ChanPoint(super::ChannelPoint),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedUpdate {
    /// The outpoint in format txid:n
    #[prost(message, optional, tag = "1")]
    pub outpoint: ::core::option::Option<OutPoint>,
    /// Reason for the policy update failure.
    #[prost(enumeration = "UpdateFailure", tag = "2")]
    pub reason: i32,
    /// A string representation of the policy update error.
    #[prost(string, tag = "3")]
    pub update_error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyUpdateResponse {
    /// List of failed policy updates.
    #[prost(message, repeated, tag = "1")]
    pub failed_updates: ::prost::alloc::vec::Vec<FailedUpdate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingHistoryRequest {
    /// Start time is the starting point of the forwarding history request. All
    /// records beyond this point will be included, respecting the end time, and
    /// the index offset.
    #[prost(uint64, tag = "1")]
    pub start_time: u64,
    /// End time is the end point of the forwarding history request. The
    /// response will carry at most 50k records between the start time and the
    /// end time. The index offset can be used to implement pagination.
    #[prost(uint64, tag = "2")]
    pub end_time: u64,
    /// Index offset is the offset in the time series to start at. As each
    /// response can only contain 50k records, callers can use this to skip
    /// around within a packed time series.
    #[prost(uint32, tag = "3")]
    pub index_offset: u32,
    /// The max number of events to return in the response to this query.
    #[prost(uint32, tag = "4")]
    pub num_max_events: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingEvent {
    /// Timestamp is the time (unix epoch offset) that this circuit was
    /// completed. Deprecated by timestamp_ns.
    #[deprecated]
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// The incoming channel ID that carried the HTLC that created the circuit.
    #[prost(uint64, tag = "2")]
    pub chan_id_in: u64,
    /// The outgoing channel ID that carried the preimage that completed the
    /// circuit.
    #[prost(uint64, tag = "4")]
    pub chan_id_out: u64,
    /// The total amount (in satoshis) of the incoming HTLC that created half
    /// the circuit.
    #[prost(uint64, tag = "5")]
    pub amt_in: u64,
    /// The total amount (in satoshis) of the outgoing HTLC that created the
    /// second half of the circuit.
    #[prost(uint64, tag = "6")]
    pub amt_out: u64,
    /// The total fee (in satoshis) that this payment circuit carried.
    #[prost(uint64, tag = "7")]
    pub fee: u64,
    /// The total fee (in milli-satoshis) that this payment circuit carried.
    #[prost(uint64, tag = "8")]
    pub fee_msat: u64,
    /// The total amount (in milli-satoshis) of the incoming HTLC that created
    /// half the circuit.
    #[prost(uint64, tag = "9")]
    pub amt_in_msat: u64,
    /// The total amount (in milli-satoshis) of the outgoing HTLC that created
    /// the second half of the circuit.
    #[prost(uint64, tag = "10")]
    pub amt_out_msat: u64,
    /// The number of nanoseconds elapsed since January 1, 1970 UTC when this
    /// circuit was completed.
    #[prost(uint64, tag = "11")]
    pub timestamp_ns: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingHistoryResponse {
    /// A list of forwarding events from the time slice of the time series
    /// specified in the request.
    #[prost(message, repeated, tag = "1")]
    pub forwarding_events: ::prost::alloc::vec::Vec<ForwardingEvent>,
    /// The index of the last time in the set of returned forwarding events. Can
    /// be used to seek further, pagination style.
    #[prost(uint32, tag = "2")]
    pub last_offset_index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportChannelBackupRequest {
    /// The target channel point to obtain a back up for.
    #[prost(message, optional, tag = "1")]
    pub chan_point: ::core::option::Option<ChannelPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBackup {
    ///
    ///Identifies the channel that this backup belongs to.
    #[prost(message, optional, tag = "1")]
    pub chan_point: ::core::option::Option<ChannelPoint>,
    ///
    ///Is an encrypted single-chan backup. this can be passed to
    ///RestoreChannelBackups, or the WalletUnlocker Init and Unlock methods in
    ///order to trigger the recovery protocol. When using REST, this field must be
    ///encoded as base64.
    #[prost(bytes = "vec", tag = "2")]
    pub chan_backup: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiChanBackup {
    ///
    ///Is the set of all channels that are included in this multi-channel backup.
    #[prost(message, repeated, tag = "1")]
    pub chan_points: ::prost::alloc::vec::Vec<ChannelPoint>,
    ///
    ///A single encrypted blob containing all the static channel backups of the
    ///channel listed above. This can be stored as a single file or blob, and
    ///safely be replaced with any prior/future versions. When using REST, this
    ///field must be encoded as base64.
    #[prost(bytes = "vec", tag = "2")]
    pub multi_chan_backup: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChanBackupExportRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChanBackupSnapshot {
    ///
    ///The set of new channels that have been added since the last channel backup
    ///snapshot was requested.
    #[prost(message, optional, tag = "1")]
    pub single_chan_backups: ::core::option::Option<ChannelBackups>,
    ///
    ///A multi-channel backup that covers all open channels currently known to
    ///lnd.
    #[prost(message, optional, tag = "2")]
    pub multi_chan_backup: ::core::option::Option<MultiChanBackup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBackups {
    ///
    ///A set of single-chan static channel backups.
    #[prost(message, repeated, tag = "1")]
    pub chan_backups: ::prost::alloc::vec::Vec<ChannelBackup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreChanBackupRequest {
    #[prost(oneof = "restore_chan_backup_request::Backup", tags = "1, 2")]
    pub backup: ::core::option::Option<restore_chan_backup_request::Backup>,
}
/// Nested message and enum types in `RestoreChanBackupRequest`.
pub mod restore_chan_backup_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Backup {
        ///
        ///The channels to restore as a list of channel/backup pairs.
        #[prost(message, tag = "1")]
        ChanBackups(super::ChannelBackups),
        ///
        ///The channels to restore in the packed multi backup format. When using
        ///REST, this field must be encoded as base64.
        #[prost(bytes, tag = "2")]
        MultiChanBackup(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreBackupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelBackupSubscription {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyChanBackupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacaroonPermission {
    /// The entity a permission grants access to.
    #[prost(string, tag = "1")]
    pub entity: ::prost::alloc::string::String,
    /// The action that is granted.
    #[prost(string, tag = "2")]
    pub action: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BakeMacaroonRequest {
    /// The list of permissions the new macaroon should grant.
    #[prost(message, repeated, tag = "1")]
    pub permissions: ::prost::alloc::vec::Vec<MacaroonPermission>,
    /// The root key ID used to create the macaroon, must be a positive integer.
    #[prost(uint64, tag = "2")]
    pub root_key_id: u64,
    ///
    ///Informs the RPC on whether to allow external permissions that LND is not
    ///aware of.
    #[prost(bool, tag = "3")]
    pub allow_external_permissions: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BakeMacaroonResponse {
    /// The hex encoded macaroon, serialized in binary format.
    #[prost(string, tag = "1")]
    pub macaroon: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMacaroonIDsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMacaroonIDsResponse {
    /// The list of root key IDs that are in use.
    #[prost(uint64, repeated, tag = "1")]
    pub root_key_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMacaroonIdRequest {
    /// The root key ID to be removed.
    #[prost(uint64, tag = "1")]
    pub root_key_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMacaroonIdResponse {
    /// A boolean indicates that the deletion is successful.
    #[prost(bool, tag = "1")]
    pub deleted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacaroonPermissionList {
    /// A list of macaroon permissions.
    #[prost(message, repeated, tag = "1")]
    pub permissions: ::prost::alloc::vec::Vec<MacaroonPermission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPermissionsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPermissionsResponse {
    ///
    ///A map between all RPC method URIs and their required macaroon permissions to
    ///access them.
    #[prost(map = "string, message", tag = "1")]
    pub method_permissions:
        ::std::collections::HashMap<::prost::alloc::string::String, MacaroonPermissionList>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Failure {
    /// Failure code as defined in the Lightning spec
    #[prost(enumeration = "failure::FailureCode", tag = "1")]
    pub code: i32,
    /// An optional channel update message.
    #[prost(message, optional, tag = "3")]
    pub channel_update: ::core::option::Option<ChannelUpdate>,
    /// A failure type-dependent htlc value.
    #[prost(uint64, tag = "4")]
    pub htlc_msat: u64,
    /// The sha256 sum of the onion payload.
    #[prost(bytes = "vec", tag = "5")]
    pub onion_sha_256: ::prost::alloc::vec::Vec<u8>,
    /// A failure type-dependent cltv expiry value.
    #[prost(uint32, tag = "6")]
    pub cltv_expiry: u32,
    /// A failure type-dependent flags value.
    #[prost(uint32, tag = "7")]
    pub flags: u32,
    ///
    ///The position in the path of the intermediate or final node that generated
    ///the failure message. Position zero is the sender node.
    #[prost(uint32, tag = "8")]
    pub failure_source_index: u32,
    /// A failure type-dependent block height.
    #[prost(uint32, tag = "9")]
    pub height: u32,
}
/// Nested message and enum types in `Failure`.
pub mod failure {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FailureCode {
        ///
        ///The numbers assigned in this enumeration match the failure codes as
        ///defined in BOLT #4. Because protobuf 3 requires enums to start with 0,
        ///a RESERVED value is added.
        Reserved = 0,
        IncorrectOrUnknownPaymentDetails = 1,
        IncorrectPaymentAmount = 2,
        FinalIncorrectCltvExpiry = 3,
        FinalIncorrectHtlcAmount = 4,
        FinalExpiryTooSoon = 5,
        InvalidRealm = 6,
        ExpiryTooSoon = 7,
        InvalidOnionVersion = 8,
        InvalidOnionHmac = 9,
        InvalidOnionKey = 10,
        AmountBelowMinimum = 11,
        FeeInsufficient = 12,
        IncorrectCltvExpiry = 13,
        ChannelDisabled = 14,
        TemporaryChannelFailure = 15,
        RequiredNodeFeatureMissing = 16,
        RequiredChannelFeatureMissing = 17,
        UnknownNextPeer = 18,
        TemporaryNodeFailure = 19,
        PermanentNodeFailure = 20,
        PermanentChannelFailure = 21,
        ExpiryTooFar = 22,
        MppTimeout = 23,
        InvalidOnionPayload = 24,
        ///
        ///An internal error occurred.
        InternalFailure = 997,
        ///
        ///The error source is known, but the failure itself couldn't be decoded.
        UnknownFailure = 998,
        ///
        ///An unreadable failure result is returned if the received failure message
        ///cannot be decrypted. In that case the error source is unknown.
        UnreadableFailure = 999,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUpdate {
    ///
    ///The signature that validates the announced data and proves the ownership
    ///of node id.
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The target chain that this channel was opened within. This value
    ///should be the genesis hash of the target chain. Along with the short
    ///channel ID, this uniquely identifies the channel globally in a
    ///blockchain.
    #[prost(bytes = "vec", tag = "2")]
    pub chain_hash: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The unique description of the funding transaction.
    #[prost(uint64, tag = "3")]
    pub chan_id: u64,
    ///
    ///A timestamp that allows ordering in the case of multiple announcements.
    ///We should ignore the message if timestamp is not greater than the
    ///last-received.
    #[prost(uint32, tag = "4")]
    pub timestamp: u32,
    ///
    ///The bitfield that describes whether optional fields are present in this
    ///update. Currently, the least-significant bit must be set to 1 if the
    ///optional field MaxHtlc is present.
    #[prost(uint32, tag = "10")]
    pub message_flags: u32,
    ///
    ///The bitfield that describes additional meta-data concerning how the
    ///update is to be interpreted. Currently, the least-significant bit must be
    ///set to 0 if the creating node corresponds to the first node in the
    ///previously sent channel announcement and 1 otherwise. If the second bit
    ///is set, then the channel is set to be disabled.
    #[prost(uint32, tag = "5")]
    pub channel_flags: u32,
    ///
    ///The minimum number of blocks this node requires to be added to the expiry
    ///of HTLCs. This is a security parameter determined by the node operator.
    ///This value represents the required gap between the time locks of the
    ///incoming and outgoing HTLC's set to this node.
    #[prost(uint32, tag = "6")]
    pub time_lock_delta: u32,
    ///
    ///The minimum HTLC value which will be accepted.
    #[prost(uint64, tag = "7")]
    pub htlc_minimum_msat: u64,
    ///
    ///The base fee that must be used for incoming HTLC's to this particular
    ///channel. This value will be tacked onto the required for a payment
    ///independent of the size of the payment.
    #[prost(uint32, tag = "8")]
    pub base_fee: u32,
    ///
    ///The fee rate that will be charged per millionth of a satoshi.
    #[prost(uint32, tag = "9")]
    pub fee_rate: u32,
    ///
    ///The maximum HTLC value which will be accepted.
    #[prost(uint64, tag = "11")]
    pub htlc_maximum_msat: u64,
    ///
    ///The set of data that was appended to this message, some of which we may
    ///not actually know how to iterate or parse. By holding onto this data, we
    ///ensure that we're able to properly validate the set of signatures that
    ///cover these new fields, and ensure we're able to make upgrades to the
    ///network in a forwards compatible manner.
    #[prost(bytes = "vec", tag = "12")]
    pub extra_opaque_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacaroonId {
    #[prost(bytes = "vec", tag = "1")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub storage_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub ops: ::prost::alloc::vec::Vec<Op>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Op {
    #[prost(string, tag = "1")]
    pub entity: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckMacPermRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub macaroon: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<MacaroonPermission>,
    #[prost(string, tag = "3")]
    pub full_method: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckMacPermResponse {
    #[prost(bool, tag = "1")]
    pub valid: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcMiddlewareRequest {
    ///
    ///The unique ID of the intercepted original gRPC request. Useful for mapping
    ///request to response when implementing full duplex message interception. For
    ///streaming requests, this will be the same ID for all incoming and outgoing
    ///middleware intercept messages of the _same_ stream.
    #[prost(uint64, tag = "1")]
    pub request_id: u64,
    ///
    ///The raw bytes of the complete macaroon as sent by the gRPC client in the
    ///original request. This might be empty for a request that doesn't require
    ///macaroons such as the wallet unlocker RPCs.
    #[prost(bytes = "vec", tag = "2")]
    pub raw_macaroon: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The parsed condition of the macaroon's custom caveat for convenient access.
    ///This field only contains the value of the custom caveat that the handling
    ///middleware has registered itself for. The condition _must_ be validated for
    ///messages of intercept_type stream_auth and request!
    #[prost(string, tag = "3")]
    pub custom_caveat_condition: ::prost::alloc::string::String,
    ///
    ///The unique message ID of this middleware intercept message. There can be
    ///multiple middleware intercept messages per single gRPC request (one for the
    ///incoming request and one for the outgoing response) or gRPC stream (one for
    ///each incoming message and one for each outgoing response). This message ID
    ///must be referenced when responding (accepting/rejecting/modifying) to an
    ///intercept message.
    #[prost(uint64, tag = "7")]
    pub msg_id: u64,
    ///
    ///There are three types of messages that will be sent to the middleware for
    ///inspection and approval: Stream authentication, request and response
    ///interception. The first two can only be accepted (=forward to main RPC
    ///server) or denied (=return error to client). Intercepted responses can also
    ///be replaced/overwritten.
    #[prost(oneof = "rpc_middleware_request::InterceptType", tags = "4, 5, 6")]
    pub intercept_type: ::core::option::Option<rpc_middleware_request::InterceptType>,
}
/// Nested message and enum types in `RPCMiddlewareRequest`.
pub mod rpc_middleware_request {
    ///
    ///There are three types of messages that will be sent to the middleware for
    ///inspection and approval: Stream authentication, request and response
    ///interception. The first two can only be accepted (=forward to main RPC
    ///server) or denied (=return error to client). Intercepted responses can also
    ///be replaced/overwritten.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InterceptType {
        ///
        ///Intercept stream authentication: each new streaming RPC call that is
        ///initiated against lnd and contains the middleware's custom macaroon
        ///caveat can be approved or denied based upon the macaroon in the stream
        ///header. This message will only be sent for streaming RPCs, unary RPCs
        ///must handle the macaroon authentication in the request interception to
        ///avoid an additional message round trip between lnd and the middleware.
        #[prost(message, tag = "4")]
        StreamAuth(super::StreamAuth),
        ///
        ///Intercept incoming gRPC client request message: all incoming messages,
        ///both on streaming and unary RPCs, are forwarded to the middleware for
        ///inspection. For unary RPC messages the middleware is also expected to
        ///validate the custom macaroon caveat of the request.
        #[prost(message, tag = "5")]
        Request(super::RpcMessage),
        ///
        ///Intercept outgoing gRPC response message: all outgoing messages, both on
        ///streaming and unary RPCs, are forwarded to the middleware for inspection
        ///and amendment. The response in this message is the original response as
        ///it was generated by the main RPC server. It can either be accepted
        ///(=forwarded to the client), replaced/overwritten with a new message of
        ///the same type, or replaced by an error message.
        #[prost(message, tag = "6")]
        Response(super::RpcMessage),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAuth {
    ///
    ///The full URI (in the format /<rpcpackage>.<ServiceName>/MethodName, for
    ///example /lnrpc.Lightning/GetInfo) of the streaming RPC method that was just
    ///established.
    #[prost(string, tag = "1")]
    pub method_full_uri: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcMessage {
    ///
    ///The full URI (in the format /<rpcpackage>.<ServiceName>/MethodName, for
    ///example /lnrpc.Lightning/GetInfo) of the RPC method the message was sent
    ///to/from.
    #[prost(string, tag = "1")]
    pub method_full_uri: ::prost::alloc::string::String,
    ///
    ///Indicates whether the message was sent over a streaming RPC method or not.
    #[prost(bool, tag = "2")]
    pub stream_rpc: bool,
    ///
    ///The full canonical gRPC name of the message type (in the format
    ///<rpcpackage>.TypeName, for example lnrpc.GetInfoRequest).
    #[prost(string, tag = "3")]
    pub type_name: ::prost::alloc::string::String,
    ///
    ///The full content of the gRPC message, serialized in the binary protobuf
    ///format.
    #[prost(bytes = "vec", tag = "4")]
    pub serialized: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcMiddlewareResponse {
    ///
    ///The request message ID this response refers to. Must always be set when
    ///giving feedback to an intercept but is ignored for the initial registration
    ///message.
    #[prost(uint64, tag = "1")]
    pub ref_msg_id: u64,
    ///
    ///The middleware can only send two types of messages to lnd: The initial
    ///registration message that identifies the middleware and after that only
    ///feedback messages to requests sent to the middleware.
    #[prost(oneof = "rpc_middleware_response::MiddlewareMessage", tags = "2, 3")]
    pub middleware_message: ::core::option::Option<rpc_middleware_response::MiddlewareMessage>,
}
/// Nested message and enum types in `RPCMiddlewareResponse`.
pub mod rpc_middleware_response {
    ///
    ///The middleware can only send two types of messages to lnd: The initial
    ///registration message that identifies the middleware and after that only
    ///feedback messages to requests sent to the middleware.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MiddlewareMessage {
        ///
        ///The registration message identifies the middleware that's being
        ///registered in lnd. The registration message must be sent immediately
        ///after initiating the RegisterRpcMiddleware stream, otherwise lnd will
        ///time out the attempt and terminate the request. NOTE: The middleware
        ///will only receive interception messages for requests that contain a
        ///macaroon with the custom caveat that the middleware declares it is
        ///responsible for handling in the registration message! As a security
        ///measure, _no_ middleware can intercept requests made with _unencumbered_
        ///macaroons!
        #[prost(message, tag = "2")]
        Register(super::MiddlewareRegistration),
        ///
        ///The middleware received an interception request and gives feedback to
        ///it. The request_id indicates what message the feedback refers to.
        #[prost(message, tag = "3")]
        Feedback(super::InterceptFeedback),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MiddlewareRegistration {
    ///
    ///The name of the middleware to register. The name should be as informative
    ///as possible and is logged on registration.
    #[prost(string, tag = "1")]
    pub middleware_name: ::prost::alloc::string::String,
    ///
    ///The name of the custom macaroon caveat that this middleware is responsible
    ///for. Only requests/responses that contain a macaroon with the registered
    ///custom caveat are forwarded for interception to the middleware. The
    ///exception being the read-only mode: All requests/responses are forwarded to
    ///a middleware that requests read-only access but such a middleware won't be
    ///allowed to _alter_ responses. As a security measure, _no_ middleware can
    ///change responses to requests made with _unencumbered_ macaroons!
    ///NOTE: Cannot be used at the same time as read_only_mode.
    #[prost(string, tag = "2")]
    pub custom_macaroon_caveat_name: ::prost::alloc::string::String,
    ///
    ///Instead of defining a custom macaroon caveat name a middleware can register
    ///itself for read-only access only. In that mode all requests/responses are
    ///forwarded to the middleware but the middleware isn't allowed to alter any of
    ///the responses.
    ///NOTE: Cannot be used at the same time as custom_macaroon_caveat_name.
    #[prost(bool, tag = "3")]
    pub read_only_mode: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterceptFeedback {
    ///
    ///The error to return to the user. If this is non-empty, the incoming gRPC
    ///stream/request is aborted and the error is returned to the gRPC client. If
    ///this value is empty, it means the middleware accepts the stream/request/
    ///response and the processing of it can continue.
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    ///
    ///A boolean indicating that the gRPC response should be replaced/overwritten.
    ///As its name suggests, this can only be used as a feedback to an intercepted
    ///response RPC message and is ignored for feedback on any other message. This
    ///boolean is needed because in protobuf an empty message is serialized as a
    ///0-length or nil byte slice and we wouldn't be able to distinguish between
    ///an empty replacement message and the "don't replace anything" case.
    #[prost(bool, tag = "2")]
    pub replace_response: bool,
    ///
    ///If the replace_response field is set to true, this field must contain the
    ///binary serialized gRPC response message in the protobuf format.
    #[prost(bytes = "vec", tag = "3")]
    pub replacement_serialized: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutputScriptType {
    ScriptTypePubkeyHash = 0,
    ScriptTypeScriptHash = 1,
    ScriptTypeWitnessV0PubkeyHash = 2,
    ScriptTypeWitnessV0ScriptHash = 3,
    ScriptTypePubkey = 4,
    ScriptTypeMultisig = 5,
    ScriptTypeNulldata = 6,
    ScriptTypeNonStandard = 7,
    ScriptTypeWitnessUnknown = 8,
}
///
///`AddressType` has to be one of:
///
///- `p2wkh`: Pay to witness key hash (`WITNESS_PUBKEY_HASH` = 0)
///- `np2wkh`: Pay to nested witness key hash (`NESTED_PUBKEY_HASH` = 1)
///- `p2tr`: Pay to taproot pubkey (`TAPROOT_PUBKEY` = 4)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddressType {
    WitnessPubkeyHash = 0,
    NestedPubkeyHash = 1,
    UnusedWitnessPubkeyHash = 2,
    UnusedNestedPubkeyHash = 3,
    TaprootPubkey = 4,
    UnusedTaprootPubkey = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommitmentType {
    ///
    ///Returned when the commitment type isn't known or unavailable.
    UnknownCommitmentType = 0,
    ///
    ///A channel using the legacy commitment format having tweaked to_remote
    ///keys.
    Legacy = 1,
    ///
    ///A channel that uses the modern commitment format where the key in the
    ///output of the remote party does not change each state. This makes back
    ///up and recovery easier as when the channel is closed, the funds go
    ///directly to that key.
    StaticRemoteKey = 2,
    ///
    ///A channel that uses a commitment format that has anchor outputs on the
    ///commitments, allowing fee bumping after a force close transaction has
    ///been broadcast.
    Anchors = 3,
    ///
    ///A channel that uses a commitment type that builds upon the anchors
    ///commitment format, but in addition requires a CLTV clause to spend outputs
    ///paying to the channel initiator. This is intended for use on leased channels
    ///to guarantee that the channel initiator has no incentives to close a leased
    ///channel before its maturity date.
    ScriptEnforcedLease = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Initiator {
    Unknown = 0,
    Local = 1,
    Remote = 2,
    Both = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResolutionType {
    TypeUnknown = 0,
    /// We resolved an anchor output.
    Anchor = 1,
    ///
    ///We are resolving an incoming htlc on chain. This if this htlc is
    ///claimed, we swept the incoming htlc with the preimage. If it is timed
    ///out, our peer swept the timeout path.
    IncomingHtlc = 2,
    ///
    ///We are resolving an outgoing htlc on chain. If this htlc is claimed,
    ///the remote party swept the htlc with the preimage. If it is timed out,
    ///we swept it with the timeout path.
    OutgoingHtlc = 3,
    /// We force closed and need to sweep our time locked commitment output.
    Commit = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResolutionOutcome {
    /// Outcome unknown.
    OutcomeUnknown = 0,
    /// An output was claimed on chain.
    Claimed = 1,
    /// An output was left unclaimed on chain.
    Unclaimed = 2,
    ///
    ///ResolverOutcomeAbandoned indicates that an output that we did not
    ///claim on chain, for example an anchor that we did not sweep and a
    ///third party claimed on chain, or a htlc that we could not decode
    ///so left unclaimed.
    Abandoned = 3,
    ///
    ///If we force closed our channel, our htlcs need to be claimed in two
    ///stages. This outcome represents the broadcast of a timeout or success
    ///transaction for this two stage htlc claim.
    FirstStage = 4,
    /// A htlc was timed out on chain.
    Timeout = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NodeMetricType {
    Unknown = 0,
    BetweennessCentrality = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InvoiceHtlcState {
    Accepted = 0,
    Settled = 1,
    Canceled = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentFailureReason {
    ///
    ///Payment isn't failed (yet).
    FailureReasonNone = 0,
    ///
    ///There are more routes to try, but the payment timeout was exceeded.
    FailureReasonTimeout = 1,
    ///
    ///All possible routes were tried and failed permanently. Or were no
    ///routes to the destination at all.
    FailureReasonNoRoute = 2,
    ///
    ///A non-recoverable error has occured.
    FailureReasonError = 3,
    ///
    ///Payment details incorrect (unknown hash, invalid amt or
    ///invalid final cltv delta)
    FailureReasonIncorrectPaymentDetails = 4,
    ///
    ///Insufficient local balance.
    FailureReasonInsufficientBalance = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FeatureBit {
    DatalossProtectReq = 0,
    DatalossProtectOpt = 1,
    InitialRouingSync = 3,
    UpfrontShutdownScriptReq = 4,
    UpfrontShutdownScriptOpt = 5,
    GossipQueriesReq = 6,
    GossipQueriesOpt = 7,
    TlvOnionReq = 8,
    TlvOnionOpt = 9,
    ExtGossipQueriesReq = 10,
    ExtGossipQueriesOpt = 11,
    StaticRemoteKeyReq = 12,
    StaticRemoteKeyOpt = 13,
    PaymentAddrReq = 14,
    PaymentAddrOpt = 15,
    MppReq = 16,
    MppOpt = 17,
    WumboChannelsReq = 18,
    WumboChannelsOpt = 19,
    AnchorsReq = 20,
    AnchorsOpt = 21,
    AnchorsZeroFeeHtlcReq = 22,
    AnchorsZeroFeeHtlcOpt = 23,
    AmpReq = 30,
    AmpOpt = 31,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateFailure {
    Unknown = 0,
    Pending = 1,
    NotFound = 2,
    InternalErr = 3,
    InvalidParameter = 4,
}
#[doc = r" Generated client implementations."]
pub mod lightning_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Lightning is the main RPC server of the daemon."]
    #[derive(Debug, Clone)]
    pub struct LightningClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LightningClient<tonic::transport::Channel> {
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
    impl<T> LightningClient<T>
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
        ) -> LightningClient<InterceptedService<T, F>>
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
            LightningClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " lncli: `walletbalance`"]
        #[doc = "WalletBalance returns total unspent outputs(confirmed and unconfirmed), all"]
        #[doc = "confirmed unspent outputs and all unconfirmed unspent outputs under control"]
        #[doc = "of the wallet."]
        pub async fn wallet_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::WalletBalanceRequest>,
        ) -> Result<tonic::Response<super::WalletBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/WalletBalance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `channelbalance`"]
        #[doc = "ChannelBalance returns a report on the total funds across all open channels,"]
        #[doc = "categorized in local/remote, pending local/remote and unsettled local/remote"]
        #[doc = "balances."]
        pub async fn channel_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelBalanceRequest>,
        ) -> Result<tonic::Response<super::ChannelBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ChannelBalance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `listchaintxns`"]
        #[doc = "GetTransactions returns a list describing all the known transactions"]
        #[doc = "relevant to the wallet."]
        pub async fn get_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsRequest>,
        ) -> Result<tonic::Response<super::TransactionDetails>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/GetTransactions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `estimatefee`"]
        #[doc = "EstimateFee asks the chain backend to estimate the fee rate and total fees"]
        #[doc = "for a transaction that pays to multiple specified outputs."]
        #[doc = ""]
        #[doc = "When using REST, the `AddrToAmount` map type can be set by appending"]
        #[doc = "`&AddrToAmount[<address>]=<amount_to_send>` to the URL. Unfortunately this"]
        #[doc = "map type doesn't appear in the REST API documentation because of a bug in"]
        #[doc = "the grpc-gateway library."]
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
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/EstimateFee");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `sendcoins`"]
        #[doc = "SendCoins executes a request to send coins to a particular address. Unlike"]
        #[doc = "SendMany, this RPC call only allows creating a single output at a time. If"]
        #[doc = "neither target_conf, or sat_per_vbyte are set, then the internal wallet will"]
        #[doc = "consult its fee model to determine a fee for the default confirmation"]
        #[doc = "target."]
        pub async fn send_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::SendCoinsRequest>,
        ) -> Result<tonic::Response<super::SendCoinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SendCoins");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `listunspent`"]
        #[doc = "Deprecated, use walletrpc.ListUnspent instead."]
        #[doc = ""]
        #[doc = "ListUnspent returns a list of all utxos spendable by the wallet with a"]
        #[doc = "number of confirmations between the specified minimum and maximum."]
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
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ListUnspent");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SubscribeTransactions creates a uni-directional stream from the server to"]
        #[doc = "the client in which any newly discovered transactions relevant to the"]
        #[doc = "wallet are sent over."]
        pub async fn subscribe_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Transaction>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SubscribeTransactions");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " lncli: `sendmany`"]
        #[doc = "SendMany handles a request for a transaction that creates multiple specified"]
        #[doc = "outputs in parallel. If neither target_conf, or sat_per_vbyte are set, then"]
        #[doc = "the internal wallet will consult its fee model to determine a fee for the"]
        #[doc = "default confirmation target."]
        pub async fn send_many(
            &mut self,
            request: impl tonic::IntoRequest<super::SendManyRequest>,
        ) -> Result<tonic::Response<super::SendManyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SendMany");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `newaddress`"]
        #[doc = "NewAddress creates a new address under control of the local wallet."]
        pub async fn new_address(
            &mut self,
            request: impl tonic::IntoRequest<super::NewAddressRequest>,
        ) -> Result<tonic::Response<super::NewAddressResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/NewAddress");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `signmessage`"]
        #[doc = "SignMessage signs a message with this node's private key. The returned"]
        #[doc = "signature string is `zbase32` encoded and pubkey recoverable, meaning that"]
        #[doc = "only the message digest and signature are needed for verification."]
        pub async fn sign_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessageRequest>,
        ) -> Result<tonic::Response<super::SignMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SignMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `verifymessage`"]
        #[doc = "VerifyMessage verifies a signature over a msg. The signature must be"]
        #[doc = "zbase32 encoded and signed by an active node in the resident node's"]
        #[doc = "channel database. In addition to returning the validity of the signature,"]
        #[doc = "VerifyMessage also returns the recovered pubkey from the signature."]
        pub async fn verify_message(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMessageRequest>,
        ) -> Result<tonic::Response<super::VerifyMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/VerifyMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `connect`"]
        #[doc = "ConnectPeer attempts to establish a connection to a remote peer. This is at"]
        #[doc = "the networking level, and is used for communication between nodes. This is"]
        #[doc = "distinct from establishing a channel with a peer."]
        pub async fn connect_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectPeerRequest>,
        ) -> Result<tonic::Response<super::ConnectPeerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ConnectPeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `disconnect`"]
        #[doc = "DisconnectPeer attempts to disconnect one peer from another identified by a"]
        #[doc = "given pubKey. In the case that we currently have a pending or active channel"]
        #[doc = "with the target peer, then this action will be not be allowed."]
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
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/DisconnectPeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `listpeers`"]
        #[doc = "ListPeers returns a verbose listing of all currently active peers."]
        pub async fn list_peers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPeersRequest>,
        ) -> Result<tonic::Response<super::ListPeersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ListPeers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SubscribePeerEvents creates a uni-directional stream from the server to"]
        #[doc = "the client in which any events relevant to the state of peers are sent"]
        #[doc = "over. Events include peers going online and offline."]
        pub async fn subscribe_peer_events(
            &mut self,
            request: impl tonic::IntoRequest<super::PeerEventSubscription>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::PeerEvent>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SubscribePeerEvents");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " lncli: `getinfo`"]
        #[doc = "GetInfo returns general information concerning the lightning node including"]
        #[doc = "it's identity pubkey, alias, the chains it is connected to, and information"]
        #[doc = "concerning the number of open+pending channels."]
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoRequest>,
        ) -> Result<tonic::Response<super::GetInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/GetInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "* lncli: `getrecoveryinfo`"]
        #[doc = "GetRecoveryInfo returns information concerning the recovery mode including"]
        #[doc = "whether it's in a recovery mode, whether the recovery is finished, and the"]
        #[doc = "progress made so far."]
        pub async fn get_recovery_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecoveryInfoRequest>,
        ) -> Result<tonic::Response<super::GetRecoveryInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/GetRecoveryInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `pendingchannels`"]
        #[doc = "PendingChannels returns a list of all the channels that are currently"]
        #[doc = "considered \"pending\". A channel is pending if it has finished the funding"]
        #[doc = "workflow and is waiting for confirmations for the funding txn, or is in the"]
        #[doc = "process of closure, either initiated cooperatively or non-cooperatively."]
        pub async fn pending_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::PendingChannelsRequest>,
        ) -> Result<tonic::Response<super::PendingChannelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/PendingChannels");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `listchannels`"]
        #[doc = "ListChannels returns a description of all the open channels that this node"]
        #[doc = "is a participant in."]
        pub async fn list_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelsRequest>,
        ) -> Result<tonic::Response<super::ListChannelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ListChannels");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SubscribeChannelEvents creates a uni-directional stream from the server to"]
        #[doc = "the client in which any updates relevant to the state of the channels are"]
        #[doc = "sent over. Events include new active channels, inactive channels, and closed"]
        #[doc = "channels."]
        pub async fn subscribe_channel_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelEventSubscription>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ChannelEventUpdate>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SubscribeChannelEvents");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " lncli: `closedchannels`"]
        #[doc = "ClosedChannels returns a description of all the closed channels that"]
        #[doc = "this node was a participant in."]
        pub async fn closed_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ClosedChannelsRequest>,
        ) -> Result<tonic::Response<super::ClosedChannelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ClosedChannels");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "OpenChannelSync is a synchronous version of the OpenChannel RPC call. This"]
        #[doc = "call is meant to be consumed by clients to the REST proxy. As with all"]
        #[doc = "other sync calls, all byte slices are intended to be populated as hex"]
        #[doc = "encoded strings."]
        pub async fn open_channel_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenChannelRequest>,
        ) -> Result<tonic::Response<super::ChannelPoint>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/OpenChannelSync");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `openchannel`"]
        #[doc = "OpenChannel attempts to open a singly funded channel specified in the"]
        #[doc = "request to a remote peer. Users are able to specify a target number of"]
        #[doc = "blocks that the funding transaction should be confirmed in, or a manual fee"]
        #[doc = "rate to us for the funding transaction. If neither are specified, then a"]
        #[doc = "lax block confirmation target is used. Each OpenStatusUpdate will return"]
        #[doc = "the pending channel ID of the in-progress channel. Depending on the"]
        #[doc = "arguments specified in the OpenChannelRequest, this pending channel ID can"]
        #[doc = "then be used to manually progress the channel funding flow."]
        pub async fn open_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenChannelRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::OpenStatusUpdate>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/OpenChannel");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " lncli: `batchopenchannel`"]
        #[doc = "BatchOpenChannel attempts to open multiple single-funded channels in a"]
        #[doc = "single transaction in an atomic way. This means either all channel open"]
        #[doc = "requests succeed at once or all attempts are aborted if any of them fail."]
        #[doc = "This is the safer variant of using PSBTs to manually fund a batch of"]
        #[doc = "channels through the OpenChannel RPC."]
        pub async fn batch_open_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchOpenChannelRequest>,
        ) -> Result<tonic::Response<super::BatchOpenChannelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/BatchOpenChannel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "FundingStateStep is an advanced funding related call that allows the caller"]
        #[doc = "to either execute some preparatory steps for a funding workflow, or"]
        #[doc = "manually progress a funding workflow. The primary way a funding flow is"]
        #[doc = "identified is via its pending channel ID. As an example, this method can be"]
        #[doc = "used to specify that we're expecting a funding flow for a particular"]
        #[doc = "pending channel ID, for which we need to use specific parameters."]
        #[doc = "Alternatively, this can be used to interactively drive PSBT signing for"]
        #[doc = "funding for partially complete funding transactions."]
        pub async fn funding_state_step(
            &mut self,
            request: impl tonic::IntoRequest<super::FundingTransitionMsg>,
        ) -> Result<tonic::Response<super::FundingStateStepResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/FundingStateStep");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ChannelAcceptor dispatches a bi-directional streaming RPC in which"]
        #[doc = "OpenChannel requests are sent to the client and the client responds with"]
        #[doc = "a boolean that tells LND whether or not to accept the channel. This allows"]
        #[doc = "node operators to specify their own criteria for accepting inbound channels"]
        #[doc = "through a single persistent connection."]
        pub async fn channel_acceptor(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ChannelAcceptResponse>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ChannelAcceptRequest>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ChannelAcceptor");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " lncli: `closechannel`"]
        #[doc = "CloseChannel attempts to close an active channel identified by its channel"]
        #[doc = "outpoint (ChannelPoint). The actions of this method can additionally be"]
        #[doc = "augmented to attempt a force close after a timeout period in the case of an"]
        #[doc = "inactive peer. If a non-force close (cooperative closure) is requested,"]
        #[doc = "then the user can specify either a target number of blocks until the"]
        #[doc = "closure transaction is confirmed, or a manual fee rate. If neither are"]
        #[doc = "specified, then a default lax, block confirmation target is used."]
        pub async fn close_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseChannelRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::CloseStatusUpdate>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/CloseChannel");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " lncli: `abandonchannel`"]
        #[doc = "AbandonChannel removes all channel state from the database except for a"]
        #[doc = "close summary. This method can be used to get rid of permanently unusable"]
        #[doc = "channels due to bugs fixed in newer versions of lnd. This method can also be"]
        #[doc = "used to remove externally funded channels where the funding transaction was"]
        #[doc = "never broadcast. Only available for non-externally funded channels in dev"]
        #[doc = "build."]
        pub async fn abandon_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::AbandonChannelRequest>,
        ) -> Result<tonic::Response<super::AbandonChannelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/AbandonChannel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `sendpayment`"]
        #[doc = "Deprecated, use routerrpc.SendPaymentV2. SendPayment dispatches a"]
        #[doc = "bi-directional streaming RPC for sending payments through the Lightning"]
        #[doc = "Network. A single RPC invocation creates a persistent bi-directional"]
        #[doc = "stream allowing clients to rapidly send payments through the Lightning"]
        #[doc = "Network with a single persistent connection."]
        pub async fn send_payment(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SendRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SendResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SendPayment");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = "SendPaymentSync is the synchronous non-streaming version of SendPayment."]
        #[doc = "This RPC is intended to be consumed by clients of the REST proxy."]
        #[doc = "Additionally, this RPC expects the destination's public key and the payment"]
        #[doc = "hash (if any) to be encoded as hex strings."]
        pub async fn send_payment_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SendRequest>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SendPaymentSync");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `sendtoroute`"]
        #[doc = "Deprecated, use routerrpc.SendToRouteV2. SendToRoute is a bi-directional"]
        #[doc = "streaming RPC for sending payment through the Lightning Network. This"]
        #[doc = "method differs from SendPayment in that it allows users to specify a full"]
        #[doc = "route manually. This can be used for things like rebalancing, and atomic"]
        #[doc = "swaps."]
        pub async fn send_to_route(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SendToRouteRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SendResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SendToRoute");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = "SendToRouteSync is a synchronous version of SendToRoute. It Will block"]
        #[doc = "until the payment either fails or succeeds."]
        pub async fn send_to_route_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SendToRouteRequest>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SendToRouteSync");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `addinvoice`"]
        #[doc = "AddInvoice attempts to add a new invoice to the invoice database. Any"]
        #[doc = "duplicated invoices are rejected, therefore all invoices *must* have a"]
        #[doc = "unique payment preimage."]
        pub async fn add_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::Invoice>,
        ) -> Result<tonic::Response<super::AddInvoiceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/AddInvoice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `listinvoices`"]
        #[doc = "ListInvoices returns a list of all the invoices currently stored within the"]
        #[doc = "database. Any active debug invoices are ignored. It has full support for"]
        #[doc = "paginated responses, allowing users to query for specific invoices through"]
        #[doc = "their add_index. This can be done by using either the first_index_offset or"]
        #[doc = "last_index_offset fields included in the response as the index_offset of the"]
        #[doc = "next request. By default, the first 100 invoices created will be returned."]
        #[doc = "Backwards pagination is also supported through the Reversed flag."]
        pub async fn list_invoices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInvoiceRequest>,
        ) -> Result<tonic::Response<super::ListInvoiceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ListInvoices");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `lookupinvoice`"]
        #[doc = "LookupInvoice attempts to look up an invoice according to its payment hash."]
        #[doc = "The passed payment hash *must* be exactly 32 bytes, if not, an error is"]
        #[doc = "returned."]
        pub async fn lookup_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::PaymentHash>,
        ) -> Result<tonic::Response<super::Invoice>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/LookupInvoice");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SubscribeInvoices returns a uni-directional stream (server -> client) for"]
        #[doc = "notifying the client of newly added/settled invoices. The caller can"]
        #[doc = "optionally specify the add_index and/or the settle_index. If the add_index"]
        #[doc = "is specified, then we'll first start by sending add invoice events for all"]
        #[doc = "invoices with an add_index greater than the specified value. If the"]
        #[doc = "settle_index is specified, the next, we'll send out all settle events for"]
        #[doc = "invoices with a settle_index greater than the specified value. One or both"]
        #[doc = "of these fields can be set. If no fields are set, then we'll only send out"]
        #[doc = "the latest add/settle events."]
        pub async fn subscribe_invoices(
            &mut self,
            request: impl tonic::IntoRequest<super::InvoiceSubscription>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Invoice>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SubscribeInvoices");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " lncli: `decodepayreq`"]
        #[doc = "DecodePayReq takes an encoded payment request string and attempts to decode"]
        #[doc = "it, returning a full description of the conditions encoded within the"]
        #[doc = "payment request."]
        pub async fn decode_pay_req(
            &mut self,
            request: impl tonic::IntoRequest<super::PayReqString>,
        ) -> Result<tonic::Response<super::PayReq>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/DecodePayReq");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `listpayments`"]
        #[doc = "ListPayments returns a list of all outgoing payments."]
        pub async fn list_payments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPaymentsRequest>,
        ) -> Result<tonic::Response<super::ListPaymentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ListPayments");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DeletePayment deletes an outgoing payment from DB. Note that it will not"]
        #[doc = "attempt to delete an In-Flight payment, since that would be unsafe."]
        pub async fn delete_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePaymentRequest>,
        ) -> Result<tonic::Response<super::DeletePaymentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/DeletePayment");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "DeleteAllPayments deletes all outgoing payments from DB. Note that it will"]
        #[doc = "not attempt to delete In-Flight payments, since that would be unsafe."]
        pub async fn delete_all_payments(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAllPaymentsRequest>,
        ) -> Result<tonic::Response<super::DeleteAllPaymentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/DeleteAllPayments");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `describegraph`"]
        #[doc = "DescribeGraph returns a description of the latest graph state from the"]
        #[doc = "point of view of the node. The graph information is partitioned into two"]
        #[doc = "components: all the nodes/vertexes, and all the edges that connect the"]
        #[doc = "vertexes themselves. As this is a directed graph, the edges also contain"]
        #[doc = "the node directional specific routing policy which includes: the time lock"]
        #[doc = "delta, fee information, etc."]
        pub async fn describe_graph(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelGraphRequest>,
        ) -> Result<tonic::Response<super::ChannelGraph>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/DescribeGraph");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `getnodemetrics`"]
        #[doc = "GetNodeMetrics returns node metrics calculated from the graph. Currently"]
        #[doc = "the only supported metric is betweenness centrality of individual nodes."]
        pub async fn get_node_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::NodeMetricsRequest>,
        ) -> Result<tonic::Response<super::NodeMetricsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/GetNodeMetrics");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `getchaninfo`"]
        #[doc = "GetChanInfo returns the latest authenticated network announcement for the"]
        #[doc = "given channel identified by its channel ID: an 8-byte integer which"]
        #[doc = "uniquely identifies the location of transaction's funding output within the"]
        #[doc = "blockchain."]
        pub async fn get_chan_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ChanInfoRequest>,
        ) -> Result<tonic::Response<super::ChannelEdge>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/GetChanInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `getnodeinfo`"]
        #[doc = "GetNodeInfo returns the latest advertised, aggregated, and authenticated"]
        #[doc = "channel information for the specified node identified by its public key."]
        pub async fn get_node_info(
            &mut self,
            request: impl tonic::IntoRequest<super::NodeInfoRequest>,
        ) -> Result<tonic::Response<super::NodeInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/GetNodeInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `queryroutes`"]
        #[doc = "QueryRoutes attempts to query the daemon's Channel Router for a possible"]
        #[doc = "route to a target destination capable of carrying a specific amount of"]
        #[doc = "satoshis. The returned route contains the full details required to craft and"]
        #[doc = "send an HTLC, also including the necessary information that should be"]
        #[doc = "present within the Sphinx packet encapsulated within the HTLC."]
        #[doc = ""]
        #[doc = "When using REST, the `dest_custom_records` map type can be set by appending"]
        #[doc = "`&dest_custom_records[<record_number>]=<record_data_base64_url_encoded>`"]
        #[doc = "to the URL. Unfortunately this map type doesn't appear in the REST API"]
        #[doc = "documentation because of a bug in the grpc-gateway library."]
        pub async fn query_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRoutesRequest>,
        ) -> Result<tonic::Response<super::QueryRoutesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/QueryRoutes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `getnetworkinfo`"]
        #[doc = "GetNetworkInfo returns some basic stats about the known channel graph from"]
        #[doc = "the point of view of the node."]
        pub async fn get_network_info(
            &mut self,
            request: impl tonic::IntoRequest<super::NetworkInfoRequest>,
        ) -> Result<tonic::Response<super::NetworkInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/GetNetworkInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `stop`"]
        #[doc = "StopDaemon will send a shutdown request to the interrupt handler, triggering"]
        #[doc = "a graceful shutdown of the daemon."]
        pub async fn stop_daemon(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/StopDaemon");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SubscribeChannelGraph launches a streaming RPC that allows the caller to"]
        #[doc = "receive notifications upon any changes to the channel graph topology from"]
        #[doc = "the point of view of the responding node. Events notified include: new"]
        #[doc = "nodes coming online, nodes updating their authenticated attributes, new"]
        #[doc = "channels being advertised, updates in the routing policy for a directional"]
        #[doc = "channel edge, and when channels are closed on-chain."]
        pub async fn subscribe_channel_graph(
            &mut self,
            request: impl tonic::IntoRequest<super::GraphTopologySubscription>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GraphTopologyUpdate>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SubscribeChannelGraph");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " lncli: `debuglevel`"]
        #[doc = "DebugLevel allows a caller to programmatically set the logging verbosity of"]
        #[doc = "lnd. The logging can be targeted according to a coarse daemon-wide logging"]
        #[doc = "level, or in a granular fashion to specify the logging for a target"]
        #[doc = "sub-system."]
        pub async fn debug_level(
            &mut self,
            request: impl tonic::IntoRequest<super::DebugLevelRequest>,
        ) -> Result<tonic::Response<super::DebugLevelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/DebugLevel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `feereport`"]
        #[doc = "FeeReport allows the caller to obtain a report detailing the current fee"]
        #[doc = "schedule enforced by the node globally for each channel."]
        pub async fn fee_report(
            &mut self,
            request: impl tonic::IntoRequest<super::FeeReportRequest>,
        ) -> Result<tonic::Response<super::FeeReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/FeeReport");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `updatechanpolicy`"]
        #[doc = "UpdateChannelPolicy allows the caller to update the fee schedule and"]
        #[doc = "channel policies for all channels globally, or a particular channel."]
        pub async fn update_channel_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyUpdateRequest>,
        ) -> Result<tonic::Response<super::PolicyUpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/UpdateChannelPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `fwdinghistory`"]
        #[doc = "ForwardingHistory allows the caller to query the htlcswitch for a record of"]
        #[doc = "all HTLCs forwarded within the target time range, and integer offset"]
        #[doc = "within that time range, for a maximum number of events. If no maximum number"]
        #[doc = "of events is specified, up to 100 events will be returned. If no time-range"]
        #[doc = "is specified, then events will be returned in the order that they occured."]
        #[doc = ""]
        #[doc = "A list of forwarding events are returned. The size of each forwarding event"]
        #[doc = "is 40 bytes, and the max message size able to be returned in gRPC is 4 MiB."]
        #[doc = "As a result each message can only contain 50k entries. Each response has"]
        #[doc = "the index offset of the last entry. The index offset can be provided to the"]
        #[doc = "request to allow the caller to skip a series of records."]
        pub async fn forwarding_history(
            &mut self,
            request: impl tonic::IntoRequest<super::ForwardingHistoryRequest>,
        ) -> Result<tonic::Response<super::ForwardingHistoryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ForwardingHistory");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `exportchanbackup`"]
        #[doc = "ExportChannelBackup attempts to return an encrypted static channel backup"]
        #[doc = "for the target channel identified by it channel point. The backup is"]
        #[doc = "encrypted with a key generated from the aezeed seed of the user. The"]
        #[doc = "returned backup can either be restored using the RestoreChannelBackup"]
        #[doc = "method once lnd is running, or via the InitWallet and UnlockWallet methods"]
        #[doc = "from the WalletUnlocker service."]
        pub async fn export_channel_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportChannelBackupRequest>,
        ) -> Result<tonic::Response<super::ChannelBackup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ExportChannelBackup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "ExportAllChannelBackups returns static channel backups for all existing"]
        #[doc = "channels known to lnd. A set of regular singular static channel backups for"]
        #[doc = "each channel are returned. Additionally, a multi-channel backup is returned"]
        #[doc = "as well, which contains a single encrypted blob containing the backups of"]
        #[doc = "each channel."]
        pub async fn export_all_channel_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::ChanBackupExportRequest>,
        ) -> Result<tonic::Response<super::ChanBackupSnapshot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ExportAllChannelBackups");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "VerifyChanBackup allows a caller to verify the integrity of a channel backup"]
        #[doc = "snapshot. This method will accept either a packed Single or a packed Multi."]
        #[doc = "Specifying both will result in an error."]
        pub async fn verify_chan_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::ChanBackupSnapshot>,
        ) -> Result<tonic::Response<super::VerifyChanBackupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/VerifyChanBackup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `restorechanbackup`"]
        #[doc = "RestoreChannelBackups accepts a set of singular channel backups, or a"]
        #[doc = "single encrypted multi-chan backup and attempts to recover any funds"]
        #[doc = "remaining within the channel. If we are able to unpack the backup, then the"]
        #[doc = "new channel will be shown under listchannels, as well as pending channels."]
        pub async fn restore_channel_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreChanBackupRequest>,
        ) -> Result<tonic::Response<super::RestoreBackupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/RestoreChannelBackups");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "SubscribeChannelBackups allows a client to sub-subscribe to the most up to"]
        #[doc = "date information concerning the state of all channel backups. Each time a"]
        #[doc = "new channel is added, we return the new set of channels, along with a"]
        #[doc = "multi-chan backup containing the backup info for all channels. Each time a"]
        #[doc = "channel is closed, we send a new update, which contains new new chan back"]
        #[doc = "ups, but the updated set of encrypted multi-chan backups with the closed"]
        #[doc = "channel(s) removed."]
        pub async fn subscribe_channel_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelBackupSubscription>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ChanBackupSnapshot>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SubscribeChannelBackups");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " lncli: `bakemacaroon`"]
        #[doc = "BakeMacaroon allows the creation of a new macaroon with custom read and"]
        #[doc = "write permissions. No first-party caveats are added since this can be done"]
        #[doc = "offline."]
        pub async fn bake_macaroon(
            &mut self,
            request: impl tonic::IntoRequest<super::BakeMacaroonRequest>,
        ) -> Result<tonic::Response<super::BakeMacaroonResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/BakeMacaroon");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `listmacaroonids`"]
        #[doc = "ListMacaroonIDs returns all root key IDs that are in use."]
        pub async fn list_macaroon_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMacaroonIDsRequest>,
        ) -> Result<tonic::Response<super::ListMacaroonIDsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ListMacaroonIDs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `deletemacaroonid`"]
        #[doc = "DeleteMacaroonID deletes the specified macaroon ID and invalidates all"]
        #[doc = "macaroons derived from that ID."]
        pub async fn delete_macaroon_id(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMacaroonIdRequest>,
        ) -> Result<tonic::Response<super::DeleteMacaroonIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/DeleteMacaroonID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `listpermissions`"]
        #[doc = "ListPermissions lists all RPC method URIs and their required macaroon"]
        #[doc = "permissions to access them."]
        pub async fn list_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPermissionsRequest>,
        ) -> Result<tonic::Response<super::ListPermissionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/ListPermissions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "CheckMacaroonPermissions checks whether a request follows the constraints"]
        #[doc = "imposed on the macaroon and that the macaroon is authorized to follow the"]
        #[doc = "provided permissions."]
        pub async fn check_macaroon_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckMacPermRequest>,
        ) -> Result<tonic::Response<super::CheckMacPermResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/CheckMacaroonPermissions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "RegisterRPCMiddleware adds a new gRPC middleware to the interceptor chain. A"]
        #[doc = "gRPC middleware is software component external to lnd that aims to add"]
        #[doc = "additional business logic to lnd by observing/intercepting/validating"]
        #[doc = "incoming gRPC client requests and (if needed) replacing/overwriting outgoing"]
        #[doc = "messages before they're sent to the client. When registering the middleware"]
        #[doc = "must identify itself and indicate what custom macaroon caveats it wants to"]
        #[doc = "be responsible for. Only requests that contain a macaroon with that specific"]
        #[doc = "custom caveat are then sent to the middleware for inspection. The other"]
        #[doc = "option is to register for the read-only mode in which all requests/responses"]
        #[doc = "are forwarded for interception to the middleware but the middleware is not"]
        #[doc = "allowed to modify any responses. As a security measure, _no_ middleware can"]
        #[doc = "modify responses for requests made with _unencumbered_ macaroons!"]
        pub async fn register_rpc_middleware(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::RpcMiddlewareResponse>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RpcMiddlewareRequest>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/RegisterRPCMiddleware");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " lncli: `sendcustom`"]
        #[doc = "SendCustomMessage sends a custom peer message."]
        pub async fn send_custom_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SendCustomMessageRequest>,
        ) -> Result<tonic::Response<super::SendCustomMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SendCustomMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `subscribecustom`"]
        #[doc = "SubscribeCustomMessages subscribes to a stream of incoming custom peer"]
        #[doc = "messages."]
        pub async fn subscribe_custom_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCustomMessagesRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::CustomMessage>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/lnrpc.Lightning/SubscribeCustomMessages");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod lightning_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with LightningServer."]
    #[async_trait]
    pub trait Lightning: Send + Sync + 'static {
        #[doc = " lncli: `walletbalance`"]
        #[doc = "WalletBalance returns total unspent outputs(confirmed and unconfirmed), all"]
        #[doc = "confirmed unspent outputs and all unconfirmed unspent outputs under control"]
        #[doc = "of the wallet."]
        async fn wallet_balance(
            &self,
            request: tonic::Request<super::WalletBalanceRequest>,
        ) -> Result<tonic::Response<super::WalletBalanceResponse>, tonic::Status>;
        #[doc = " lncli: `channelbalance`"]
        #[doc = "ChannelBalance returns a report on the total funds across all open channels,"]
        #[doc = "categorized in local/remote, pending local/remote and unsettled local/remote"]
        #[doc = "balances."]
        async fn channel_balance(
            &self,
            request: tonic::Request<super::ChannelBalanceRequest>,
        ) -> Result<tonic::Response<super::ChannelBalanceResponse>, tonic::Status>;
        #[doc = " lncli: `listchaintxns`"]
        #[doc = "GetTransactions returns a list describing all the known transactions"]
        #[doc = "relevant to the wallet."]
        async fn get_transactions(
            &self,
            request: tonic::Request<super::GetTransactionsRequest>,
        ) -> Result<tonic::Response<super::TransactionDetails>, tonic::Status>;
        #[doc = " lncli: `estimatefee`"]
        #[doc = "EstimateFee asks the chain backend to estimate the fee rate and total fees"]
        #[doc = "for a transaction that pays to multiple specified outputs."]
        #[doc = ""]
        #[doc = "When using REST, the `AddrToAmount` map type can be set by appending"]
        #[doc = "`&AddrToAmount[<address>]=<amount_to_send>` to the URL. Unfortunately this"]
        #[doc = "map type doesn't appear in the REST API documentation because of a bug in"]
        #[doc = "the grpc-gateway library."]
        async fn estimate_fee(
            &self,
            request: tonic::Request<super::EstimateFeeRequest>,
        ) -> Result<tonic::Response<super::EstimateFeeResponse>, tonic::Status>;
        #[doc = " lncli: `sendcoins`"]
        #[doc = "SendCoins executes a request to send coins to a particular address. Unlike"]
        #[doc = "SendMany, this RPC call only allows creating a single output at a time. If"]
        #[doc = "neither target_conf, or sat_per_vbyte are set, then the internal wallet will"]
        #[doc = "consult its fee model to determine a fee for the default confirmation"]
        #[doc = "target."]
        async fn send_coins(
            &self,
            request: tonic::Request<super::SendCoinsRequest>,
        ) -> Result<tonic::Response<super::SendCoinsResponse>, tonic::Status>;
        #[doc = " lncli: `listunspent`"]
        #[doc = "Deprecated, use walletrpc.ListUnspent instead."]
        #[doc = ""]
        #[doc = "ListUnspent returns a list of all utxos spendable by the wallet with a"]
        #[doc = "number of confirmations between the specified minimum and maximum."]
        async fn list_unspent(
            &self,
            request: tonic::Request<super::ListUnspentRequest>,
        ) -> Result<tonic::Response<super::ListUnspentResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeTransactions method."]
        type SubscribeTransactionsStream: futures_core::Stream<Item = Result<super::Transaction, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "SubscribeTransactions creates a uni-directional stream from the server to"]
        #[doc = "the client in which any newly discovered transactions relevant to the"]
        #[doc = "wallet are sent over."]
        async fn subscribe_transactions(
            &self,
            request: tonic::Request<super::GetTransactionsRequest>,
        ) -> Result<tonic::Response<Self::SubscribeTransactionsStream>, tonic::Status>;
        #[doc = " lncli: `sendmany`"]
        #[doc = "SendMany handles a request for a transaction that creates multiple specified"]
        #[doc = "outputs in parallel. If neither target_conf, or sat_per_vbyte are set, then"]
        #[doc = "the internal wallet will consult its fee model to determine a fee for the"]
        #[doc = "default confirmation target."]
        async fn send_many(
            &self,
            request: tonic::Request<super::SendManyRequest>,
        ) -> Result<tonic::Response<super::SendManyResponse>, tonic::Status>;
        #[doc = " lncli: `newaddress`"]
        #[doc = "NewAddress creates a new address under control of the local wallet."]
        async fn new_address(
            &self,
            request: tonic::Request<super::NewAddressRequest>,
        ) -> Result<tonic::Response<super::NewAddressResponse>, tonic::Status>;
        #[doc = " lncli: `signmessage`"]
        #[doc = "SignMessage signs a message with this node's private key. The returned"]
        #[doc = "signature string is `zbase32` encoded and pubkey recoverable, meaning that"]
        #[doc = "only the message digest and signature are needed for verification."]
        async fn sign_message(
            &self,
            request: tonic::Request<super::SignMessageRequest>,
        ) -> Result<tonic::Response<super::SignMessageResponse>, tonic::Status>;
        #[doc = " lncli: `verifymessage`"]
        #[doc = "VerifyMessage verifies a signature over a msg. The signature must be"]
        #[doc = "zbase32 encoded and signed by an active node in the resident node's"]
        #[doc = "channel database. In addition to returning the validity of the signature,"]
        #[doc = "VerifyMessage also returns the recovered pubkey from the signature."]
        async fn verify_message(
            &self,
            request: tonic::Request<super::VerifyMessageRequest>,
        ) -> Result<tonic::Response<super::VerifyMessageResponse>, tonic::Status>;
        #[doc = " lncli: `connect`"]
        #[doc = "ConnectPeer attempts to establish a connection to a remote peer. This is at"]
        #[doc = "the networking level, and is used for communication between nodes. This is"]
        #[doc = "distinct from establishing a channel with a peer."]
        async fn connect_peer(
            &self,
            request: tonic::Request<super::ConnectPeerRequest>,
        ) -> Result<tonic::Response<super::ConnectPeerResponse>, tonic::Status>;
        #[doc = " lncli: `disconnect`"]
        #[doc = "DisconnectPeer attempts to disconnect one peer from another identified by a"]
        #[doc = "given pubKey. In the case that we currently have a pending or active channel"]
        #[doc = "with the target peer, then this action will be not be allowed."]
        async fn disconnect_peer(
            &self,
            request: tonic::Request<super::DisconnectPeerRequest>,
        ) -> Result<tonic::Response<super::DisconnectPeerResponse>, tonic::Status>;
        #[doc = " lncli: `listpeers`"]
        #[doc = "ListPeers returns a verbose listing of all currently active peers."]
        async fn list_peers(
            &self,
            request: tonic::Request<super::ListPeersRequest>,
        ) -> Result<tonic::Response<super::ListPeersResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribePeerEvents method."]
        type SubscribePeerEventsStream: futures_core::Stream<Item = Result<super::PeerEvent, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "SubscribePeerEvents creates a uni-directional stream from the server to"]
        #[doc = "the client in which any events relevant to the state of peers are sent"]
        #[doc = "over. Events include peers going online and offline."]
        async fn subscribe_peer_events(
            &self,
            request: tonic::Request<super::PeerEventSubscription>,
        ) -> Result<tonic::Response<Self::SubscribePeerEventsStream>, tonic::Status>;
        #[doc = " lncli: `getinfo`"]
        #[doc = "GetInfo returns general information concerning the lightning node including"]
        #[doc = "it's identity pubkey, alias, the chains it is connected to, and information"]
        #[doc = "concerning the number of open+pending channels."]
        async fn get_info(
            &self,
            request: tonic::Request<super::GetInfoRequest>,
        ) -> Result<tonic::Response<super::GetInfoResponse>, tonic::Status>;
        #[doc = "* lncli: `getrecoveryinfo`"]
        #[doc = "GetRecoveryInfo returns information concerning the recovery mode including"]
        #[doc = "whether it's in a recovery mode, whether the recovery is finished, and the"]
        #[doc = "progress made so far."]
        async fn get_recovery_info(
            &self,
            request: tonic::Request<super::GetRecoveryInfoRequest>,
        ) -> Result<tonic::Response<super::GetRecoveryInfoResponse>, tonic::Status>;
        #[doc = " lncli: `pendingchannels`"]
        #[doc = "PendingChannels returns a list of all the channels that are currently"]
        #[doc = "considered \"pending\". A channel is pending if it has finished the funding"]
        #[doc = "workflow and is waiting for confirmations for the funding txn, or is in the"]
        #[doc = "process of closure, either initiated cooperatively or non-cooperatively."]
        async fn pending_channels(
            &self,
            request: tonic::Request<super::PendingChannelsRequest>,
        ) -> Result<tonic::Response<super::PendingChannelsResponse>, tonic::Status>;
        #[doc = " lncli: `listchannels`"]
        #[doc = "ListChannels returns a description of all the open channels that this node"]
        #[doc = "is a participant in."]
        async fn list_channels(
            &self,
            request: tonic::Request<super::ListChannelsRequest>,
        ) -> Result<tonic::Response<super::ListChannelsResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeChannelEvents method."]
        type SubscribeChannelEventsStream: futures_core::Stream<Item = Result<super::ChannelEventUpdate, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "SubscribeChannelEvents creates a uni-directional stream from the server to"]
        #[doc = "the client in which any updates relevant to the state of the channels are"]
        #[doc = "sent over. Events include new active channels, inactive channels, and closed"]
        #[doc = "channels."]
        async fn subscribe_channel_events(
            &self,
            request: tonic::Request<super::ChannelEventSubscription>,
        ) -> Result<tonic::Response<Self::SubscribeChannelEventsStream>, tonic::Status>;
        #[doc = " lncli: `closedchannels`"]
        #[doc = "ClosedChannels returns a description of all the closed channels that"]
        #[doc = "this node was a participant in."]
        async fn closed_channels(
            &self,
            request: tonic::Request<super::ClosedChannelsRequest>,
        ) -> Result<tonic::Response<super::ClosedChannelsResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "OpenChannelSync is a synchronous version of the OpenChannel RPC call. This"]
        #[doc = "call is meant to be consumed by clients to the REST proxy. As with all"]
        #[doc = "other sync calls, all byte slices are intended to be populated as hex"]
        #[doc = "encoded strings."]
        async fn open_channel_sync(
            &self,
            request: tonic::Request<super::OpenChannelRequest>,
        ) -> Result<tonic::Response<super::ChannelPoint>, tonic::Status>;
        #[doc = "Server streaming response type for the OpenChannel method."]
        type OpenChannelStream: futures_core::Stream<Item = Result<super::OpenStatusUpdate, tonic::Status>>
            + Send
            + 'static;
        #[doc = " lncli: `openchannel`"]
        #[doc = "OpenChannel attempts to open a singly funded channel specified in the"]
        #[doc = "request to a remote peer. Users are able to specify a target number of"]
        #[doc = "blocks that the funding transaction should be confirmed in, or a manual fee"]
        #[doc = "rate to us for the funding transaction. If neither are specified, then a"]
        #[doc = "lax block confirmation target is used. Each OpenStatusUpdate will return"]
        #[doc = "the pending channel ID of the in-progress channel. Depending on the"]
        #[doc = "arguments specified in the OpenChannelRequest, this pending channel ID can"]
        #[doc = "then be used to manually progress the channel funding flow."]
        async fn open_channel(
            &self,
            request: tonic::Request<super::OpenChannelRequest>,
        ) -> Result<tonic::Response<Self::OpenChannelStream>, tonic::Status>;
        #[doc = " lncli: `batchopenchannel`"]
        #[doc = "BatchOpenChannel attempts to open multiple single-funded channels in a"]
        #[doc = "single transaction in an atomic way. This means either all channel open"]
        #[doc = "requests succeed at once or all attempts are aborted if any of them fail."]
        #[doc = "This is the safer variant of using PSBTs to manually fund a batch of"]
        #[doc = "channels through the OpenChannel RPC."]
        async fn batch_open_channel(
            &self,
            request: tonic::Request<super::BatchOpenChannelRequest>,
        ) -> Result<tonic::Response<super::BatchOpenChannelResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "FundingStateStep is an advanced funding related call that allows the caller"]
        #[doc = "to either execute some preparatory steps for a funding workflow, or"]
        #[doc = "manually progress a funding workflow. The primary way a funding flow is"]
        #[doc = "identified is via its pending channel ID. As an example, this method can be"]
        #[doc = "used to specify that we're expecting a funding flow for a particular"]
        #[doc = "pending channel ID, for which we need to use specific parameters."]
        #[doc = "Alternatively, this can be used to interactively drive PSBT signing for"]
        #[doc = "funding for partially complete funding transactions."]
        async fn funding_state_step(
            &self,
            request: tonic::Request<super::FundingTransitionMsg>,
        ) -> Result<tonic::Response<super::FundingStateStepResp>, tonic::Status>;
        #[doc = "Server streaming response type for the ChannelAcceptor method."]
        type ChannelAcceptorStream: futures_core::Stream<Item = Result<super::ChannelAcceptRequest, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "ChannelAcceptor dispatches a bi-directional streaming RPC in which"]
        #[doc = "OpenChannel requests are sent to the client and the client responds with"]
        #[doc = "a boolean that tells LND whether or not to accept the channel. This allows"]
        #[doc = "node operators to specify their own criteria for accepting inbound channels"]
        #[doc = "through a single persistent connection."]
        async fn channel_acceptor(
            &self,
            request: tonic::Request<tonic::Streaming<super::ChannelAcceptResponse>>,
        ) -> Result<tonic::Response<Self::ChannelAcceptorStream>, tonic::Status>;
        #[doc = "Server streaming response type for the CloseChannel method."]
        type CloseChannelStream: futures_core::Stream<Item = Result<super::CloseStatusUpdate, tonic::Status>>
            + Send
            + 'static;
        #[doc = " lncli: `closechannel`"]
        #[doc = "CloseChannel attempts to close an active channel identified by its channel"]
        #[doc = "outpoint (ChannelPoint). The actions of this method can additionally be"]
        #[doc = "augmented to attempt a force close after a timeout period in the case of an"]
        #[doc = "inactive peer. If a non-force close (cooperative closure) is requested,"]
        #[doc = "then the user can specify either a target number of blocks until the"]
        #[doc = "closure transaction is confirmed, or a manual fee rate. If neither are"]
        #[doc = "specified, then a default lax, block confirmation target is used."]
        async fn close_channel(
            &self,
            request: tonic::Request<super::CloseChannelRequest>,
        ) -> Result<tonic::Response<Self::CloseChannelStream>, tonic::Status>;
        #[doc = " lncli: `abandonchannel`"]
        #[doc = "AbandonChannel removes all channel state from the database except for a"]
        #[doc = "close summary. This method can be used to get rid of permanently unusable"]
        #[doc = "channels due to bugs fixed in newer versions of lnd. This method can also be"]
        #[doc = "used to remove externally funded channels where the funding transaction was"]
        #[doc = "never broadcast. Only available for non-externally funded channels in dev"]
        #[doc = "build."]
        async fn abandon_channel(
            &self,
            request: tonic::Request<super::AbandonChannelRequest>,
        ) -> Result<tonic::Response<super::AbandonChannelResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SendPayment method."]
        type SendPaymentStream: futures_core::Stream<Item = Result<super::SendResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " lncli: `sendpayment`"]
        #[doc = "Deprecated, use routerrpc.SendPaymentV2. SendPayment dispatches a"]
        #[doc = "bi-directional streaming RPC for sending payments through the Lightning"]
        #[doc = "Network. A single RPC invocation creates a persistent bi-directional"]
        #[doc = "stream allowing clients to rapidly send payments through the Lightning"]
        #[doc = "Network with a single persistent connection."]
        async fn send_payment(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendRequest>>,
        ) -> Result<tonic::Response<Self::SendPaymentStream>, tonic::Status>;
        #[doc = ""]
        #[doc = "SendPaymentSync is the synchronous non-streaming version of SendPayment."]
        #[doc = "This RPC is intended to be consumed by clients of the REST proxy."]
        #[doc = "Additionally, this RPC expects the destination's public key and the payment"]
        #[doc = "hash (if any) to be encoded as hex strings."]
        async fn send_payment_sync(
            &self,
            request: tonic::Request<super::SendRequest>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SendToRoute method."]
        type SendToRouteStream: futures_core::Stream<Item = Result<super::SendResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " lncli: `sendtoroute`"]
        #[doc = "Deprecated, use routerrpc.SendToRouteV2. SendToRoute is a bi-directional"]
        #[doc = "streaming RPC for sending payment through the Lightning Network. This"]
        #[doc = "method differs from SendPayment in that it allows users to specify a full"]
        #[doc = "route manually. This can be used for things like rebalancing, and atomic"]
        #[doc = "swaps."]
        async fn send_to_route(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendToRouteRequest>>,
        ) -> Result<tonic::Response<Self::SendToRouteStream>, tonic::Status>;
        #[doc = ""]
        #[doc = "SendToRouteSync is a synchronous version of SendToRoute. It Will block"]
        #[doc = "until the payment either fails or succeeds."]
        async fn send_to_route_sync(
            &self,
            request: tonic::Request<super::SendToRouteRequest>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status>;
        #[doc = " lncli: `addinvoice`"]
        #[doc = "AddInvoice attempts to add a new invoice to the invoice database. Any"]
        #[doc = "duplicated invoices are rejected, therefore all invoices *must* have a"]
        #[doc = "unique payment preimage."]
        async fn add_invoice(
            &self,
            request: tonic::Request<super::Invoice>,
        ) -> Result<tonic::Response<super::AddInvoiceResponse>, tonic::Status>;
        #[doc = " lncli: `listinvoices`"]
        #[doc = "ListInvoices returns a list of all the invoices currently stored within the"]
        #[doc = "database. Any active debug invoices are ignored. It has full support for"]
        #[doc = "paginated responses, allowing users to query for specific invoices through"]
        #[doc = "their add_index. This can be done by using either the first_index_offset or"]
        #[doc = "last_index_offset fields included in the response as the index_offset of the"]
        #[doc = "next request. By default, the first 100 invoices created will be returned."]
        #[doc = "Backwards pagination is also supported through the Reversed flag."]
        async fn list_invoices(
            &self,
            request: tonic::Request<super::ListInvoiceRequest>,
        ) -> Result<tonic::Response<super::ListInvoiceResponse>, tonic::Status>;
        #[doc = " lncli: `lookupinvoice`"]
        #[doc = "LookupInvoice attempts to look up an invoice according to its payment hash."]
        #[doc = "The passed payment hash *must* be exactly 32 bytes, if not, an error is"]
        #[doc = "returned."]
        async fn lookup_invoice(
            &self,
            request: tonic::Request<super::PaymentHash>,
        ) -> Result<tonic::Response<super::Invoice>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeInvoices method."]
        type SubscribeInvoicesStream: futures_core::Stream<Item = Result<super::Invoice, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "SubscribeInvoices returns a uni-directional stream (server -> client) for"]
        #[doc = "notifying the client of newly added/settled invoices. The caller can"]
        #[doc = "optionally specify the add_index and/or the settle_index. If the add_index"]
        #[doc = "is specified, then we'll first start by sending add invoice events for all"]
        #[doc = "invoices with an add_index greater than the specified value. If the"]
        #[doc = "settle_index is specified, the next, we'll send out all settle events for"]
        #[doc = "invoices with a settle_index greater than the specified value. One or both"]
        #[doc = "of these fields can be set. If no fields are set, then we'll only send out"]
        #[doc = "the latest add/settle events."]
        async fn subscribe_invoices(
            &self,
            request: tonic::Request<super::InvoiceSubscription>,
        ) -> Result<tonic::Response<Self::SubscribeInvoicesStream>, tonic::Status>;
        #[doc = " lncli: `decodepayreq`"]
        #[doc = "DecodePayReq takes an encoded payment request string and attempts to decode"]
        #[doc = "it, returning a full description of the conditions encoded within the"]
        #[doc = "payment request."]
        async fn decode_pay_req(
            &self,
            request: tonic::Request<super::PayReqString>,
        ) -> Result<tonic::Response<super::PayReq>, tonic::Status>;
        #[doc = " lncli: `listpayments`"]
        #[doc = "ListPayments returns a list of all outgoing payments."]
        async fn list_payments(
            &self,
            request: tonic::Request<super::ListPaymentsRequest>,
        ) -> Result<tonic::Response<super::ListPaymentsResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "DeletePayment deletes an outgoing payment from DB. Note that it will not"]
        #[doc = "attempt to delete an In-Flight payment, since that would be unsafe."]
        async fn delete_payment(
            &self,
            request: tonic::Request<super::DeletePaymentRequest>,
        ) -> Result<tonic::Response<super::DeletePaymentResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "DeleteAllPayments deletes all outgoing payments from DB. Note that it will"]
        #[doc = "not attempt to delete In-Flight payments, since that would be unsafe."]
        async fn delete_all_payments(
            &self,
            request: tonic::Request<super::DeleteAllPaymentsRequest>,
        ) -> Result<tonic::Response<super::DeleteAllPaymentsResponse>, tonic::Status>;
        #[doc = " lncli: `describegraph`"]
        #[doc = "DescribeGraph returns a description of the latest graph state from the"]
        #[doc = "point of view of the node. The graph information is partitioned into two"]
        #[doc = "components: all the nodes/vertexes, and all the edges that connect the"]
        #[doc = "vertexes themselves. As this is a directed graph, the edges also contain"]
        #[doc = "the node directional specific routing policy which includes: the time lock"]
        #[doc = "delta, fee information, etc."]
        async fn describe_graph(
            &self,
            request: tonic::Request<super::ChannelGraphRequest>,
        ) -> Result<tonic::Response<super::ChannelGraph>, tonic::Status>;
        #[doc = " lncli: `getnodemetrics`"]
        #[doc = "GetNodeMetrics returns node metrics calculated from the graph. Currently"]
        #[doc = "the only supported metric is betweenness centrality of individual nodes."]
        async fn get_node_metrics(
            &self,
            request: tonic::Request<super::NodeMetricsRequest>,
        ) -> Result<tonic::Response<super::NodeMetricsResponse>, tonic::Status>;
        #[doc = " lncli: `getchaninfo`"]
        #[doc = "GetChanInfo returns the latest authenticated network announcement for the"]
        #[doc = "given channel identified by its channel ID: an 8-byte integer which"]
        #[doc = "uniquely identifies the location of transaction's funding output within the"]
        #[doc = "blockchain."]
        async fn get_chan_info(
            &self,
            request: tonic::Request<super::ChanInfoRequest>,
        ) -> Result<tonic::Response<super::ChannelEdge>, tonic::Status>;
        #[doc = " lncli: `getnodeinfo`"]
        #[doc = "GetNodeInfo returns the latest advertised, aggregated, and authenticated"]
        #[doc = "channel information for the specified node identified by its public key."]
        async fn get_node_info(
            &self,
            request: tonic::Request<super::NodeInfoRequest>,
        ) -> Result<tonic::Response<super::NodeInfo>, tonic::Status>;
        #[doc = " lncli: `queryroutes`"]
        #[doc = "QueryRoutes attempts to query the daemon's Channel Router for a possible"]
        #[doc = "route to a target destination capable of carrying a specific amount of"]
        #[doc = "satoshis. The returned route contains the full details required to craft and"]
        #[doc = "send an HTLC, also including the necessary information that should be"]
        #[doc = "present within the Sphinx packet encapsulated within the HTLC."]
        #[doc = ""]
        #[doc = "When using REST, the `dest_custom_records` map type can be set by appending"]
        #[doc = "`&dest_custom_records[<record_number>]=<record_data_base64_url_encoded>`"]
        #[doc = "to the URL. Unfortunately this map type doesn't appear in the REST API"]
        #[doc = "documentation because of a bug in the grpc-gateway library."]
        async fn query_routes(
            &self,
            request: tonic::Request<super::QueryRoutesRequest>,
        ) -> Result<tonic::Response<super::QueryRoutesResponse>, tonic::Status>;
        #[doc = " lncli: `getnetworkinfo`"]
        #[doc = "GetNetworkInfo returns some basic stats about the known channel graph from"]
        #[doc = "the point of view of the node."]
        async fn get_network_info(
            &self,
            request: tonic::Request<super::NetworkInfoRequest>,
        ) -> Result<tonic::Response<super::NetworkInfo>, tonic::Status>;
        #[doc = " lncli: `stop`"]
        #[doc = "StopDaemon will send a shutdown request to the interrupt handler, triggering"]
        #[doc = "a graceful shutdown of the daemon."]
        async fn stop_daemon(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeChannelGraph method."]
        type SubscribeChannelGraphStream: futures_core::Stream<Item = Result<super::GraphTopologyUpdate, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "SubscribeChannelGraph launches a streaming RPC that allows the caller to"]
        #[doc = "receive notifications upon any changes to the channel graph topology from"]
        #[doc = "the point of view of the responding node. Events notified include: new"]
        #[doc = "nodes coming online, nodes updating their authenticated attributes, new"]
        #[doc = "channels being advertised, updates in the routing policy for a directional"]
        #[doc = "channel edge, and when channels are closed on-chain."]
        async fn subscribe_channel_graph(
            &self,
            request: tonic::Request<super::GraphTopologySubscription>,
        ) -> Result<tonic::Response<Self::SubscribeChannelGraphStream>, tonic::Status>;
        #[doc = " lncli: `debuglevel`"]
        #[doc = "DebugLevel allows a caller to programmatically set the logging verbosity of"]
        #[doc = "lnd. The logging can be targeted according to a coarse daemon-wide logging"]
        #[doc = "level, or in a granular fashion to specify the logging for a target"]
        #[doc = "sub-system."]
        async fn debug_level(
            &self,
            request: tonic::Request<super::DebugLevelRequest>,
        ) -> Result<tonic::Response<super::DebugLevelResponse>, tonic::Status>;
        #[doc = " lncli: `feereport`"]
        #[doc = "FeeReport allows the caller to obtain a report detailing the current fee"]
        #[doc = "schedule enforced by the node globally for each channel."]
        async fn fee_report(
            &self,
            request: tonic::Request<super::FeeReportRequest>,
        ) -> Result<tonic::Response<super::FeeReportResponse>, tonic::Status>;
        #[doc = " lncli: `updatechanpolicy`"]
        #[doc = "UpdateChannelPolicy allows the caller to update the fee schedule and"]
        #[doc = "channel policies for all channels globally, or a particular channel."]
        async fn update_channel_policy(
            &self,
            request: tonic::Request<super::PolicyUpdateRequest>,
        ) -> Result<tonic::Response<super::PolicyUpdateResponse>, tonic::Status>;
        #[doc = " lncli: `fwdinghistory`"]
        #[doc = "ForwardingHistory allows the caller to query the htlcswitch for a record of"]
        #[doc = "all HTLCs forwarded within the target time range, and integer offset"]
        #[doc = "within that time range, for a maximum number of events. If no maximum number"]
        #[doc = "of events is specified, up to 100 events will be returned. If no time-range"]
        #[doc = "is specified, then events will be returned in the order that they occured."]
        #[doc = ""]
        #[doc = "A list of forwarding events are returned. The size of each forwarding event"]
        #[doc = "is 40 bytes, and the max message size able to be returned in gRPC is 4 MiB."]
        #[doc = "As a result each message can only contain 50k entries. Each response has"]
        #[doc = "the index offset of the last entry. The index offset can be provided to the"]
        #[doc = "request to allow the caller to skip a series of records."]
        async fn forwarding_history(
            &self,
            request: tonic::Request<super::ForwardingHistoryRequest>,
        ) -> Result<tonic::Response<super::ForwardingHistoryResponse>, tonic::Status>;
        #[doc = " lncli: `exportchanbackup`"]
        #[doc = "ExportChannelBackup attempts to return an encrypted static channel backup"]
        #[doc = "for the target channel identified by it channel point. The backup is"]
        #[doc = "encrypted with a key generated from the aezeed seed of the user. The"]
        #[doc = "returned backup can either be restored using the RestoreChannelBackup"]
        #[doc = "method once lnd is running, or via the InitWallet and UnlockWallet methods"]
        #[doc = "from the WalletUnlocker service."]
        async fn export_channel_backup(
            &self,
            request: tonic::Request<super::ExportChannelBackupRequest>,
        ) -> Result<tonic::Response<super::ChannelBackup>, tonic::Status>;
        #[doc = ""]
        #[doc = "ExportAllChannelBackups returns static channel backups for all existing"]
        #[doc = "channels known to lnd. A set of regular singular static channel backups for"]
        #[doc = "each channel are returned. Additionally, a multi-channel backup is returned"]
        #[doc = "as well, which contains a single encrypted blob containing the backups of"]
        #[doc = "each channel."]
        async fn export_all_channel_backups(
            &self,
            request: tonic::Request<super::ChanBackupExportRequest>,
        ) -> Result<tonic::Response<super::ChanBackupSnapshot>, tonic::Status>;
        #[doc = ""]
        #[doc = "VerifyChanBackup allows a caller to verify the integrity of a channel backup"]
        #[doc = "snapshot. This method will accept either a packed Single or a packed Multi."]
        #[doc = "Specifying both will result in an error."]
        async fn verify_chan_backup(
            &self,
            request: tonic::Request<super::ChanBackupSnapshot>,
        ) -> Result<tonic::Response<super::VerifyChanBackupResponse>, tonic::Status>;
        #[doc = " lncli: `restorechanbackup`"]
        #[doc = "RestoreChannelBackups accepts a set of singular channel backups, or a"]
        #[doc = "single encrypted multi-chan backup and attempts to recover any funds"]
        #[doc = "remaining within the channel. If we are able to unpack the backup, then the"]
        #[doc = "new channel will be shown under listchannels, as well as pending channels."]
        async fn restore_channel_backups(
            &self,
            request: tonic::Request<super::RestoreChanBackupRequest>,
        ) -> Result<tonic::Response<super::RestoreBackupResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeChannelBackups method."]
        type SubscribeChannelBackupsStream: futures_core::Stream<Item = Result<super::ChanBackupSnapshot, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "SubscribeChannelBackups allows a client to sub-subscribe to the most up to"]
        #[doc = "date information concerning the state of all channel backups. Each time a"]
        #[doc = "new channel is added, we return the new set of channels, along with a"]
        #[doc = "multi-chan backup containing the backup info for all channels. Each time a"]
        #[doc = "channel is closed, we send a new update, which contains new new chan back"]
        #[doc = "ups, but the updated set of encrypted multi-chan backups with the closed"]
        #[doc = "channel(s) removed."]
        async fn subscribe_channel_backups(
            &self,
            request: tonic::Request<super::ChannelBackupSubscription>,
        ) -> Result<tonic::Response<Self::SubscribeChannelBackupsStream>, tonic::Status>;
        #[doc = " lncli: `bakemacaroon`"]
        #[doc = "BakeMacaroon allows the creation of a new macaroon with custom read and"]
        #[doc = "write permissions. No first-party caveats are added since this can be done"]
        #[doc = "offline."]
        async fn bake_macaroon(
            &self,
            request: tonic::Request<super::BakeMacaroonRequest>,
        ) -> Result<tonic::Response<super::BakeMacaroonResponse>, tonic::Status>;
        #[doc = " lncli: `listmacaroonids`"]
        #[doc = "ListMacaroonIDs returns all root key IDs that are in use."]
        async fn list_macaroon_i_ds(
            &self,
            request: tonic::Request<super::ListMacaroonIDsRequest>,
        ) -> Result<tonic::Response<super::ListMacaroonIDsResponse>, tonic::Status>;
        #[doc = " lncli: `deletemacaroonid`"]
        #[doc = "DeleteMacaroonID deletes the specified macaroon ID and invalidates all"]
        #[doc = "macaroons derived from that ID."]
        async fn delete_macaroon_id(
            &self,
            request: tonic::Request<super::DeleteMacaroonIdRequest>,
        ) -> Result<tonic::Response<super::DeleteMacaroonIdResponse>, tonic::Status>;
        #[doc = " lncli: `listpermissions`"]
        #[doc = "ListPermissions lists all RPC method URIs and their required macaroon"]
        #[doc = "permissions to access them."]
        async fn list_permissions(
            &self,
            request: tonic::Request<super::ListPermissionsRequest>,
        ) -> Result<tonic::Response<super::ListPermissionsResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "CheckMacaroonPermissions checks whether a request follows the constraints"]
        #[doc = "imposed on the macaroon and that the macaroon is authorized to follow the"]
        #[doc = "provided permissions."]
        async fn check_macaroon_permissions(
            &self,
            request: tonic::Request<super::CheckMacPermRequest>,
        ) -> Result<tonic::Response<super::CheckMacPermResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the RegisterRPCMiddleware method."]
        type RegisterRPCMiddlewareStream: futures_core::Stream<Item = Result<super::RpcMiddlewareRequest, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = "RegisterRPCMiddleware adds a new gRPC middleware to the interceptor chain. A"]
        #[doc = "gRPC middleware is software component external to lnd that aims to add"]
        #[doc = "additional business logic to lnd by observing/intercepting/validating"]
        #[doc = "incoming gRPC client requests and (if needed) replacing/overwriting outgoing"]
        #[doc = "messages before they're sent to the client. When registering the middleware"]
        #[doc = "must identify itself and indicate what custom macaroon caveats it wants to"]
        #[doc = "be responsible for. Only requests that contain a macaroon with that specific"]
        #[doc = "custom caveat are then sent to the middleware for inspection. The other"]
        #[doc = "option is to register for the read-only mode in which all requests/responses"]
        #[doc = "are forwarded for interception to the middleware but the middleware is not"]
        #[doc = "allowed to modify any responses. As a security measure, _no_ middleware can"]
        #[doc = "modify responses for requests made with _unencumbered_ macaroons!"]
        async fn register_rpc_middleware(
            &self,
            request: tonic::Request<tonic::Streaming<super::RpcMiddlewareResponse>>,
        ) -> Result<tonic::Response<Self::RegisterRPCMiddlewareStream>, tonic::Status>;
        #[doc = " lncli: `sendcustom`"]
        #[doc = "SendCustomMessage sends a custom peer message."]
        async fn send_custom_message(
            &self,
            request: tonic::Request<super::SendCustomMessageRequest>,
        ) -> Result<tonic::Response<super::SendCustomMessageResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeCustomMessages method."]
        type SubscribeCustomMessagesStream: futures_core::Stream<Item = Result<super::CustomMessage, tonic::Status>>
            + Send
            + 'static;
        #[doc = " lncli: `subscribecustom`"]
        #[doc = "SubscribeCustomMessages subscribes to a stream of incoming custom peer"]
        #[doc = "messages."]
        async fn subscribe_custom_messages(
            &self,
            request: tonic::Request<super::SubscribeCustomMessagesRequest>,
        ) -> Result<tonic::Response<Self::SubscribeCustomMessagesStream>, tonic::Status>;
    }
    #[doc = " Lightning is the main RPC server of the daemon."]
    #[derive(Debug)]
    pub struct LightningServer<T: Lightning> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Lightning> LightningServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LightningServer<T>
    where
        T: Lightning,
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
                "/lnrpc.Lightning/WalletBalance" => {
                    #[allow(non_camel_case_types)]
                    struct WalletBalanceSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::WalletBalanceRequest>
                        for WalletBalanceSvc<T>
                    {
                        type Response = super::WalletBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WalletBalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).wallet_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WalletBalanceSvc(inner);
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
                "/lnrpc.Lightning/ChannelBalance" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelBalanceSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ChannelBalanceRequest>
                        for ChannelBalanceSvc<T>
                    {
                        type Response = super::ChannelBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelBalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).channel_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelBalanceSvc(inner);
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
                "/lnrpc.Lightning/GetTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::GetTransactionsRequest>
                        for GetTransactionsSvc<T>
                    {
                        type Response = super::TransactionDetails;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transactions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsSvc(inner);
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
                "/lnrpc.Lightning/EstimateFee" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateFeeSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::EstimateFeeRequest> for EstimateFeeSvc<T> {
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
                "/lnrpc.Lightning/SendCoins" => {
                    #[allow(non_camel_case_types)]
                    struct SendCoinsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::SendCoinsRequest> for SendCoinsSvc<T> {
                        type Response = super::SendCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendCoinsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_coins(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendCoinsSvc(inner);
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
                "/lnrpc.Lightning/ListUnspent" => {
                    #[allow(non_camel_case_types)]
                    struct ListUnspentSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ListUnspentRequest> for ListUnspentSvc<T> {
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
                "/lnrpc.Lightning/SubscribeTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeTransactionsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::GetTransactionsRequest>
                        for SubscribeTransactionsSvc<T>
                    {
                        type Response = super::Transaction;
                        type ResponseStream = T::SubscribeTransactionsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_transactions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeTransactionsSvc(inner);
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
                "/lnrpc.Lightning/SendMany" => {
                    #[allow(non_camel_case_types)]
                    struct SendManySvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::SendManyRequest> for SendManySvc<T> {
                        type Response = super::SendManyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendManyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_many(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendManySvc(inner);
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
                "/lnrpc.Lightning/NewAddress" => {
                    #[allow(non_camel_case_types)]
                    struct NewAddressSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::NewAddressRequest> for NewAddressSvc<T> {
                        type Response = super::NewAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewAddressRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_address(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAddressSvc(inner);
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
                "/lnrpc.Lightning/SignMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SignMessageSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::SignMessageRequest> for SignMessageSvc<T> {
                        type Response = super::SignMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignMessageRequest>,
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
                "/lnrpc.Lightning/VerifyMessage" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyMessageSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::VerifyMessageRequest>
                        for VerifyMessageSvc<T>
                    {
                        type Response = super::VerifyMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyMessageRequest>,
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
                "/lnrpc.Lightning/ConnectPeer" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectPeerSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ConnectPeerRequest> for ConnectPeerSvc<T> {
                        type Response = super::ConnectPeerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectPeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).connect_peer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectPeerSvc(inner);
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
                "/lnrpc.Lightning/DisconnectPeer" => {
                    #[allow(non_camel_case_types)]
                    struct DisconnectPeerSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::DisconnectPeerRequest>
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
                "/lnrpc.Lightning/ListPeers" => {
                    #[allow(non_camel_case_types)]
                    struct ListPeersSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ListPeersRequest> for ListPeersSvc<T> {
                        type Response = super::ListPeersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPeersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_peers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPeersSvc(inner);
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
                "/lnrpc.Lightning/SubscribePeerEvents" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribePeerEventsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::PeerEventSubscription>
                        for SubscribePeerEventsSvc<T>
                    {
                        type Response = super::PeerEvent;
                        type ResponseStream = T::SubscribePeerEventsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PeerEventSubscription>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_peer_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribePeerEventsSvc(inner);
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
                "/lnrpc.Lightning/GetInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::GetInfoRequest> for GetInfoSvc<T> {
                        type Response = super::GetInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoSvc(inner);
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
                "/lnrpc.Lightning/GetRecoveryInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetRecoveryInfoSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::GetRecoveryInfoRequest>
                        for GetRecoveryInfoSvc<T>
                    {
                        type Response = super::GetRecoveryInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRecoveryInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_recovery_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRecoveryInfoSvc(inner);
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
                "/lnrpc.Lightning/PendingChannels" => {
                    #[allow(non_camel_case_types)]
                    struct PendingChannelsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::PendingChannelsRequest>
                        for PendingChannelsSvc<T>
                    {
                        type Response = super::PendingChannelsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PendingChannelsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pending_channels(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PendingChannelsSvc(inner);
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
                "/lnrpc.Lightning/ListChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ListChannelsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ListChannelsRequest> for ListChannelsSvc<T> {
                        type Response = super::ListChannelsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListChannelsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_channels(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListChannelsSvc(inner);
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
                "/lnrpc.Lightning/SubscribeChannelEvents" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeChannelEventsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::ChannelEventSubscription>
                        for SubscribeChannelEventsSvc<T>
                    {
                        type Response = super::ChannelEventUpdate;
                        type ResponseStream = T::SubscribeChannelEventsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelEventSubscription>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_channel_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeChannelEventsSvc(inner);
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
                "/lnrpc.Lightning/ClosedChannels" => {
                    #[allow(non_camel_case_types)]
                    struct ClosedChannelsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ClosedChannelsRequest>
                        for ClosedChannelsSvc<T>
                    {
                        type Response = super::ClosedChannelsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClosedChannelsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).closed_channels(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClosedChannelsSvc(inner);
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
                "/lnrpc.Lightning/OpenChannelSync" => {
                    #[allow(non_camel_case_types)]
                    struct OpenChannelSyncSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::OpenChannelRequest>
                        for OpenChannelSyncSvc<T>
                    {
                        type Response = super::ChannelPoint;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenChannelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).open_channel_sync(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenChannelSyncSvc(inner);
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
                "/lnrpc.Lightning/OpenChannel" => {
                    #[allow(non_camel_case_types)]
                    struct OpenChannelSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::OpenChannelRequest>
                        for OpenChannelSvc<T>
                    {
                        type Response = super::OpenStatusUpdate;
                        type ResponseStream = T::OpenChannelStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenChannelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).open_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenChannelSvc(inner);
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
                "/lnrpc.Lightning/BatchOpenChannel" => {
                    #[allow(non_camel_case_types)]
                    struct BatchOpenChannelSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::BatchOpenChannelRequest>
                        for BatchOpenChannelSvc<T>
                    {
                        type Response = super::BatchOpenChannelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchOpenChannelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).batch_open_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchOpenChannelSvc(inner);
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
                "/lnrpc.Lightning/FundingStateStep" => {
                    #[allow(non_camel_case_types)]
                    struct FundingStateStepSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::FundingTransitionMsg>
                        for FundingStateStepSvc<T>
                    {
                        type Response = super::FundingStateStepResp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FundingTransitionMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).funding_state_step(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FundingStateStepSvc(inner);
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
                "/lnrpc.Lightning/ChannelAcceptor" => {
                    #[allow(non_camel_case_types)]
                    struct ChannelAcceptorSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::StreamingService<super::ChannelAcceptResponse>
                        for ChannelAcceptorSvc<T>
                    {
                        type Response = super::ChannelAcceptRequest;
                        type ResponseStream = T::ChannelAcceptorStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ChannelAcceptResponse>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).channel_acceptor(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChannelAcceptorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lnrpc.Lightning/CloseChannel" => {
                    #[allow(non_camel_case_types)]
                    struct CloseChannelSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::CloseChannelRequest>
                        for CloseChannelSvc<T>
                    {
                        type Response = super::CloseStatusUpdate;
                        type ResponseStream = T::CloseChannelStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloseChannelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).close_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseChannelSvc(inner);
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
                "/lnrpc.Lightning/AbandonChannel" => {
                    #[allow(non_camel_case_types)]
                    struct AbandonChannelSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::AbandonChannelRequest>
                        for AbandonChannelSvc<T>
                    {
                        type Response = super::AbandonChannelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AbandonChannelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).abandon_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AbandonChannelSvc(inner);
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
                "/lnrpc.Lightning/SendPayment" => {
                    #[allow(non_camel_case_types)]
                    struct SendPaymentSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::StreamingService<super::SendRequest> for SendPaymentSvc<T> {
                        type Response = super::SendResponse;
                        type ResponseStream = T::SendPaymentStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::SendRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_payment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPaymentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lnrpc.Lightning/SendPaymentSync" => {
                    #[allow(non_camel_case_types)]
                    struct SendPaymentSyncSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::SendRequest> for SendPaymentSyncSvc<T> {
                        type Response = super::SendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_payment_sync(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPaymentSyncSvc(inner);
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
                "/lnrpc.Lightning/SendToRoute" => {
                    #[allow(non_camel_case_types)]
                    struct SendToRouteSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::StreamingService<super::SendToRouteRequest>
                        for SendToRouteSvc<T>
                    {
                        type Response = super::SendResponse;
                        type ResponseStream = T::SendToRouteStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::SendToRouteRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_to_route(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendToRouteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lnrpc.Lightning/SendToRouteSync" => {
                    #[allow(non_camel_case_types)]
                    struct SendToRouteSyncSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::SendToRouteRequest>
                        for SendToRouteSyncSvc<T>
                    {
                        type Response = super::SendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendToRouteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_to_route_sync(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendToRouteSyncSvc(inner);
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
                "/lnrpc.Lightning/AddInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct AddInvoiceSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::Invoice> for AddInvoiceSvc<T> {
                        type Response = super::AddInvoiceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Invoice>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_invoice(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddInvoiceSvc(inner);
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
                "/lnrpc.Lightning/ListInvoices" => {
                    #[allow(non_camel_case_types)]
                    struct ListInvoicesSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ListInvoiceRequest> for ListInvoicesSvc<T> {
                        type Response = super::ListInvoiceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInvoiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_invoices(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListInvoicesSvc(inner);
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
                "/lnrpc.Lightning/LookupInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct LookupInvoiceSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::PaymentHash> for LookupInvoiceSvc<T> {
                        type Response = super::Invoice;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PaymentHash>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).lookup_invoice(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LookupInvoiceSvc(inner);
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
                "/lnrpc.Lightning/SubscribeInvoices" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeInvoicesSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::InvoiceSubscription>
                        for SubscribeInvoicesSvc<T>
                    {
                        type Response = super::Invoice;
                        type ResponseStream = T::SubscribeInvoicesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InvoiceSubscription>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_invoices(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeInvoicesSvc(inner);
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
                "/lnrpc.Lightning/DecodePayReq" => {
                    #[allow(non_camel_case_types)]
                    struct DecodePayReqSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::PayReqString> for DecodePayReqSvc<T> {
                        type Response = super::PayReq;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PayReqString>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).decode_pay_req(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DecodePayReqSvc(inner);
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
                "/lnrpc.Lightning/ListPayments" => {
                    #[allow(non_camel_case_types)]
                    struct ListPaymentsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ListPaymentsRequest> for ListPaymentsSvc<T> {
                        type Response = super::ListPaymentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPaymentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_payments(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPaymentsSvc(inner);
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
                "/lnrpc.Lightning/DeletePayment" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePaymentSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::DeletePaymentRequest>
                        for DeletePaymentSvc<T>
                    {
                        type Response = super::DeletePaymentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePaymentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_payment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePaymentSvc(inner);
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
                "/lnrpc.Lightning/DeleteAllPayments" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAllPaymentsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::DeleteAllPaymentsRequest>
                        for DeleteAllPaymentsSvc<T>
                    {
                        type Response = super::DeleteAllPaymentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAllPaymentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_all_payments(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAllPaymentsSvc(inner);
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
                "/lnrpc.Lightning/DescribeGraph" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeGraphSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ChannelGraphRequest> for DescribeGraphSvc<T> {
                        type Response = super::ChannelGraph;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelGraphRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).describe_graph(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DescribeGraphSvc(inner);
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
                "/lnrpc.Lightning/GetNodeMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeMetricsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::NodeMetricsRequest> for GetNodeMetricsSvc<T> {
                        type Response = super::NodeMetricsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NodeMetricsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_node_metrics(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNodeMetricsSvc(inner);
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
                "/lnrpc.Lightning/GetChanInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetChanInfoSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ChanInfoRequest> for GetChanInfoSvc<T> {
                        type Response = super::ChannelEdge;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChanInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_chan_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetChanInfoSvc(inner);
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
                "/lnrpc.Lightning/GetNodeInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeInfoSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::NodeInfoRequest> for GetNodeInfoSvc<T> {
                        type Response = super::NodeInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NodeInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_node_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNodeInfoSvc(inner);
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
                "/lnrpc.Lightning/QueryRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct QueryRoutesSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::QueryRoutesRequest> for QueryRoutesSvc<T> {
                        type Response = super::QueryRoutesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRoutesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_routes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryRoutesSvc(inner);
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
                "/lnrpc.Lightning/GetNetworkInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetNetworkInfoSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::NetworkInfoRequest> for GetNetworkInfoSvc<T> {
                        type Response = super::NetworkInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NetworkInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_network_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNetworkInfoSvc(inner);
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
                "/lnrpc.Lightning/StopDaemon" => {
                    #[allow(non_camel_case_types)]
                    struct StopDaemonSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::StopRequest> for StopDaemonSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_daemon(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopDaemonSvc(inner);
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
                "/lnrpc.Lightning/SubscribeChannelGraph" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeChannelGraphSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::GraphTopologySubscription>
                        for SubscribeChannelGraphSvc<T>
                    {
                        type Response = super::GraphTopologyUpdate;
                        type ResponseStream = T::SubscribeChannelGraphStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GraphTopologySubscription>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_channel_graph(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeChannelGraphSvc(inner);
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
                "/lnrpc.Lightning/DebugLevel" => {
                    #[allow(non_camel_case_types)]
                    struct DebugLevelSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::DebugLevelRequest> for DebugLevelSvc<T> {
                        type Response = super::DebugLevelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DebugLevelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).debug_level(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DebugLevelSvc(inner);
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
                "/lnrpc.Lightning/FeeReport" => {
                    #[allow(non_camel_case_types)]
                    struct FeeReportSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::FeeReportRequest> for FeeReportSvc<T> {
                        type Response = super::FeeReportResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FeeReportRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fee_report(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FeeReportSvc(inner);
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
                "/lnrpc.Lightning/UpdateChannelPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateChannelPolicySvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::PolicyUpdateRequest>
                        for UpdateChannelPolicySvc<T>
                    {
                        type Response = super::PolicyUpdateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyUpdateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_channel_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateChannelPolicySvc(inner);
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
                "/lnrpc.Lightning/ForwardingHistory" => {
                    #[allow(non_camel_case_types)]
                    struct ForwardingHistorySvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ForwardingHistoryRequest>
                        for ForwardingHistorySvc<T>
                    {
                        type Response = super::ForwardingHistoryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ForwardingHistoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).forwarding_history(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ForwardingHistorySvc(inner);
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
                "/lnrpc.Lightning/ExportChannelBackup" => {
                    #[allow(non_camel_case_types)]
                    struct ExportChannelBackupSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::UnaryService<super::ExportChannelBackupRequest>
                        for ExportChannelBackupSvc<T>
                    {
                        type Response = super::ChannelBackup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportChannelBackupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).export_channel_backup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExportChannelBackupSvc(inner);
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
                "/lnrpc.Lightning/ExportAllChannelBackups" => {
                    #[allow(non_camel_case_types)]
                    struct ExportAllChannelBackupsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ChanBackupExportRequest>
                        for ExportAllChannelBackupsSvc<T>
                    {
                        type Response = super::ChanBackupSnapshot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChanBackupExportRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).export_all_channel_backups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExportAllChannelBackupsSvc(inner);
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
                "/lnrpc.Lightning/VerifyChanBackup" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyChanBackupSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ChanBackupSnapshot>
                        for VerifyChanBackupSvc<T>
                    {
                        type Response = super::VerifyChanBackupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChanBackupSnapshot>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).verify_chan_backup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VerifyChanBackupSvc(inner);
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
                "/lnrpc.Lightning/RestoreChannelBackups" => {
                    #[allow(non_camel_case_types)]
                    struct RestoreChannelBackupsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::RestoreChanBackupRequest>
                        for RestoreChannelBackupsSvc<T>
                    {
                        type Response = super::RestoreBackupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestoreChanBackupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).restore_channel_backups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RestoreChannelBackupsSvc(inner);
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
                "/lnrpc.Lightning/SubscribeChannelBackups" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeChannelBackupsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::ChannelBackupSubscription>
                        for SubscribeChannelBackupsSvc<T>
                    {
                        type Response = super::ChanBackupSnapshot;
                        type ResponseStream = T::SubscribeChannelBackupsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelBackupSubscription>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_channel_backups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeChannelBackupsSvc(inner);
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
                "/lnrpc.Lightning/BakeMacaroon" => {
                    #[allow(non_camel_case_types)]
                    struct BakeMacaroonSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::BakeMacaroonRequest> for BakeMacaroonSvc<T> {
                        type Response = super::BakeMacaroonResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BakeMacaroonRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).bake_macaroon(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BakeMacaroonSvc(inner);
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
                "/lnrpc.Lightning/ListMacaroonIDs" => {
                    #[allow(non_camel_case_types)]
                    struct ListMacaroonIDsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ListMacaroonIDsRequest>
                        for ListMacaroonIDsSvc<T>
                    {
                        type Response = super::ListMacaroonIDsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMacaroonIDsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_macaroon_i_ds(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListMacaroonIDsSvc(inner);
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
                "/lnrpc.Lightning/DeleteMacaroonID" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMacaroonIDSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::DeleteMacaroonIdRequest>
                        for DeleteMacaroonIDSvc<T>
                    {
                        type Response = super::DeleteMacaroonIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMacaroonIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_macaroon_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteMacaroonIDSvc(inner);
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
                "/lnrpc.Lightning/ListPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct ListPermissionsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::ListPermissionsRequest>
                        for ListPermissionsSvc<T>
                    {
                        type Response = super::ListPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPermissionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPermissionsSvc(inner);
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
                "/lnrpc.Lightning/CheckMacaroonPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct CheckMacaroonPermissionsSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::CheckMacPermRequest>
                        for CheckMacaroonPermissionsSvc<T>
                    {
                        type Response = super::CheckMacPermResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckMacPermRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).check_macaroon_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckMacaroonPermissionsSvc(inner);
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
                "/lnrpc.Lightning/RegisterRPCMiddleware" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterRPCMiddlewareSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::StreamingService<super::RpcMiddlewareResponse>
                        for RegisterRPCMiddlewareSvc<T>
                    {
                        type Response = super::RpcMiddlewareRequest;
                        type ResponseStream = T::RegisterRPCMiddlewareStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::RpcMiddlewareResponse>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).register_rpc_middleware(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterRPCMiddlewareSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lnrpc.Lightning/SendCustomMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SendCustomMessageSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning> tonic::server::UnaryService<super::SendCustomMessageRequest>
                        for SendCustomMessageSvc<T>
                    {
                        type Response = super::SendCustomMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendCustomMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_custom_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendCustomMessageSvc(inner);
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
                "/lnrpc.Lightning/SubscribeCustomMessages" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeCustomMessagesSvc<T: Lightning>(pub Arc<T>);
                    impl<T: Lightning>
                        tonic::server::ServerStreamingService<super::SubscribeCustomMessagesRequest>
                        for SubscribeCustomMessagesSvc<T>
                    {
                        type Response = super::CustomMessage;
                        type ResponseStream = T::SubscribeCustomMessagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeCustomMessagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_custom_messages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeCustomMessagesSvc(inner);
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
    impl<T: Lightning> Clone for LightningServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Lightning> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Lightning> tonic::transport::NamedService for LightningServer<T> {
        const NAME: &'static str = "lnrpc.Lightning";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeStateRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeStateResponse {
    #[prost(enumeration = "WalletState", tag = "1")]
    pub state: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStateRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStateResponse {
    #[prost(enumeration = "WalletState", tag = "1")]
    pub state: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WalletState {
    NonExisting = 0,
    Locked = 1,
    Unlocked = 2,
    RpcActive = 3,
    /// SERVER_ACTIVE means that the lnd server is ready to accept calls.
    ServerActive = 4,
    WaitingToStart = 255,
}
#[doc = r" Generated client implementations."]
pub mod state_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " State service is a always running service that exposes the current state of"]
    #[doc = " the wallet and RPC server."]
    #[derive(Debug, Clone)]
    pub struct StateClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StateClient<tonic::transport::Channel> {
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
    impl<T> StateClient<T>
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
        ) -> StateClient<InterceptedService<T, F>>
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
            StateClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " SubscribeState subscribes to the state of the wallet. The current wallet"]
        #[doc = " state will always be delivered immediately."]
        pub async fn subscribe_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SubscribeStateResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.State/SubscribeState");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " GetState returns the current wallet state without streaming further"]
        #[doc = " changes."]
        pub async fn get_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStateRequest>,
        ) -> Result<tonic::Response<super::GetStateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.State/GetState");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod state_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with StateServer."]
    #[async_trait]
    pub trait State: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SubscribeState method."]
        type SubscribeStateStream: futures_core::Stream<Item = Result<super::SubscribeStateResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " SubscribeState subscribes to the state of the wallet. The current wallet"]
        #[doc = " state will always be delivered immediately."]
        async fn subscribe_state(
            &self,
            request: tonic::Request<super::SubscribeStateRequest>,
        ) -> Result<tonic::Response<Self::SubscribeStateStream>, tonic::Status>;
        #[doc = " GetState returns the current wallet state without streaming further"]
        #[doc = " changes."]
        async fn get_state(
            &self,
            request: tonic::Request<super::GetStateRequest>,
        ) -> Result<tonic::Response<super::GetStateResponse>, tonic::Status>;
    }
    #[doc = " State service is a always running service that exposes the current state of"]
    #[doc = " the wallet and RPC server."]
    #[derive(Debug)]
    pub struct StateServer<T: State> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: State> StateServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StateServer<T>
    where
        T: State,
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
                "/lnrpc.State/SubscribeState" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeStateSvc<T: State>(pub Arc<T>);
                    impl<T: State>
                        tonic::server::ServerStreamingService<super::SubscribeStateRequest>
                        for SubscribeStateSvc<T>
                    {
                        type Response = super::SubscribeStateResponse;
                        type ResponseStream = T::SubscribeStateStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeStateSvc(inner);
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
                "/lnrpc.State/GetState" => {
                    #[allow(non_camel_case_types)]
                    struct GetStateSvc<T: State>(pub Arc<T>);
                    impl<T: State> tonic::server::UnaryService<super::GetStateRequest> for GetStateSvc<T> {
                        type Response = super::GetStateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStateSvc(inner);
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
    impl<T: State> Clone for StateServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: State> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: State> tonic::transport::NamedService for StateServer<T> {
        const NAME: &'static str = "lnrpc.State";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenSeedRequest {
    ///
    ///aezeed_passphrase is an optional user provided passphrase that will be used
    ///to encrypt the generated aezeed cipher seed. When using REST, this field
    ///must be encoded as base64.
    #[prost(bytes = "vec", tag = "1")]
    pub aezeed_passphrase: ::prost::alloc::vec::Vec<u8>,
    ///
    ///seed_entropy is an optional 16-bytes generated via CSPRNG. If not
    ///specified, then a fresh set of randomness will be used to create the seed.
    ///When using REST, this field must be encoded as base64.
    #[prost(bytes = "vec", tag = "2")]
    pub seed_entropy: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenSeedResponse {
    ///
    ///cipher_seed_mnemonic is a 24-word mnemonic that encodes a prior aezeed
    ///cipher seed obtained by the user. This field is optional, as if not
    ///provided, then the daemon will generate a new cipher seed for the user.
    ///Otherwise, then the daemon will attempt to recover the wallet state linked
    ///to this cipher seed.
    #[prost(string, repeated, tag = "1")]
    pub cipher_seed_mnemonic: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    ///enciphered_seed are the raw aezeed cipher seed bytes. This is the raw
    ///cipher text before run through our mnemonic encoding scheme.
    #[prost(bytes = "vec", tag = "2")]
    pub enciphered_seed: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitWalletRequest {
    ///
    ///wallet_password is the passphrase that should be used to encrypt the
    ///wallet. This MUST be at least 8 chars in length. After creation, this
    ///password is required to unlock the daemon. When using REST, this field
    ///must be encoded as base64.
    #[prost(bytes = "vec", tag = "1")]
    pub wallet_password: ::prost::alloc::vec::Vec<u8>,
    ///
    ///cipher_seed_mnemonic is a 24-word mnemonic that encodes a prior aezeed
    ///cipher seed obtained by the user. This may have been generated by the
    ///GenSeed method, or be an existing seed.
    #[prost(string, repeated, tag = "2")]
    pub cipher_seed_mnemonic: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    ///aezeed_passphrase is an optional user provided passphrase that will be used
    ///to encrypt the generated aezeed cipher seed. When using REST, this field
    ///must be encoded as base64.
    #[prost(bytes = "vec", tag = "3")]
    pub aezeed_passphrase: ::prost::alloc::vec::Vec<u8>,
    ///
    ///recovery_window is an optional argument specifying the address lookahead
    ///when restoring a wallet seed. The recovery window applies to each
    ///individual branch of the BIP44 derivation paths. Supplying a recovery
    ///window of zero indicates that no addresses should be recovered, such after
    ///the first initialization of the wallet.
    #[prost(int32, tag = "4")]
    pub recovery_window: i32,
    ///
    ///channel_backups is an optional argument that allows clients to recover the
    ///settled funds within a set of channels. This should be populated if the
    ///user was unable to close out all channels and sweep funds before partial or
    ///total data loss occurred. If specified, then after on-chain recovery of
    ///funds, lnd begin to carry out the data loss recovery protocol in order to
    ///recover the funds in each channel from a remote force closed transaction.
    #[prost(message, optional, tag = "5")]
    pub channel_backups: ::core::option::Option<ChanBackupSnapshot>,
    ///
    ///stateless_init is an optional argument instructing the daemon NOT to create
    ///any *.macaroon files in its filesystem. If this parameter is set, then the
    ///admin macaroon returned in the response MUST be stored by the caller of the
    ///RPC as otherwise all access to the daemon will be lost!
    #[prost(bool, tag = "6")]
    pub stateless_init: bool,
    ///
    ///extended_master_key is an alternative to specifying cipher_seed_mnemonic and
    ///aezeed_passphrase. Instead of deriving the master root key from the entropy
    ///of an aezeed cipher seed, the given extended master root key is used
    ///directly as the wallet's master key. This allows users to import/use a
    ///master key from another wallet. When doing so, lnd still uses its default
    ///SegWit only (BIP49/84) derivation paths and funds from custom/non-default
    ///derivation paths will not automatically appear in the on-chain wallet. Using
    ///an 'xprv' instead of an aezeed also has the disadvantage that the wallet's
    ///birthday is not known as that is an information that's only encoded in the
    ///aezeed, not the xprv. Therefore a birthday needs to be specified in
    ///extended_master_key_birthday_timestamp or a "safe" default value will be
    ///used.
    #[prost(string, tag = "7")]
    pub extended_master_key: ::prost::alloc::string::String,
    ///
    ///extended_master_key_birthday_timestamp is the optional unix timestamp in
    ///seconds to use as the wallet's birthday when using an extended master key
    ///to restore the wallet. lnd will only start scanning for funds in blocks that
    ///are after the birthday which can speed up the process significantly. If the
    ///birthday is not known, this should be left at its default value of 0 in
    ///which case lnd will start scanning from the first SegWit block (481824 on
    ///mainnet).
    #[prost(uint64, tag = "8")]
    pub extended_master_key_birthday_timestamp: u64,
    ///
    ///watch_only is the third option of initializing a wallet: by importing
    ///account xpubs only and therefore creating a watch-only wallet that does not
    ///contain any private keys. That means the wallet won't be able to sign for
    ///any of the keys and _needs_ to be run with a remote signer that has the
    ///corresponding private keys and can serve signing RPC requests.
    #[prost(message, optional, tag = "9")]
    pub watch_only: ::core::option::Option<WatchOnly>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitWalletResponse {
    ///
    ///The binary serialized admin macaroon that can be used to access the daemon
    ///after creating the wallet. If the stateless_init parameter was set to true,
    ///this is the ONLY copy of the macaroon and MUST be stored safely by the
    ///caller. Otherwise a copy of this macaroon is also persisted on disk by the
    ///daemon, together with other macaroon files.
    #[prost(bytes = "vec", tag = "1")]
    pub admin_macaroon: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchOnly {
    ///
    ///The unix timestamp in seconds of when the master key was created. lnd will
    ///only start scanning for funds in blocks that are after the birthday which
    ///can speed up the process significantly. If the birthday is not known, this
    ///should be left at its default value of 0 in which case lnd will start
    ///scanning from the first SegWit block (481824 on mainnet).
    #[prost(uint64, tag = "1")]
    pub master_key_birthday_timestamp: u64,
    ///
    ///The fingerprint of the root key (also known as the key with derivation path
    ///m/) from which the account public keys were derived from. This may be
    ///required by some hardware wallets for proper identification and signing. The
    ///bytes must be in big-endian order.
    #[prost(bytes = "vec", tag = "2")]
    pub master_key_fingerprint: ::prost::alloc::vec::Vec<u8>,
    ///
    ///The list of accounts to import. There _must_ be an account for all of lnd's
    ///main key scopes: BIP49/BIP84 (m/49'/0'/0', m/84'/0'/0', note that the
    ///coin type is always 0, even for testnet/regtest) and lnd's internal key
    ///scope (m/1017'/<coin_type>'/<account>'), where account is the key family as
    ///defined in `keychain/derivation.go` (currently indices 0 to 9).
    #[prost(message, repeated, tag = "3")]
    pub accounts: ::prost::alloc::vec::Vec<WatchOnlyAccount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchOnlyAccount {
    ///
    ///Purpose is the first number in the derivation path, must be either 49, 84
    ///or 1017.
    #[prost(uint32, tag = "1")]
    pub purpose: u32,
    ///
    ///Coin type is the second number in the derivation path, this is _always_ 0
    ///for purposes 49 and 84. It only needs to be set to 1 for purpose 1017 on
    ///testnet or regtest.
    #[prost(uint32, tag = "2")]
    pub coin_type: u32,
    ///
    ///Account is the third number in the derivation path. For purposes 49 and 84
    ///at least the default account (index 0) needs to be created but optional
    ///additional accounts are allowed. For purpose 1017 there needs to be exactly
    ///one account for each of the key families defined in `keychain/derivation.go`
    ///(currently indices 0 to 9)
    #[prost(uint32, tag = "3")]
    pub account: u32,
    ///
    ///The extended public key at depth 3 for the given account.
    #[prost(string, tag = "4")]
    pub xpub: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockWalletRequest {
    ///
    ///wallet_password should be the current valid passphrase for the daemon. This
    ///will be required to decrypt on-disk material that the daemon requires to
    ///function properly. When using REST, this field must be encoded as base64.
    #[prost(bytes = "vec", tag = "1")]
    pub wallet_password: ::prost::alloc::vec::Vec<u8>,
    ///
    ///recovery_window is an optional argument specifying the address lookahead
    ///when restoring a wallet seed. The recovery window applies to each
    ///individual branch of the BIP44 derivation paths. Supplying a recovery
    ///window of zero indicates that no addresses should be recovered, such after
    ///the first initialization of the wallet.
    #[prost(int32, tag = "2")]
    pub recovery_window: i32,
    ///
    ///channel_backups is an optional argument that allows clients to recover the
    ///settled funds within a set of channels. This should be populated if the
    ///user was unable to close out all channels and sweep funds before partial or
    ///total data loss occurred. If specified, then after on-chain recovery of
    ///funds, lnd begin to carry out the data loss recovery protocol in order to
    ///recover the funds in each channel from a remote force closed transaction.
    #[prost(message, optional, tag = "3")]
    pub channel_backups: ::core::option::Option<ChanBackupSnapshot>,
    ///
    ///stateless_init is an optional argument instructing the daemon NOT to create
    ///any *.macaroon files in its file system.
    #[prost(bool, tag = "4")]
    pub stateless_init: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockWalletResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePasswordRequest {
    ///
    ///current_password should be the current valid passphrase used to unlock the
    ///daemon. When using REST, this field must be encoded as base64.
    #[prost(bytes = "vec", tag = "1")]
    pub current_password: ::prost::alloc::vec::Vec<u8>,
    ///
    ///new_password should be the new passphrase that will be needed to unlock the
    ///daemon. When using REST, this field must be encoded as base64.
    #[prost(bytes = "vec", tag = "2")]
    pub new_password: ::prost::alloc::vec::Vec<u8>,
    ///
    ///stateless_init is an optional argument instructing the daemon NOT to create
    ///any *.macaroon files in its filesystem. If this parameter is set, then the
    ///admin macaroon returned in the response MUST be stored by the caller of the
    ///RPC as otherwise all access to the daemon will be lost!
    #[prost(bool, tag = "3")]
    pub stateless_init: bool,
    ///
    ///new_macaroon_root_key is an optional argument instructing the daemon to
    ///rotate the macaroon root key when set to true. This will invalidate all
    ///previously generated macaroons.
    #[prost(bool, tag = "4")]
    pub new_macaroon_root_key: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePasswordResponse {
    ///
    ///The binary serialized admin macaroon that can be used to access the daemon
    ///after rotating the macaroon root key. If both the stateless_init and
    ///new_macaroon_root_key parameter were set to true, this is the ONLY copy of
    ///the macaroon that was created from the new root key and MUST be stored
    ///safely by the caller. Otherwise a copy of this macaroon is also persisted on
    ///disk by the daemon, together with other macaroon files.
    #[prost(bytes = "vec", tag = "1")]
    pub admin_macaroon: ::prost::alloc::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod wallet_unlocker_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " WalletUnlocker is a service that is used to set up a wallet password for"]
    #[doc = " lnd at first startup, and unlock a previously set up wallet."]
    #[derive(Debug, Clone)]
    pub struct WalletUnlockerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletUnlockerClient<tonic::transport::Channel> {
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
    impl<T> WalletUnlockerClient<T>
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
        ) -> WalletUnlockerClient<InterceptedService<T, F>>
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
            WalletUnlockerClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "GenSeed is the first method that should be used to instantiate a new lnd"]
        #[doc = "instance. This method allows a caller to generate a new aezeed cipher seed"]
        #[doc = "given an optional passphrase. If provided, the passphrase will be necessary"]
        #[doc = "to decrypt the cipherseed to expose the internal wallet seed."]
        #[doc = ""]
        #[doc = "Once the cipherseed is obtained and verified by the user, the InitWallet"]
        #[doc = "method should be used to commit the newly generated seed, and create the"]
        #[doc = "wallet."]
        pub async fn gen_seed(
            &mut self,
            request: impl tonic::IntoRequest<super::GenSeedRequest>,
        ) -> Result<tonic::Response<super::GenSeedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.WalletUnlocker/GenSeed");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = "InitWallet is used when lnd is starting up for the first time to fully"]
        #[doc = "initialize the daemon and its internal wallet. At the very least a wallet"]
        #[doc = "password must be provided. This will be used to encrypt sensitive material"]
        #[doc = "on disk."]
        #[doc = ""]
        #[doc = "In the case of a recovery scenario, the user can also specify their aezeed"]
        #[doc = "mnemonic and passphrase. If set, then the daemon will use this prior state"]
        #[doc = "to initialize its internal wallet."]
        #[doc = ""]
        #[doc = "Alternatively, this can be used along with the GenSeed RPC to obtain a"]
        #[doc = "seed, then present it to the user. Once it has been verified by the user,"]
        #[doc = "the seed can be fed into this RPC in order to commit the new wallet."]
        pub async fn init_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::InitWalletRequest>,
        ) -> Result<tonic::Response<super::InitWalletResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.WalletUnlocker/InitWallet");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `unlock`"]
        #[doc = "UnlockWallet is used at startup of lnd to provide a password to unlock"]
        #[doc = "the wallet database."]
        pub async fn unlock_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlockWalletRequest>,
        ) -> Result<tonic::Response<super::UnlockWalletResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.WalletUnlocker/UnlockWallet");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " lncli: `changepassword`"]
        #[doc = "ChangePassword changes the password of the encrypted wallet. This will"]
        #[doc = "automatically unlock the wallet database if successful."]
        pub async fn change_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangePasswordRequest>,
        ) -> Result<tonic::Response<super::ChangePasswordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/lnrpc.WalletUnlocker/ChangePassword");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod wallet_unlocker_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WalletUnlockerServer."]
    #[async_trait]
    pub trait WalletUnlocker: Send + Sync + 'static {
        #[doc = ""]
        #[doc = "GenSeed is the first method that should be used to instantiate a new lnd"]
        #[doc = "instance. This method allows a caller to generate a new aezeed cipher seed"]
        #[doc = "given an optional passphrase. If provided, the passphrase will be necessary"]
        #[doc = "to decrypt the cipherseed to expose the internal wallet seed."]
        #[doc = ""]
        #[doc = "Once the cipherseed is obtained and verified by the user, the InitWallet"]
        #[doc = "method should be used to commit the newly generated seed, and create the"]
        #[doc = "wallet."]
        async fn gen_seed(
            &self,
            request: tonic::Request<super::GenSeedRequest>,
        ) -> Result<tonic::Response<super::GenSeedResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = "InitWallet is used when lnd is starting up for the first time to fully"]
        #[doc = "initialize the daemon and its internal wallet. At the very least a wallet"]
        #[doc = "password must be provided. This will be used to encrypt sensitive material"]
        #[doc = "on disk."]
        #[doc = ""]
        #[doc = "In the case of a recovery scenario, the user can also specify their aezeed"]
        #[doc = "mnemonic and passphrase. If set, then the daemon will use this prior state"]
        #[doc = "to initialize its internal wallet."]
        #[doc = ""]
        #[doc = "Alternatively, this can be used along with the GenSeed RPC to obtain a"]
        #[doc = "seed, then present it to the user. Once it has been verified by the user,"]
        #[doc = "the seed can be fed into this RPC in order to commit the new wallet."]
        async fn init_wallet(
            &self,
            request: tonic::Request<super::InitWalletRequest>,
        ) -> Result<tonic::Response<super::InitWalletResponse>, tonic::Status>;
        #[doc = " lncli: `unlock`"]
        #[doc = "UnlockWallet is used at startup of lnd to provide a password to unlock"]
        #[doc = "the wallet database."]
        async fn unlock_wallet(
            &self,
            request: tonic::Request<super::UnlockWalletRequest>,
        ) -> Result<tonic::Response<super::UnlockWalletResponse>, tonic::Status>;
        #[doc = " lncli: `changepassword`"]
        #[doc = "ChangePassword changes the password of the encrypted wallet. This will"]
        #[doc = "automatically unlock the wallet database if successful."]
        async fn change_password(
            &self,
            request: tonic::Request<super::ChangePasswordRequest>,
        ) -> Result<tonic::Response<super::ChangePasswordResponse>, tonic::Status>;
    }
    #[doc = " WalletUnlocker is a service that is used to set up a wallet password for"]
    #[doc = " lnd at first startup, and unlock a previously set up wallet."]
    #[derive(Debug)]
    pub struct WalletUnlockerServer<T: WalletUnlocker> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WalletUnlocker> WalletUnlockerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WalletUnlockerServer<T>
    where
        T: WalletUnlocker,
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
                "/lnrpc.WalletUnlocker/GenSeed" => {
                    #[allow(non_camel_case_types)]
                    struct GenSeedSvc<T: WalletUnlocker>(pub Arc<T>);
                    impl<T: WalletUnlocker> tonic::server::UnaryService<super::GenSeedRequest> for GenSeedSvc<T> {
                        type Response = super::GenSeedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenSeedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).gen_seed(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenSeedSvc(inner);
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
                "/lnrpc.WalletUnlocker/InitWallet" => {
                    #[allow(non_camel_case_types)]
                    struct InitWalletSvc<T: WalletUnlocker>(pub Arc<T>);
                    impl<T: WalletUnlocker> tonic::server::UnaryService<super::InitWalletRequest> for InitWalletSvc<T> {
                        type Response = super::InitWalletResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitWalletRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).init_wallet(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitWalletSvc(inner);
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
                "/lnrpc.WalletUnlocker/UnlockWallet" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockWalletSvc<T: WalletUnlocker>(pub Arc<T>);
                    impl<T: WalletUnlocker> tonic::server::UnaryService<super::UnlockWalletRequest>
                        for UnlockWalletSvc<T>
                    {
                        type Response = super::UnlockWalletResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnlockWalletRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).unlock_wallet(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnlockWalletSvc(inner);
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
                "/lnrpc.WalletUnlocker/ChangePassword" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePasswordSvc<T: WalletUnlocker>(pub Arc<T>);
                    impl<T: WalletUnlocker>
                        tonic::server::UnaryService<super::ChangePasswordRequest>
                        for ChangePasswordSvc<T>
                    {
                        type Response = super::ChangePasswordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangePasswordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).change_password(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangePasswordSvc(inner);
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
    impl<T: WalletUnlocker> Clone for WalletUnlockerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WalletUnlocker> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WalletUnlocker> tonic::transport::NamedService for WalletUnlockerServer<T> {
        const NAME: &'static str = "lnrpc.WalletUnlocker";
    }
}
