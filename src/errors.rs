#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Protobuf decode error: {0}")]
    ProtobufDecodeError(#[from] prost::DecodeError),
    #[error("Couldn't parse from json: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Hex decode error: {0}")]
    HexDecodeError(#[from] hex::FromHexError),
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::transport::Error),
    #[error("Invalid uri: {0}")]
    UriError(#[from] hyper::http::uri::InvalidUri),
    #[error("Error: {0}")]
    GenericError(std::string::String),
}
