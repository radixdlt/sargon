use radix_rust::prelude::IndexMap;

use crate::prelude::*;

/// A summary of the execution of the manifest and the information that helps
/// wallets present the contents of a transaction.
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ExecutionSummary {
    /// Per account, a list of all token balances that has been withdrawn from that account.
    pub withdrawals: HashMap<AccountAddress, Vec<ResourceIndicator>>,

    /// Per account, a list of all token balances that has been deposited into that account.
    pub deposits: HashMap<AccountAddress, Vec<ResourceIndicator>>,

    /// Addresses of accounts encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the accounts of all those addresses, which might be multiple
    /// signatures per Account, if MFA has been setup.
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,

    /// Addresses of identities (Personas) encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the identities of all those addresses, which might be multiple
    /// signatures per Persona, if MFA has been setup.
    pub addresses_of_identities_requiring_auth: Vec<IdentityAddress>,

    /// Information on the global entities created in the transaction.
    pub new_entities: NewEntities,

    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub detailed_classification: Vec<DetailedManifestClass>,

    /// List of newly created Non-Fungibles during this transaction.
    pub newly_created_non_fungibles: Vec<NonFungibleGlobalId>,

    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: Vec<ReservedInstruction>,

    /// The list of the resources of proofs that were presented in the manifest.
    pub presented_proofs: Vec<ResourceSpecifier>,

    /// The set of all the encountered `ManifestEncounteredAddress`es` in the manifest. This is
    /// to be primarily used for the "using dApps" section of the wallet's tx
    /// review screen.
    pub encountered_addresses: Vec<ManifestEncounteredAddress>,

    /// Information on how much fees were contingent and how much were not.
    pub fee_locks: FeeLocks,

    /// Detailed information on the amount of cost units consumed.
    pub fee_summary: FeeSummary,
}

impl ExecutionSummary {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        addresses_of_accounts_withdrawn_from: impl Into<
            HashMap<AccountAddress, Vec<ResourceIndicator>>,
        >,
        addresses_of_accounts_deposited_into: impl Into<
            HashMap<AccountAddress, Vec<ResourceIndicator>>,
        >,
        addresses_of_accounts_requiring_auth: impl IntoIterator<
            Item = AccountAddress,
        >,
        addresses_of_identities_requiring_auth: impl IntoIterator<
            Item = IdentityAddress,
        >,
        newly_created_non_fungibles: impl IntoIterator<Item = NonFungibleGlobalId>,
        reserved_instructions: impl IntoIterator<Item = ReservedInstruction>,
        presented_proofs: impl IntoIterator<Item = ResourceSpecifier>,
        encountered_addresses: impl IntoIterator<Item = ManifestEncounteredAddress>,
        detailed_classification: impl IntoIterator<Item = DetailedManifestClass>,
        fee_locks: impl Into<FeeLocks>,
        fee_summary: impl Into<FeeSummary>,
        new_entities: impl Into<NewEntities>,
    ) -> Self {
        Self {
            withdrawals: addresses_of_accounts_withdrawn_from.into(),
            deposits: addresses_of_accounts_deposited_into.into(),
            addresses_of_accounts_requiring_auth:
                addresses_of_accounts_requiring_auth
                    .into_iter()
                    .collect_vec(),
            addresses_of_identities_requiring_auth:
                addresses_of_identities_requiring_auth
                    .into_iter()
                    .collect_vec(),
            newly_created_non_fungibles: newly_created_non_fungibles
                .into_iter()
                .collect_vec(),
            reserved_instructions: reserved_instructions
                .into_iter()
                .collect_vec(),
            presented_proofs: presented_proofs.into_iter().collect_vec(),
            encountered_addresses: encountered_addresses
                .into_iter()
                .collect_vec(),
            detailed_classification: detailed_classification
                .into_iter()
                .collect_vec(),
            fee_locks: fee_locks.into(),
            fee_summary: fee_summary.into(),
            new_entities: new_entities.into(),
        }
    }
}

fn addresses_of_accounts_from_ret(
    ret: IndexMap<ScryptoComponentAddress, Vec<RetResourceIndicator>>,
    network_id: NetworkID,
) -> HashMap<AccountAddress, Vec<ResourceIndicator>> {
    ret.into_iter()
        .map(|p| {
            (
                AccountAddress::from((p.0, network_id)),
                p.1.into_iter()
                    .map(|i| (i, network_id))
                    .map(ResourceIndicator::from)
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>()
}

impl From<(RetExecutionSummary, NetworkID)> for ExecutionSummary {
    fn from(value: (RetExecutionSummary, NetworkID)) -> Self {
        let (ret, n) = value;

        let mut newly_created_non_fungibles =
            to_vec_network_aware(ret.newly_created_non_fungibles, n);
        newly_created_non_fungibles.sort();

        Self::new(
            addresses_of_accounts_from_ret(ret.account_withdraws, n),
            addresses_of_accounts_from_ret(ret.account_deposits, n),
            to_vec_network_aware(ret.accounts_requiring_auth, n),
            to_vec_network_aware(ret.identities_requiring_auth, n),
            newly_created_non_fungibles,
            ret.reserved_instructions
                .into_iter()
                .map(ReservedInstruction::from),
            ret.presented_proofs
                .values()
                .cloned()
                .flat_map(|vec| filter_try_to_vec_network_aware(vec, n)),
            filter_try_to_vec_network_aware(ret.encountered_entities, n),
            ret.detailed_classification
                .into_iter()
                .map(|d| DetailedManifestClass::from((d, n))),
            ret.fee_locks,
            ret.fee_summary,
            (ret.new_entities, n),
        )
    }
}
