use crate::prelude::*;
use thiserror::Error as ThisError;

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

#[repr(u32)]
#[derive(Clone, Debug, Eq, ThisError, PartialEq)]
pub enum CommonError {
    #[error("Unknown Error")]
    Unknown = 10000,

    #[error("Failed to create Ed25519 Private key from bytes {bad_value:?}")]
    InvalidEd25519PrivateKeyFromBytes { bad_value: String } = 10001,

    #[error("Failed to create Ed25519 Private key from String {bad_value}.")]
    InvalidEd25519PrivateKeyFromString { bad_value: String } = 10002,

    #[error(
        "Failed to create Secp256k1 Private key from bytes {bad_value:?}."
    )]
    InvalidSecp256k1PrivateKeyFromBytes { bad_value: String } = 10003,

    #[error(
        "Failed to create Secp256k1 Private key from String {bad_value:?}."
    )]
    InvalidSecp256k1PrivateKeyFromString { bad_value: String } = 10004,

    #[error("Failed to create Ed25519 Public key from bytes {bad_value:?}.")]
    InvalidEd25519PublicKeyFromBytes { bad_value: String } = 10005,

    #[error("Failed to create Ed25519 Public key from String {bad_value}.")]
    InvalidEd25519PublicKeyFromString { bad_value: String } = 10006,

    #[error("Failed to create Secp256k1 Public key from bytes {bad_value:?}.")]
    InvalidSecp256k1PublicKeyFromBytes { bad_value: String } = 10007,

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
    WrongEntityKind { expected: String, found: String } = 10024,

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
    AccountOnWrongNetwork { expected: String, found: String } = 10053,

    #[error("FactorSources must not be empty.")]
    FactorSourcesMustNotBeEmpty = 10054,

    #[error("Failed to update FactorSource, error while mutating.")]
    UpdateFactorSourceMutateFailed = 10055,

    #[error("Failed to cast factor source, wrong kind, , expected: {expected}, found: {found}")]
    CastFactorSourceWrongKind { expected: String, found: String } = 10056,

    #[error("Length check failed, expected: {expected}, found: {found}, data: {data:?}")]
    InvalidLength {
        expected: u64,
        found: u64,
        data: String,
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

    #[error("Failed to access secure storage due to \"{error_message}\" for key {} ", key)]
    SecureStorageAccessError {
        key: String,
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
    ProfileDoesNotContainFactorSourceWithID { bad_value: String } = 10073,

    #[error("Account Already Present {bad_value}")]
    AccountAlreadyPresent { bad_value: String } = 10074,

    #[error("Unable to acquire write lock for Profile.")]
    UnableToAcquireWriteLockForProfile = 10075,

    #[error("Failed save Mnemonic to SecureStorageDriver with FactorSourceID: {bad_value}")]
    UnableToSaveMnemonicToSecureStorage { bad_value: String } = 10076,

    #[error(
        "Failed load Mnemonic from SecureStorageDriver with FactorSourceID: {bad_value}"
    )]
    UnableToLoadMnemonicFromSecureStorage { bad_value: String } = 10077,

    #[error("Failed save FactorSource to SecureStorageDriver, FactorSourceID: {bad_value}")]
    UnableToSaveFactorSourceToProfile { bad_value: String } = 10078,

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
        network_id: String,
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
        found_in_instructions: String,
        specified_to_instructions_ctor: String,
    } = 10093,

    #[error("Failed to decode bytes into Transaction Manifest Instructions")]
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

    #[error("Networking response bad code: {code}")]
    NetworkResponseBadCode { code: u16 } = 10112,

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
    NetworkDiscrepancy { expected: String, actual: String } = 10129,

    #[error("Discrepancy, Authorized Dapp references Persona which does not exist {address}")]
    DiscrepancyAuthorizedDappReferencedPersonaWhichDoesNotExist {
        address: String,
    } = 10130,

    #[error("Discrepancy, Authorized Dapp references Account which does not exist {address}")]
    DiscrepancyAuthorizedDappReferencedAccountWhichDoesNotExist {
        address: String,
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
    InvalidKeyAgreementPublicKeyFromBytes { bad_value: String } = 10138,

    #[error(
        "Failed to create KeyAgreementPrivateKey from bytes: {bad_value:?}"
    )]
    InvalidKeyAgreementPrivateKeyFromBytes { bad_value: String } = 10139,

    #[error("RadixConnectMobileSession not found, session id: {session_id}")]
    RadixConnectMobileSessionNotFound { session_id: String } = 10140,

    #[error("RadixConnectMobileDappRequest not found, interaction id: {interaction_id}")]
    RadixConnectMobileDappRequestNotFound { interaction_id: String } = 10141,

    #[error("RadixConnectMobileDappCallbackPath not found, origin: {origin}")]
    RadixConnectMobileDappCallbackPathNotFound { origin: String } = 10142,

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
    UnableToLoadProfileFromSecureStorage { profile_id: String } = 10155,

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

    #[error("Failed to add all entities, found duplicated entity.")]
    UnableToAddAllEntitiesDuplicatesFound = 10162,

    #[error("Profile last used on other device {other_device_id} (this device: {this_device_id})")]
    ProfileUsedOnOtherDevice {
        other_device_id: String,
        this_device_id: String,
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
        "Invalid SecurityStructure, it references Factors not in profile, FactorSourceID {bad_value}."
    )]
    StructureReferencesUnknownFactorSource { bad_value: String } = 10178,

    #[error("Invalid Questions and Answers count, expected: {expected}, found: {found}")]
    InvalidQuestionsAndAnswersCount { expected: u16, found: u16 } = 10179,

    #[error("No Profile is yet loaded. Current state is: {current_state}")]
    ProfileStateNotLoaded { current_state: String } = 10180,

    #[error("Failed to create Address from manifest address (hex): {manifest_address}, network_id: {network_id}")]
    FailedToCreateAddressFromManifestAddressAndNetworkID {
        manifest_address: String,
        network_id: String,
    } = 10181,

    #[error(
        "The provided entities do not derive from the given factor source"
    )]
    EntitiesNotDerivedByFactorSource = 10182,

    #[error("The network {network_id} does not exist in profile")]
    NoNetworkInProfile { network_id: String } = 10183,

    #[error("Empty FactorSources list")]
    FactorSourcesOfKindEmptyFactors = 10184,

    #[error("Expected Password factor source got something else")]
    ExpectedPasswordFactorSourceGotSomethingElse = 10185,

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

    #[error("Subintent has already expired")]
    SubintentExpired = 10206,

    #[error("UNKNOWN ERROR (FREE_TO_USE)")]
    #[allow(non_camel_case_types)]
    FREE_TO_USE = 10207,

    #[error("Unable to make {amount} transfers in one single transaction")]
    MaxTransfersPerTransactionReached { amount: u64 } = 10208,

    #[error("Transaction Manifest class is reserved: {class}")]
    ReservedManifestClass { class: String } = 10209,

    #[error("FactorInstancesProvider did not derive enough factors")]
    FactorInstancesProviderDidNotDeriveEnoughFactors = 10210,

    #[error("FactorInstancesCache already contains FactorInstance")]
    CacheAlreadyContainsFactorInstance { derivation_path: String } = 10211,

    #[error("Expected Persona but Got Account, address of account {address}")]
    ExpectedPersonaButGotAccount { address: String } = 10212,

    #[error(
        "Account not securified, but was expected to be, address {address}"
    )]
    AccountNotSecurified { address: String } = 10213,

    #[error(
        "Account not securified, but was expected to be, address {address}"
    )]
    PersonaNotSecurified { address: String } = 10214,

    #[error(
        "Entity of kind {entity_kind}, on wrong network: {wrong_network}, expected: {expected_network}"
    )]
    EntityOnWrongNetwork {
        entity_kind: String,
        wrong_network: String,
        expected_network: String,
    } = 10215,

    #[error("SecurityState not securified")]
    SecurityStateNotSecurified = 10216,

    #[error("SecurityState securified but expected unsecurified")]
    SecurityStateSecurifiedButExpectedUnsecurified = 10217,

    #[error("Failed to add all personas, found duplicated persona.")]
    UnableToAddAllPersonasDuplicatesFound = 10218,

    #[error("Missing Factor Mapping Instances Into RoleWithFactors.")]
    MissingFactorMappingInstancesIntoRole = 10219,

    #[error("FactorSource Discrepancy")]
    FactorSourceDiscrepancy = 10220,

    #[error("Expected Account but Got Persona, address of persona {address}")]
    ExpectedAccountButGotPersona { address: String } = 10221,

    #[error("Invalid FactorSourceIDFromHash String, wrong component count, expected: {expected}, found: {found}")]
    InvalidFactorSourceIDFromHashStringWrongComponentCount {
        expected: u64,
        found: u64,
    } = 10222,

    #[error("File already exists, path: {path}")]
    FileAlreadyExists { path: String } = 10223,

    #[error("Failed to canonicalize path: {path}")]
    FailedToCanonicalize { path: String } = 10224,

    #[error("Factor Instances discrepancy in address_of_entity1: {address_of_entity1}, address_of_entity2: {address_of_entity2}, factor source id: {factor_source_id}")]
    FactorInstancesDiscrepancy {
        address_of_entity1: String,
        address_of_entity2: String,
        factor_source_id: String,
    } = 10225,

    #[error("Failed to add all accounts, found duplicated accounts.")]
    UnableToAddAllAccountsDuplicatesFound = 10226,

    #[error("Invalid Index Agnostic Path, wrong length")]
    InvalidIndexAgnosticPathWrongLength = 10227,

    #[error("Invalid Index Agnostic Path, does not end with suffix")]
    InvalidIndexAgnosticPathDoesNotEndWithSuffix = 10228,

    #[error("Failed to encode transaction preview v2 - '{underlying}'")]
    FailedToEncodeTransactionPreviewV2 { underlying: String } = 10229,

    #[error("Could not validate signature for the given input.")]
    InvalidHDSignature = 10230,

    #[error("Could not validate signature for the given rola challenge.")]
    InvalidSignatureForRolaChallenge = 10231,

    #[error("User aborted the interaction with sargon on host.")]
    HostInteractionAborted = 10232,

    #[error("Failed to automatically build shield, reason: '{underlying}'")]
    AutomaticShieldBuildingFailure { underlying: String } = 10233,

    #[error("No Transaction Signing Factor")]
    NoTransactionSigningFactorInstance = 10234,

    #[error("Authentication Signing FactorInstance not securified")]
    AuthenticationSigningFactorInstanceNotSecurified = 10235,

    #[error("SecurityEntityControl has no QueuedTransaction, unable to mark it as cancelled")]
    SecurityEntityControlHasNoProvisionallyQueuedTransaction = 10236,

    #[error("SecurityEntityControl has derived instances, which would be lost if discarded. Implement a way to put them back in the cache.")]
    SecurityEntityControlCannotChangeProvisionalAlreadyDerivedInstances = 10237,

    #[error("SecurityEntityControl has QueuedTransaction, unable override it, use `cancel_queued_transaction`")]
    SecurityEntityControlCannotChangeProvisionalAlreadyHasQueuedTransaction =
        10238,

    #[error(
        "Entity kind of FactorInstances does not match EntityKind of entity"
    )]
    SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind {
        entity_kind_of_entity: String,
        entity_kind_of_factor_instances: String,
    } = 10239,

    #[error(
        "Cannot securify entity it is already securified according to profile"
    )]
    CannotSecurifyEntityItIsAlreadySecurifiedAccordingToProfile = 10240,

    #[error("Cannot securify entity that has provisional security config")]
    CannotSecurifyEntityHasProvisionalSecurityConfig = 10241,

    #[error("Too few FactorInstances derived")]
    TooFewFactorInstancesDerived = 10242,

    #[error("Missing Authentication Signing FactorInstance mapping SecurityStructureOfFactorSources into SecurityStructureOfFactorInstances.")]
    MissingRolaKeyForSecurityStructureOfFactorInstances = 10243,

    #[error("SecurityStateAccessController address mismatch")]
    SecurityStateAccessControllerAddressMismatch = 10244,

    #[error("Not all signatures are produced with the same factor source.")]
    FactorOutcomeSignedFactorSourceIDMismatch = 10245,

    #[error("Unknown SecurityStructureID {id}")]
    UnknownSecurityStructureID { id: String } = 10246,

    #[error("Signing failed due to too many factor sources were neglected.")]
    SigningFailedTooManyFactorSourcesNeglected = 10247,

    #[error(
        "SecurityStructure already exists in profile, FactorSourceID {bad_value}."
    )]
    StructureAlreadyExists { bad_value: String } = 10248,

    #[error(
        "Tried to create {address_kind}Address with wrong entity type: {entity_type}, for node id: {node_id_as_hex}"
    )]
    AddressInvalidEntityType {
        address_kind: String,
        entity_type: u8,
        node_id_as_hex: String,
    } = 10249,

    #[error(
        "Tried to create an Address with node id: {node_id_as_hex} which does not have entity type"
    )]
    AddressNodeIdNoEntityType { node_id_as_hex: String } = 10250,

    #[error(
        "Failed to find network id from Bech32m string: {bech32m_encoded_address}"
    )]
    FailedToFindNetworkIdFromBech32mString { bech32m_encoded_address: String } =
        10251,

    #[error("Invalid NodeId length: {actual}, expected: {expected}")]
    InvalidNodeIdLength { expected: usize, actual: usize } = 10252,

    #[error("No entity found with AccessController address {bad_value}")]
    NoEntityFoundWithAccessControllerAddress { bad_value: String } = 10253,

    #[error("Failed to cast Address to specific type '{expected_specific_type}', from value: '{got_value}'")]
    FailedToMapAddressToSpecificType {
        expected_specific_type: String,
        got_value: String,
    } = 10254,

    #[error("Payer cannot be in batch of entities applying shield")]
    PayerCannotBeInBatchOfEntitiesApplyingShield = 10255,

    #[error("No XRD balance fetched for entity applying shield (or XRD Vault of AC), address {address}")]
    NoXrdBalanceFetchedForEntityApplyingShieldOrItsVault { address: String } =
        10256,

    #[error("No XRD balance fetched for payer of application of shield, address_of_payer {address_of_payer}")]
    NoXrdBalanceFetchedForPayerOfApplicationOfShield {
        address_of_payer: String,
    } = 10257,

    #[error("Unable to contribute to AccessControllers Xrd Vault, insufficient balance of payer {payer}, vault of entity {vault_of_entity}, payer balance {payer_balance}, needed balance {needed_balance}")]
    UnableContributeToAcXrdVaultInsufficientBalanceOfPayer {
        payer: String,
        vault_of_entity: String,
        payer_balance: String,
        needed_balance: String,
    } = 10258,

    #[error("Unable to contribute to AccessControllers Xrd Vault, persona requires payer")]
    UnableContributeToAcXrdVaultPersonaRequiresPayer = 10259,

    #[error("Unable to top up Xrd Vault, payer is entity applying shield: {payer_is_entity_applying_shield}, can exercise primary role: {can_exercise_primary_role} for entity owning AccessController: {entity_owning_access_controller}")]
    UnableToTopUpXrdVault {
        entity_owning_access_controller: String,
        payer_is_entity_applying_shield: bool,
        can_exercise_primary_role: bool,
    } = 10260,

    #[error("Unsecurified Personas require an account fee payer, but none was provided, for persona with address: {identity_address}")]
    UnsecurifiedPersonasRequireAnAccountFeePayerButNoneWasProvided {
        identity_address: String,
    } = 10261,

    #[error("Named addresses are not supported")]
    NamedAddressesAreNotSupported = 10262,

    #[error("Entity has no provisional security config set")]
    EntityHasNoProvisionalSecurityConfigSet = 10263,

    #[error("Entity's provisional config is wrong. It is expected to be in instances derived state.")]
    ProvisionalConfigInWrongStateExpectedInstancesDerived = 10264,

    #[error("Entity {entity_bech32m_encoded_address} is not controller by access controller on ledger")]
    EntityIsNotControlledByAnAccessControllerOnLedger {
        entity_bech32m_encoded_address: String,
    } = 10265,

    #[error("Invalid mnemonic words")]
    InvalidMnemonicWords { indices_in_mnemonic: Vec<usize> } = 10266,

    #[error("Factor source already exists")]
    FactorSourceAlreadyExists = 10267,

    #[error("Missing NFT Data field {field}")]
    MissingNFTDataField { field: String } = 10268,

    #[error("Unexpected NFT Data format")]
    UnexpectedNFTDataFormat = 10269,

    #[error("Invalid RNS domain")]
    RnsInvalidDomain = 10270,

    #[error("Unauthentic RNS domain: {reason}")]
    RnsUnauthenticDomain { reason: String } = 10271,

    #[error("Invalid RNS domain configuration: {reason}")]
    RnsInvalidDomainConfiguration { reason: String } = 10272,

    #[error("RNS unsupported network: {network}")]
    RnsUnsupportedNetwork { network: u8 } = 10273,
}

impl CommonError {
    pub fn error_code(&self) -> u32 {
        core::intrinsics::discriminant_value(self)
    }

    pub fn is_safe_to_show_error_message(&self) -> bool {
        matches!(self, CommonError::FailedToDeserializeJSONToValue { .. })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn error_message() {
        let sut = CommonError::UnknownNetworkForID { bad_value: 0 };
        assert_eq!(sut.to_string(), "No network found with id: '0'");
    }

    #[test]
    fn error_code() {
        let sut = CommonError::UnknownNetworkForID { bad_value: 0 };
        assert_eq!(sut.error_code(), 10049);
    }

    #[test]
    fn is_safe_to_show_error_message() {
        let sut = CommonError::FailedToDeserializeJSONToValue {
            json_byte_count: 100,
            type_name: "TypeName".to_string(),
            serde_message: "message".to_string(),
        };
        assert!(sut.is_safe_to_show_error_message());
    }

    #[test]
    fn is_not_safe_to_show_error_message() {
        let sut = CommonError::UnknownNetworkForID { bad_value: 0 };
        assert!(!sut.is_safe_to_show_error_message());
    }
}
