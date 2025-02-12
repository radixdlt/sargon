use crate::prelude::*;

/// A summary of the manifest
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestSummary {
    /// The withdrawals done in the manifest.
    pub account_withdrawals: HashMap<AccountAddress, Vec<AccountWithdraw>>,

    /// The deposits done in the manifest.
    pub account_deposits: HashMap<AccountAddress, AccountDeposits>,

    /// The list of the resources of proofs that were presented in the manifest.
    pub presented_proofs: Vec<ResourceSpecifier>,

    /// Addresses of accounts withdrawn from in the manifest.
    pub addresses_of_accounts_withdrawn_from: Vec<AccountAddress>,

    /// Addresses of accounts deposited into in the manifest.
    pub addresses_of_accounts_deposited_into: Vec<AccountAddress>,

    /// The set of all the global entities encountered in the manifest. This is
    /// to be primarily used for the "using dApps" section of the wallet's tx
    /// review screen.
    pub encountered_entities: Vec<ManifestEncounteredComponentAddress>,

    /// Addresses of accounts encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the accounts of all those addresses, which might be multiple
    /// signatures per Account, if MFA has been setup.
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,

    /// Addresses of identities (Personas) encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the identities of all those addresses, which might be multiple
    /// signatures per Persona, if MFA has been setup.
    pub addresses_of_personas_requiring_auth: Vec<IdentityAddress>,

    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: Vec<ReservedInstruction>,

    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub classification: IndexSet<RetManifestClass>,
}

impl ManifestSummary {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_withdraws: impl Into<HashMap<AccountAddress, Vec<AccountWithdraw>>>,
        account_deposits: impl Into<HashMap<AccountAddress, AccountDeposits>>,
        presented_proofs: impl IntoIterator<Item = ResourceSpecifier>,
        withdrawn_from: impl IntoIterator<Item = AccountAddress>,
        deposited_into: impl IntoIterator<Item = AccountAddress>,
        encountered_entities: impl IntoIterator<
            Item = ManifestEncounteredComponentAddress,
        >,
        accounts_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        personas_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
        reserved_instructions: impl IntoIterator<Item = ReservedInstruction>,
        classification: impl IntoIterator<Item = RetManifestClass>,
    ) -> Self {
        Self {
            account_withdrawals: account_withdraws.into(),
            account_deposits: account_deposits.into(),
            presented_proofs: presented_proofs.into_iter().collect(),
            addresses_of_accounts_withdrawn_from: withdrawn_from
                .into_iter()
                .collect::<IndexSet<_>>()
                .into_iter()
                .collect_vec(),
            addresses_of_accounts_deposited_into: deposited_into
                .into_iter()
                .collect::<IndexSet<_>>()
                .into_iter()
                .collect_vec(),
            encountered_entities: encountered_entities
                .into_iter()
                .collect::<IndexSet<_>>()
                .into_iter()
                .collect_vec(),
            addresses_of_accounts_requiring_auth: accounts_requiring_auth
                .into_iter()
                .collect::<IndexSet<_>>()
                .into_iter()
                .collect_vec(),
            addresses_of_personas_requiring_auth: personas_requiring_auth
                .into_iter()
                .collect::<IndexSet<_>>()
                .into_iter()
                .collect_vec(),
            reserved_instructions: reserved_instructions.into_iter().collect(),
            classification: classification.into_iter().collect::<IndexSet<_>>(),
        }
    }
}

fn convert_from_scrypto<T, U>(
    scrypto: IndexMap<ScryptoComponentAddress, Vec<T>>,
    network_id: NetworkID,
    convert_item: fn(T, NetworkID) -> U,
) -> HashMap<AccountAddress, Vec<U>> {
    scrypto
        .into_iter()
        .map(|(addr, items)| {
            (
                AccountAddress::from((addr, network_id)),
                items
                    .into_iter()
                    .map(|item| convert_item(item, network_id))
                    .collect(),
            )
        })
        .collect()
}

fn account_withdraw_from_scrypto(
    item: ScryptoAccountWithdraw,
    network_id: NetworkID,
) -> AccountWithdraw {
    AccountWithdraw::from((item, network_id))
}

