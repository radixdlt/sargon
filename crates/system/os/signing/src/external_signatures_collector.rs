use crate::prelude::*;
use addresses::prelude::NonFungibleLocalId;
use gateway_models::prelude::{
    AccessRule, BasicRequirement, CompositeRequirement, NonFungible,
    Requirement,
};

#[derive(Clone, Debug)]
pub struct ExternalAccountAccessRule {
    pub owner: AddressOfAccountOrPersona,
    pub access_rule: AccessRule,
}

#[async_trait::async_trait]
pub trait FactorInstanceLookupByNftIds: Send + Sync {
    async fn factor_instances_for_nfts(
        &self,
        nft_ids: Vec<NonFungibleGlobalId>,
    ) -> Result<Vec<HierarchicalDeterministicFactorInstance>>;
}

pub async fn collect_external_signatures<S: Signable>(
    signable: S,
    external_accounts: Vec<ExternalAccountAccessRule>,
    lookup: &impl FactorInstanceLookupByNftIds,
    interactor: Arc<dyn SignInteractor<S>>,
) -> Result<IndexSet<HDSignature<S::ID>>> {
    let owned_instances =
        owned_factor_instances_from_access_rules(external_accounts, lookup)
            .await?;

    if owned_instances.is_empty() {
        return Ok(IndexSet::new());
    }

    let mut signatures = IndexSet::new();
    let mut per_kind =
        IndexMap::<FactorSourceKind, IndexMap<FactorSourceIDFromHash, IndexSet<OwnedFactorInstance>>>::new();

    for owned in owned_instances {
        let factor_source_id = owned.factor_source_id();
        per_kind
            .entry(factor_source_id.kind)
            .or_default()
            .entry(factor_source_id)
            .or_default()
            .insert(owned);
    }

    for (kind, per_factor_source) in per_kind {
        let per_factor_source_input = per_factor_source
            .into_iter()
            .map(|(factor_source_id, owned)| {
                let per_transaction = IndexSet::just(
                    TransactionSignRequestInput::new(
                        signable.get_payload(),
                        factor_source_id,
                        owned,
                    ),
                );
                (
                    factor_source_id,
                    PerFactorSourceInput::new(
                        factor_source_id,
                        per_transaction,
                        IndexSet::new(),
                    ),
                )
            })
            .collect::<IndexMap<_, _>>();

        let request = SignRequest::new(kind, per_factor_source_input);
        let response = interactor.sign(request).await?;
        signatures.extend(signatures_from_response(response));
    }

    Ok(signatures)
}

async fn owned_factor_instances_from_access_rules(
    external_accounts: Vec<ExternalAccountAccessRule>,
    lookup: &impl FactorInstanceLookupByNftIds,
) -> Result<IndexSet<OwnedFactorInstance>> {
    let mut owned_instances = IndexSet::new();
    for external in external_accounts {
        let nft_ids =
            extract_nft_ids_from_access_rule(&external.access_rule)?;
        let factor_instances = lookup
            .factor_instances_for_nfts(nft_ids.into_iter().collect())
            .await?;
        for instance in factor_instances {
            owned_instances.insert(OwnedFactorInstance::owned_factor_instance(
                external.owner,
                instance,
            ));
        }
    }

    Ok(owned_instances)
}

fn signatures_from_response<ID: SignableID>(
    response: SignResponse<ID>,
) -> IndexSet<HDSignature<ID>> {
    response
        .per_factor_outcome
        .values()
        .filter_map(|outcome| outcome.as_signed().cloned())
        .flatten()
        .collect()
}

pub fn extract_nft_ids_from_access_rule(
    access_rule: &AccessRule,
) -> Result<IndexSet<NonFungibleGlobalId>> {
    match access_rule {
        AccessRule::AllowAll | AccessRule::DenyAll => Ok(IndexSet::new()),
        AccessRule::Protected { access_rule } => {
            extract_nft_ids_from_composite_requirement(access_rule)
        }
    }
}

fn extract_nft_ids_from_composite_requirement(
    requirement: &CompositeRequirement,
) -> Result<IndexSet<NonFungibleGlobalId>> {
    match requirement {
        CompositeRequirement::ProofRule { proof_rule } => {
            extract_nft_ids_from_basic_requirement(proof_rule)
        }
        CompositeRequirement::AnyOf { access_rules }
        | CompositeRequirement::AllOf { access_rules } => {
            let mut ids = IndexSet::new();
            for rule in access_rules {
                ids.extend(extract_nft_ids_from_composite_requirement(rule)?);
            }
            Ok(ids)
        }
    }
}

fn extract_nft_ids_from_basic_requirement(
    requirement: &BasicRequirement,
) -> Result<IndexSet<NonFungibleGlobalId>> {
    match requirement {
        BasicRequirement::Require { requirement } => {
            extract_nft_ids_from_requirement(requirement)
        }
        BasicRequirement::AllOf { list }
        | BasicRequirement::AnyOf { list }
        | BasicRequirement::CountOf { list, .. } => {
            let mut ids = IndexSet::new();
            for requirement in list {
                ids.extend(extract_nft_ids_from_requirement(requirement)?);
            }
            Ok(ids)
        }
        BasicRequirement::AmountOf { .. } => Ok(IndexSet::new()),
    }
}

fn extract_nft_ids_from_requirement(
    requirement: &Requirement,
) -> Result<IndexSet<NonFungibleGlobalId>> {
    match requirement {
        Requirement::Resource { .. } => Ok(IndexSet::new()),
        Requirement::NonFungible { non_fungible } => Ok(IndexSet::just(
            non_fungible_global_id(non_fungible)?,
        )),
    }
}

fn non_fungible_global_id(
    non_fungible: &NonFungible,
) -> Result<NonFungibleGlobalId> {
    let local_id =
        NonFungibleLocalId::from_str(&non_fungible.local_id.simple_rep)?;
    Ok(NonFungibleGlobalId::new_unchecked(
        non_fungible.resource_address,
        local_id,
    ))
}
