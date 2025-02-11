use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VirtualEntityCreatingInstance {
    /// The instance which as known to have created `address`
    factor_instance: HierarchicalDeterministicFactorInstance,

    /// The address of the entity.
    address: AddressOfAccountOrPersona,
}
impl VirtualEntityCreatingInstance {
    /// # Panics
    /// Panics if factor_instance does not result in address.
    ///
    /// Panics if factor_instance is not in unsecurified space.
    pub fn new(
        factor_instance: HierarchicalDeterministicFactorInstance,
        address: AddressOfAccountOrPersona,
    ) -> Self {
        assert!(
            !factor_instance.is_securified(),
            "FactorInstance is in Securified KeySpace, but expected Unsecurified."
        );

        assert!(
            address.matches_public_key(factor_instance.public_key()),
            "Discrepancy! PublicKeys does not match, this is a programmer error!"
        );

        Self::check_for_derivation_path_discrepancies(
            &factor_instance,
            &address,
        );

        assert_eq!(
            factor_instance.derivation_path().network_id(),
            address.network_id(),
            "Discrepancy! Network mismatch between derivation path of factor instance and address."
        );

        Self {
            address,
            factor_instance,
        }
    }

    pub fn address(&self) -> AddressOfAccountOrPersona {
        self.address
    }

    pub fn factor_instance(&self) -> HierarchicalDeterministicFactorInstance {
        self.factor_instance.clone()
    }

    fn with_factor_instance_on_network(
        factor_instance: HierarchicalDeterministicFactorInstance,
        entity_kind: CAP26EntityKind,
        network_id: NetworkID,
    ) -> Self {
        let public_key = factor_instance.public_key();
        let address = match entity_kind {
            CAP26EntityKind::Account => AddressOfAccountOrPersona::from(
                AccountAddress::new_from_public_key(public_key, network_id),
            ),
            CAP26EntityKind::Identity => AddressOfAccountOrPersona::from(
                IdentityAddress::new_from_public_key(public_key, network_id),
            ),
        };
        Self::new(factor_instance, address)
    }

    /// In 2024, for circa 6 months, the Android host had a bug where Personas
    /// was created with Account DerivationPath => same FactorInstance (PublicKey)
    /// was used between Personas and Accounts!
    ///
    /// The bug was introduced in [Android Host PR][badpr], in 2024-07-11.
    /// The bug was fixed in [Android Host PR][goodpr], in 2024-11-27.
    ///
    /// However, even though the bug was fixed after less than 5 months, we have
    /// end users which are in this bad state (shared instances between Personas and Acccounts).
    ///
    /// Thus we can't assert the discrepancy.
    ///
    /// [identpr]: https://github.com/radixdlt/sargon/pull/254/files#r1860748013
    /// [badpr]: https://github.com/radixdlt/babylon-wallet-android/pull/1042
    /// [goodpr]: https://github.com/radixdlt/babylon-wallet-android/pull/1256
    fn check_for_derivation_path_discrepancies(
        factor_instance: &HierarchicalDeterministicFactorInstance,
        address: &AddressOfAccountOrPersona,
    ) {
        let discrepancy_found =
            factor_instance.clone().derivation_path().get_entity_kind()
                != address.clone().get_entity_kind();
        let error_msg = "Discrepancy! Address and DerivationPath of FactorInstances have different entity kinds.";
        if discrepancy_found {
            error!("{}", error_msg);
        }
        #[cfg(test)]
        debug_assert!(!discrepancy_found, "{}", error_msg);
    }
}

impl HasSampleValues for VirtualEntityCreatingInstance {
    fn sample() -> Self {
        Self::with_factor_instance_on_network(
            HierarchicalDeterministicFactorInstance::sample(),
            CAP26EntityKind::Account,
            NetworkID::Mainnet,
        )
    }
    fn sample_other() -> Self {
        let network_id = NetworkID::Stokenet;
        let entity_kind = CAP26EntityKind::Identity;
        Self::with_factor_instance_on_network(
            HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network(
                network_id,
                CAP26KeyKind::TransactionSigning,
                entity_kind,
                0,
            ),
            entity_kind,
            network_id,
        )
    }
}

#[cfg(test)]
mod test_instance {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = VirtualEntityCreatingInstance;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    #[should_panic(
        expected = "FactorInstance is in Securified KeySpace, but expected Unsecurified."
    )]
    fn panics_if_factor_is_securified() {
        let _ = SUT::new(HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0), AddressOfAccountOrPersona::sample_account_mainnet());
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy! PublicKeys does not match, this is a programmer error!"
    )]
    fn panics_if_public_key_does_not_match() {
        let _ = SUT::new(
            HierarchicalDeterministicFactorInstance::sample(),
            AddressOfAccountOrPersona::sample(),
        );
    }
    #[test]
    #[should_panic(
        expected = "Discrepancy! Address and DerivationPath of FactorInstances have different entity kinds."
    )]
    fn panics_if_derivation_path_of_factor_and_address_has_mismatching_entity_kind(
    ) {
        let fi = HierarchicalDeterministicFactorInstance::sample();
        assert_eq!(
            fi.derivation_path().get_entity_kind(),
            CAP26EntityKind::Account
        );
        let identity_address = IdentityAddress::new_from_public_key(
            fi.public_key(),
            fi.derivation_path().network_id(),
        );
        let _ = SUT::new(fi, identity_address.into());
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy! Network mismatch between derivation path of factor instance and address."
    )]
    fn panics_if_derivation_path_of_factor_and_address_has_mismatching_network()
    {
        let _ = SUT::with_factor_instance_on_network(
            HierarchicalDeterministicFactorInstance::sample(),
            CAP26EntityKind::Account,
            NetworkID::Nergalnet,
        );
    }
}
