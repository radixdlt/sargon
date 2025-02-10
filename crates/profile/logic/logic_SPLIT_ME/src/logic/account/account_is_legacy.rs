use crate::prelude::*;

pub trait AccountIsLegacy {
    fn is_legacy(&self) -> bool;
}

impl AccountIsLegacy for Account {
    fn is_legacy(&self) -> bool {
        let Some(unsecured_entity_control) = self.security_state.as_unsecured()
        else {
            return false;
        };

        unsecured_entity_control
            .transaction_signing
            .public_key
            .public_key
            .is_secp256k1()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Account;

    #[test]
    fn test_unsecured_legacy_account_is_legacy() {
        let olympia_mnemonic = MnemonicWithPassphrase::sample_device_12_words();
        let seed = olympia_mnemonic.to_seed();
        let private_key = seed.derive_private_key(&BIP44LikePath::sample());

        let account = SUT::sample_unsecurified_mainnet(
            "Legacy",
            HierarchicalDeterministicFactorInstance::new(
                FactorSourceIDFromHash::from_mnemonic_with_passphrase(
                    FactorSourceKind::Device,
                    &olympia_mnemonic,
                ),
                private_key.public_key(),
            ),
        );

        assert!(account.is_legacy());
    }

    #[test]
    fn test_unsecured_babylon_account_is_not_legacy() {
        let babylon_mnemonic = MnemonicWithPassphrase::sample_device();
        let seed = babylon_mnemonic.to_seed();
        let private_key = seed.derive_private_key(&AccountPath::sample());

        let account = SUT::sample_unsecurified_mainnet(
            "Babylon",
            HierarchicalDeterministicFactorInstance::new(
                FactorSourceIDFromHash::from_mnemonic_with_passphrase(
                    FactorSourceKind::Device,
                    &babylon_mnemonic,
                ),
                private_key.public_key(),
            ),
        );

        assert!(!account.is_legacy());
    }

    #[test]
    fn test_securified_account_is_not_legacy() {
        let account = SUT::sample_securified_mainnet(
            "Securified",
            0,
            HierarchicalDeterministicFactorInstance::sample_fia10(),
            || {
                GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
                    RoleKind::Primary, [], 0,
                    FactorSource::sample_all().into_iter().map(|f| {
                        HierarchicalDeterministicFactorInstance::new_for_entity(
                            *f.factor_source_id().as_hash().unwrap(),
                            CAP26EntityKind::Account,
                            Hardened::from_local_key_space(U31::ZERO, IsSecurified(true)).unwrap(),
                        )
                    }),
                ).unwrap()
            },
        );

        assert!(!account.is_legacy());
    }
}
