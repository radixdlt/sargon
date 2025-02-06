use crate::prelude::*;

#[async_trait::async_trait]
pub trait GetEntitiesAffectedByBdfsError {
    /// There used to be a bug on Android hosts that entities created with
    /// 1. an olympia factor source but
    /// 2. their hd public keys were using the Ed25519 curve
    ///
    /// The users affected by this bug were prompted to contact support from
    /// the [1.0.5](https://github.com/radixdlt/babylon-wallet-android/releases/tag/1.0.5) version.
    ///
    /// **See also**
    /// Related PR that detects the issue: [#533](https://github.com/radixdlt/babylon-wallet-android/pull/533)
    /// Later PR that reports which entities are affected: [#897](https://github.com/radixdlt/babylon-wallet-android/pull/897)
    async fn get_entities_affected_by_bdfs_error(
        &self,
    ) -> Option<EntitiesAffectedWithBdfsError>;
}

#[async_trait::async_trait]
impl GetEntitiesAffectedByBdfsError for SargonOS {
    async fn get_entities_affected_by_bdfs_error(
        &self,
    ) -> Option<EntitiesAffectedWithBdfsError> {
        let unsecurified_accounts = self
            .accounts_on_current_network()
            .ok()?
            .iter()
            .filter(|a| !a.is_securified())
            .collect_vec();
        let unsecurified_personas = self
            .personas_on_current_network()
            .ok()?
            .iter()
            .filter(|a| !a.is_securified())
            .collect_vec();

        if unsecurified_accounts.is_empty() && unsecurified_personas.is_empty()
        {
            return None;
        }

        let olympia_device_factor_source_ids = self
            .factor_sources()
            .map(|factor_sources| {
                factor_sources
                    .iter()
                    .filter_map(|f| f.as_device().cloned())
                    .filter(|f| {
                        f.supports_olympia()
                            && f.hint.mnemonic_word_count
                                < BIP39WordCount::TwentyFour
                    })
                    .map(|f| f.id)
                    .collect_vec()
            })
            .ok()?;

        let affected_accounts = unsecurified_accounts
            .into_iter()
            .filter(|a| {
                let security_state = a
                    .security_state
                    .as_unsecured()
                    .expect("Accounts are already filtered to be unsecured")
                    .clone();
                let transaction_signing = security_state.transaction_signing;

                olympia_device_factor_source_ids
                    .contains(&transaction_signing.factor_source_id)
                    && transaction_signing.public_key.public_key.is_ed25519()
            })
            .collect_vec();

        let affected_personas = unsecurified_personas
            .into_iter()
            .filter(|p| {
                let security_state = p
                    .security_state
                    .as_unsecured()
                    .expect("Personas are already filtered to be unsecured")
                    .clone();
                let transaction_signing = security_state.transaction_signing;

                olympia_device_factor_source_ids
                    .contains(&transaction_signing.factor_source_id)
                    && transaction_signing.public_key.public_key.is_ed25519()
            })
            .collect_vec();

        if affected_accounts.is_empty() && affected_personas.is_empty() {
            None
        } else {
            Some(EntitiesAffectedWithBdfsError::new_from(
                affected_accounts,
                affected_personas,
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntitiesAffectedWithBdfsError {
    pub affected_account_addresses: IndexSet<AccountAddress>,
    pub affected_identity_addresses: IndexSet<IdentityAddress>,
}

impl EntitiesAffectedWithBdfsError {
    pub fn new_from(
        accounts: impl IntoIterator<Item = Account>,
        personas: impl IntoIterator<Item = Persona>,
    ) -> Self {
        Self {
            affected_account_addresses: accounts
                .into_iter()
                .map(|a| a.address)
                .collect::<IndexSet<_>>(),
            affected_identity_addresses: personas
                .into_iter()
                .map(|p| p.address)
                .collect::<IndexSet<_>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::fixture_profiles;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn profile_with_bdfs_error_is_detected() {
        let profile_json = fixture_profiles!("profile_with_bdfs_error");
        let profile = serde_json::from_str::<Profile>(profile_json).unwrap();

        let sut = SUT::fast_boot().await;
        sut.import_wallet(&profile, false).await.unwrap();

        let entities_affected =
            sut.get_entities_affected_by_bdfs_error().await.unwrap();
        assert_eq!(entities_affected.affected_account_addresses.len(), 1);
        assert_eq!(
            entities_affected.affected_account_addresses.first().unwrap().to_string(),
            "account_rdx12xxaawy86ka2r8tq4dh02tv02kv9gy40cwd3p26rpd8fu4pcat5k7k".to_string()
        );
        assert_eq!(entities_affected.affected_identity_addresses.len(), 1);
        assert_eq!(
            entities_affected.affected_identity_addresses.first().unwrap().to_string(),
            "identity_rdx12gylc3wv5m4dfydsd2qayawqmkva7kuf5c65gkqmzkst7pxe8dtz9e".to_string()
        );
    }
}
