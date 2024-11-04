use crate::prelude::*;
use sargon::CommonError as InternalCommonError;

use thiserror::Error as ThisError;

#[repr(u32)]
#[derive(
    Clone, Debug, ThisError, PartialEq, InternalConversion, uniffi::Error,
)]
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

    #[error("Failed to access secure storage due to \"{error_message}\" for key {} ", secure_storage_key_identifier(key))]
    SecureStorageAccessError {
        key: SecureStorageKey,
        error_kind: SecureStorageAccessErrorKind,
        error_message: String,
    } = 10063,

    #[error("Invalid ISO8601 Time string: {bad_value}")]
    InvalidISO8601String { bad_value: String } = 10064,

    #[error("Unknown account.")]
    UnknownAccount = 10065,

    #[error("Failed to read from secure storage.")]
    SecureStorageReadError = 10066,

    #[error("Failed to load DeviceFactorSource from secure storage")]
    UnableToLoadDeviceFactorSourceFromSecureStorage = 10067,

    #[error("Failed to write to secure storage.")]
    SecureStorageWriteError = 10068,

    #[error("Failed Serialize value to JSON.")]
    FailedToSerializeToJSON = 10069,

    #[error("Failed deserialize JSON with #{json_byte_count} bytes to value of type {type_name} with error: \"{serde_message}\"")]
    FailedToDeserializeJSONToValue {
        json_byte_count: u64,
        type_name: String,
        serde_message: String,
    } = 10070,

    #[error("Failed To create ProfileID (UUID) from string: {bad_value}")]
    InvalidProfileID { bad_value: String } = 10071,

    #[error("Failed to load Profile Headers list")]
    FailedToLoadProfileHeadersList = 10072,

    #[error("FactorSource with ID not found in Profile: {bad_value:?}")]
    ProfileDoesNotContainFactorSourceWithID { bad_value: FactorSourceID } =
        10073,

    #[error("Account Already Present {bad_value}")]
    AccountAlreadyPresent { bad_value: AccountAddress } = 10074,

    #[error("Unable to acquire write lock for Profile.")]
    UnableToAcquireWriteLockForProfile = 10075,

    #[error("Failed save Mnemonic to SecureStorageDriver with FactorSourceID: {bad_value}")]
    UnableToSaveMnemonicToSecureStorage { bad_value: FactorSourceIDFromHash } =
        10076,

    #[error(
        "Failed load Mnemonic from SecureStorageDriver with FactorSourceID: {bad_value}"
    )]
    UnableToLoadMnemonicFromSecureStorage { bad_value: FactorSourceIDFromHash } =
        10077,

    #[error("Failed save FactorSource to SecureStorageDriver, FactorSourceID: {bad_value}")]
    UnableToSaveFactorSourceToProfile { bad_value: FactorSourceID } = 10078,

    #[error("Expected IdentityPath but got something else.")]
    ExpectedIdentityPathButGotSomethingElse = 10079,

    #[error("Invalid PersonaData - phone number empty")]
    PersonaDataInvalidPhoneNumberEmpty = 10080,

    #[error("Invalid email address, cannot be empty")]
    EmailAddressEmpty = 10081,

    #[error("Invalid PersonaData - family name empty ")]
    PersonaDataInvalidNameFamilyNameEmpty = 10082,

    #[error("Invalid PersonaData - given names empty")]
    PersonaDataInvalidNameGivenNamesEmpty = 10083,

    #[error("Invalid UUID (v4), got: {bad_value}")]
    InvalidUUIDv4 { bad_value: String } = 10084,

    #[error("Unrecognized Locale Identifier: {bad_value}")]
    UnrecognizedLocaleIdentifier { bad_value: String } = 10085,

    #[error("Failed to create Address (via RetAddress) from node_id (hex): {node_id_as_hex}, network_id: {network_id}")]
    FailedToCreateAddressViaRetAddressFromNodeIdAndNetworkID {
        node_id_as_hex: String,
        network_id: NetworkID,
    } = 10086,

    #[error("Invalid Olympia address string: {bad_value}")]
    InvalidOlympiaAddressString { bad_value: String } = 10087,

    #[error(
        "Invalid Transaction Manifest Instructions String: '{underlying}'"
    )]
    InvalidInstructionsString { underlying: String } = 10088,

    #[error(
        "Failed to get execution summary from TransactionManifest using RET {underlying}"
    )]
    ExecutionSummaryFail { underlying: String } = 10089,

    #[error("Failed to get TransactionReceipt from engine toolkit bytes.")]
    FailedToDecodeEngineToolkitReceipt = 10090,

    #[error("Invalid byte count, was empty")]
    BytesEmpty = 10091,

    #[error("Invalid byte count, expected at most {max}, found: {found}")]
    TooManyBytes { max: u64, found: u64 } = 10092,

    #[error("Invalid Manifest Instructions String, found network in instructions {found_in_instructions}, but specified to constructor: {specified_to_instructions_ctor}")]
    InvalidInstructionsWrongNetwork {
        found_in_instructions: NetworkID,
        specified_to_instructions_ctor: NetworkID,
    } = 10093,

    #[error(
        "Failed to UniFFI decode bytes into Transaction Manifest Instructions"
    )]
    FailedToDecodeBytesToManifestInstructions = 10094,

    #[error("Failed to decode Transaction Hash, value: {bad_value}")]
    FailedToDecodeTransactionHash { bad_value: String } = 10095,

    #[error("Failed to hash transaction intent")]
    FailedToHashIntent = 10096,

    #[error("Encrypted Messages are not yet supported")]
    EncryptedMessagesAreNotYetSupported = 10097,

    #[error("Failed to Bech32 decode transaction Hash after having tested all Network IDs, from: {bad_value}")]
    FailedToBech32DecodeTransactionHashAfterHavingTestedAllNetworkID {
        bad_value: String,
    } = 10098,

    #[error("Failed to parse Signature from {bad_value}")]
    FailedToParseSignatureFromString { bad_value: String } = 10099,

    #[error(
        "Invalid IntentSignatures for Intent some didn't validate IntentHash"
    )]
    InvalidSignaturesForIntentSomeDidNotValidateIntentHash = 10100,

    #[error("Failed to decompile bytes into NotarizedTransaction")]
    FailedToDecompileBytesIntoNotarizedTransaction = 10101,

    #[error("Failed to recover secp256k1 PublicKey from signature")]
    FailedToRecoverSecp256k1PublicKeyFromSignature = 10102,

    #[error("Fungible ResourceAddress in NonFungible context is not allowed.")]
    FungibleResourceAddressNotAcceptedInNonFungibleContext = 10103,

    #[error("Failed to convert to Decimal192 from f32 due to overflow, value: {bad_value}")]
    DecimalOverflow { bad_value: String } = 10104,

    #[error("Invalid Olympia address, not mainnet: {bad_value}")]
    InvalidAddressNotOlympiaMainnet { bad_value: String } = 10105,

    #[error("Failed to parse Signature from {bad_value}")]
    FailedToParseSignatureFromBytes { bad_value: String } = 10106,

    #[error(
        "Invalid Transaction Intent, failed to encode, reason: '{underlying}'"
    )]
    InvalidIntentFailedToEncode { underlying: String } = 10107,

    #[error(
        "Invalid Instructions, failed to decompile, reason: '{underlying}'"
    )]
    InvalidInstructionsFailedToDecompile { underlying: String } = 10108,

    #[error("Invalid Transaction, max SBOR depth exceeded: '{max}'")]
    InvalidTransactionMaxSBORDepthExceeded { max: u16 } = 10109,

    #[error("Invalid Signed Intent, failed to encode, reason: '{underlying}'")]
    InvalidSignedIntentFailedToEncode { underlying: String } = 10110,

    #[error(
        "Invalid Notarized Intent, failed to encode, reason: '{underlying}'"
    )]
    InvalidNotarizedIntentFailedToEncode { underlying: String } = 10111,

    #[error("Networking response bad code")]
    NetworkResponseBadCode = 10112,

    #[error("Networking response body was empty")]
    NetworkResponseEmptyBody = 10113,

    #[error("Networking response fail json deserialize into {into_type}")]
    NetworkResponseJSONDeserialize { into_type: String } = 10114,

    #[error("Networking request invalid url {bad_value}")]
    NetworkRequestInvalidUrl { bad_value: String } = 10115,

    #[error("Networking request failed, reason: '{underlying}'")]
    NetworkRequestGenericFailure { underlying: String } = 10116,

    #[error("Submitted transaction was duplicate.")]
    GatewaySubmitDuplicateTX { intent_hash: String } = 10117,

    #[error("SupportedCurves must not be empty.")]
    SupportedCurvesMustNotBeEmpty = 10118,

    #[error("Networks must not be empty")]
    ProfileNetworksMustNotBeEmpty = 10119,

    #[error("Unknown SLIP10 Curve '{bad_value}'")]
    UnknownSLIP10Curve { bad_value: String } = 10120,

    #[error("AES Decryption failed")]
    AESDecryptionFailed = 10121,

    #[error("Invalid AES Sealedbox, too few bytes expected at least: {expected_at_least}, found: {found}.")]
    InvalidAESBytesTooShort { expected_at_least: u64, found: u64 } = 10122,

    #[error("Invalid Factor Source kind, bad value: {bad_value}")]
    InvalidFactorSourceKind { bad_value: String } = 10123,

    #[error("Invalid LedgerHardwareWalletModel, bad value: {bad_value}")]
    InvalidLedgerHardwareWalletModel { bad_value: String } = 10124,

    #[error("RadixConnectMobile invalid URL, bad value: {bad_value}")]
    RadixConnectMobileInvalidRequestUrl { bad_value: String } = 10125,

    #[error("RadixConnectMobile invalid origin, bad value: {bad_value}")]
    RadixConnectMobileInvalidOrigin { bad_value: String } = 10126,

    #[error("Failed to create Session (UUID) from string: {bad_value}")]
    RadixConnectMobileInvalidSessionID { bad_value: String } = 10127,

    #[error("Failed to create InteractionID (UUID) from string: {bad_value}")]
    RadixMobileInvalidInteractionID { bad_value: String } = 10128,

    #[error("Network discrepancy, expected : {expected}, actual: {actual}")]
    NetworkDiscrepancy {
        expected: NetworkID,
        actual: NetworkID,
    } = 10129,

    #[error("Discrepancy, Authorized Dapp references Persona which does not exist {address}")]
    DiscrepancyAuthorizedDappReferencedPersonaWhichDoesNotExist {
        address: IdentityAddress,
    } = 10130,

    #[error("Discrepancy, Authorized Dapp references Account which does not exist {address}")]
    DiscrepancyAuthorizedDappReferencedAccountWhichDoesNotExist {
        address: AccountAddress,
    } = 10131,

    #[error("AuthorizedDapp references field id that does not exist")]
    AuthorizedDappReferencesFieldIDThatDoesNotExist = 10132,

    #[error("Item identified by ID {id} does not exist")]
    ElementDoesNotExist { id: String } = 10133,

    #[error("Item identified by ID {id} already exist")]
    IdentifiableItemAlreadyExist { id: String } = 10134,

    #[error("Invalid RadixConnectPurpose, bad value: {bad_value}")]
    InvalidRadixConnectPurpose { bad_value: String } = 10135,

    #[error(
        "Transaction Guarantee's 'instruction_index' is out of bounds, the provided manifest contains #{count}, but an 'instruction_index' of {index} was specified."
    )]
    TXGuaranteeIndexOutOfBounds { index: u64, count: u64 } = 10136,

    #[error("Failed to create KeyAgreementPublicKey from hex: {bad_value}")]
    InvalidKeyAgreementPublicKeyFromHex { bad_value: String } = 10137,

    #[error(
        "Failed to create KeyAgreementPublicKey from bytes: {bad_value:?}"
    )]
    InvalidKeyAgreementPublicKeyFromBytes { bad_value: BagOfBytes } = 10138,

    #[error(
        "Failed to create KeyAgreementPrivateKey from bytes: {bad_value:?}"
    )]
    InvalidKeyAgreementPrivateKeyFromBytes { bad_value: BagOfBytes } = 10139,

    #[error("RadixConnectMobileSession not found, session id: {session_id}")]
    RadixConnectMobileSessionNotFound { session_id: SessionID } = 10140,

    #[error("RadixConnectMobileDappRequest not found, interaction id: {interaction_id}")]
    RadixConnectMobileDappRequestNotFound {
        interaction_id: WalletInteractionId,
    } = 10141,

    #[error("RadixConnectMobileDappCallbackPath not found, origin: {origin}")]
    RadixConnectMobileDappCallbackPathNotFound { origin: Url } = 10142,

    #[error("Failed to create Ed25519 Signature from String {bad_value}.")]
    InvalidEd25519SignatureFromString { bad_value: String } = 10143,

    #[error("Radix Connect Mobile dApp public key does not match the session's dApp public key")]
    RadixConnectMobileDappPublicKeyMismatch = 10144,

    #[error("Radix Connect Mobile dApp identity not match the session's dApp identity")]
    RadixConnectMobileDappIdentityMismatch = 10145,

    #[error(
        "Radix Connect Mobile dApp origin not match the session's dApp origin"
    )]
    RadixConnectMobileDappOriginMismatch = 10146,

    #[error("Radix Connect Mobile dApp sent an invalid signature")]
    RadixConnectMobileInvalidDappSignature = 10147,

    #[error("Radix Connect Mobile dApp sent an invalid signature")]
    RadixConnectMobileInvalidRequestFormat = 10148,

    #[error("Radix Connect Mobile failed to create new in flight session")]
    RadixConnectMobileFailedToCreateNewSession = 10149,

    #[error("Deferred Deep Link invalid value format {bad_value}.")]
    DeferredDeepLinkInvalidValueFormat { bad_value: String } = 10150,

    #[error("Failed updating home cards")]
    FailedUpdatingHomeCards = 10151,

    #[error("Entity not found")]
    EntityNotFound = 10152,

    #[error("Home cards not found")]
    HomeCardsNotFound = 10153,

    #[error("Failed saving home cards")]
    FailedSavingHomeCards = 10154,

    #[error(
        "Failed to load Profile from secure storage, profile id: {profile_id}"
    )]
    UnableToLoadProfileFromSecureStorage { profile_id: ProfileID } = 10155,

    #[error("Failed to save HostId to secure storage")]
    UnableToSaveHostIdToSecureStorage = 10156,

    #[error("Unable to acquire read lock for profile")]
    UnableToAcquireReadLockForProfile = 10157,

    #[error("Failed to read from unsafe storage.")]
    UnsafeStorageReadError = 10158,

    #[error("Failed to write to unsafe storage.")]
    UnsafeStorageWriteError = 10159,

    #[error("Failed to create file path from string: '{bad_value}'")]
    FailedToCreateFilePathFromString { bad_value: String } = 10160,

    #[error("Expected collection to not be empty")]
    ExpectedNonEmptyCollection = 10161,

    #[error("Failed to add all accounts, found duplicated account.")]
    UnableToAddAllAccountsDuplicatesFound = 10162,

    #[error("Profile last used on other device {other_device_id} (this device: {this_device_id})")]
    ProfileUsedOnOtherDevice {
        other_device_id: DeviceID,
        this_device_id: DeviceID,
    } = 10163,

    #[error("Failed To create DeviceID (UUID) from string: {bad_value}")]
    InvalidDeviceID { bad_value: String } = 10164,

    #[error("Tried to replace profile with one with a different ProfileID than the current one. Use `import_profile` instead.")]
    TriedToUpdateProfileWithOneWithDifferentID = 10165,

    #[error("Invalid path, bad value: '{bad_value}'")]
    InvalidPath { bad_value: String } = 10166,

    #[error("Failed to save file: '{path}'")]
    FailedToSaveFile { path: String } = 10167,

    #[error("Failed to load file: '{path}'")]
    FailedToLoadFile { path: String } = 10168,

    #[error("Failed to delete file: '{path}'")]
    FailedToDeleteFile { path: String } = 10169,

    #[error("Not permission enough to access file: '{path}'")]
    NotPermissionToAccessFile { path: String } = 10170,

    #[error("Invalid Arculus Card Model")]
    InvalidArculusCardModel { bad_value: String } = 10171,

    #[error("Expected ArculusCard factor source got something else")]
    ExpectedArculusCardFactorSourceGotSomethingElse = 10172,

    #[error("Failed to Derive Key after max attempts")]
    FailedToDeriveKeyAfterMaxAttempts = 10173,

    #[error("Failed to decrypt sealed mnemonic")]
    FailedToDecryptSealedMnemonic = 10174,

    #[error("Answers to Security Questions cannot be empty")]
    AnswersToSecurityQuestionsCannotBeEmpty = 10175,

    #[error("Integrity Violation, mutation of FactorSource is not allowed to mutate its ID")]
    IntegrityViolationMutationOfFactorSourceIsNotAllowedToMutateItsID = 10176,

    #[error("Invalid SecurityStructureID, bad value: '{bad_value}'")]
    InvalidSecurityStructureID { bad_value: String } = 10177,

    #[error(
        "Invalid SecurityStructure, it references Factors not in profile (by FactorSourceID)."
    )]
    StructureReferencesUnknownFactorSource = 10178,

    #[error("Invalid Questions and Answers count, expected: {expected}, found: {found}")]
    InvalidQuestionsAndAnswersCount { expected: u16, found: u16 } = 10179,

    #[error("No Profile is yet loaded. Current state is: {current_state}")]
    ProfileStateNotLoaded { current_state: String } = 10180,

    #[error("Failed to create Address from global_address (hex): {global_address_as_hex}, network_id: {network_id}")]
    FailedToCreateAddressFromGlobalAddressAndNetworkID {
        global_address_as_hex: String,
        network_id: NetworkID,
    } = 10181,

    #[error(
        "The provided entities do not derive from the given factor source"
    )]
    EntitiesNotDerivedByFactorSource = 10182,

    #[error("The network {network_id} does not exist in profile")]
    NoNetworkInProfile { network_id: NetworkID } = 10183,

    #[error("Empty FactorSources list")]
    FactorSourcesOfKindEmptyFactors = 10184,

    #[error("Expected Passphrase factor source got something else")]
    ExpectedPassphraseFactorSourceGotSomethingElse = 10185,

    #[error("Unknown persona.")]
    UnknownPersona = 10186,

    #[error("Invalid security structure. Threshold ({}) cannot exceed threshold factors ({}).", threshold, factors)]
    InvalidSecurityStructureThresholdExceedsFactors {
        threshold: u8,
        factors: u8,
    } = 10187,

    #[error("Invalid security structure. A factor must not be present in both threshold and override list.")]
    InvalidSecurityStructureFactorInBothThresholdAndOverride = 10188,

    #[error("One of the receiving accounts does not allow deposits")]
    OneOfReceivingAccountsDoesNotAllowDeposits = 10189,

    #[error("Failed transaction preview with status: {error_message}")]
    FailedTransactionPreview { error_message: String } = 10190,

    #[error("Failed to extract radix engine toolkit receipt bytes")]
    FailedToExtractTransactionReceiptBytes = 10191,

    #[error("Transaction Manifest contains forbidden instructions: {reserved_instructions}")]
    ReservedInstructionsNotAllowedInManifest { reserved_instructions: String } =
        10192,

    #[error("Failed to decompile bytes into TransactionIntent")]
    FailedToDecompileBytesIntoTransactionIntent = 10193,

    #[error("Invalid Transaction Manifest, failed to decompile, reason: '{underlying}'")]
    InvalidManifestFailedToDecompile { underlying: String } = 10194,

    #[error("Invalid SignedPartialTransaction, failed to decompile")]
    InvalidSignedPartialTransactionFailedToCompile = 10195,

    #[error("Invalid SignedPartialTransaction, failed to decompile")]
    InvalidSignedPartialTransactionFailedToDecompile = 10196,

    #[error("Invalid Signed Partial Transaction, failed to encode, reason: '{underlying}'")]
    InvalidSignedPartialTransactionFailedToEncode { underlying: String } =
        10197,

    #[error("Failed to generate manifest summary, reason: '{underlying}'")]
    FailedToGenerateManifestSummary { underlying: String } = 10198,

    #[error("Index Securified expected Unsecurified")]
    IndexSecurifiedExpectedUnsecurified = 10199,

    #[error("Index Unsecurified expected Securified")]
    IndexUnsecurifiedExpectedSecurified = 10200,

    #[error("Index In Global Key Space Is Lower Than Offset")]
    IndexInGlobalKeySpaceIsLowerThanOffset = 10201,

    #[error("Index Overflow")]
    IndexOverflow = 10202,

    #[error("Cannot Add To Index Since It Would Change KeySpace")]
    CannotAddMoreToIndexSinceItWouldChangeKeySpace = 10203,

    #[error("Index Not Hardened {bad_value}")]
    IndexNotHardened { bad_value: u32 } = 10204,

    #[error("Failed to decompile bytes into Subintent")]
    FailedToDecompileBytesIntoSubintent = 10205,

    #[error("FactorSource Discrepancy")]
    FactorSourceDiscrepancy = 10206,

    #[error("FactorInstancesProvider did not derive enough factors")]
    FactorInstancesProviderDidNotDeriveEnoughFactors = 10207,

    #[error("FactorInstancesCache already contains FactorInstance")]
    CacheAlreadyContainsFactorInstance { derivation_path: String } = 10208,

    #[error("Expected Account but Got Persona, address of persona {address}")]
    ExpectedAccountButGotPersona { address: String } = 10209,

    #[error("Expected Persona but Got Account, address of account {address}")]
    ExpectedPersonaButGotAccount { address: String } = 10210,

    #[error(
        "Account not securified, but was expected to be, address {address}"
    )]
    AccountNotSecurified { address: String } = 10211,

    #[error(
        "Account not securified, but was expected to be, address {address}"
    )]
    PersonaNotSecurified { address: String } = 10212,

    #[error(
        "Entity of kind {entity_kind}, on wrong network: {wrong_network}, expected: {expected_network}"
    )]
    EntityOnWrongNetwork {
        entity_kind: String,
        wrong_network: NetworkID,
        expected_network: NetworkID,
    } = 10213,

    #[error("SecurityState not securified")]
    SecurityStateNotSecurified = 10214,
}

#[uniffi::export]
pub fn error_message_from_error(error: &CommonError) -> String {
    error.to_string()
}

impl CommonError {
    pub fn error_code(&self) -> u32 {
        core::intrinsics::discriminant_value(self)
    }

    pub fn is_safe_to_show_error_message(&self) -> bool {
        matches!(self, CommonError::FailedToDeserializeJSONToValue { .. })
    }
}

#[uniffi::export]
pub fn error_code_from_error(error: &CommonError) -> u32 {
    error.error_code()
}

#[uniffi::export]
pub fn is_safe_to_show_error_message_from_error(error: &CommonError) -> bool {
    error.is_safe_to_show_error_message()
}
