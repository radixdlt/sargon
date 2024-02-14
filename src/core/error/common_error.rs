use crate::prelude::*;

use thiserror::Error as ThisError;

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

#[repr(u32)]
#[derive(Clone, Debug, ThisError, PartialEq, uniffi::Error)]
#[uniffi(flat_error)]
pub enum CommonError {
    #[error("Unknown Error")]
    Unknown = 10000,

    #[error("Failed to create Ed25519 Private key from bytes {0:?}")]
    InvalidEd25519PrivateKeyFromBytes(Vec<u8>) = 10001,

    #[error("Failed to create Ed25519 Private key from String {0}.")]
    InvalidEd25519PrivateKeyFromString(String) = 10002,

    #[error("Failed to create Secp256k1 Private key from bytes {0:?}.")]
    InvalidSecp256k1PrivateKeyFromBytes(Vec<u8>) = 10003,

    #[error("Failed to create Secp256k1 Private key from String {0:?}.")]
    InvalidSecp256k1PrivateKeyFromString(String) = 10004,

    #[error("Failed to create Ed25519 Public key from bytes {0:?}.")]
    InvalidEd25519PublicKeyFromBytes(Vec<u8>) = 10005,

    #[error("Failed to create Ed25519 Public key from String {0}.")]
    InvalidEd25519PublicKeyFromString(String) = 10006,

    #[error("Failed to create Secp256k1 Public key from bytes {0:?}.")]
    InvalidSecp256k1PublicKeyFromBytes(Vec<u8>) = 10007,

    #[error("Failed to create Secp256k1 Public key from String {0}.")]
    InvalidSecp256k1PublicKeyFromString(String) = 10008,

