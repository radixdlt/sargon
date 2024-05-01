use std::ops::DerefMut;

use crate::prelude::*;

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

json_data_convertible!(Profile);

// ################
// FROM BYTES
// ################
#[uniffi::export]
pub fn new_profile_from_json_bytes_fast_by_ref(
    reference: Arc<RefBytes>,
) -> Result<Arc<RefProfile>> {
    Profile::new_from_json_bytes(reference.take().unwrap().as_ref())
        .map(RefProfile::new)
}

// ################
// TO BYTES
// ################
#[uniffi::export]
pub fn profile_to_json_bytes_fast_by_ref(
    reference: Arc<RefProfile>,
) -> Arc<RefBytes> {
    RefBytes::new(BagOfBytes::from(reference.take().unwrap().to_json_bytes()))
}

// ################
// ENCRYPTION
// ################
#[uniffi::export]
pub fn new_profile_from_encryption_bytes_fast_by_ref(
    reference: Arc<RefBytes>,
    decryption_password: String,
) -> Result<Arc<RefProfile>> {
    Profile::new_from_encryption_bytes(
        reference.take().unwrap().as_ref(),
        decryption_password,
    )
    .map(RefProfile::new)
}

#[uniffi::export]
pub fn profile_encrypt_with_password_fast_by_ref(
    reference: Arc<RefProfile>,
    encryption_password: String,
) -> Arc<RefBytes> {
    RefBytes::new(BagOfBytes::from(
        reference
            .take()
            .unwrap()
            .to_encryption_bytes(encryption_password),
    ))
}

#[uniffi::export]
pub fn profile_analyze_contents_of_file_fast_by_ref(
    reference: Arc<RefBytes>,
) -> ProfileFileContents {
    Profile::analyze_contents_of_file(reference.take().unwrap().as_ref())
}

// #########
// SLOWer not by ref
// #########

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

#[uniffi::export]
pub fn profile_analyze_contents_of_file(
    bytes: BagOfBytes,
) -> ProfileFileContents {
    Profile::analyze_contents_of_file(bytes)
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
            profile_analyze_contents_of_file_fast_by_ref(RefBytes::new(
                BagOfBytes::sample()
            )),
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
        assert_eq!(profile_to_string(&SUT::sample()).len(), 4444);
        assert_eq!(profile_to_debug_string(&SUT::sample()).len(), 27410);
        assert_ne!(
            profile_to_debug_string(&SUT::sample()),
            profile_to_debug_string(&SUT::sample_other())
        );
    }

    #[test]
    fn serialize_deserialize() {
        let sut = SUT::sample();

        assert_eq!(
            new_profile_from_json_bytes_fast_by_ref(
                profile_to_json_bytes_fast_by_ref(RefProfile::new(sut.clone()))
            )
            .unwrap()
            .take()
            .unwrap(),
            sut
        )
    }

    #[test]
    fn deserialize_malformed() {
        let malformed_profile_snapshot = BagOfBytes::from("{}".as_bytes());
        assert_eq!(
            new_profile_from_json_bytes_fast_by_ref(RefBytes::new(
                malformed_profile_snapshot.clone()
            ))
            .err()
            .unwrap(),
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: malformed_profile_snapshot.len() as u64,
                type_name: String::from("Profile")
            }
        );
    }

    #[test]
    fn test_new_profile_from_encryption_bytes() {
        assert!(new_profile_from_encryption_bytes_fast_by_ref(
            RefBytes::new(BagOfBytes::sample()),
            "invalid".to_string()
        )
        .is_err());
    }

    #[test]
    fn encryption_roundtrip() {
        let sut = SUT::sample();

        let password = "super secret".to_owned();
        let encryption_bytes = profile_encrypt_with_password_fast_by_ref(
            RefProfile::new(sut.clone()),
            password.clone(),
        );
        assert_eq!(
            new_profile_from_encryption_bytes_fast_by_ref(
                encryption_bytes,
                password
            )
            .unwrap(),
            RefProfile::new(sut.clone())
        );
    }

    #[test]
    fn test_profile_analyze_contents_of_file_by_value() {
        assert_eq!(
            profile_analyze_contents_of_file(BagOfBytes::sample()),
            ProfileFileContents::NotProfile
        )
    }

    #[test]
    fn serialize_deserialize_by_value() {
        let sut = SUT::sample();

        assert_eq!(
            new_profile_from_json_bytes(&profile_to_json_bytes(&sut)).unwrap(),
            sut
        )
    }
}
