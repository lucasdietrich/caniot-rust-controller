use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid device id")]
    DeviceIdCreationError,
    #[error("Payload format error")]
    PayloadDecodeError,
    #[error("Command format error")]
    CommandEncodeError,
    #[error("Unknown attribute key")]
    UnknownAttributeKey,
}
