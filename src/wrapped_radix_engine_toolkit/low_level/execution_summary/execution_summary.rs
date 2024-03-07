use crate::prelude::*;

use radix_engine::types::indexmap::IndexMap;
use radix_engine_common::types::ComponentAddress as ScryptoComponentAddress;
use radix_engine_toolkit::transaction_types::ExecutionSummary as RetExecutionSummary;
use radix_engine_toolkit::transaction_types::ResourceIndicator as RetResourceIndicator;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ExecutionSummary {
    /// Addresses of accounts withdraws from in the manifest.
    pub addresses_of_account_withdraws:
        HashMap<AccountAddress, Vec<ResourceIndicator>>,

    /// Information on the global entities created in the transaction.
    pub new_entities: NewEntities,

    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub(crate) detailed_classification: Vec<DetailedManifestClass>,

    /// List of newly created Non-Fungibles during this transaction.
    pub newly_created_non_fungibles: Vec<NonFungibleGlobalId>,
}

fn addresses_of_account_withdraws_from_ret(
    ret: IndexMap<ScryptoComponentAddress, Vec<RetResourceIndicator>>,
    network_id: NetworkID,
) -> HashMap<AccountAddress, Vec<ResourceIndicator>> {
    ret.into_iter()
        .map(|p| {
            (
                Into::<AccountAddress>::into((p.0, network_id)),
                p.1.into_iter()
                    .map(|i| (i, network_id))
                    .map(Into::<ResourceIndicator>::into)
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>()
}

impl From<(RetExecutionSummary, NetworkID)> for ExecutionSummary {
    fn from(value: (RetExecutionSummary, NetworkID)) -> Self {
        let (ret_summary, network_id) = value;
        let _addresses_of_account_withdraws =
            addresses_of_account_withdraws_from_ret(
                ret_summary.account_withdraws,
                network_id,
            );

        let _new_entities: NewEntities =
            (ret_summary.new_entities, network_id).into();

        let _detailed_classification: Vec<DetailedManifestClass> = ret_summary
            .detailed_classification
            .into_iter()
            .map(|d| Into::<DetailedManifestClass>::into((d, network_id)))
            .collect_vec();

        todo!()
    }
}

/*
import Foundation

// MARK: - ExecutionSummary
public struct ExecutionSummary: DummySargon {
    public var accountWithdraws: [String: [ResourceIndicator]] {
        sargon()
    }

    public var accountDeposits: [String: [ResourceIndicator]] {
        sargon()
    }

    public var reservedInstructions: [ReservedInstruction] {
        sargon()
    }

    public var presentedProofs: [ResourceAddress] {
        sargon()
    }

    public var encounteredEntities: [Address] {
        sargon()
    }

    public var feeLocks: FeeLocks { sargon() }

    public var feeSummary: FeeSummary { sargon() }

    public var addressesOfNewlyCreatedEntities: [Address] {
        sargon()
    }
}

// MARK: - FeeSummary
public enum FeeSummary: DummySargon {
    public var executionCost: RETDecimal { sargon() }
    public var finalizationCost: RETDecimal { sargon() }
    public var storageExpansionCost: RETDecimal { sargon() }
    public var royaltyCost: RETDecimal { sargon() }
}

// MARK: - FeeLocks
public enum FeeLocks: DummySargon {
    public var lock: RETDecimal { sargon() }
    public var contingentLock: RETDecimal { sargon() }
}

// MARK: - ReservedInstruction
public enum ReservedInstruction: DummySargon {}

*/