impl From<(RetStaticAnalysis, NetworkID)> for ManifestSummary {
    fn from(value: (RetStaticAnalysis, NetworkID)) -> Self {
        let (ret, network_id) = value;

        let account_withdraws = convert_from_scrypto(
            ret.account_static_resource_movements_summary
                .account_withdraws,
            network_id,
            account_withdraw_from_scrypto,
        );

        let account_deposits: HashMap<AccountAddress, AccountDeposits> = ret
            .account_static_resource_movements_summary
            .account_deposits
            .into_iter()
            .map(|(addr, items)| {
                (
                    AccountAddress::from((addr, network_id)),
                    AccountDeposits::from((items, network_id)),
                )
            })
            .collect();

        let addresses_of_accounts_withdrawn_from =
            filter_try_to_vec_network_aware(
                ret.account_interactions_summary.accounts_withdrawn_from,
                network_id,
            );

        let addresses_of_accounts_deposited_into =
            filter_try_to_vec_network_aware(
                ret.account_interactions_summary.accounts_deposited_into,
                network_id,
            );

        let addresses_of_accounts_requiring_auth =
            filter_try_to_vec_network_aware(
                ret.entities_requiring_auth_summary.accounts,
                network_id,
            );

        let addresses_of_personas_requiring_auth =
            filter_try_to_vec_network_aware(
                ret.entities_requiring_auth_summary.identities,
                network_id,
            );

        let presented_proofs = ret
            .proofs_created_summary
            .created_proofs
            .values()
            .cloned()
            .flat_map(|vec| filter_try_to_vec_network_aware(vec, network_id));

        let encountered_entities = filter_try_to_vec_network_aware(
            ret.entities_encountered_summary.entities,
            network_id,
        );
        let reserved_instructions =
            ReservedInstruction::from_ret_reserved_instructions_output(
                ret.reserved_instructions_summary,
            );

        Self::new(
            account_withdraws,
            account_deposits,
            presented_proofs,
            addresses_of_accounts_withdrawn_from,
            addresses_of_accounts_deposited_into,
            encountered_entities,
            addresses_of_accounts_requiring_auth,
            addresses_of_personas_requiring_auth,
            reserved_instructions,
            ret.manifest_classification,
        )
    }
}

impl HasSampleValues for ManifestSummary {
    fn sample() -> Self {
        Self {
            account_withdrawals: HashMap::new(),
            account_deposits: HashMap::new(),
            presented_proofs: Vec::<_>::sample(),
            addresses_of_accounts_withdrawn_from: Vec::<_>::sample(),
            addresses_of_accounts_deposited_into: Vec::<_>::sample(),
            encountered_entities: Vec::<_>::sample(),
            addresses_of_accounts_requiring_auth: Vec::<_>::sample(),
            addresses_of_personas_requiring_auth: Vec::<_>::sample(),
            reserved_instructions: Vec::<_>::sample(),
            classification: IndexSet::just(RetManifestClass::GeneralSubintent),
        }
    }

    fn sample_other() -> Self {
        Self {
            account_withdrawals: HashMap::new(),
            account_deposits: HashMap::new(),
            presented_proofs: Vec::<_>::sample_other(),
            addresses_of_accounts_withdrawn_from: Vec::<_>::sample_other(),
            addresses_of_accounts_deposited_into: Vec::<_>::sample_other(),
            encountered_entities: Vec::<_>::sample_other(),
            addresses_of_accounts_requiring_auth: Vec::<_>::sample_other(),
            addresses_of_personas_requiring_auth: Vec::<_>::sample_other(),
            reserved_instructions: Vec::<_>::sample_other(),
            classification: IndexSet::just(RetManifestClass::Transfer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ManifestSummary;

    #[test]
    fn duplicates_are_removed_from_addresses_of_accounts_withdrawn_from() {
        let duplicates =
            vec![AccountAddress::sample(), AccountAddress::sample()];
        assert_eq!(duplicates.len(), 2);
        let sut = SUT::new(
            HashMap::default(),
            HashMap::default(),
            Vec::default(),
            duplicates,
            Vec::default(),
            Vec::default(),
            Vec::default(),
            Vec::default(),
            Vec::default(),
            Vec::default(),
        );
        assert_eq!(sut.addresses_of_accounts_withdrawn_from.len(), 1);
    }

    #[test]
    fn duplicates_are_removed_from_addresses_of_accounts_deposited_into() {
        let duplicates =
            vec![AccountAddress::sample(), AccountAddress::sample()];
        assert_eq!(duplicates.len(), 2);
        let sut = SUT::new(
            HashMap::default(),
            HashMap::default(),
            Vec::default(),
            Vec::default(),
            duplicates,
            Vec::default(),
            Vec::default(),
            Vec::default(),
            Vec::default(),
            Vec::default(),
        );
        assert_eq!(sut.addresses_of_accounts_deposited_into.len(), 1);
    }
}
