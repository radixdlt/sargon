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
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    
     uniffi::Record,
)]
pub struct Profile {
    /// The header of a Profile(Snapshot) contains crucial metadata
    /// about this Profile, such as which JSON data format it is
    /// compatible with and which device was used to create it and
    /// a hint about its contents.
    pub header: Header,

    /// All sources of factors, used for authorization such as spending funds, contains no
    /// secrets.
    pub factor_sources: FactorSources,

    /// Settings for this profile in the app, contains default security configs
    /// as well as display settings.
    pub app_preferences: AppPreferences,

    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    pub networks: ProfileNetworks,
}

impl From<InternalProfile> for Profile {
	fn from(value: InternalProfile) -> Self {
        Profile {
            header: value.header.into(),
            factor_sources: value.factor_sources.into(),
            app_preferences: value.app_preferences.into(),
            networks: value.networks.into(),
        }
	}
}

impl Into<InternalProfile> for Profile {
	fn into(self) -> InternalProfile {
        InternalProfile {
            header: self.header.into(),
            factor_sources: self.factor_sources.into(),
            app_preferences: self.app_preferences.into(),
            networks: self.networks.into(),
        }
	}
}

#[uniffi::export]
pub fn new_profile_from_json_string(json_str: String) -> Result<Profile> {
    InternalProfile::new_from_json_string(json_str).map_result()
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
    InternalProfile::new(mnemonic.into(), host_id.into(), host_info.into()).into()
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
        device_factor_source.into(),
        host_id.into(),
        host_info.into(),
        None::<Accounts>,
    ).into()
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
    Profile::new_from_encrypted_profile_json_string(
        json_string,
        decryption_password,
    )
}

#[uniffi::export]
pub fn profile_encrypt_with_password(
    profile: &Profile,
    encryption_password: String,
) -> String {
    profile.to_encrypted_profile_json_str(encryption_password)
}

// ################
// Analyze
// ################
#[uniffi::export]
pub fn profile_analyze_contents_of_file(
    contents: String,
) -> ProfileFileContents {
    Profile::analyze_contents_of_file(contents)
}

#[uniffi::export]
pub fn check_if_profile_json_contains_legacy_p2p_links(
    json_str: String,
) -> bool {
    Profile::check_if_profile_json_contains_legacy_p2p_links(json_str)
}

#[uniffi::export]
pub fn check_if_encrypted_profile_json_contains_legacy_p2p_links(
    json_str: String,
    password: String,
) -> bool {
    Profile::check_if_encrypted_profile_json_contains_legacy_p2p_links(
        json_str, password,
    )
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_profile_sample());
        assert_eq!(SUT::sample_other(), new_profile_sample_other());
    }

    #[test]
    fn test_profile_analyze_contents_of_file() {
        assert_eq!(
            profile_analyze_contents_of_file(
                "ring ring ring ring ring ring, banana phone!".to_owned()
            ),
            ProfileFileContents::NotProfile
        )
    }

    #[test]
    fn test_new_with_mnemonic() {
        assert_eq!(
            new_profile_with_mnemonic(
                Mnemonic::sample(),
                HostId::sample(),
                HostInfo::sample()
            )
            .bdfs()
            .id,
            Profile::new(
                Mnemonic::sample(),
                HostId::sample(),
                HostInfo::sample()
            )
            .bdfs()
            .id,
        );
    }

    #[test]
    fn new_private_hd() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let lhs = new_profile(
            private.factor_source.clone(),
            HostId::sample(),
            HostInfo::sample(),
        );
        assert_eq!(
            lhs.bdfs().factor_source_id(),
            private.factor_source.factor_source_id()
        );
    }

    #[test]
    fn to_string_and_debug_string() {
        assert!(
            profile_to_debug_string(&SUT::sample()).len()
                > profile_to_string(&SUT::sample()).len()
        );
        assert_ne!(
            profile_to_debug_string(&SUT::sample()),
            profile_to_debug_string(&SUT::sample_other())
        );
    }

    #[test]
    fn encryption_roundtrip() {
        let sut = SUT::sample();
        let password = "super secret".to_owned();
        let encrypted = profile_encrypt_with_password(&sut, password.clone());
        assert_eq!(
            new_profile_from_encryption_bytes(encrypted, password).unwrap(),
            sut
        );
    }

    #[test]
    fn check_if_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present(
    ) {
        let json = r#"
        {
            "appPreferences": {
              "p2pLinks": [
                {
                  "connectionPassword": "babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe",
                  "displayName": "Brave on PC"
                }
              ]
            }
          }
        "#;
        assert!(check_if_profile_json_contains_legacy_p2p_links(
            json.to_owned()
        ));
    }

    #[test]
    fn check_if_profile_json_contains_legacy_p2p_links_when_empty_json() {
        assert!(!check_if_profile_json_contains_legacy_p2p_links(
            "".to_owned()
        ));
    }

    // #[test]
    // fn check_if_encrypted_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present(
    // ) {
    //     let json =
    //         serde_json::to_string(&EncryptedProfileSnapshot::sample()).unwrap();
    //     let password = "babylon";
    //     assert!(check_if_encrypted_profile_json_contains_legacy_p2p_links(
    //         json,
    //         password.to_owned()
    //     ));
    // }

    #[test]
    fn check_if_encrypted_profile_json_contains_legacy_p2p_links_when_empty_json(
    ) {
        let password = "babylon";
        assert!(!check_if_encrypted_profile_json_contains_legacy_p2p_links(
            "".to_owned(),
            password.to_owned()
        ));
    }

    #[test]
    fn profile_json_string_roundtrip() {
        let sut = SUT::sample();
        let pretty_string = profile_to_json_string(&sut, false);
        let from_str =
            new_profile_from_json_string(pretty_string.clone()).unwrap();
        assert_eq!(from_str, sut);
        let ugly_string = profile_to_json_string(&sut, true);
        let from_str =
            new_profile_from_json_string(ugly_string.clone()).unwrap();
        assert_eq!(from_str, sut);
        assert_ne!(pretty_string, ugly_string);
    }

    #[test]
    fn profile_from_invalid_json_string_throws() {
        assert_eq!(
            new_profile_from_json_string("".to_owned()),
            Err(CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: 0,
                type_name: "Profile".to_owned(),
                serde_message: "EOF while parsing a value at line 1 column 0"
                    .to_string()
            })
        )
    }
}
