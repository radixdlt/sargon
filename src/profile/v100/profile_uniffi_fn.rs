use crate::prelude::*;

#[uniffi::export]
pub fn new_profile(
    private_hd_factor_source: PrivateHierarchicalDeterministicFactorSource,
    creating_device_name: String,
) -> Profile {
    Profile::new(private_hd_factor_source, creating_device_name.as_str())
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
    fn new_private_hd() {
        let private = PrivateHierarchicalDeterministicFactorSource::sample();
        let lhs = super::new_profile(private.clone(), "iPhone".to_string());
        assert_eq!(
            lhs.bdfs().factor_source_id(),
            private.factor_source.factor_source_id()
        );
    }

    #[test]
    fn to_string_and_debug_string() {
        assert_eq!(profile_to_string(&SUT::sample()).len(), 4447);
        assert_eq!(profile_to_debug_string(&SUT::sample()).len(), 28088);
        assert_ne!(
            profile_to_debug_string(&SUT::sample()),
            profile_to_debug_string(&SUT::sample_other())
        );
    }
}
