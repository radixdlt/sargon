use thiserror::Error;

use crate::bip32::hd_path_component::HDPathValue;

#[derive(Debug, Error, PartialEq)]
pub enum CAP26Error {
    #[error("Invalid BIP32 path '{0}'.")]
    InvalidBIP32Path(String),

    #[error("Invalid depth of CAP26 Path.")]
    InvalidDepthOfCAP26Path,

    #[error("Found non hardened components in path, invalid!")]
    NotAllComponentsAreHardened,

    #[error("Did not find 44H at expected index 1, found value: '{0}'")]
    BIP44PurposeNotFoundAtIndex1(HDPathValue),

    #[error("Did not find cointype 1022H at expected index 2, found value: '{0}'")]
    CoinTypeNotFoundAtIndex2(HDPathValue),

    #[error("Network ID exceeds limit of 255, will never be valid, at index 3, found value: '{0}', known network IDs: [1 (mainnet), 2 (stokenet)]")]
    InvalidNetworkIDExceedsLimit(HDPathValue),

    #[error("InvalidEntityKind, got: '{0}', expected any of: [525H, 618H].")]
    InvalidEntityKind(HDPathValue),

    #[error("InvalidKeyKind, got: '{0}', expected any of: [1460H, 1678H, 1391H].")]
    InvalidKeyKind(HDPathValue),
}
