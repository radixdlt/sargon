use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid Account Address '{0}'.")]
    InvalidAccountAddress(String),

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

    #[error("Failed to parse InvalidNonFungibleGlobalID from str.")]
    InvalidNonFungibleGlobalID,
}
