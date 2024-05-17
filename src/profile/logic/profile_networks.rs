use crate::prelude::*;

impl Profile {
    pub fn has_any_account_on_any_network(&self) -> bool {
        self.networks.iter().any(|n| !n.accounts.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn test_empty_profile_has_any_account_on_any_network_is_false() {
        let sut = SUT::new(DeviceFactorSource::sample(), DeviceInfo::sample());
        assert!(!sut.has_any_account_on_any_network());
    }

    #[test]
    fn test_sample_profile_has_any_account_on_any_network() {
        assert!(SUT::sample().has_any_account_on_any_network());
        assert!(SUT::sample_other().has_any_account_on_any_network());
    }
}
