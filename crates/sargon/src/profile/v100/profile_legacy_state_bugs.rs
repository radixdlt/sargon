use crate::prelude::*;

impl Profile {
    // TODO: Sometimes later it would be nice to remove this method
    // and only use `diagnostics_for_factor_instances_valid_with_handler` and then
    // send a handler from SargonOS which has access to some new driver which
    // can use Swift Issue Reporting API:
    // https://github.com/pointfreeco/swift-issue-reporting
    // which will cause execution to halt with a runtime issue, which will be great
    // for debugging and finding issues!
    // Maybe android host can raise an exception..?
    pub(crate) fn diagnostics_for_factor_instances_valid(
        &self,
        is_android: bool,
    ) {
        self.diagnostics_for_factor_instances_valid_with_handler(
            is_android,
            |_| {},
        );
    }

    pub(crate) fn diagnostics_for_factor_instances_valid_with_handler(
        &self,
        is_android: bool,
        mut on_duplicate: impl FnMut(DuplicateInstancesWithKnownOs),
    ) {
        let Some(duplicate_instances) = self.check_for_duplicated_instances()
        else {
            return;
        };

        // Android had a bug for some months in 2024 which resulted in
        // Personas having same PublicKey as Accounts. If that is the case
        // don't `error!` but use `warn!`, if host is known to be Android.
        // For full details see `is_unfortunate_android_bug`
        if duplicate_instances.is_unfortunate_android_bug() && is_android {
            warn!(
                "Duplicated FactorInstances found {:?} but due to Android bug",
                duplicate_instances
            );
        } else {
            error!(
                "Duplicated FactorInstances found {:?}",
                duplicate_instances
            );
        }
        on_duplicate(DuplicateInstancesWithKnownOs {
            duplicate_instances,
            is_android,
        })
    }
}

impl DuplicateInstances {
    /// In 2024, for cirka 6 months, the Android host had a bug where Personas
    /// was created with Account DerivationPath => same FactorInstance (PublicKey)
    /// was used between Personas and Accounts!
    ///
    /// The bug was introduced in [Android Host PR][badpr], in 2024-07-11.
    /// The bug was fixed in [Android Host PR][goodpr], in 2024-11-27.
    ///
    /// However, even though the bug was fixed after less than 5 months, we have
    /// end users which are in this bad state (shared instances between Personas and Acccounts).
    ///
    /// Thus for this type, `DuplicateInstances`, we should not always throw it as
    /// an err (with `into_error`) for in all contexts. Rather we need to be more
    /// "lenient" if the `DuplicateInstances` was caused by this unfortunate Android bug.
    ///
    /// What identified this bug was the `assert_factor_instances_valid`
    /// which was introduced [in Sargon in the big `FactorInstancesProvider` PR][identpr].
    ///
    /// [identpr]: https://github.com/radixdlt/sargon/pull/254/files#r1860748013
    /// [badpr]: https://github.com/radixdlt/babylon-wallet-android/pull/1042
    /// [goodpr]: https://github.com/radixdlt/babylon-wallet-android/pull/1256
    pub(crate) fn is_unfortunate_android_bug(&self) -> bool {
        let Ok(instance) = self.factor_instance.try_as_hd_factor_instances()
        else {
            return false;
        };
        if instance.is_securified() {
            // The android bug only concerned unsecurified FactorInstances
            return false;
        }

        if instance.get_key_kind() != CAP26KeyKind::TransactionSigning {
            // The android bug only happened for transaction signing keys
            return false;
        }

        let p1a2 = self.entity1.is_persona_entity()
            && self.entity2.is_account_entity();
        let a1p2 = self.entity1.is_account_entity()
            && self.entity2.is_persona_entity();
        let one_account_one_persona = p1a2 || a1p2;
        if !one_account_one_persona {
            // The Android bug was that the same public key was used between an account and a persona
            return false;
        }

        true
    }
}

