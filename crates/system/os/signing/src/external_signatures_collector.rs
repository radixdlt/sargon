use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct ExternalAccountNftRequirements {
    pub owner: AddressOfAccountOrPersona,
    pub required_nft_ids: IndexSet<NonFungibleGlobalId>,
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
    external_accounts: Vec<ExternalAccountNftRequirements>,
    lookup: &impl FactorInstanceLookupByNftIds,
    interactor: Arc<dyn SignInteractor<S>>,
) -> Result<IndexSet<HDSignature<S::ID>>> {
    info!(
        "External signing: start for signable {:?} (accounts: {})",
        signable.get_id(),
        external_accounts.len()
    );
    let owned_instances = owned_factor_instances_from_nft_requirements(
        external_accounts,
        lookup,
    )
    .await?;

    if owned_instances.is_empty() {
        debug!("External signing: no owned factor instances resolved");
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
        debug!(
            "External signing: signing with kind {:?} (factor sources: {})",
            kind,
            per_factor_source.len()
        );
        let per_factor_source_input = per_factor_source
            .into_iter()
            .map(|(factor_source_id, owned)| {
                debug!(
                    "External signing: building request for factor source {} (instances: {})",
                    factor_source_id,
                    owned.len()
                );
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
        let response_signatures = signatures_from_response(response);
        debug!(
            "External signing: received {} signatures for kind {:?}",
            response_signatures.len(),
            kind
        );
        signatures.extend(response_signatures);
    }

    info!(
        "External signing: collected {} signatures total",
        signatures.len()
    );
    Ok(signatures)
}

async fn owned_factor_instances_from_nft_requirements(
    external_accounts: Vec<ExternalAccountNftRequirements>,
    lookup: &impl FactorInstanceLookupByNftIds,
) -> Result<IndexSet<OwnedFactorInstance>> {
    debug!(
        "External signing: resolving owned factor instances for {} external accounts",
        external_accounts.len()
    );
    let mut owned_instances = IndexSet::new();
    for external in external_accounts {
        let nft_ids = external.required_nft_ids;
        if nft_ids.is_empty() {
            debug!(
                "External signing: account {} has no required NFT ids",
                external.owner
            );
            continue;
        }
        debug!(
            "External signing: account {} requires {} NFT ids",
            external.owner,
            nft_ids.len()
        );
        let factor_instances = lookup
            .factor_instances_for_nfts(nft_ids.into_iter().collect())
            .await?;
        debug!(
            "External signing: resolved {} factor instances for account {}",
            factor_instances.len(),
            external.owner
        );
        for instance in factor_instances {
            owned_instances.insert(OwnedFactorInstance::owned_factor_instance(
                external.owner,
                instance,
            ));
        }
    }

    debug!(
        "External signing: total owned factor instances resolved: {}",
        owned_instances.len()
    );
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

#[cfg(test)]
mod tests {
    use super::*;

    fn nft(resource: ResourceAddress, simple_rep: &str) -> NonFungible {
        NonFungible {
            resource_address: resource,
            local_id: NonFungibleLocalIdId {
                id_type: "Integer".to_string(),
                sbor_hex: "".to_string(),
                simple_rep: simple_rep.to_string(),
            },
        }
    }

    #[test]
    fn extracts_nft_ids_from_access_rule() {
        let resource = ResourceAddress::sample();
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AnyOf {
                access_rules: vec![
                    CompositeRequirement::ProofRule {
                        proof_rule: BasicRequirement::Require {
                            requirement: Requirement::NonFungible {
                                non_fungible: nft(resource, "#1#"),
                            },
                        },
                    },
                    CompositeRequirement::ProofRule {
                        proof_rule: BasicRequirement::Require {
                            requirement: Requirement::NonFungible {
                                non_fungible: nft(resource, "#2#"),
                            },
                        },
                    },
                ],
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        let expected: IndexSet<NonFungibleGlobalId> = IndexSet::from_iter([
            NonFungibleGlobalId::new_unchecked(
                resource,
                NonFungibleLocalId::integer(1),
            ),
            NonFungibleGlobalId::new_unchecked(
                resource,
                NonFungibleLocalId::integer(2),
            ),
        ]);

        assert_eq!(ids, expected);
    }

    #[test]
    fn ignores_non_nft_requirements() {
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::AmountOf {
                    amount: Decimal192::one(),
                    resource: ResourceAddress::sample_other(),
                },
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert!(ids.is_empty());
    }
}
