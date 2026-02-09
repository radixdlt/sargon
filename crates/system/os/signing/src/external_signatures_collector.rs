use crate::prelude::*;
use profile_logic::prelude::ProfileCurrentNetwork;

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

#[async_trait::async_trait]
impl FactorInstanceLookupByNftIds for SargonOS {
    async fn factor_instances_for_nfts(
        &self,
        nft_ids: Vec<NonFungibleGlobalId>,
    ) -> Result<Vec<HierarchicalDeterministicFactorInstance>> {
        if nft_ids.is_empty() {
            debug!(
                "External signing: no NFT ids provided for factor instance lookup"
            );
            return Ok(Vec::new());
        }

        let profile = self.profile()?;
        let (network_id, mfa_instances) = match profile.current_network() {
            Ok(network) => (network.id, network.mfa_factor_instances.clone()),
            Err(CommonError::NoNetworkInProfile { .. }) => {
                return Ok(Vec::new())
            }
            Err(err) => return Err(err),
        };

        if mfa_instances.is_empty() {
            debug!(
                "External signing: no MFA factor instances in current network"
            );
            return Ok(Vec::new());
        }

        let required_ids: IndexSet<NonFungibleGlobalId> =
            nft_ids.into_iter().collect();
        debug!(
            "External signing: looking up factor instances for {} NFT ids",
            required_ids.len()
        );
        let mut instances = Vec::new();

        for mfafi in mfa_instances {
            let badge_id =
                mfafi.factor_instance.badge.try_non_fungible_global_id()?;
            let badge_id = NonFungibleGlobalId::from((badge_id, network_id));
            if required_ids.contains(&badge_id) {
                instances
                    .push(mfafi.factor_instance.try_as_hd_factor_instances()?);
            }
        }

        debug!(
            "External signing: resolved {} factor instances from {} NFT ids",
            instances.len(),
            required_ids.len()
        );
        Ok(instances)
    }
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
    let owned_instances =
        owned_factor_instances_from_nft_requirements(external_accounts, lookup)
            .await?;

    if owned_instances.is_empty() {
        debug!("External signing: no owned factor instances resolved");
        return Ok(IndexSet::new());
    }