#[cfg(test)]
impl Profile {
    fn with_android_bug_with_shared_pubkey_between_account_and_persona() -> Self
    {
        let mwp = MnemonicWithPassphrase::sample_device();
        let mut sut = Profile::from_mnemonic_with_passphrase(
            mwp.clone(),
            HostId::sample(),
            HostInfo::sample(),
        );
        let seed = mwp.clone().to_seed();
        let fsid = FactorSourceIDFromHash::new_for_device(&mwp);
        let path = AccountPath::sample();
        let public_key = seed
            .derive_ed25519_private_key(path.clone().to_hd_path())
            .public_key();
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            fsid,
            HierarchicalDeterministicPublicKey::new(
                public_key.into(),
                path.into(),
            ),
        );
        let veci = HDFactorInstanceAccountCreation::new(hd_fi.clone()).unwrap();
        let account =
            Account::new(veci, DisplayName::sample(), AppearanceID::sample());
        let mut persona = Persona::sample();
        persona.address =
            IdentityAddress::new(public_key.into(), NetworkID::Mainnet);
        persona.security_state = EntitySecurityState::Unsecured {
            value: UnsecuredEntityControl::new(hd_fi, None).unwrap(),
        };
        assert_eq!(
            account.unique_factor_instances(),
            persona.unique_factor_instances()
        );
        sut.networks = ProfileNetworks::just(ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::just(account),
            Personas::just(persona),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
        ));
        sut
    }

    fn with_instance_collision_not_android_bug_both_accounts() -> Self {
        let mwp = MnemonicWithPassphrase::sample_device();
        let mut sut = Profile::from_mnemonic_with_passphrase(
            mwp.clone(),
            HostId::sample(),
            HostInfo::sample(),
        );
        let seed = mwp.clone().to_seed();
        let fsid = FactorSourceIDFromHash::new_for_device(&mwp);
        let path = AccountPath::sample();
        let public_key = seed
            .derive_ed25519_private_key(path.clone().to_hd_path())
            .public_key();
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            fsid,
            HierarchicalDeterministicPublicKey::new(
                public_key.into(),
                path.into(),
            ),
        );
        let veci = HDFactorInstanceAccountCreation::new(hd_fi.clone()).unwrap();
        let account =
            Account::new(veci, DisplayName::sample(), AppearanceID::sample());

        let mut account2 = Account::sample_other();
        account2.security_state = EntitySecurityState::Unsecured {
            value: UnsecuredEntityControl::new(hd_fi, None).unwrap(),
        };

        assert_eq!(
            account.unique_factor_instances(),
            account2.unique_factor_instances()
        );
        sut.networks = ProfileNetworks::just(ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::from_iter([account, account2]),
            Personas::default(),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
        ));
        sut
    }

    fn with_instance_collision_not_android_bug_authentication_signing_key_kind(
    ) -> Self {
        let mwp = MnemonicWithPassphrase::sample_device();
        let mut sut = Profile::from_mnemonic_with_passphrase(
            mwp.clone(),
            HostId::sample(),
            HostInfo::sample(),
        );
        let mut account1 = Account::sample();
        let mut account2 = Account::sample_other();
        let mut uec1 = account1.try_get_unsecured_control().unwrap();
        uec1.authentication_signing = Some(
            HierarchicalDeterministicFactorInstance::sample_auth_signing(),
        );
        account1.security_state =
            EntitySecurityState::Unsecured { value: uec1 };

        let mut uec2 = account1.try_get_unsecured_control().unwrap();
        uec2.authentication_signing = Some(
            HierarchicalDeterministicFactorInstance::sample_auth_signing(),
        );
        account2.security_state =
            EntitySecurityState::Unsecured { value: uec2 };

        sut.networks = ProfileNetworks::just(ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::from_iter([account1, account2]),
            Personas::default(),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
        ));
        sut
    }

    fn with_instance_collision_not_android_bug_securified() -> Self {
        let mwp = MnemonicWithPassphrase::sample_device();
        let mut sut = Profile::from_mnemonic_with_passphrase(
            mwp.clone(),
            HostId::sample(),
            HostInfo::sample(),
        );
        let mut account1 = Account::sample();
        let mut account2 = Account::sample_other();
        account1.security_state = EntitySecurityState::Securified {
            value: SecuredEntityControl::sample(),
        };
        account2.security_state = EntitySecurityState::Securified {
            value: SecuredEntityControl::sample(),
        };

        sut.networks = ProfileNetworks::just(ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::from_iter([account1, account2]),
            Personas::default(),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
        ));
        sut
    }

    fn with_instance_collision_not_android_bug_both_personas() -> Self {
        let mwp = MnemonicWithPassphrase::sample_device();
        let mut sut = Profile::from_mnemonic_with_passphrase(
            mwp.clone(),
            HostId::sample(),
            HostInfo::sample(),
        );
        let seed = mwp.clone().to_seed();
        let fsid = FactorSourceIDFromHash::new_for_device(&mwp);
        let path = IdentityPath::sample();
        let public_key = seed
            .derive_ed25519_private_key(path.clone().to_hd_path())
            .public_key();
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            fsid,
            HierarchicalDeterministicPublicKey::new(
                public_key.into(),
                path.into(),
            ),
        );
        let mut persona1 = Persona::sample();
        persona1.address =
            IdentityAddress::new(public_key.into(), NetworkID::Mainnet);
        persona1.security_state = EntitySecurityState::Unsecured {
            value: UnsecuredEntityControl::new(hd_fi.clone(), None).unwrap(),
        };

        let mut persona2 = Persona::sample_other();
        persona2.security_state = EntitySecurityState::Unsecured {
            value: UnsecuredEntityControl::new(hd_fi, None).unwrap(),
        };

        assert_eq!(
            persona1.unique_factor_instances(),
            persona2.unique_factor_instances()
        );
        sut.networks = ProfileNetworks::just(ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::default(),
            Personas::from_iter([persona1, persona2]),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
        ));
        sut
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn unfortunate_android_bug_detection() {
        let sut = SUT::with_android_bug_with_shared_pubkey_between_account_and_persona();

        #[derive(Debug)]
        struct NotAndroidLog;
        impl LoggingDriver for NotAndroidLog {
            fn log(&self, level: LogLevel, msg: String) {
                assert_eq!(level, LogLevel::Error);
                assert!(msg.contains("Duplicated FactorInstances found"));
                assert!(!msg.contains("due to Android bug"));
            }
        }
        install_logger(Arc::new(NotAndroidLog));
        let accounts = sut.accounts_on_current_network().unwrap();
        let acc = accounts.first().unwrap();
        let factor_instance = acc
            .unique_factor_instances()
            .into_iter()
            .next()
            .clone()
            .unwrap();
        let duplicate_instances = DuplicateInstances {
            entity1: acc.clone().into(),
            entity2: sut
                .personas_on_current_network()
                .unwrap()
                .first()
                .unwrap()
                .clone()
                .into(),
            factor_instance,
        };

        let mut detected = Option::<DuplicateInstancesWithKnownOs>::None;

        sut.diagnostics_for_factor_instances_valid_with_handler(false, |d| {
            detected = Some(d)
        });

        assert!(detected
            .clone()
            .unwrap()
            .duplicate_instances
            .is_unfortunate_android_bug());
        pretty_assertions::assert_eq!(
            detected,
            Some(DuplicateInstancesWithKnownOs {
                is_android: false,
                duplicate_instances: duplicate_instances.clone()
            })
        );

        #[derive(Debug)]
        struct IsAndroidLog;
        impl LoggingDriver for IsAndroidLog {
            fn log(&self, level: LogLevel, msg: String) {
                assert_eq!(level, LogLevel::Warn);
                assert!(msg.contains("Duplicated FactorInstances found"));
                assert!(msg.contains("due to Android bug"));
            }
        }
        install_logger(Arc::new(IsAndroidLog));

        sut.diagnostics_for_factor_instances_valid_with_handler(true, |d| {
            detected = Some(d)
        });

        assert!(detected
            .clone()
            .unwrap()
            .duplicate_instances
            .is_unfortunate_android_bug());

        assert_eq!(
            detected,
            Some(DuplicateInstancesWithKnownOs {
                is_android: true,
                duplicate_instances: duplicate_instances.clone()
            })
        );
    }

    #[test]
    fn instance_detection_not_android_bug_both_accounts() {
        let sut = SUT::with_instance_collision_not_android_bug_both_accounts();
        let accounts = sut.accounts_on_current_network().unwrap();
        let acc1 = accounts.clone().first().unwrap().clone();
        let acc2 = accounts.items().into_iter().next_back().unwrap();

        instance_detection_not_android_bug(sut, acc1, acc2)
    }

    #[test]
    fn instance_detection_not_android_bug_securified() {
        let sut = SUT::with_instance_collision_not_android_bug_securified();
        let accounts = sut.accounts_on_current_network().unwrap();
        let acc1 = accounts.clone().first().unwrap().clone();
        let acc2 = accounts.items().into_iter().next_back().unwrap();

        instance_detection_not_android_bug(sut, acc1, acc2)
    }

    #[test]
    fn instance_detection_not_android_bug_auth_sign() {
        let sut = SUT::with_instance_collision_not_android_bug_authentication_signing_key_kind();
        let accounts = sut.accounts_on_current_network().unwrap();
        let acc1 = accounts.clone().first().unwrap().clone();
        let acc2 = accounts.items().into_iter().next_back().unwrap();

        instance_detection_not_android_bug(sut, acc1, acc2)
    }

    #[test]
    fn instance_detection_not_android_bug_both_personas() {
        let sut = SUT::with_instance_collision_not_android_bug_both_personas();
        let personas = sut.personas_on_current_network().unwrap();
        let p1 = personas.clone().first().unwrap().clone();
        let p2 = personas.items().into_iter().next_back().unwrap();

        instance_detection_not_android_bug(sut, p1, p2)
    }

    fn instance_detection_not_android_bug(
        sut: SUT,
        e1: impl Into<AccountOrPersona>,
        e2: impl Into<AccountOrPersona>,
    ) {
        #[derive(Debug)]
        struct NotAndroidLog;
        impl LoggingDriver for NotAndroidLog {
            fn log(&self, level: LogLevel, msg: String) {
                assert_eq!(level, LogLevel::Error);
                assert!(msg.contains("Duplicated FactorInstances found"));
                assert!(!msg.contains("due to Android bug"));
            }
        }
        install_logger(Arc::new(NotAndroidLog));

        let e1 = e1.into();
        let factor_instance = e1
            .unique_factor_instances()
            .into_iter()
            .next()
            .clone()
            .unwrap();

        let duplicate_instances = DuplicateInstances {
            entity1: e1,
            entity2: e2.into(),
            factor_instance,
        };

        let mut detected = Option::<DuplicateInstancesWithKnownOs>::None;

        sut.diagnostics_for_factor_instances_valid_with_handler(false, |d| {
            detected = Some(d)
        });

        assert!(!detected
            .clone()
            .unwrap()
            .duplicate_instances
            .is_unfortunate_android_bug());
        pretty_assertions::assert_eq!(
            detected,
            Some(DuplicateInstancesWithKnownOs {
                is_android: false,
                duplicate_instances: duplicate_instances.clone()
            })
        );

        sut.diagnostics_for_factor_instances_valid_with_handler(true, |d| {
            detected = Some(d)
        });

        assert!(!detected
            .clone()
            .unwrap()
            .duplicate_instances
            .is_unfortunate_android_bug());
        assert_eq!(
            detected,
            Some(DuplicateInstancesWithKnownOs {
                is_android: true,
                duplicate_instances: duplicate_instances.clone()
            })
        );
    }
}
