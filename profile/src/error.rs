use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid Account Address '{0}'.")]
    InvalidAccountAddress(String),

    #[error("Invalid Account, Discrepancy between specified NetworkID and ID of AccountAddress.")]
    InvalidAccountNetworkMismatch,

    #[error("Unsupported engine entity type.")]
    UnsupportedEntityType,

    #[error("Failed to decode address from bech32.")]
    FailedToDecodeAddressFromBech32,

    #[error("Failed to decode address mismatching entity type")]
    MismatchingEntityTypeWhileDecodingAddress,

    #[error("Failed to decode address mismatching HRP")]
    MismatchingHRPWhileDecodingAddress,

    #[error("Unknown network ID '{0}'")]
    UnknownNetworkID(u8),
}
