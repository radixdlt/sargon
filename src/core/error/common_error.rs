use crate::prelude::*;

use thiserror::Error as ThisError;

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

#[repr(u32)]
#[derive(Clone, Debug, ThisError, PartialEq, uniffi::Error)]
#[uniffi(flat_error)]
pub enum CommonError {
    #[error("Unknown Error")]
    Unknown = 10000,

    #[error("Failed to create Ed25519 Private key from bytes {bad_value:?}")]
    InvalidEd25519PrivateKeyFromBytes { bad_value: BagOfBytes } = 10001,

    #[error("Failed to create Ed25519 Private key from String {bad_value}.")]
    InvalidEd25519PrivateKeyFromString { bad_value: String } = 10002,

    #[error(
        "Failed to create Secp256k1 Private key from bytes {bad_value:?}."
    )]
    InvalidSecp256k1PrivateKeyFromBytes { bad_value: BagOfBytes } = 10003,

    #[error(
        "Failed to create Secp256k1 Private key from String {bad_value:?}."
    )]
    InvalidSecp256k1PrivateKeyFromString { bad_value: String } = 10004,

    #[error("Failed to create Ed25519 Public key from bytes {bad_value:?}.")]
    InvalidEd25519PublicKeyFromBytes { bad_value: BagOfBytes } = 10005,

    #[error("Failed to create Ed25519 Public key from String {bad_value}.")]
    InvalidEd25519PublicKeyFromString { bad_value: String } = 10006,

    #[error("Failed to create Secp256k1 Public key from bytes {bad_value:?}.")]
    InvalidSecp256k1PublicKeyFromBytes { bad_value: BagOfBytes } = 10007,

    #[error("Failed to create Secp256k1 Public key from String {bad_value}.")]
    InvalidSecp256k1PublicKeyFromString { bad_value: String } = 10008,

    #[error(
        "Failed to create Secp256k1 Public key, invalid point, not on curve."
    )]
    InvalidSecp256k1PublicKeyPointNotOnCurve = 10009,

    #[error(
        "Failed to create Ed25519 Public key, invalid point, not on curve."
    )]
    InvalidEd25519PublicKeyPointNotOnCurve = 10010,

    #[error("String not hex {bad_value}")]
    StringNotHex { bad_value: String } = 10011,

    #[error("Invalid byte count, expected {expected}, found: {found}")]
    InvalidByteCount { expected: u64, found: u64 } = 10012,

    #[error("Invalid BIP32 path '{bad_value}'.")]
    InvalidBIP32Path { bad_value: String } = 10013,

    #[error("Invalid depth of BIP44 Path, expected {expected}, found {found}")]
    InvalidDepthOfBIP44Path { expected: u64, found: u64 } = 10014,

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
    InvalidDepthOfCAP26Path { expected: u64, found: u64 } = 10018,

    #[error("Found non hardened components in path, invalid!")]
    NotAllComponentsAreHardened = 10019,

    #[error("Did not find 44H, found value: '{bad_value}'")]
    BIP44PurposeNotFound { bad_value: u32 } = 10020,

    #[error("Did not find cointype 1022H, found value: '{bad_value}'")]
    CoinTypeNotFound { bad_value: u32 } = 10021,

    #[error("Network ID exceeds limit of 255, will never be valid, at index 3, found value: '{bad_value}', known network IDs: [1 (mainnet), 2 (stokenet)]")]
    InvalidNetworkIDExceedsLimit { bad_value: u32 } = 10022,

    #[error(
        "InvalidEntityKind, got: '{bad_value}', expected any of: [525H, 618H]."
    )]
    InvalidEntityKind { bad_value: u32 } = 10023,

    #[error("Wrong entity kind, (expected {expected}, found {found})")]
    WrongEntityKind {
        expected: CAP26EntityKind,
        found: CAP26EntityKind,
    } = 10024,

    #[error(
        "InvalidKeyKind, got: '{bad_value}', expected any of: [1460H, 1678H, 1391H]."
    )]
    InvalidKeyKind { bad_value: u32 } = 10025,

    #[error("Unsupported NetworkID, found value: '{bad_value}', known network IDs: [1 (mainnet), 2 (stokenet)]")]
    UnsupportedNetworkID { bad_value: u8 } = 10026,

    #[error(
        "Invalid GetID path, last component was not 365' but {bad_value}'"
    )]
    InvalidGetIDPath { bad_value: u32 } = 10027,

    #[error("Unknown BIP39 word.")]
    UnknownBIP39Word = 10028,

    #[error("Invalid mnemonic phrase.")]
    InvalidMnemonicPhrase = 10029,

    #[error("Invalid bip39 word count: '{bad_value}', valid values are: 12-24 with multiples of 3.")]
    InvalidBIP39WordCount { bad_value: u64 } = 10030,

    #[error("Appearance id not recognized {bad_value}")]
    InvalidAppearanceID { bad_value: u8 } = 10031,

    #[error("Invalid Account Address '{bad_value}'.")]
    InvalidAccountAddress { bad_value: String } = 10032,

    #[error("Unsupported engine entity type.")]
    UnsupportedEntityType = 10033,

    #[error("Failed to decode address from bech32 {bad_value}.")]
    FailedToDecodeAddressFromBech32 { bad_value: String } = 10034,

    #[error("Failed to decode address mismatching entity type")]
    MismatchingEntityTypeWhileDecodingAddress = 10035,

    #[error("Failed to decode address mismatching HRP")]
    MismatchingHRPWhileDecodingAddress = 10036,

    #[error("Unknown network ID '{bad_value}'")]
    UnknownNetworkID { bad_value: u8 } = 10037,

    #[error("Failed to parse InvalidNonFungibleGlobalID from {bad_value}.")]
    InvalidNonFungibleGlobalID { bad_value: String } = 10038,

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

    #[error("No network found with name: '{bad_value}'")]
    UnknownNetworkWithName { bad_value: String } = 10048,

    #[error("No network found with id: '{bad_value}'")]
    UnknownNetworkForID { bad_value: u8 } = 10049,

    #[error("Gateway discrepancy, 'other' should not contain 'current'.")]
    GatewaysDiscrepancyOtherShouldNotContainCurrent = 10050,

    #[error(
        "Gateways discrepancy, invalid JSON, current not found amongst saved."
    )]
    InvalidGatewaysJSONCurrentNotFoundAmongstSaved = 10051,

    #[error("Invalid URL: '{bad_value}'")]
    InvalidURL { bad_value: String } = 10052,

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
        expected: u64,
        found: u64,
        data: BagOfBytes,
    } = 10057,

    #[error("Invalid NonFungibleLocalID::String")]
    InvalidNonFungibleLocalIDString = 10058,

    #[error("Invalid NonFungibleLocalID::Bytes")]
    InvalidNonFungibleLocalIDBytes = 10059,

    #[error("Invalid Decimal")]
    DecimalError = 10060,

    #[error("Invalid BIP39 Index {bad_value}")]
    InvalidBIP39Index { bad_value: u16 } = 10061,

    #[error("Invalid DisplayName cannot be empty.")]
    InvalidDisplayNameEmpty = 10062,

    #[error("Invalid DisplayName too long, expected max: {expected}, found: {found}")]
    InvalidDisplayNameTooLong { expected: u64, found: u64 } = 10063,

    #[error("Invalid ISO8601 Time string: {bad_value}")]
    InvalidISO8601String { bad_value: String } = 10064,

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
        json_byte_count: u64,
        type_name: String,
    } = 10070,

    #[error("Failed To create ProfileID (UUID) from string: {bad_value}")]
    InvalidProfileID { bad_value: String } = 10071,

    #[error("Failed to load Profile Headers list")]
    FailedToLoadProfileHeadersList = 10072,

    #[error("FactorSource with ID not found in Profile: {bad_value:?}")]
    ProfileDoesNotContainFactorSourceWithID { bad_value: FactorSourceID } =
        10073,

    #[error("No active ProfileID found in SecureStorage.")]
    NoActiveProfileIDSet = 10074,

    #[error("No Profile snapshot found for ProfileID {bad_value}")]
    ProfileSnapshotNotFound { bad_value: ProfileID } = 10075,

    #[error("Account Already Present {bad_value}")]
    AccountAlreadyPresent { bad_value: AccountAddress } = 10076,

    #[error("Unable to acquire write lock for Profile inside Wallet")]
    UnableToAcquireWriteLockForProfile = 10077,

    #[error("Failed save Mnemonic to SecureStorage with FactorSourceID: {bad_value}")]
    UnableToSaveMnemonicToSecureStorage { bad_value: FactorSourceIDFromHash } =
        10078,

    #[error(
        "Failed load Mnemonic from SecureStorage with FactorSourceID: {bad_value}"
    )]
    UnableToLoadMnemonicFromSecureStorage { bad_value: FactorSourceIDFromHash } =
        10079,

    #[error("Failed save FactorSource to SecureStorage, FactorSourceID: {bad_value}")]
    UnableToSaveFactorSourceToProfile { bad_value: FactorSourceID } = 10080,

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

    #[error("Invalid UUID (v4), got: {bad_value}")]
    InvalidUUIDv4 { bad_value: String } = 10086,

    #[error("Unrecognized Locale Identifier: {bad_value}")]
    UnrecognizedLocaleIdentifier { bad_value: String } = 10087,

    #[error("Failed to create Address (via RetAddress) from node_id (hex): {node_id_as_hex}, network_id: {network_id}")]
    FailedToCreateAddressViaRetAddressFromNodeIdAndNetworkID {
        node_id_as_hex: String,
        network_id: NetworkID,
    } = 10088,

    #[error("Invalid Olympia address string: {bad_value}")]
    InvalidOlympiaAddressString { bad_value: String } = 10089,

    #[error("Invalid Transaction Manifest Instructions String")]
    InvalidInstructionsString = 10090,

    #[error(
        "Failed to get execution summary from TransactionManifest using RET"
    )]
    FailedToGetRetExecutionSummaryFromManifest = 10091,

    #[error("Failed to get TransactionReceipt from encoded bytes.")]
    FailedToDecodeEncodedReceipt = 10092,

    #[error("Invalid byte count, was empty")]
    BytesEmpty = 10093,

    #[error("Invalid byte count, expected at most {max}, found: {found}")]
    TooManyBytes { max: u64, found: u64 } = 10094,

    #[error("Invalid Manifest Instructions String, found network in instructions {found_in_instructions}, but specified to constructor: {specified_to_instructions_ctor}")]
    InvalidInstructionsWrongNetwork {
        found_in_instructions: NetworkID,
        specified_to_instructions_ctor: NetworkID,
    } = 10095,

    #[error(
        "Failed to UniFFI decode bytes into Transaction Manifest Instructions"
    )]
    FailedToUniFFIDecodeBytesToManifestInstructions = 10096,

    #[error("Failed to decode Transaction Hash, value: {bad_value}")]
    FailedToDecodeTransactionHash { bad_value: String } = 10097,

    #[error("Failed to hash transaction intent")]
    FailedToHashIntent = 10098,

    #[error("Encrypted Messages are not yet supported")]
    EncryptedMessagesAreNotYetSupported = 10099,

    #[error("Failed to Bech32 decode transaction Hash after having tested all Network IDs, from: {bad_value}")]
    FailedToBech32DecodeTransactionHashAfterHavingTestedAllNetworkID {
        bad_value: String,
    } = 10100,

    #[error("Failed to parse Signature from {bad_value}")]
    FailedToParseSignatureFromString { bad_value: String } = 10101,

    #[error(
        "Invalid IntentSignatures for Intent some didn't validate IntentHash"
    )]
    InvalidSignaturesForIntentSomeDidNotValidateIntentHash = 10102,

    #[error("Failed to decompile bytes into NotarizedTransaction")]
    FailedToDecompileBytesIntoNotarizedTransaction = 10103,

    #[error("Failed to recover secp256k1 PublicKey from signature")]
    FailedToRecoverSecp256k1PublicKeyFromSignature = 10104,

    #[error("Fungible ResourceAddress in NonFungible context is not allowed.")]
    FungibleResourceAddressNotAcceptedInNonFungibleContext = 10105,

    #[error("Failed to convert to Decimal192 from f32 due to overflow, value: {bad_value}")]
    DecimalOverflow { bad_value: String } = 10106,

    #[error("Invalid Olympia address, not mainnet: {bad_value}")]
    InvalidAddressNotOlympiaMainnet { bad_value: String } = 10107,

    #[error("Failed to parse Signature from {bad_value}")]
    FailedToParseSignatureFromBytes { bad_value: String } = 10108,
}

/*
// FIXME: We want this! We want to be able to get the error description
of an error, but we get some strange uniffi error!
#[uniffi::export]
pub fn error_message_from_error(error: &CommonError) -> String {
    format!("{}", error)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn error_message() {
        let sut = CommonError::UnknownNetworkForID { bad_value: 0 };
        // assert_eq!(error_message_from_error(&sut), "Unknown network ID '0'");
    }
}
*/
