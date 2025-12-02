use crate::prelude::*;
use radix_rust::prelude::IndexMap;

pub(crate) fn to_vec_network_aware<T, U>(
    values: impl IntoIterator<Item = T>,
    network_id: NetworkID,
) -> Vec<U>
where
    U: From<(T, NetworkID)>,
{
    values
        .into_iter()
        .map(|x| (x, network_id))
        .map(U::from)
        .collect_vec()
}

pub(crate) fn to_hashmap_network_aware_key<K, V, L, U>(
    values: impl IntoIterator<Item = (K, V)>,
    network_id: NetworkID,
) -> HashMap<L, U>
where
    L: Eq + std::hash::Hash + From<(K, NetworkID)>,
    U: From<V>,
{
    values
        .into_iter()
        .map(|(k, v)| (L::from((k, network_id)), U::from(v)))
        .collect::<HashMap<L, U>>()
}

pub(crate) fn filter_try_to_hashmap_network_aware_key<K, V, L, U>(
    values: impl IntoIterator<Item = (K, V)>,
    network_id: NetworkID,
) -> HashMap<L, U>
where
    L: Eq + std::hash::Hash + TryFrom<(K, NetworkID)>,
    U: From<V>,
{
    values
        .into_iter()
        .filter_map(|(k, v)| {
            let l = L::try_from((k, network_id)).ok()?;
            Some((l, U::from(v)))
        })
        .collect::<HashMap<L, U>>()
}

pub(crate) fn filter_try_to_vec_network_aware<T, U>(
    values: impl IntoIterator<Item = T>,
    network_id: NetworkID,
) -> Vec<U>
where
    U: TryFrom<(T, NetworkID)>,
{
    values
        .into_iter()
        .map(|x| (x, network_id))
        .map(U::try_from)
        .filter_map(Result::ok)
        .collect_vec()
}

/// A summary of the execution of the manifest and the information that helps
/// wallets present the contents of a transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
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

    /// The manifest classification if any. None means that the manifest is non-conforming.
    pub detailed_classification: Option<DetailedManifestClass>,

    /// List of newly created Non-Fungibles during this transaction.
    pub newly_created_non_fungibles: Vec<NonFungibleGlobalId>,

    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: IndexSet<ReservedInstruction>,

    /// The list of the resources of proofs that were presented in the manifest.
    pub presented_proofs: Vec<ResourceSpecifier>,

    /// The set of all the encountered `ManifestEncounteredComponentAddress`es` in the manifest. This is
    /// to be primarily used for the "using dApps" section of the wallet's tx
    /// review screen.
    pub encountered_addresses: Vec<ManifestEncounteredComponentAddress>,

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
        reserved_instructions: impl Into<IndexSet<ReservedInstruction>>,
        presented_proofs: impl IntoIterator<Item = ResourceSpecifier>,
        encountered_addresses: impl IntoIterator<
            Item = ManifestEncounteredComponentAddress,
        >,
        detailed_classification: Option<DetailedManifestClass>,
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
            reserved_instructions: reserved_instructions.into(),
            presented_proofs: presented_proofs.into_iter().collect_vec(),
            encountered_addresses: encountered_addresses
                .into_iter()
                .collect_vec(),
            detailed_classification,
            fee_locks: fee_locks.into(),
            fee_summary: fee_summary.into(),
            new_entities: new_entities.into(),
        }
    }
}

impl ExecutionSummary {
    pub fn classify_delete_accounts_if_present(&mut self) {
        // Only try to classify if RET analysis didn't yield any classification
        if self.detailed_classification.is_some() {
            return;
        }

        let deleted_accounts: Vec<AccountAddress> = self
            .deposits
            .iter()
            .filter_map(|deposit| {
                let (account_address, resources) = deposit;

                resources
                    .iter()
                    .filter_map(|resource| {
                        resource.get_non_fungible_indicator()
                    })
                    .flat_map(|indicator| indicator.get_value())
                    // Find the account badge in the list of deposits
                    .any(|id| id.derives_account_address(*account_address))
                    .then_some(*account_address)
            })
            .collect();

        if !deleted_accounts.is_empty() {
            self.detailed_classification =
                Some(DetailedManifestClass::DeleteAccounts {
                    account_addresses: deleted_accounts,
                });
        }
    }
}

