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

#[derive(Clone, PartialEq, Eq, Debug, Hash, uniffi::Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct RefBytes {
    pub bytes: BagOfBytes,
}

impl From<BagOfBytes> for RefBytes {
    fn from(value: BagOfBytes) -> Self {
        Self { bytes: value }
    }
}

impl From<RefBytes> for BagOfBytes {
    fn from(value: RefBytes) -> BagOfBytes {
        value.bytes
    }
}

impl RefBytes {
    fn with_bytes(bytes: Vec<u8>) -> Arc<Self> {
        Self::new(BagOfBytes::from(bytes))
    }
}

#[uniffi::export]
impl RefBytes {
    #[uniffi::constructor]
    pub fn new(bytes: BagOfBytes) -> Arc<Self> {
        Arc::new(Self { bytes })
    }

    pub fn bytes(self: Arc<Self>) -> BagOfBytes {
        self.bytes.clone()
    }
}
use std::sync::RwLock;

#[derive(Debug, uniffi::Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct RefProfile {
    pub profile: RwLock<Option<Profile>>,
}
impl RefProfile {
    fn with_profile(profile: Profile) -> Self {
        Self {
            profile: RwLock::new(Some(profile)),
        }
    }
}
impl std::hash::Hash for RefProfile {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self.profile.read() {
            Ok(ref guard) => {
                state.write_u8(1);
                match guard.as_ref() {
                    Some(prof) => {
                        prof.hash(state);
                        state.write_u8(100);
                    }
                    None => state.write_u8(200),
                }
            }
            _ => {
                state.write_u8(255);
            }
        }
    }
}

impl Eq for RefProfile {}
impl PartialEq for RefProfile {
    fn eq(&self, other: &Self) -> bool {
        {
            match self.profile.read() {
                Ok(ref rhs) => match other.profile.read() {
                    Ok(ref lhs) => match rhs.as_ref() {
                        Some(r) => match lhs.as_ref() {
                            Some(l) => r == l,
                            None => false,
                        },
                        None => lhs.as_ref().is_none(),
                    },
                    _ => false,
                },
                _ => false,
            }
        }
    }
}

impl From<Profile> for RefProfile {
    fn from(value: Profile) -> Self {
        Self::with_profile(value)
    }
}

impl JsonDataSerializing for Profile {}
impl JsonDataDeserializing for Profile {}

#[uniffi::export]
impl RefProfile {
    #[uniffi::constructor]
    pub fn new(profile: Profile) -> Arc<Self> {
        Arc::new(Self::from(profile))
    }

    pub fn take_profile(self: Arc<Self>) -> Result<Profile> {
        self.profile
            .try_write()
            .unwrap()
            .take()
            .ok_or(CommonError::ProfileAlreadyTakenFromContainer)
    }
}

// ################
// FROM BYTES
// ################
#[uniffi::export]
pub fn new_profile_from_json_bytes(
    reference: Arc<RefBytes>,
) -> Result<Arc<RefProfile>> {
    Profile::new_from_json_bytes(reference.bytes.as_ref()).map(RefProfile::new)
}

// ################
// TO BYTES
// ################
#[uniffi::export]
pub fn profile_to_json_bytes(reference: Arc<RefProfile>) -> Arc<RefBytes> {
    RefBytes::with_bytes(reference.take_profile().unwrap().to_json_bytes())
}

// ################
// ENCRYPTION
// ################
#[uniffi::export]
pub fn new_profile_from_encryption_bytes(
    reference: Arc<RefBytes>,
    decryption_password: String,
) -> Result<Arc<RefProfile>> {
    Profile::new_from_encryption_bytes(
        reference.bytes.as_ref(),
        decryption_password,
    )
    .map(RefProfile::new)
}

#[uniffi::export]
pub fn profile_encrypt_with_password(
    reference: Arc<RefProfile>,
    encryption_password: String,
) -> Arc<RefBytes> {
    RefBytes::with_bytes(
        reference
            .take_profile()
            .unwrap()
            .to_encryption_bytes(encryption_password),
    )
}

#[uniffi::export]
pub fn profile_analyze_contents_of_file(
    reference: Arc<RefBytes>,
) -> ProfileFileContents {
    Profile::analyze_contents_of_file(reference.bytes.as_ref())
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    impl RefProfile {
        fn clone_inner(&self) -> Result<Self> {
            self.profile
                .try_read()
                .map_err(|_| CommonError::ProfileAlreadyTakenFromContainer)
                .and_then(|lock| match lock.as_ref() {
                    Some(p) => Ok(Self::with_profile(p.clone())),
                    None => Err(CommonError::ProfileAlreadyTakenFromContainer),
                })
        }
    }

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_profile_sample());
        assert_eq!(SUT::sample_other(), new_profile_sample_other());
    }

    #[test]
    fn test_profile_analyze_contents_of_file() {
        assert_eq!(
            profile_analyze_contents_of_file(RefBytes::new(
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
            new_profile_from_json_bytes(profile_to_json_bytes(
                RefProfile::new(sut.clone())
            ))
            .unwrap()
            .take_profile()
            .unwrap(),
            sut
        )
    }

    #[test]
    fn deserialize_malformed() {
        let malformed_profile_snapshot = BagOfBytes::from("{}".as_bytes());
        assert_eq!(
            new_profile_from_json_bytes(RefBytes::new(
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
        assert!(new_profile_from_encryption_bytes(
            RefBytes::new(BagOfBytes::sample()),
            "invalid".to_string()
        )
        .is_err());
    }

    #[test]
    fn encryption_roundtrip() {
        let sut = SUT::sample();
        let ref_profile = RefProfile::new(sut.clone());

        let password = "super secret".to_owned();
        let encryption_bytes = profile_encrypt_with_password(
            Arc::new(ref_profile.clone_inner().unwrap()),
            password.clone(),
        );
        assert_eq!(
            new_profile_from_encryption_bytes(encryption_bytes, password)
                .unwrap(),
            ref_profile
        );
    }
}