    #[error(
        "Failed to create Secp256k1 Public key, invalid point, not on curve."
    )]
    InvalidSecp256k1PublicKeyPointNotOnCurve = 10009,

    #[error(
        "Failed to create Ed25519 Public key, invalid point, not on curve."
    )]
    InvalidEd25519PublicKeyPointNotOnCurve = 10010,

    #[error("String not hex {0}")]
    StringNotHex(String) = 10011,

    #[error("Invalid byte count, expected 32, found: {0}")]
    InvalidByteCountExpected32(usize) = 10012,

    #[error("Invalid BIP32 path '{0}'.")]
    InvalidBIP32Path(String) = 10013,

    #[error("Invalid depth of BIP44 Path, expected {expected}, found {found}")]
    InvalidDepthOfBIP44Path { expected: usize, found: usize } = 10014,

    #[error("Invalid BIP44Like Path, account was not hardened")]
    InvalidBIP44LikePathAccountWasNotHardened = 10015,

    #[error(
        "Invalid BIP44Like Path, 'change' component was unexpectedly hardened"
    )]
    InvalidBIP44LikePathChangeWasUnexpectedlyHardened = 10016,

    /// Radix Olympia did follow BIP44, we accidentally hardened the last component `"index"`,
    /// and for backwards compatibility we require it to be hardened in Babylon too.
    #[error("Invalid BIP44Like Path, 'index' component was not hardened")]
    InvalidBIP44LikePathIndexWasNotHardened = 10017,

    #[error(
        "Invalid depth of CAP26 Path, (expected {expected}, found {found})"
    )]
    InvalidDepthOfCAP26Path { expected: usize, found: usize } = 10018,

    #[error("Found non hardened components in path, invalid!")]
    NotAllComponentsAreHardened = 10019,

    #[error("Did not find 44H, found value: '{0}'")]
    BIP44PurposeNotFound(u32) = 10020,

    #[error("Did not find cointype 1022H, found value: '{0}'")]
    CoinTypeNotFound(u32) = 10021,

    #[error("Network ID exceeds limit of 255, will never be valid, at index 3, found value: '{0}', known network IDs: [1 (mainnet), 2 (stokenet)]")]
    InvalidNetworkIDExceedsLimit(u32) = 10022,

    #[error("InvalidEntityKind, got: '{0}', expected any of: [525H, 618H].")]
    InvalidEntityKind(u32) = 10023,

    #[error("Wrong entity kind, (expected {expected}, found {found})")]
    WrongEntityKind {
        expected: CAP26EntityKind,
        found: CAP26EntityKind,
    } = 10024,

    #[error(
        "InvalidKeyKind, got: '{0}', expected any of: [1460H, 1678H, 1391H]."
    )]
    InvalidKeyKind(u32) = 10025,

    #[error("Unsupported NetworkID, got: '{0}', found value: '{0}', known network IDs: [1 (mainnet), 2 (stokenet)]")]
    UnsupportedNetworkID(u8) = 10026,

    #[error("Invalid GetID path, last component was not 365' but {0}'")]
    InvalidGetIDPath(u32) = 10027,

    #[error("Unknown BIP39 word.")]
    UnknownBIP39Word = 10028,

    #[error("Invalid mnemonic phrase.")]
    InvalidMnemonicPhrase = 10029,

    #[error("Invalid bip39 word count: '{0}', valid values are: 12-24 with multiples of 3.")]
    InvalidBIP39WordCount(usize) = 10030,

    #[error("Appearance id not recognized {0}")]
    InvalidAppearanceID(u8) = 10031,

    #[error("Invalid Account Address '{0}'.")]
    InvalidAccountAddress(String) = 10032,

    #[error("Unsupported engine entity type.")]
    UnsupportedEntityType = 10033,

    #[error("Failed to decode address from bech32 {0}.")]
    FailedToDecodeAddressFromBech32(String) = 10034,

    #[error("Failed to decode address mismatching entity type")]
    MismatchingEntityTypeWhileDecodingAddress = 10035,

    #[error("Failed to decode address mismatching HRP")]
    MismatchingHRPWhileDecodingAddress = 10036,

    #[error("Unknown network ID '{0}'")]
    UnknownNetworkID(u8) = 10037,

    #[error("Failed to parse InvalidNonFungibleGlobalID from {0}.")]
    InvalidNonFungibleGlobalID(String) = 10038,

    #[error("Supported SLIP10 curves in FactorSource crypto parameters is either empty or contains more elements than allowed.")]
    FactorSourceCryptoParametersSupportedCurvesInvalidSize = 10039,

    #[error("Failed to convert FactorInstance into HierarchicalDeterministicFactorInstance, badge is not virtual HD.")]
    BadgeIsNotVirtualHierarchicalDeterministic = 10040,

    #[error("Failed to create FactorSourceIDFromHash from FactorSourceID")]
    FactorSourceIDNotFromHash = 10041,

    #[error("Expected AccountPath but got something else.")]
    ExpectedAccountPathButGotSomethingElse = 10042,

    #[error("Wrong entity kind in path of FactorInstance")]
    WrongEntityKindOfInFactorInstancesPath = 10043,

    #[error("Wrong key kind of FactorInstance - expected transaction signing")]
    WrongKeyKindOfTransactionSigningFactorInstance = 10044,

    #[error(
        "Wrong key kind of FactorInstance - expected authentication signing"
    )]
    WrongKeyKindOfAuthenticationSigningFactorInstance = 10045,

    #[error("Expected DeviceFactorSource")]
    ExpectedDeviceFactorSourceGotSomethingElse = 10046,

    #[error("Expected LedgerHardwareWalletFactorSource")]
    ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse = 10047,

    #[error("No network found with name: '{0}'")]
    UnknownNetworkWithName(String) = 10048,

    #[error("No network found with id: '{0}'")]
    UnknownNetworkForID(u8) = 10049,

    #[error("Gateway discrepancy, 'other' should not contain 'current'.")]
    GatewaysDiscrepancyOtherShouldNotContainCurrent = 10050,

    #[error(
        "Gateways discrepancy, invalid JSON, current not found amongst saved."
    )]
    InvalidGatewaysJSONCurrentNotFoundAmongstSaved = 10051,

    #[error("Invalid URL: '{0}'")]
    InvalidURL(String) = 10052,

    #[error(
        "Accounts on different networks, expected: {expected}, found: {found}"
    )]
    AccountOnWrongNetwork {
        expected: NetworkID,
        found: NetworkID,
    } = 10053,

    #[error("FactorSources must not be empty.")]
    FactorSourcesMustNotBeEmpty = 10054,

    #[error("Failed to update FactorSource, error while mutating.")]
    UpdateFactorSourceMutateFailed = 10055,

    #[error("Failed to cast factor source, wrong kind, , expected: {expected}, found: {found}")]
    CastFactorSourceWrongKind {
        expected: FactorSourceKind,
        found: FactorSourceKind,
    } = 10056,

    #[error("Length check failed, expected: {expected}, found: {found}, data: {data:?}")]
    InvalidLength {
        expected: usize,
        found: usize,
        data: Vec<u8>,
    } = 10057,

    #[error("Invalid NonFungibleLocalID::String")]
    InvalidNonFungibleLocalIDString = 10058,

    #[error("Invalid NonFungibleLocalID::Bytes")]
    InvalidNonFungibleLocalIDBytes = 10059,

    #[error("Invalid Decimal")]
    DecimalError = 10060,

    #[error("Invalid BIP39 Index {0}")]
    InvalidBIP39Index(u16) = 10061,

    #[error("Invalid DisplayName cannot be empty.")]
    InvalidDisplayNameEmpty = 10062,

    #[error("Invalid DisplayName too long, expected max: {expected}, found: {found}")]
    InvalidDisplayNameTooLong { expected: usize, found: usize } = 10063,

    #[error("Invalid ISO8601 Time string: {0}")]
    InvalidISO8601String(String) = 10064,

    #[error("Unknown account.")]
    UnknownAccount = 10065,

    #[error("Failed to read from secure storage (Keychain).")]
    SecureStorageReadError = 10066,

    #[error("Failed to load DeviceFactorSource from secure storage")]
    UnableToLoadDeviceFactorSourceFromSecureStorage = 10067,

    #[error("Failed to write to secure storage (Keychain).")]
    SecureStorageWriteError = 10068,

    #[error("Failed Serialize value to JSON.")]
    FailedToSerializeToJSON = 10069,

    #[error("Failed deserialize JSON with #{json_byte_count} bytes to value of type {type_name}")]
    FailedToDeserializeJSONToValue {
        json_byte_count: usize,
        type_name: String,
    } = 10070,

    #[error("Failed To create ProfileID (UUID) from string: {0}")]
    InvalidProfileID(String) = 10071,

    #[error("Failed to load Profile Headers list")]
    FailedToLoadProfileHeadersList = 10072,

    #[error("FactorSource with ID not found in Profile: {0:?}")]
    ProfileDoesNotContainFactorSourceWithID(FactorSourceID) = 10073,

    #[error("No active ProfileID found in SecureStorage.")]
    NoActiveProfileIDSet = 10074,

    #[error("No Profile snapshot found for ProfileID {0}")]
    ProfileSnapshotNotFound(ProfileID) = 10075,

    #[error("Account Already Present {0}")]
    AccountAlreadyPresent(AccountAddress) = 10076,

    #[error("Unable to acquire write lock for Profile inside Wallet")]
    UnableToAcquireWriteLockForProfile = 10077,

    #[error("Failed save Mnemonic to SecureStorage with FactorSourceID: {0}")]
    UnableToSaveMnemonicToSecureStorage(FactorSourceIDFromHash) = 10078,

    #[error(
        "Failed load Mnemonic from SecureStorage with FactorSourceID: {0}"
    )]
    UnableToLoadMnemonicFromSecureStorage(FactorSourceIDFromHash) = 10079,

    #[error("Failed save FactorSource to SecureStorage, FactorSourceID: {0}")]
    UnableToSaveFactorSourceToProfile(FactorSourceID) = 10080,

    #[error("Expected IdentityPath but got something else.")]
    ExpectedIdentityPathButGotSomethingElse = 10081,

    #[error("Invalid PersonaData - phone number empty")]
    PersonaDataInvalidPhoneNumberEmpty = 10082,

    #[error("Invalid PersonaData - email address empty")]
    PersonaDataInvalidEmailAddressEmpty = 10083,

    #[error("Invalid PersonaData - family name empty ")]
    PersonaDataInvalidNameFamilyNameEmpty = 10084,

    #[error("Invalid PersonaData - given names empty")]
    PersonaDataInvalidNameGivenNamesEmpty = 10085,

    #[error("Invalid UUID (v4), got: {0}")]
    InvalidUUIDv4(String) = 10086,
}
