use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Failed to create Ed25519 Private key from bytes.")]
    InvalidEd25519PrivateKeyFromBytes,

    #[error("Failed to create Ed25519 Private key from String.")]
    InvalidEd25519PrivateKeyFromString,

    #[error("Failed to create Secp256k1 Private key from bytes.")]
    InvalidSecp256k1PrivateKeyFromBytes,

    #[error("Failed to create Secp256k1 Private key from String.")]
    InvalidSecp256k1PrivateKeyFromString,

    #[error("Failed to create Ed25519 Public key from bytes.")]
    InvalidEd25519PublicKeyFromBytes,

    #[error("Failed to create Ed25519 Public key from String.")]
    InvalidEd25519PublicKeyFromString,

    #[error("Failed to create Secp256k1 Public key from bytes.")]
    InvalidSecp256k1PublicKeyFromBytes,

    #[error("Failed to create Secp256k1 Public key from String.")]
    InvalidSecp256k1PublicKeyFromString,

    #[error("Failed to create Secp256k1 Public key, invalid point, not on curve.")]
    InvalidSecp256k1PublicKeyPointNotOnCurve,

    #[error("Failed to create Ed25519 Public key, invalid point, not on curve.")]
    InvalidEd25519PublicKeyPointNotOnCurve,

    #[error("Appearance id not recognized.")]
    InvalidAppearanceID,

    #[error("String not not a valid display name, did not pass validation.")]
    InvalidDisplayName,

    #[error("String not hex")]
    StringNotHex,

    #[error("Invalid byte count, expected 32.")]
    InvalidByteCountExpected32,

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

    #[error("Supported SLIP10 curves in FactorSource crypto parameters is either empty or contains more elements than allowed.")]
    FactorSourceCryptoParametersSupportedCurvesInvalidSize,

    #[error("Unknown BIP39 word.")]
    UnknownBIP39Word,

    #[error("Invalid mnemonic phrase.")]
    InvalidMnemonicPhrase,

    #[error("Invalid bip39 word count : '{0}'")]
    InvalidBIP39WordCount(usize),

    #[error("Failed to convert FactorInstance into HierarchicalDeterministicFactorInstance, badge is not virtual HD.")]
    BadgeIsNotVirtualHierarchicalDeterministic,

    #[error("Failed to create FactorSourceIDFromHash from FactorSourceID")]
    FactorSourceIDNotFromHash,

    #[error("Failed to create UnsecuredEntityControl, transactionSigning factor instance path has wrong key kind")]
    WrongKeyKindOfTransactionSigningFactorInstance,

    #[error("Failed to create UnsecuredEntityControl, authenticationSigning factor instance path has wrong key kind")]
    WrongKeyKindOfAuthenticationSigningFactorInstance,
}
