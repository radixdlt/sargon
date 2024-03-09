use crate::prelude::*;

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier as ScryptoResourceSpecifier;
use radix_engine::types::indexmap::IndexMap;
use radix_engine_common::types::ComponentAddress as ScryptoComponentAddress;

use radix_engine_toolkit::transaction_types::ExecutionSummary as RetExecutionSummary;
use radix_engine_toolkit::transaction_types::ResourceIndicator as RetResourceIndicator;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ExecutionSummary {
    /// Addresses of accounts withdraws from in the manifest.
    pub addresses_of_account_withdraws:
        HashMap<AccountAddress, Vec<ResourceIndicator>>,

    /// Addresses of accounts deposited to in the manifest.
    pub addresses_of_account_deposits:
        HashMap<AccountAddress, Vec<ResourceIndicator>>,

    /// Addresses of accounts encountered in the manifest where privileged
    /// methods were called.
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,

    /// Addresses of identities (Personas) encountered in the manifest where privileged
    /// methods were called.
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
    pub presented_proofs: Vec<ResourceAddress>,

    /// The set of all the encountered `ComponentAddress`es` in the manifest. This is
    /// to be primarily used for the "using dApps" section of the wallet's tx
    /// review screen.
    pub encountered_component_addresses: Vec<ComponentAddress>,

    /// Information on how much fees were contingent and how much were not.
    pub fee_locks: FeeLocks,

    /// Detailed information on the amount of cost units consumed.
    pub fee_summary: FeeSummary,
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

trait HasResourceAddressOnNetwork {
    fn resource_address(&self, network_id: NetworkID) -> ResourceAddress;
}
impl HasResourceAddressOnNetwork for ScryptoResourceSpecifier {
    fn resource_address(&self, network_id: NetworkID) -> ResourceAddress {
        match self {
            ScryptoResourceSpecifier::Amount(resource_address, _) => {
                (*resource_address, network_id).into()
            }
            ScryptoResourceSpecifier::Ids(resource_address, _) => {
                (*resource_address, network_id).into()
            }
        }
    }
}

impl From<(RetExecutionSummary, NetworkID)> for ExecutionSummary {
    fn from(value: (RetExecutionSummary, NetworkID)) -> Self {
        let (ret_summary, network_id) = value;
        let addresses_of_account_withdraws = addresses_of_accounts_from_ret(
            ret_summary.account_withdraws,
            network_id,
        );

        let addresses_of_account_deposits = addresses_of_accounts_from_ret(
            ret_summary.account_deposits,
            network_id,
        );

        let new_entities: NewEntities =
            (ret_summary.new_entities, network_id).into();

        let detailed_classification: Vec<DetailedManifestClass> = ret_summary
            .detailed_classification
            .into_iter()
            .map(|d| DetailedManifestClass::from((d, network_id)))
            .collect_vec();

        let reserved_instructions: Vec<ReservedInstruction> = ret_summary
            .reserved_instructions
            .into_iter()
            .map(ReservedInstruction::from)
            .collect();

        let mut newly_created_non_fungibles = to_vec_network_aware(
            ret_summary.newly_created_non_fungibles,
            network_id,
        );
        newly_created_non_fungibles.sort();

        // iOS Wallet only use `Vec<ResourceAddress>` for `presented_proofs` today,
        // have to assert Android does the same.
        let presented_proofs = ret_summary
            .presented_proofs
            .values()
            .cloned()
            .flat_map(|x| x.into_iter().map(|y| y.resource_address(network_id)))
            .collect_vec();

        let encountered_component_addresses = filter_try_to_vec_network_aware(
            ret_summary.encountered_entities,
            network_id,
        );

        let fee_locks = ret_summary.fee_locks.into();

        let fee_summary = ret_summary.fee_summary.into();

        let addresses_of_accounts_requiring_auth = to_vec_network_aware(
            ret_summary.accounts_requiring_auth,
            network_id,
        );

        let addresses_of_identities_requiring_auth = to_vec_network_aware(
            ret_summary.identities_requiring_auth,
            network_id,
        );

        Self {
            addresses_of_account_withdraws,
            addresses_of_account_deposits,
            addresses_of_accounts_requiring_auth,
            addresses_of_identities_requiring_auth,
            new_entities,
            detailed_classification,
            newly_created_non_fungibles,
            reserved_instructions,
            presented_proofs,
            encountered_component_addresses,
            fee_locks,
            fee_summary,
        }
    }
}
