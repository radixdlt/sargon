use crate::prelude::*;

/// Utility to extract entities requiring auth from a profile and a manifest summary.
pub struct ExtractorOfEntitiesRequiringAuth;
impl ExtractorOfEntitiesRequiringAuth {
    /// Matches entities requiring auth from a manifest summary with the entities in the given profile.
    /// Returns a set of `AccountOrPersona` or empty if the manifest summary does not require auth.
    /// Returns an error if an account or persona is unknown.
    pub fn extract(
        profile: &Profile,
        summary: ManifestSummary,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let mut entities_requiring_auth: IndexSet<AccountOrPersona> =
            IndexSet::new();

        let accounts = summary
            .addresses_of_accounts_requiring_auth
            .iter()
            .map(|a| profile.account_by_address(*a))
            .collect::<Result<Vec<_>>>()?;

        entities_requiring_auth.extend(
            accounts
                .into_iter()
                .map(AccountOrPersona::from)
                .collect_vec(),
        );

        let personas = summary
            .addresses_of_personas_requiring_auth
            .into_iter()
            .map(|a| profile.persona_by_address(a))
            .collect::<Result<Vec<_>>>()?;

        entities_requiring_auth.extend(
            personas
                .into_iter()
                .map(AccountOrPersona::from)
                .collect_vec(),
        );
        Ok(entities_requiring_auth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_transactions::prelude::ManifestBuilder;

    #[test]
    fn extract_when_account_is_unknown() {
        let profile = Profile::sample();

        let manifest_builder = ManifestBuilder::new();
        let mut manifest = TransactionManifest::sargon_built(
            manifest_builder,
            NetworkID::Mainnet,
        );
        manifest = manifest.modify_add_lock_fee(
            &AccountAddress::sample_stokenet(),
            Some(Decimal192::one()),
        );
        let manifest_summary = manifest.summary();

        let result = ExtractorOfEntitiesRequiringAuth::extract(
            &profile,
            manifest_summary,
        );

        assert!(matches!(result, Err(CommonError::UnknownAccount)));
    }

    #[test]
    fn extract_when_persona_is_unknown() {
        let profile = Profile::sample();

        let manifest = TransactionManifest::set_owner_keys_hashes(
            &Persona::sample_mainnet_third().address.into(),
            vec![PublicKeyHash::sample()],
        );
        let manifest_summary = manifest.summary();

        let result = ExtractorOfEntitiesRequiringAuth::extract(
            &profile,
            manifest_summary,
        );

        assert!(matches!(result, Err(CommonError::UnknownPersona)));
    }

    #[test]
    fn extract_when_no_entities_require_auth() {
        let profile = Profile::sample();

        let manifest_builder = ManifestBuilder::new();
        let manifest = TransactionManifest::sargon_built(
            manifest_builder,
            NetworkID::Mainnet,
        );
        let manifest_summary = manifest.summary();

        let result = ExtractorOfEntitiesRequiringAuth::extract(
            &profile,
            manifest_summary,
        );

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn extract_entities_success() {
        let profile = Profile::sample();
        let account = Account::sample_mainnet();
        let persona = Persona::sample_mainnet();

        let manifest = TransactionManifest::set_owner_keys_hashes(
            &persona.address.into(),
            vec![PublicKeyHash::sample()],
        )
        .modify_add_lock_fee(&account.address, Some(Decimal192::one()));
        let manifest_summary = manifest.summary();

        let result = ExtractorOfEntitiesRequiringAuth::extract(
            &profile,
            manifest_summary,
        );

        assert_eq!(
            result,
            Ok(IndexSet::from_iter(vec![
                AccountOrPersona::from(account),
                AccountOrPersona::from(persona),
            ]))
        );
    }
}
