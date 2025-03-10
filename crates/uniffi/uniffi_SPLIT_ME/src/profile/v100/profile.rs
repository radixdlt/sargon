use crate::prelude::*;
use sargon::Profile as InternalProfile;

/// The canonical representation of a users accounts, personas,
/// authorized dapps, security factors, settings and more.
///
/// This large structure of values is called 'wallet backup data'
/// in user facing tests in host applications, but internally at
/// RDX Works known as "the Profile".
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// assert_eq!(Profile::sample(), Profile::sample())
/// ```
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct Profile {
    /// The header of a Profile(Snapshot) contains crucial metadata
    /// about this Profile, such as which JSON data format it is
    /// compatible with and which device was used to create it and
    /// a hint about its contents.
    pub header: Header,

    /// All sources of factors, used for authorization such as spending funds, contains no
    /// secrets.
    pub factor_sources: Vec<FactorSource>,

    /// Settings for this profile in the app, contains default security configs
    /// as well as display settings.
    pub app_preferences: AppPreferences,

    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    pub networks: Vec<ProfileNetwork>,
}

#[uniffi::export]
pub fn new_profile_from_json_string(json_str: String) -> Result<Profile> {
    InternalProfile::new_from_json_string(json_str).into_result()
}

#[uniffi::export]
pub fn profile_to_json_string(
    profile: &Profile,
    pretty_printed: bool,
) -> String {
    profile.into_internal().to_json_string(pretty_printed)
}

#[uniffi::export]
pub fn new_profile_with_mnemonic(
    mnemonic: Mnemonic,
    host_id: HostId,
    host_info: HostInfo,
) -> Profile {
    InternalProfile::new(
        mnemonic.into_internal(),
        host_id.into_internal(),
        host_info.into_internal(),
    )
    .into()
}

/// # Panics
/// Panics if `device_factor_source` is not a main BDFS.
#[uniffi::export]
pub fn new_profile(
    device_factor_source: DeviceFactorSource,
    host_id: HostId,
    host_info: HostInfo,
) -> Profile {
    InternalProfile::from_device_factor_source(
        device_factor_source.into_internal(),
        host_id.into_internal(),
        host_info.into_internal(),
        None::<sargon::Accounts>,
    )
    .into()
}

#[uniffi::export]
pub fn new_profile_sample() -> Profile {
    InternalProfile::sample().into()
}

#[uniffi::export]
pub fn new_profile_sample_other() -> Profile {
    InternalProfile::sample_other().into()
}

#[uniffi::export]
pub fn profile_to_string(profile: &Profile) -> String {
    format!("{}", profile.into_internal())
}

#[uniffi::export]
pub fn profile_to_debug_string(profile: &Profile) -> String {
    format!("{:?}", profile.into_internal())
}

// ################
// Encryption
// ################

#[uniffi::export]
pub fn new_profile_from_encryption_bytes(
    json_string: String,
    decryption_password: String,
) -> Result<Profile> {
    InternalProfile::new_from_encrypted_profile_json_string(
        json_string,
        decryption_password,
    )
    .into_result()
}

#[uniffi::export]
pub fn profile_encrypt_with_password(
    profile: &Profile,
    encryption_password: String,
) -> String {
    profile
        .into_internal()
        .to_encrypted_profile_json_str(encryption_password)
}

// ################
// Analyze
// ################
#[uniffi::export]
pub fn profile_analyze_contents_of_file(
    contents: String,
) -> ProfileFileContents {
    InternalProfile::analyze_contents_of_file(contents).into()
}

#[uniffi::export]
pub fn check_if_profile_json_contains_legacy_p2p_links(
    json_str: String,
) -> bool {
    InternalProfile::check_if_profile_json_contains_legacy_p2p_links(json_str)
}

#[uniffi::export]
pub fn check_if_encrypted_profile_json_contains_legacy_p2p_links(
    json_str: String,
    password: String,
) -> bool {
    InternalProfile::check_if_encrypted_profile_json_contains_legacy_p2p_links(
        json_str, password,
    )
}
