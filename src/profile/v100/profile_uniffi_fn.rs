use crate::prelude::*;

json_data_convertible!(Profile);

#[uniffi::export]
pub fn new_profile_from_json_string(json: String) -> Result<Profile> {
    serde_json::from_str(&json).map_err(|_| {
        CommonError::FailedToDeserializeJSONToValue {
            json_byte_count: json.len() as u64,
            type_name: type_name::<Profile>(),
        }
    })
}

#[uniffi::export]
pub fn profile_to_json_string(
    profile: &Profile,
    pretty_printed: bool,
) -> String {
    if pretty_printed {
        serde_json::to_string_pretty(profile)
    } else {
        serde_json::to_string(profile)
    }
    .expect("Should always be able to JSON encode Profile.")
}

#[uniffi::export]
pub fn new_profile(
    device_factor_source: DeviceFactorSource,
    creating_device_name: String,
) -> Profile {
    Profile::new(device_factor_source, creating_device_name.as_str())
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
    json: BagOfBytes,
    decryption_password: String,
) -> Result<Profile> {
    Profile::new_from_encryption_bytes(json.to_vec(), decryption_password)
}

#[uniffi::export]
pub fn profile_encrypt_with_password(
    profile: &Profile,
    encryption_password: String,
) -> BagOfBytes {
    profile.to_encryption_bytes(encryption_password).into()
}

// ################
// Analyze
// ################
#[uniffi::export]
pub fn profile_analyze_contents_of_file(
    bytes: BagOfBytes,
) -> ProfileFileContents {
    Profile::analyze_contents_of_file(bytes)
}

#[uniffi::export]
pub fn check_if_profile_json_contains_legacy_p2p_links(
    json: BagOfBytes,
) -> bool {
    Profile::check_if_profile_json_contains_legacy_p2p_links(json.to_vec())
}

#[uniffi::export]
pub fn check_if_encrypted_profile_json_contains_legacy_p2p_links(
    json: BagOfBytes,
    password: String,
) -> bool {
    Profile::check_if_encrypted_profile_json_contains_legacy_p2p_links(
        json.to_vec(),
        password,
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
            profile_analyze_contents_of_file(BagOfBytes::sample()),
            ProfileFileContents::NotProfile
        )
    }

    #[test]
    fn new_private_hd() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let lhs = super::new_profile(
            private.factor_source.clone(),
            "iPhone".to_string(),
        );
        assert_eq!(
            lhs.bdfs().factor_source_id(),
            private.factor_source.factor_source_id()
        );
    }

    #[test]
    fn to_string_and_debug_string() {
        assert_eq!(profile_to_string(&SUT::sample()).len(), 4314);
        assert_eq!(profile_to_debug_string(&SUT::sample()).len(), 27076);
        assert_ne!(
            profile_to_debug_string(&SUT::sample()),
            profile_to_debug_string(&SUT::sample_other())
        );
    }

    #[test]
    fn serialize_deserialize() {
        let sut = SUT::sample();

        assert_eq!(
            new_profile_from_json_bytes(&profile_to_json_bytes(&sut)).unwrap(),
            sut
        )
    }

    #[test]
    fn deserialize_malformed() {
        let malformed_profile_snapshot = BagOfBytes::from("{}".as_bytes());
        assert_eq!(
            new_profile_from_json_bytes(&malformed_profile_snapshot),
            Result::Err(CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: malformed_profile_snapshot.len() as u64,
                type_name: String::from("Profile")
            })
        );
    }

    #[test]
    fn test_new_profile_from_encryption_bytes() {
        assert!(new_profile_from_encryption_bytes(
            BagOfBytes::sample(),
            "invalid".to_string()
        )
        .is_err());
    }

    #[test]
    fn encryption_roundtrip() {
        let sut = SUT::sample();
        let password = "super secret".to_owned();
        let encryption_bytes =
            profile_encrypt_with_password(&sut, password.clone());
        assert_eq!(
            new_profile_from_encryption_bytes(encryption_bytes, password)
                .unwrap(),
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
            BagOfBytes::from(json.as_bytes())
        ));
    }

    #[test]
    fn check_if_profile_json_contains_legacy_p2p_links_when_empty_json() {
        assert!(!check_if_profile_json_contains_legacy_p2p_links(
            BagOfBytes::new()
        ));
    }

    #[test]
    fn check_if_encrypted_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present(
    ) {
        let json =
            serde_json::to_vec(&EncryptedProfileSnapshot::sample()).unwrap();
        let password = "babylon";
        assert!(check_if_encrypted_profile_json_contains_legacy_p2p_links(
            BagOfBytes::from(json),
            password.to_owned()
        ));
    }

    #[test]
    fn check_if_encrypted_profile_json_contains_legacy_p2p_links_when_empty_json(
    ) {
        let password = "babylon";
        assert!(!check_if_encrypted_profile_json_contains_legacy_p2p_links(
            BagOfBytes::new(),
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
}