fn addresses_of_accounts_from_ret(
    ret: IndexMap<ScryptoGlobalAddress, Vec<RetInvocationIoItem>>,
    network_id: NetworkID,
) -> HashMap<AccountAddress, Vec<ResourceIndicator>> {
    ret.into_iter()
        .filter(|(_, deposits)| !deposits.is_empty())
        .map(|(address, invocation)| {
            (
                (address, network_id).into(),
                invocation
                    .into_iter()
                    .map(|i| (i, network_id))
                    .map(ResourceIndicator::from)
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>()
}

impl From<(RetDynamicAnalysis, NetworkID)> for ExecutionSummary {
    fn from(value: (RetDynamicAnalysis, NetworkID)) -> Self {
        let (ret, n) = value;

        let mut newly_created_non_fungibles = to_vec_network_aware(
            ret.entities_newly_created_summary.new_non_fungibles,
            n,
        );
        newly_created_non_fungibles.sort();

        let new_entities = NewEntities::from((
            (
                ret.entities_newly_created_summary.new_resource_entities,
                ret.entities_newly_created_summary.global_entities_metadata,
            ),
            n,
        ));

        let classification = ret
            .detailed_manifest_classification
            .into_iter()
            .filter_map(|d| DetailedManifestClass::new_from(d, n))
            .find_or_first(|class| !class.is_general());

        let mut summary = Self::new(
            addresses_of_accounts_from_ret(
                ret.account_dynamic_resource_movements_summary
                    .account_withdraws,
                n,
            ),
            addresses_of_accounts_from_ret(
                ret.account_dynamic_resource_movements_summary
                    .account_deposits,
                n,
            ),
            filter_try_to_vec_network_aware(
                ret.entities_requiring_auth_summary.accounts,
                n,
            ),
            filter_try_to_vec_network_aware(
                ret.entities_requiring_auth_summary.identities,
                n,
            ),
            newly_created_non_fungibles,
            ReservedInstruction::from_ret_reserved_instructions_output(
                ret.reserved_instructions_summary,
            ),
            ret.proofs_created_summary
                .created_proofs
                .values()
                .cloned()
                .flat_map(|vec| filter_try_to_vec_network_aware(vec, n)),
            filter_try_to_vec_network_aware(
                ret.entities_encountered_summary.entities,
                n,
            ),
            classification,
            ret.fee_locks_summary,
            ret.fee_consumption_summary,
            new_entities,
        );

        summary.classify_delete_accounts_if_present();

        summary
    }
}

impl ExecutionSummary {
    pub fn sample_stokenet() -> Self {
        Self {
            withdrawals: vec![(
                AccountAddress::sample_stokenet(),
                vec![ResourceIndicator::sample_stokenet()],
            )]
            .into_iter()
            .collect(),
            deposits: vec![(
                AccountAddress::sample_stokenet_other(),
                vec![ResourceIndicator::sample_stokenet()],
            )]
            .into_iter()
            .collect(),
            addresses_of_accounts_requiring_auth: vec![
                AccountAddress::sample_stokenet_other(),
            ],
            addresses_of_identities_requiring_auth: Vec::new(),
            newly_created_non_fungibles: Vec::new(),
            reserved_instructions: IndexSet::from([
                ReservedInstruction::sample(),
            ]),
            presented_proofs: Vec::new(),
            encountered_addresses: vec![
                ManifestEncounteredComponentAddress::sample_component_stokenet(
                ),
            ],
            detailed_classification: Some(DetailedManifestClass::sample()),
            fee_locks: FeeLocks::sample(),
            fee_summary: FeeSummary::sample(),
            new_entities: NewEntities::sample(),
        }
    }
}
impl HasSampleValues for ExecutionSummary {
    fn sample() -> Self {
        Self {
            withdrawals: vec![(
                AccountAddress::sample(),
                vec![ResourceIndicator::sample()],
            )]
            .into_iter()
            .collect(),
            deposits: vec![(
                AccountAddress::sample(),
                vec![ResourceIndicator::sample()],
            )]
            .into_iter()
            .collect(),
            addresses_of_accounts_requiring_auth: vec![AccountAddress::sample()],
            addresses_of_identities_requiring_auth: vec![
                IdentityAddress::sample(),
            ],
            newly_created_non_fungibles: vec![NonFungibleGlobalId::sample()],
            reserved_instructions: IndexSet::from([
                ReservedInstruction::sample(),
            ]),
            presented_proofs: vec![ResourceSpecifier::sample()],
            encountered_addresses: vec![
                ManifestEncounteredComponentAddress::sample(),
            ],
            detailed_classification: Some(DetailedManifestClass::sample()),
            fee_locks: FeeLocks::sample(),
            fee_summary: FeeSummary::sample(),
            new_entities: NewEntities::sample(),
        }
    }
    fn sample_other() -> Self {
        Self {
            withdrawals: vec![(
                AccountAddress::sample_other(),
                vec![ResourceIndicator::sample_other()],
            )]
            .into_iter()
            .collect(),
            deposits: vec![(
                AccountAddress::sample_other(),
                vec![ResourceIndicator::sample_other()],
            )]
            .into_iter()
            .collect(),
            addresses_of_accounts_requiring_auth: vec![
                AccountAddress::sample_other(),
            ],
            addresses_of_identities_requiring_auth: vec![
                IdentityAddress::sample_other(),
            ],
            newly_created_non_fungibles: vec![
                NonFungibleGlobalId::sample_other(),
            ],
            reserved_instructions: IndexSet::from([
                ReservedInstruction::sample_other(),
            ]),
            presented_proofs: vec![ResourceSpecifier::sample_other()],
            encountered_addresses: vec![
                ManifestEncounteredComponentAddress::sample_other(),
            ],
            detailed_classification: Some(DetailedManifestClass::sample_other()),
            fee_locks: FeeLocks::sample_other(),
            fee_summary: FeeSummary::sample_other(),
            new_entities: NewEntities::sample_other(),
        }
    }
}
