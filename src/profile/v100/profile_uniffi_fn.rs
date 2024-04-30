use std::ops::DerefMut;

use crate::prelude::*;

json_data_convertible!(Profile);

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

#[uniffi::export]
pub fn new_profile_from_json_bytes_simd(bytes: BagOfBytes) -> Result<Profile> {
    let json_byte_count = bytes.len() as u64;
    let mut bytes = bytes;
    simd_json::serde::from_slice(bytes.as_mut()).map_err(|_| {
        CommonError::FailedToDeserializeJSONToValue {
            json_byte_count,
            type_name: type_name::<Profile>(),
        }
    })
}

#[uniffi::export]
pub fn profile_to_json_bytes_simd(profile: &Profile) -> BagOfBytes {
    simd_json::serde::to_vec(profile)
        .map(BagOfBytes::from)
        .unwrap_or_else(|_| {
            unreachable!(
                "JSON serialization of {} should never fail.",
                type_name::<Profile>()
            )
        })
}

#[uniffi::export]
pub fn new_profile_from_json_bytes_simd_arc(
    bytes: Arc<BagOfBytesObj>,
) -> Result<Arc<ProfileObj>> {
    let json_byte_count = bytes.bytes.len() as u64;
    let mut bytes = bytes.bytes(); //clones
    simd_json::serde::from_slice::<Profile>(bytes.as_mut())
        // .map(|profile| ProfileObj { profile })
        // .map(Arc::new)
        .map(ProfileObj::new)
        .map_err(|_| CommonError::FailedToDeserializeJSONToValue {
            json_byte_count,
            type_name: type_name::<Profile>(),
        })
}

#[uniffi::export]
pub fn profile_to_json_bytes_simd_arc(
    profile: Arc<ProfileObj>,
) -> Arc<BagOfBytesObj> {
    let profile = &profile.profile;
    simd_json::serde::to_vec(profile)
        .map(BagOfBytes::from)
        // .map(|bytes| BagOfBytesObj { bytes })
        // .map(Arc::new)
        .map(BagOfBytesObj::new)
        .unwrap_or_else(|_| {
            unreachable!(
                "JSON serialization of {} should never fail.",
                type_name::<Profile>()
            )
        })
}

#[derive(uniffi::Object)]
pub struct BagOfBytesObj {
    pub bytes: BagOfBytes,
}

#[uniffi::export]
impl BagOfBytesObj {
    #[uniffi::constructor]
    pub fn new(bytes: BagOfBytes) -> Arc<Self> {
        Arc::new(Self { bytes })
    }

    pub fn bytes(self: Arc<Self>) -> BagOfBytes {
        self.bytes.clone()
    }
}

#[derive(uniffi::Object)]
pub struct ProfileObj {
    pub profile: Profile,
}

#[uniffi::export]
impl ProfileObj {
    #[uniffi::constructor]
    pub fn new(profile: Profile) -> Arc<Self> {
        Arc::new(Self { profile })
    }
    pub fn profile(self: Arc<Self>) -> Profile {
        self.profile.clone()
    }
}

#[uniffi::export]
pub fn new_profile_from_json_bytes_arc(
    json: Arc<BagOfBytesObj>,
) -> Result<Arc<ProfileObj>> {
    let json = json.bytes.as_ref();
    serde_json::from_slice::<Profile>(json)
        // .map(|profile| ProfileObj { profile })
        // .map(Arc::new)
        .map(ProfileObj::new)
        .map_err(|_| CommonError::FailedToDeserializeJSONToValue {
            json_byte_count: json.len() as u64,
            type_name: type_name::<Profile>(),
        })
}

#[uniffi::export]
pub fn profile_to_json_bytes_arc(
    profile: Arc<ProfileObj>,
) -> Arc<BagOfBytesObj> {
    let profile = &profile.profile;
    serde_json::to_vec(profile)
        .map(BagOfBytes::from)
        // .map(|bytes| BagOfBytesObj { bytes })
        // .map(Arc::new)
        .map(BagOfBytesObj::new)
        .unwrap_or_else(|_| {
            unreachable!(
                "JSON serialization of {} should never fail.",
                type_name::<Profile>()
            )
        })
}

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
    fn simd_json_roundtrip() {
        let sut = SUT::sample();
        let bytes = profile_to_json_bytes_simd(&sut);
        let deserialized = new_profile_from_json_bytes_simd(bytes).unwrap();
        assert_eq!(deserialized, sut);
    }
}
