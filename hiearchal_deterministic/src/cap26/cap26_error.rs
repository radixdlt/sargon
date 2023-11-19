use thiserror::Error;

use crate::bip32::hd_path_component::HDPathValue;

use super::cap26_entity_kind::CAP26EntityKind;

#[derive(Debug, Error, PartialEq)]
pub enum CAP26Error {
    #[error("Invalid BIP32 path '{0}'.")]
    InvalidBIP32Path(String),

    #[error("Invalid depth of CAP26 Path.")]
    InvalidDepthOfCAP26Path,

    #[error("Found non hardened components in path, invalid!")]
    NotAllComponentsAreHardened,

    #[error("Did not find 44H, found value: '{0}'")]
    BIP44PurposeNotFound(HDPathValue),

    #[error("Did not find cointype 1022H, found value: '{0}'")]
    CoinTypeNotFound(HDPathValue),

    #[error("Network ID exceeds limit of 255, will never be valid, at index 3, found value: '{0}', known network IDs: [1 (mainnet), 2 (stokenet)]")]
    InvalidNetworkIDExceedsLimit(HDPathValue),

    #[error("InvalidEntityKind, got: '{0}', expected any of: [525H, 618H].")]
    InvalidEntityKind(HDPathValue),
    
    #[error("Wrong entity kind, got: '{0}', but expected: '{1}'")]
    WrongEntityKind(CAP26EntityKind, CAP26EntityKind),

    #[error("InvalidKeyKind, got: '{0}', expected any of: [1460H, 1678H, 1391H].")]
    InvalidKeyKind(HDPathValue),

    #[error("Unsupported NetworkID, got: '{0}', found value: '{0}', known network IDs: [1 (mainnet), 2 (stokenet)]")]
    UnsupportedNetworkID(u8),
}