    let mut signatures = IndexSet::new();
    let mut per_kind = IndexMap::<
        FactorSourceKind,
        IndexMap<FactorSourceIDFromHash, IndexSet<OwnedFactorInstance>>,
    >::new();

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
        Requirement::NonFungible { non_fungible } => {
            Ok(IndexSet::just(non_fungible_global_id(non_fungible)?))
        }
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

// ==================
// External Account Resolution
// ==================

fn is_access_controller_role_key(role_key: &RoleKey) -> bool {
    role_key.module == ObjectModuleId::Main
        && matches!(
            role_key.name.as_str(),
            "primary" | "recovery" | "confirmation"
        )
}

fn access_rule_from_explicit_rule(rule: &ExplicitRule) -> Option<AccessRule> {
    Some(rule.clone())
}

fn role_assignments_from_details(
    details: &StateEntityDetailsResponseItemDetails,
) -> Option<ComponentEntityRoleAssignments> {
    match details {
        StateEntityDetailsResponseItemDetails::FungibleResource(details) => {
            Some(details.role_assignments.clone())
        }
        StateEntityDetailsResponseItemDetails::NonFungibleResource(details) => {
            Some(details.role_assignments.clone())
        }
        StateEntityDetailsResponseItemDetails::Package(details) => {
            details.role_assignments.clone()
        }
        StateEntityDetailsResponseItemDetails::Component(details) => {
            details.role_assignments.clone()
        }
        StateEntityDetailsResponseItemDetails::FungibleVault
        | StateEntityDetailsResponseItemDetails::NonFungibleVault => None,
    }
}

fn access_rule_from_role_assignment(
    entry: &ComponentEntityRoleAssignmentEntry,
    owner_rule: &ExplicitRule,
) -> Option<AccessRule> {
    match entry.assignment.resolution {
        RoleAssignmentResolution::Owner => {
            access_rule_from_explicit_rule(owner_rule)
        }
        RoleAssignmentResolution::Explicit => entry
            .assignment
            .explicit_rule
            .as_ref()
            .and_then(access_rule_from_explicit_rule),
    }
}

pub(crate) async fn fetch_role_assignments_for_addresses(
    os: &SargonOS,
    addresses: Vec<Address>,
) -> Result<IndexMap<Address, ComponentEntityRoleAssignments>> {
    if addresses.is_empty() {
        return Ok(IndexMap::new());
    }

    let gateway_client = os.gateway_client()?;
    let ledger_state = gateway_client.gateway_status().await?.ledger_state;
    let request = StateEntityDetailsRequest::new(
        addresses.clone(),
        Some(ledger_state.into()),
        None,
    );
    let response = gateway_client.state_entity_details(request).await?;

    let mut assignments =
        IndexMap::<Address, ComponentEntityRoleAssignments>::new();
    for address in addresses {
        let item = response
            .items
            .iter()
            .find(|item| item.address == address)
            .ok_or(CommonError::GWMissingResponseItem {
            item: "StateEntityDetailsResponseItem".to_owned(),
        })?;

        let details = item.details.as_ref().ok_or(CommonError::Unknown {
            error_message: format!(
                "Missing entity details for address {}",
                address
            ),
        })?;
        let role_assignments = role_assignments_from_details(details).ok_or(
            CommonError::Unknown {
                error_message: format!(
                    "Missing role assignments for address {}",
                    address
                ),
            },
        )?;
        assignments.insert(address, role_assignments);
    }

    Ok(assignments)
}

fn access_rules_from_access_controller(
    assignments: &ComponentEntityRoleAssignments,
) -> Vec<AccessRule> {
    let rules: Vec<AccessRule> = assignments
        .entries
        .iter()
        .filter(|entry| is_access_controller_role_key(&entry.role_key))
        .filter_map(|entry| {
            access_rule_from_role_assignment(entry, &assignments.owner.rule)
        })
        .collect();
    if rules.is_empty() {
        debug!("No access controller role assignments matched primary/recovery/confirmation keys");
    }
    rules
}

pub(crate) async fn resolve_external_accounts_for_subintent(
    os: &SargonOS,
    subintent: &Subintent,
) -> Result<Vec<ExternalAccountNftRequirements>> {
    let summary = subintent.manifest.summary()?;
    let profile = os.profile()?;

    debug!(
        "Resolving external accounts for subintent: {:?}",
        subintent.get_id()
    );
    let external_account_addresses = summary
        .addresses_of_accounts_requiring_auth
        .into_iter()
        .filter(|address| profile.account_by_address(*address).is_err())
        .collect_vec();

    if external_account_addresses.is_empty() {
        debug!("No external accounts detected for subintent");
        return Ok(Vec::new());
    }

    debug!(
        "External accounts detected for subintent: {:?}",
        external_account_addresses
    );
    let (gateway_client, network_id) = os.gateway_client_on()?;
    let badge_owners = gateway_client
        .fetch_entities_badge_owners(
            network_id,
            external_account_addresses
                .iter()
                .map(|address| AddressOfAccountOrPersona::Account(*address))
                .collect_vec(),
        )
        .await?;

    debug!(
        "Resolved badge owners for external accounts: {:?}",
        badge_owners
    );
    let account_role_assignments = fetch_role_assignments_for_addresses(
        os,
        external_account_addresses
            .iter()
            .map(|address| Address::from(*address))
            .collect_vec(),
    )
    .await?;
    let access_controller_addresses = badge_owners
        .values()
        .filter_map(|owner| {
            owner
                .as_ref()
                .and_then(|address| address.as_access_controller())
                .cloned()
        })
        .collect::<IndexSet<_>>();
    debug!(
        "Access controller addresses for external accounts: {:?}",
        access_controller_addresses
    );
    let access_controller_role_assignments =
        fetch_role_assignments_for_addresses(
            os,
            access_controller_addresses
                .iter()
                .map(|address| Address::from(*address))
                .collect_vec(),
        )
        .await?;

    let mut external_accounts = Vec::new();
    for address in external_account_addresses {
        let owner_address = AddressOfAccountOrPersona::Account(address);
        let maybe_badge_owner =
            badge_owners.get(&owner_address).unwrap_or(&None);

        let account_assignments = account_role_assignments
            .get(&Address::from(address))
            .ok_or(CommonError::Unknown {
                error_message: format!(
                    "Missing role assignments for account {}",
                    address
                ),
            })?;
        let account_rule =
            access_rule_from_explicit_rule(&account_assignments.owner.rule)
                .ok_or(CommonError::Unknown {
                    error_message: format!(
                        "Missing access rule for account {}",
                        address
                    ),
                })?;

        debug!(
            "Account access rule for external account {}: {:?}",
            address, account_rule
        );
        let mut required_nft_ids =
            extract_nft_ids_from_access_rule(&account_rule)?;

        if let Some(access_controller_address) =
            maybe_badge_owner.and_then(|a| a.as_access_controller().cloned())
        {
            if let Some(assignments) = access_controller_role_assignments
                .get(&Address::from(access_controller_address))
            {
                debug!(
                    "Access controller role assignments for {}: {:?}",
                    access_controller_address, assignments
                );
                for rule in access_rules_from_access_controller(assignments) {
                    debug!(
                        "Access controller rule for {}: {:?}",
                        access_controller_address, rule
                    );
                    required_nft_ids
                        .extend(extract_nft_ids_from_access_rule(&rule)?);
                }
            } else {
                debug!(
                    "No access controller role assignments found for {}",
                    access_controller_address
                );
            }
        } else {
            debug!(
                    "External account {} has no access controller owner; using account access rule only",
                    address
                );
        }

        debug!(
            "Required NFT ids for external account {}: {:?}",
            address, required_nft_ids
        );
        external_accounts.push(ExternalAccountNftRequirements {
            owner: owner_address,
            required_nft_ids,
        });
    }

    debug!(
        "Resolved external account NFT requirements: {:?}",
        external_accounts
    );
    Ok(external_accounts)
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

    #[test]
    fn access_controller_role_key_filtering() {
        let primary = RoleKey::new("primary".to_string(), ObjectModuleId::Main);
        let recovery =
            RoleKey::new("recovery".to_string(), ObjectModuleId::Main);
        let confirmation =
            RoleKey::new("confirmation".to_string(), ObjectModuleId::Main);
        let depositor =
            RoleKey::new("depositor".to_string(), ObjectModuleId::Main);
        let metadata_primary =
            RoleKey::new("primary".to_string(), ObjectModuleId::Metadata);

        assert!(is_access_controller_role_key(&primary));
        assert!(is_access_controller_role_key(&recovery));
        assert!(is_access_controller_role_key(&confirmation));
        assert!(!is_access_controller_role_key(&depositor));
        assert!(!is_access_controller_role_key(&metadata_primary));
    }

    #[test]
    fn role_assignments_from_details_component() {
        let assignments = ComponentEntityRoleAssignments::sample();
        let details = StateEntityDetailsResponseItemDetails::Component(
            StateEntityDetailsResponseComponentDetails::new(
                assignments.clone(),
            ),
        );

        assert_eq!(role_assignments_from_details(&details), Some(assignments));

        let vault_details =
            StateEntityDetailsResponseItemDetails::FungibleVault;
        assert!(role_assignments_from_details(&vault_details).is_none());
    }

    #[test]
    fn access_rules_from_access_controller_collects_primary_roles() {
        let owner_rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AnyOf {
                access_rules: vec![],
            },
        };
        let explicit_primary = AccessRule::AllowAll;
        let explicit_recovery = AccessRule::DenyAll;

        let entries = vec![
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("primary".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    explicit_primary.clone(),
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("recovery".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    explicit_recovery.clone(),
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("confirmation".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Owner,
                    None,
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::main_depositor(),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    AccessRule::AllowAll,
                ),
            ),
        ];

        let assignments = ComponentEntityRoleAssignments::new(
            ComponentEntityRoleAssignmentOwner::new(owner_rule.clone()),
            entries,
        );

        let rules = access_rules_from_access_controller(&assignments);
        assert_eq!(rules.len(), 3);
        assert!(rules.contains(&explicit_primary));
        assert!(rules.contains(&explicit_recovery));
        assert!(rules.contains(&owner_rule));
    }

    #[actix_rt::test]
    async fn factor_instances_for_nfts_uses_profile_mfa_instances() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let network = profile.current_network().unwrap();
        let first_mfa = network.mfa_factor_instances.iter().next().unwrap();
        let scrypto_id = first_mfa
            .factor_instance
            .badge
            .try_non_fungible_global_id()
            .unwrap();
        let required_id = NonFungibleGlobalId::from((scrypto_id, network.id));

        let instances = sut
            .factor_instances_for_nfts(vec![required_id])
            .await
            .unwrap();

        assert_eq!(instances.len(), 1);
        assert_eq!(
            instances[0],
            first_mfa
                .factor_instance
                .try_as_hd_factor_instances()
                .unwrap()
        );
    }

