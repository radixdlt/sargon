use crate::prelude::*;

impl Profile {
    /// If the user has **any** accounts on any network at all, including hidden
    /// accounts. This can be used by host devices to prompt user to create their
    /// first account or not, e.g. if user starts app after fresh install, the
    /// SargonOS will create an "empty" Profile and BDFS and save it, before user
    /// has had the chance to create their first account. If the user force quits
    /// the app and then restart it, the app can still prompt user to create their
    /// first account - as if no force-restart happened.
    pub fn has_any_account_on_any_network(&self) -> bool {
        self.networks.iter().any(|n| !n.accounts.is_empty())
    }
    pub fn contains_entity_by_address<A: IsEntityAddress>(
        &self,
        entity_address: &A,
    ) -> bool {
        self.networks.iter().any(|n: ProfileNetwork| {
            n.contains_entity_by_address::<A>(entity_address)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn test_empty_profile_has_any_account_on_any_network_is_false() {
        let sut =
            SUT::new(Mnemonic::sample(), HostId::sample(), HostInfo::sample());
        assert!(!sut.has_any_account_on_any_network());
    }

    #[test]
    fn test_sample_profile_has_any_account_on_any_network() {
        assert!(SUT::sample().has_any_account_on_any_network());
        assert!(SUT::sample_other().has_any_account_on_any_network());
    }
}
