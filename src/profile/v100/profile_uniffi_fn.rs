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

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Object)]
pub struct JsonContainerBytes {
    pub bytes: BagOfBytes,
}

impl From<BagOfBytes> for JsonContainerBytes {
    fn from(value: BagOfBytes) -> Self {
        Self { bytes: value }
    }
}

impl From<JsonContainerBytes> for BagOfBytes {
    fn from(value: JsonContainerBytes) -> BagOfBytes {
        value.bytes
    }
}

impl JsonContainerBytes {
    fn with_bytes(bytes: Vec<u8>) -> Arc<Self> {
        Self::new(BagOfBytes::from(bytes))
    }
}

#[uniffi::export]
impl JsonContainerBytes {
    #[uniffi::constructor]
    pub fn new(bytes: BagOfBytes) -> Arc<Self> {
        Arc::new(Self { bytes })
    }

    pub fn bytes(self: Arc<Self>) -> BagOfBytes {
        self.bytes.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Object)]
pub struct JsonContainerProfile {
    pub profile: Profile,
}

impl From<Profile> for JsonContainerProfile {
    fn from(value: Profile) -> Self {
        Self { profile: value }
    }
}

impl From<JsonContainerProfile> for Profile {
    fn from(value: JsonContainerProfile) -> Self {
        value.profile
    }
}

#[uniffi::export]
impl JsonContainerProfile {
    #[uniffi::constructor]
    pub fn new(profile: Profile) -> Arc<Self> {
        Arc::new(Self { profile })
    }
    pub fn profile(self: Arc<Self>) -> Profile {
        self.profile.clone()
    }
}

// impl JsonDataDeserializing for Profile {}
// impl JsonDataSerializing for Profile {}
json_data_convertible!(Profile);

// ################
// FROM BYTES
// ################
#[uniffi::export]
pub fn new_profile_from_json_bytes_arc_in_arc_out(
    json: Arc<JsonContainerBytes>,
) -> Result<Arc<JsonContainerProfile>> {
    Profile::new_from_json_bytes(json.bytes.as_ref())
        .map(JsonContainerProfile::new)
}

#[uniffi::export]
pub fn new_profile_from_json_bytes_not_arc_in_arc_out(
    json: &BagOfBytes,
) -> Result<Arc<JsonContainerProfile>> {
    Profile::new_from_json_bytes(json).map(JsonContainerProfile::new)
}

#[uniffi::export]
pub fn new_profile_from_json_bytes_arc_in_not_arc_out(
    json: Arc<JsonContainerBytes>,
) -> Result<Profile> {
    Profile::new_from_json_bytes(json.bytes.as_ref())
}

// ################
// TO BYTES
// ################
#[uniffi::export]
pub fn profile_to_json_bytes_arc_in_not_arc_out(
    profile: Arc<JsonContainerProfile>,
) -> BagOfBytes {
    BagOfBytes::from(profile.profile.to_json_bytes())
}

#[uniffi::export]
pub fn profile_to_json_bytes_arc_in_arc_out(
    profile: Arc<JsonContainerProfile>,
) -> Arc<JsonContainerBytes> {
    JsonContainerBytes::with_bytes(profile.profile.to_json_bytes())
}

#[uniffi::export]
pub fn profile_to_json_bytes_not_arc_in_arc_out(
    profile: &Profile,
) -> Arc<JsonContainerBytes> {
    JsonContainerBytes::with_bytes(profile.to_json_bytes())
}

// ################
// ENCRYPTION
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
            new_profile_from_json_bytes_arc_in_arc_out(
                profile_to_json_bytes_arc_in_arc_out(
                    JsonContainerProfile::new(sut.clone())
                )
            )
            .unwrap()
            .profile,
            sut
        )
    }

    #[test]
    fn deserialize_malformed() {
        let malformed_profile_snapshot = BagOfBytes::from("{}".as_bytes());
        assert_eq!(
            new_profile_from_json_bytes_arc_in_arc_out(
                JsonContainerBytes::new(malformed_profile_snapshot.clone())
            ),
            Err(CommonError::FailedToDeserializeJSONToValue {
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
}