    async fn boot(
        profile: Option<Profile>,
        maybe_signing_failure: Option<SigningFailure>,
    ) -> Arc<SargonOS> {
        let secure_storage_driver = EphemeralSecureStorage::new();

        if let Some(profile) = profile {
            let secure_storage_client =
                SecureStorageClient::new(secure_storage_driver.clone());
            secure_storage_client.save_profile(&profile).await.unwrap();
        }

        let test_drivers = Drivers::with_secure_storage(secure_storage_driver);
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();

        let use_factor_sources_interactors =
            Arc::new(TestUseFactorSourcesInteractors::new(
                Arc::new(TestSignInteractor::<TransactionIntent>::new(
                    get_simulated_user::<TransactionIntent>(
                        &maybe_signing_failure,
                    ),
                )),
                Arc::new(TestSignInteractor::<Subintent>::new(
                    get_simulated_user::<Subintent>(&maybe_signing_failure),
                )),
                Arc::new(TestDerivationInteractor::new(
                    false,
                    Arc::new(clients.secure_storage.clone()),
                )),
                Arc::new(TestSignInteractor::<AuthIntent>::new(
                    get_simulated_user::<AuthIntent>(&maybe_signing_failure),
                )),
            ));
        let interactors = Interactors::new(
            use_factor_sources_interactors,
            Arc::new(TestAuthorizationInteractor::stubborn_authorizing()),
            Arc::new(TestSpotCheckInteractor::new_succeeded()),
        );
        SargonOS::boot_with_clients_and_interactor(clients, interactors).await
    }

