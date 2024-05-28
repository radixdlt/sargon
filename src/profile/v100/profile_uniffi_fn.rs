use crate::prelude::*;

#[uniffi::export]
pub fn new_profile_from_json_string(json_str: String) -> Result<Profile> {
    Profile::new_from_json_string(json_str)
}

impl Profile {
    pub fn new_from_json_string(json_str: impl AsRef<str>) -> Result<Profile> {
        let json_str = json_str.as_ref();
        let json_byte_count = json_str.len() as u64;
        serde_json::from_str(json_str).map_err(|_| {
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count,
                type_name: type_name::<Profile>(),
            }
        })
    }
}

#[uniffi::export]
pub fn profile_to_json_string(
    profile: &Profile,
    pretty_printed: bool,
) -> String {
    profile.to_json_string(pretty_printed)
}

impl Profile {
    pub fn to_json_string(&self, pretty_printed: bool) -> String {
        if pretty_printed {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
        .expect("Should always be able to JSON encode Profile.")
    }
}

#[uniffi::export]
pub fn new_profile_with_mnemonic(
    mnemonic: Mnemonic,
    device_info: DeviceInfo,
) -> Profile {
    Profile::new(mnemonic, device_info)
}

/// # Panics
/// Panics if `device_factor_source` is not a main BDFS.
#[uniffi::export]
pub fn new_profile(
    device_factor_source: DeviceFactorSource,
    device_info: DeviceInfo,
) -> Profile {
    Profile::from_device_factor_source(device_factor_source, device_info)
}

#[uniffi::export]
pub fn new_profile_sample() -> Profile {
    Profile::sample()
}

#[uniffi::export]
pub fn new_profile_sample_other() -> Profile {
    Profile::sample_other()
}

#[uniffi::export]
pub fn profile_to_string(profile: &Profile) -> String {
    format!("{}", profile)
}

#[uniffi::export]
pub fn profile_to_debug_string(profile: &Profile) -> String {
    format!("{:?}", profile)
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
            new_profile_with_mnemonic(Mnemonic::sample(), DeviceInfo::sample())
                .bdfs()
                .id,
            Profile::new(Mnemonic::sample(), DeviceInfo::sample())
                .bdfs()
                .id,
        );
    }

    #[test]
    fn new_private_hd() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let lhs =
            new_profile(private.factor_source.clone(), DeviceInfo::sample());
        assert_eq!(
            lhs.bdfs().factor_source_id(),
            private.factor_source.factor_source_id()
        );
    }

    #[test]
    fn to_string_and_debug_string() {
        assert_eq!(profile_to_string(&SUT::sample()).len(), 4276);
        assert_eq!(profile_to_debug_string(&SUT::sample()).len(), 27200);
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

    #[test]
    fn check_if_encrypted_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present(
    ) {
        let json =
            serde_json::to_string(&EncryptedProfileSnapshot::sample()).unwrap();
        let password = "babylon";
        assert!(check_if_encrypted_profile_json_contains_legacy_p2p_links(
            json,
            password.to_owned()
        ));
    }

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
                type_name: "Profile".to_owned()
            })
        )
    }
}
