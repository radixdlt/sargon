use crate::prelude::*;

pub trait AccountIsUnsecuredLedgerControlled {
    fn is_unsecured_ledger_controlled(&self) -> bool;

    fn unsecured_ledger_controlled_public_key(
        &self,
    ) -> Option<HierarchicalDeterministicPublicKey>;
}

impl AccountIsUnsecuredLedgerControlled for Account {
    fn is_unsecured_ledger_controlled(&self) -> bool {
        let Some(unsecured_entity_control) = self.security_state.as_unsecured()
        else {
            return false;
        };

        unsecured_entity_control
            .transaction_signing
            .factor_source_id
            .kind
            == FactorSourceKind::LedgerHQHardwareWallet
    }

    fn unsecured_ledger_controlled_public_key(
        &self,
    ) -> Option<HierarchicalDeterministicPublicKey> {
        let unsecured_entity_control = self.security_state.as_unsecured()?;

        let transaction_signing =
            unsecured_entity_control.transaction_signing.clone();
        if transaction_signing.factor_source_id.kind
            == FactorSourceKind::LedgerHQHardwareWallet
        {
            Some(transaction_signing.public_key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Account;

    #[test]
    fn test_ledger_controlled_unsecured_account_returns_true() {
        assert!(ledger_controlled_account().is_unsecured_ledger_controlled());
    }

    #[test]
    fn test_device_controlled_unsecured_account_returns_false() {
        let sut = SUT::sample();

        assert!(!sut.is_unsecured_ledger_controlled());
    }

    #[test]
    fn test_nsecured_account_returns_false() {
        let sut = SUT::sample_securified_mainnet(
            "Grace",
            6,
            HierarchicalDeterministicFactorInstance::sample_fii10(),
            || {
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap(),
                ))
            },
        );

        assert!(!sut.is_unsecured_ledger_controlled());
    }

    #[test]
    fn test_ledger_controlled_account_returns_true() {
        assert!(ledger_controlled_account().is_unsecured_ledger_controlled());
    }

    #[test]
    fn test_ledger_controlled_account_returns_controlling_factor_instance() {
        let sut = ledger_controlled_account();

        assert!(sut.unsecured_ledger_controlled_public_key().is_some())
    }

    #[test]
    fn test_device_controlled_account_returns_no_controlling_factor_instance() {
        let sut = SUT::sample();

        assert!(sut.unsecured_ledger_controlled_public_key().is_none())
    }

    #[test]
    fn test_securified_account_returns_no_controlling_factor_instance() {
        let sut = SUT::sample_securified_mainnet(
            "Grace",
            6,
            HierarchicalDeterministicFactorInstance::sample_fii10(),
            || {
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap(),
                ))
            },
        );

        assert!(sut.unsecured_ledger_controlled_public_key().is_none())
    }

    fn ledger_controlled_account() -> SUT {
        let ledger_mnemonic = MnemonicWithPassphrase::sample_ledger();
        let path = AccountPath::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            UnsecurifiedHardened::from_local_key_space(0u32).unwrap(),
        );
        let seed = ledger_mnemonic.to_seed();
        let private_key = seed.derive_private_key(&path);
        let factor_instance = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::new_for_ledger(&ledger_mnemonic),
            private_key.public_key(),
        );

        SUT::new(
            HDFactorInstanceAccountCreation::new(factor_instance).unwrap(),
            DisplayName::sample(),
            AppearanceID::sample(),
        )
    }
}