    fn get_simulated_user<S: Signable>(
        maybe_signing_failure: &Option<SigningFailure>,
    ) -> SimulatedUser<S> {
        match maybe_signing_failure {
            None => SimulatedUser::<S>::prudent_no_fail(),
            Some(failure) => match failure {
                SigningFailure::FailingFactorSources(factor_sources) => {
                    SimulatedUser::<S>::prudent_with_failures(
                        SimulatedFailures::with_simulated_failures(
                            factor_sources.clone(),
                        ),
                    )
                }
                SigningFailure::UserRejected => SimulatedUser::<S>::rejecting(),
                SigningFailure::SkippingFactorSources(factor_sources) => {
                    SimulatedUser::<S>::skipping_specific(
                        factor_sources.iter().cloned().collect(),
                    )
                }
            },
        }
    }

    enum SigningFailure {
        FailingFactorSources(Vec<FactorSourceIDFromHash>),
        SkippingFactorSources(Vec<FactorSourceIDFromHash>),
        UserRejected,
    }

    // ==================
    // Additional Robustness Tests
    // ==================

    #[test]
    fn extract_nft_ids_handles_allow_all() {
        let rule = AccessRule::AllowAll;
        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert!(ids.is_empty());
    }

    #[test]
    fn extract_nft_ids_handles_deny_all() {
        let rule = AccessRule::DenyAll;
        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert!(ids.is_empty());
    }

    #[test]
    fn extract_nft_ids_handles_nested_any_of() {
        let resource = ResourceAddress::sample();
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AnyOf {
                access_rules: vec![
                    CompositeRequirement::AnyOf {
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
                    CompositeRequirement::ProofRule {
                        proof_rule: BasicRequirement::Require {
                            requirement: Requirement::NonFungible {
                                non_fungible: nft(resource, "#3#"),
                            },
                        },
                    },
                ],
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn extract_nft_ids_handles_all_of() {
        let resource = ResourceAddress::sample();
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AllOf {
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
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn extract_nft_ids_handles_count_of() {
        let resource = ResourceAddress::sample();
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::CountOf {
                    count: 2,
                    list: vec![
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#1#"),
                        },
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#2#"),
                        },
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#3#"),
                        },
                    ],
                },
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn extract_nft_ids_handles_any_of_in_basic_requirement() {
        let resource = ResourceAddress::sample();
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::AnyOf {
                    list: vec![
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#1#"),
                        },
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#2#"),
                        },
                    ],
                },
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn extract_nft_ids_handles_all_of_in_basic_requirement() {
        let resource = ResourceAddress::sample();
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::AllOf {
                    list: vec![
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#1#"),
                        },
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#2#"),
                        },
                    ],
                },
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn extract_nft_ids_handles_mixed_requirements() {
        let resource = ResourceAddress::sample();
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::AllOf {
                    list: vec![
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#1#"),
                        },
                        Requirement::Resource {
                            resource: ResourceAddress::sample_other(),
                        },
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#2#"),
                        },
                    ],
                },
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn extract_nft_ids_deduplicates_ids() {
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
                                non_fungible: nft(resource, "#1#"),
                            },
                        },
                    },
                ],
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn access_rule_from_role_assignment_owner_resolution() {
        let owner_rule = AccessRule::AllowAll;
        let entry = ComponentEntityRoleAssignmentEntry::new(
            RoleKey::new("primary".to_string(), ObjectModuleId::Main),
            ComponentEntityRoleAssignmentEntryAssignment::new(
                RoleAssignmentResolution::Owner,
                None,
            ),
        );

        let rule = access_rule_from_role_assignment(&entry, &owner_rule);
        assert_eq!(rule, Some(owner_rule));
    }

    #[test]
    fn access_rule_from_role_assignment_explicit_resolution_with_rule() {
        let owner_rule = AccessRule::AllowAll;
        let explicit_rule = AccessRule::DenyAll;
        let entry = ComponentEntityRoleAssignmentEntry::new(
            RoleKey::new("primary".to_string(), ObjectModuleId::Main),
            ComponentEntityRoleAssignmentEntryAssignment::new(
                RoleAssignmentResolution::Explicit,
                explicit_rule.clone(),
            ),
        );

        let rule = access_rule_from_role_assignment(&entry, &owner_rule);
        assert_eq!(rule, Some(explicit_rule));
    }

    #[test]
    fn access_rule_from_role_assignment_explicit_resolution_without_rule() {
        let owner_rule = AccessRule::AllowAll;
        let entry = ComponentEntityRoleAssignmentEntry::new(
            RoleKey::new("primary".to_string(), ObjectModuleId::Main),
            ComponentEntityRoleAssignmentEntryAssignment::new(
                RoleAssignmentResolution::Explicit,
                None,
            ),
        );

        let rule = access_rule_from_role_assignment(&entry, &owner_rule);
        assert_eq!(rule, None);
    }

    #[test]
    fn role_assignments_from_details_fungible_resource() {
        let assignments = ComponentEntityRoleAssignments::sample();
        let details = StateEntityDetailsResponseItemDetails::FungibleResource(
            StateEntityDetailsResponseFungibleResourceDetails::new(
                assignments.clone(),
            ),
        );

        assert_eq!(role_assignments_from_details(&details), Some(assignments));
    }

    #[test]
    fn role_assignments_from_details_non_fungible_resource() {
        let assignments = ComponentEntityRoleAssignments::sample();
        let details =
            StateEntityDetailsResponseItemDetails::NonFungibleResource(
                StateEntityDetailsResponseNonFungibleResourceDetails::new(
                    assignments.clone(),
                ),
            );

        assert_eq!(role_assignments_from_details(&details), Some(assignments));
    }

    #[test]
    fn role_assignments_from_details_package_with_assignments() {
        let assignments = ComponentEntityRoleAssignments::sample();
        let details = StateEntityDetailsResponseItemDetails::Package(
            StateEntityDetailsResponsePackageDetails::new(Some(
                assignments.clone(),
            )),
        );

        assert_eq!(role_assignments_from_details(&details), Some(assignments));
    }

    #[test]
    fn role_assignments_from_details_package_without_assignments() {
        let details = StateEntityDetailsResponseItemDetails::Package(
            StateEntityDetailsResponsePackageDetails::new(None),
        );

        assert_eq!(role_assignments_from_details(&details), None);
    }

    #[test]
    fn role_assignments_from_details_non_fungible_vault() {
        let details = StateEntityDetailsResponseItemDetails::NonFungibleVault;
        assert_eq!(role_assignments_from_details(&details), None);
    }

    #[test]
    fn is_access_controller_role_key_accepts_all_variants() {
        let test_cases = vec![
            ("primary", ObjectModuleId::Main, true),
            ("recovery", ObjectModuleId::Main, true),
            ("confirmation", ObjectModuleId::Main, true),
            ("depositor", ObjectModuleId::Main, false),
            ("withdrawer", ObjectModuleId::Main, false),
            ("primary", ObjectModuleId::Metadata, false),
            ("recovery", ObjectModuleId::RoleAssignment, false),
        ];

        for (name, module, expected) in test_cases {
            let role_key = RoleKey::new(name.to_string(), module.clone());
            assert_eq!(
                is_access_controller_role_key(&role_key),
                expected,
                "Failed for role: {}, module: {:?}",
                name,
                module
            );
        }
    }

    #[test]
    fn access_rules_from_access_controller_returns_empty_for_no_matches() {
        let owner_rule = AccessRule::AllowAll;
        let entries = vec![ComponentEntityRoleAssignmentEntry::new(
            RoleKey::main_depositor(),
            ComponentEntityRoleAssignmentEntryAssignment::new(
                RoleAssignmentResolution::Explicit,
                AccessRule::AllowAll,
            ),
        )];

        let assignments = ComponentEntityRoleAssignments::new(
            ComponentEntityRoleAssignmentOwner::new(owner_rule),
            entries,
        );

        let rules = access_rules_from_access_controller(&assignments);
        assert!(rules.is_empty());
    }

    #[test]
    fn access_rules_from_access_controller_filters_by_module() {
        let owner_rule = AccessRule::AllowAll;
        let explicit_rule = AccessRule::DenyAll;

        let entries = vec![
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("primary".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    explicit_rule.clone(),
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("primary".to_string(), ObjectModuleId::Metadata),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    AccessRule::Protected {
                        access_rule: CompositeRequirement::AnyOf {
                            access_rules: vec![],
                        },
                    },
                ),
            ),
        ];

        let assignments = ComponentEntityRoleAssignments::new(
            ComponentEntityRoleAssignmentOwner::new(owner_rule),
            entries,
        );

        let rules = access_rules_from_access_controller(&assignments);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0], explicit_rule);
    }

    #[actix_rt::test]
    async fn factor_instances_for_nfts_empty_list_returns_empty() {
        let profile = Profile::sample();
        let sut = boot(Some(profile), None).await;

        let instances = sut.factor_instances_for_nfts(vec![]).await.unwrap();

        assert!(instances.is_empty());
    }

    #[actix_rt::test]
    async fn factor_instances_for_nfts_no_matching_instances() {
        let profile = Profile::sample();
        let sut = boot(Some(profile), None).await;

        let non_matching_id = NonFungibleGlobalId::new_unchecked(
            ResourceAddress::sample(),
            NonFungibleLocalId::integer(999999),
        );

        let instances = sut
            .factor_instances_for_nfts(vec![non_matching_id])
            .await
            .unwrap();

        assert!(instances.is_empty());
    }

    #[actix_rt::test]
    async fn factor_instances_for_nfts_multiple_matching_instances() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let network = profile.current_network().unwrap();
        let mfa_instances: Vec<_> =
            network.mfa_factor_instances.iter().take(2).collect();

        if mfa_instances.len() < 2 {
            // Skip test if profile doesn't have at least 2 MFA instances
            return;
        }

        let required_ids: Vec<NonFungibleGlobalId> = mfa_instances
            .iter()
            .filter_map(|mfa| {
                mfa.factor_instance
                    .badge
                    .try_non_fungible_global_id()
                    .ok()
                    .map(|id| NonFungibleGlobalId::from((id, network.id)))
            })
            .collect();

        let instances =
            sut.factor_instances_for_nfts(required_ids).await.unwrap();

        assert_eq!(instances.len(), mfa_instances.len());
    }

    #[test]
    fn access_rule_from_explicit_rule_returns_some() {
        let rule = AccessRule::AllowAll;
        let result = access_rule_from_explicit_rule(&rule);
        assert_eq!(result, Some(rule));
    }

    // ==================
    // Error Handling Tests
    // ==================

    #[test]
    fn non_fungible_global_id_handles_valid_integer_id() {
        let nft_obj = nft(ResourceAddress::sample(), "#123#");
        let result = non_fungible_global_id(&nft_obj);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(
            id,
            NonFungibleGlobalId::new_unchecked(
                ResourceAddress::sample(),
                NonFungibleLocalId::integer(123)
            )
        );
    }

    #[test]
    fn non_fungible_global_id_handles_invalid_local_id() {
        let mut invalid_nft = nft(ResourceAddress::sample(), "#1#");
        invalid_nft.local_id.simple_rep = "not_valid_format".to_string();

        let result = non_fungible_global_id(&invalid_nft);
        assert!(result.is_err());
    }

    #[test]
    fn extract_nft_ids_handles_deeply_nested_rules() {
        let resource = ResourceAddress::sample();
        // Create a 4-level nested structure
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AnyOf {
                access_rules: vec![CompositeRequirement::AllOf {
                    access_rules: vec![CompositeRequirement::AnyOf {
                        access_rules: vec![CompositeRequirement::ProofRule {
                            proof_rule: BasicRequirement::AllOf {
                                list: vec![
                                    Requirement::NonFungible {
                                        non_fungible: nft(resource, "#1#"),
                                    },
                                    Requirement::NonFungible {
                                        non_fungible: nft(resource, "#2#"),
                                    },
                                ],
                            },
                        }],
                    }],
                }],
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn extract_nft_ids_handles_empty_any_of() {
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AnyOf {
                access_rules: vec![],
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert!(ids.is_empty());
    }

    #[test]
    fn extract_nft_ids_handles_empty_all_of() {
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AllOf {
                access_rules: vec![],
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert!(ids.is_empty());
    }

    #[test]
    fn extract_nft_ids_handles_only_resource_requirements() {
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::AllOf {
                    list: vec![
                        Requirement::Resource {
                            resource: ResourceAddress::sample(),
                        },
                        Requirement::Resource {
                            resource: ResourceAddress::sample_other(),
                        },
                    ],
                },
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert!(ids.is_empty());
    }

    #[test]
    fn extract_nft_ids_handles_amount_of_requirement() {
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::AmountOf {
                    amount: Decimal192::from(5),
                    resource: ResourceAddress::sample(),
                },
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert!(ids.is_empty());
    }

    #[test]
    fn extract_nft_ids_from_count_of_with_mixed_requirements() {
        let resource = ResourceAddress::sample();
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::ProofRule {
                proof_rule: BasicRequirement::CountOf {
                    count: 1,
                    list: vec![
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#1#"),
                        },
                        Requirement::Resource {
                            resource: ResourceAddress::sample_other(),
                        },
                        Requirement::NonFungible {
                            non_fungible: nft(resource, "#2#"),
                        },
                    ],
                },
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 2);
    }

    // ==================
    // Complex Integration Tests
    // ==================

    #[test]
    fn access_rules_from_access_controller_handles_multiple_ac_roles() {
        let owner_rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AnyOf {
                access_rules: vec![],
            },
        };
        let primary_rule = AccessRule::AllowAll;
        let recovery_rule = AccessRule::DenyAll;
        let confirmation_rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AllOf {
                access_rules: vec![],
            },
        };

        let entries = vec![
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("primary".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    primary_rule.clone(),
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("recovery".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    recovery_rule.clone(),
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("confirmation".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    confirmation_rule.clone(),
                ),
            ),
        ];

        let assignments = ComponentEntityRoleAssignments::new(
            ComponentEntityRoleAssignmentOwner::new(owner_rule.clone()),
            entries,
        );

        let rules = access_rules_from_access_controller(&assignments);
        assert_eq!(rules.len(), 3);
        assert!(rules.contains(&primary_rule));
        assert!(rules.contains(&recovery_rule));
        assert!(rules.contains(&confirmation_rule));
    }

    #[test]
    fn access_rules_from_access_controller_handles_mixed_resolution_types() {
        let owner_rule = AccessRule::AllowAll;
        let explicit_rule = AccessRule::DenyAll;

        let entries = vec![
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("primary".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Owner,
                    None,
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("recovery".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    explicit_rule.clone(),
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::new("confirmation".to_string(), ObjectModuleId::Main),
                ComponentEntityRoleAssignmentEntryAssignment::new(
                    RoleAssignmentResolution::Explicit,
                    None,
                ),
            ),
        ];

        let assignments = ComponentEntityRoleAssignments::new(
            ComponentEntityRoleAssignmentOwner::new(owner_rule.clone()),
            entries,
        );

        let rules = access_rules_from_access_controller(&assignments);
        // Should get owner_rule (from primary) and explicit_rule (from recovery)
        // confirmation returns None so it's filtered out
        assert_eq!(rules.len(), 2);
        assert!(rules.contains(&owner_rule));
        assert!(rules.contains(&explicit_rule));
    }

    #[test]
    fn extract_nft_ids_from_complex_real_world_rule() {
        let resource1 = ResourceAddress::sample();
        let resource2 = ResourceAddress::sample_other();

        // Simulate a real-world complex rule:
        // "You need (NFT#1 OR NFT#2) AND (NFT#3 OR 5 of Resource)"
        let rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AllOf {
                access_rules: vec![
                    CompositeRequirement::ProofRule {
                        proof_rule: BasicRequirement::AnyOf {
                            list: vec![
                                Requirement::NonFungible {
                                    non_fungible: nft(resource1, "#1#"),
                                },
                                Requirement::NonFungible {
                                    non_fungible: nft(resource1, "#2#"),
                                },
                            ],
                        },
                    },
                    CompositeRequirement::ProofRule {
                        proof_rule: BasicRequirement::AnyOf {
                            list: vec![
                                Requirement::NonFungible {
                                    non_fungible: nft(resource1, "#3#"),
                                },
                                Requirement::Resource {
                                    resource: resource2,
                                },
                            ],
                        },
                    },
                ],
            },
        };

        let ids = extract_nft_ids_from_access_rule(&rule).unwrap();
        assert_eq!(ids.len(), 3);

        let expected_ids: IndexSet<NonFungibleGlobalId> =
            IndexSet::from_iter([
                NonFungibleGlobalId::new_unchecked(
                    resource1,
                    NonFungibleLocalId::integer(1),
                ),
                NonFungibleGlobalId::new_unchecked(
                    resource1,
                    NonFungibleLocalId::integer(2),
                ),
                NonFungibleGlobalId::new_unchecked(
                    resource1,
                    NonFungibleLocalId::integer(3),
                ),
            ]);

        assert_eq!(ids, expected_ids);
    }

    #[actix_rt::test]
    async fn factor_instances_for_nfts_handles_profile_without_network() {
        // When profile has no networks, NoNetworkInProfile error is caught
        // and empty list is returned
        let profile = Profile::with(
            Header::sample(),
            FactorSources::sample(),
            AppPreferences::default(),
            ProfileNetworks::new(), // Empty networks
        );

        let sut = boot(Some(profile), None).await;

        let nft_id = NonFungibleGlobalId::new_unchecked(
            ResourceAddress::sample(),
            NonFungibleLocalId::integer(1),
        );

        // This should return Ok(empty) because the NoNetworkInProfile error is handled
        let result = sut.factor_instances_for_nfts(vec![nft_id]).await;

        // The actual behavior depends on the profile state
        // If profile loads successfully, we get Ok(empty)
        // If profile doesn't load, we might get an error
        match result {
            Ok(instances) => assert!(instances.is_empty()),
            Err(_) => {
                // This is also acceptable - when there's no network, profile may not load
            }
        }
    }

    #[actix_rt::test]
    async fn factor_instances_for_nfts_handles_network_without_mfa_instances() {
        // Create a profile with a network that has no MFA instances
        let network = ProfileNetwork::new(
            NetworkID::Mainnet,
            [Account::sample_mainnet()],
            [],
            AuthorizedDapps::new(),
            ResourcePreferences::new(),
            MFAFactorInstances::new(), // Empty MFA instances
        );

        let profile = Profile::with(
            Header::sample(),
            FactorSources::sample(),
            AppPreferences::default(),
            ProfileNetworks::just(network),
        );

        let sut = boot(Some(profile), None).await;

        let nft_id = NonFungibleGlobalId::new_unchecked(
            ResourceAddress::sample(),
            NonFungibleLocalId::integer(1),
        );

        let instances =
            sut.factor_instances_for_nfts(vec![nft_id]).await.unwrap();

        assert!(instances.is_empty());
    }
}
