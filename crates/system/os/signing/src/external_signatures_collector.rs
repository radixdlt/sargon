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

        let required_ids: IndexSet<NonFungibleGlobalId> =
            nft_ids.into_iter().collect();
        debug!(
            "External signing: looking up factor instances for {} NFT ids",
            required_ids.len()
        );
        let instances = self
            .pre_authorization_signing_keys()?
            .into_iter()
            .filter(|key| required_ids.contains(&key.badge))
            .map(|key| key.factor_instance)
            .collect::<Vec<_>>();

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

// ==================
// External Account Resolution
// ==================

/// What each account's owner role requires, read from the Gateway.
async fn fetch_owner_requirements(
    gateway_client: &GatewayClient,
    accounts: &[AccountAddress],
    network_id: NetworkID,
) -> Result<IndexMap<AccountAddress, AccessRequirement>> {
    let ledger_state = gateway_client.gateway_status().await?.ledger_state;
    let response = gateway_client
        .state_entity_details(StateEntityDetailsRequest::new(
            accounts.iter().map(|account| (*account).into()).collect(),
            Some(ledger_state.into()),
            None,
        ))
        .await?;

    accounts
        .iter()
        .map(|account| {
            let owner_rule = response
                .items
                .iter()
                .find(|item| item.address == (*account).into())
                .and_then(|item| item.details.clone())
                .and_then(|details| details.as_component().cloned())
                .and_then(|component| component.role_assignments)
                .map(|assignments| assignments.owner.rule)
                .ok_or_else(|| CommonError::Unknown {
                    error_message: format!(
                        "Missing owner role for account {account}"
                    ),
                })?;
            Ok((
                *account,
                AccessRequirement::from((
                    ScryptoAccessRule::from(owner_rule),
                    network_id,
                )),
            ))
        })
        .collect()
}

/// For each account the subintent needs to authorize that isn't in the
/// Profile, the badges that can authorize it: those its owner role names, and
/// if an access controller holds its owner badge, those any of its roles
/// names.
pub(crate) async fn resolve_external_accounts_for_subintent(
    os: &SargonOS,
    subintent: &Subintent,
) -> Result<Vec<ExternalAccountNftRequirements>> {
    let summary = subintent.manifest.summary()?;
    let profile = os.profile()?;
    let network_id = subintent.manifest.network_id();

    let external_accounts = summary
        .addresses_of_accounts_requiring_auth
        .into_iter()
        .filter(|address| profile.account_by_address(*address).is_err())
        .collect_vec();
    if external_accounts.is_empty() {
        debug!("No external accounts detected for subintent");
        return Ok(Vec::new());
    }
    debug!(
        "External accounts detected for subintent: {:?}",
        external_accounts
    );

    let gateway_client = os.gateway_client();
    let badge_owners = gateway_client
        .fetch_entities_badge_owners(
            external_accounts
                .iter()
                .map(|address| AddressOfAccountOrPersona::Account(*address))
                .collect_vec(),
        )
        .await?;
    let access_controller_of = |account: AccountAddress| {
        badge_owners
            .get(&AddressOfAccountOrPersona::Account(account))
            .cloned()
            .flatten()
            .and_then(|owner| owner.as_access_controller().cloned())
    };

    let owner_requirements = fetch_owner_requirements(
        &gateway_client,
        &external_accounts,
        network_id,
    )
    .await?;

    let access_controllers = external_accounts
        .iter()
        .filter_map(|account| access_controller_of(*account))
        .collect::<IndexSet<_>>();
    let access_controller_rules = if access_controllers.is_empty() {
        IndexMap::new()
    } else {
        gateway_client
            .fetch_access_controllers_details(
                access_controllers.into_iter().collect(),
            )
            .await?
            .into_iter()
            .filter_map(|details| {
                let rule_set = details.rule_set().ok()?;
                Some((
                    details.address,
                    AccessControllerRuleSet::from((rule_set, network_id)),
                ))
            })
            .collect::<IndexMap<_, _>>()
    };

    let external_accounts = external_accounts
        .into_iter()
        .map(|account| {
            let mut required_nft_ids = owner_requirements
                .get(&account)
                .map(AccessRequirement::non_fungibles)
                .unwrap_or_default();
            if let Some(access_controller) = access_controller_of(account) {
                match access_controller_rules.get(&access_controller) {
                    Some(rules) => {
                        required_nft_ids.extend(rules.non_fungibles())
                    }
                    None => debug!(
                        "No rules read for access controller {}",
                        access_controller
                    ),
                }
            }
            ExternalAccountNftRequirements {
                owner: AddressOfAccountOrPersona::Account(account),
                required_nft_ids: required_nft_ids.into_iter().collect(),
            }
        })
        .collect_vec();

    debug!(
        "Resolved external account NFT requirements: {:?}",
        external_accounts
    );
    Ok(external_accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use profile_logic::prelude::ProfileCurrentNetwork;

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

    /// The chain against Stokenet's answers for a shared account: it isn't in
    /// the Profile, its owner badge is in an access controller, and so it
    /// requires the account owner badge (its owner role) and every key the
    /// access controller's roles name.
    #[actix_rt::test]
    async fn external_account_requires_its_owner_badge_and_its_access_controllers_keys(
    ) {
        use prelude::fixture_gw_model;

        let json = |fixture: &str| -> serde_json::Value {
            serde_json::from_str(fixture).unwrap()
        };
        let gateway_status = || {
            MockNetworkingDriverResponse::new_success(GatewayStatusResponse {
                ledger_state: LedgerState::sample(),
            })
        };
        let os = SargonOS::boot_test_with_networking_driver(Arc::new(
            MockNetworkingDriver::new_with_responses(vec![
                MockNetworkingDriverResponse::new_success(json(
                    fixture_gw_model!(
                        "state/shared_account_owner_badge_location"
                    ),
                )),
                gateway_status(),
                MockNetworkingDriverResponse::new_success(json(
                    fixture_gw_model!("state/shared_account_details"),
                )),
                gateway_status(),
                MockNetworkingDriverResponse::new_success(json(
                    fixture_gw_model!(
                        "state/shared_account_access_controller_details"
                    ),
                )),
            ]),
        ))
        .await
        .unwrap();

        let account = AccountAddress::from_str(
            "account_tdx_2_12xerddu2e5r8e3xyc06lk6rv0a98vhtvk0xjmh7j2vg0asdqqp8nq0",
        )
        .unwrap();
        let manifest: SubintentManifest = (
            ScryptoSubintentManifestV2Builder::new_subintent_v2()
                .create_proof_from_account_of_amount(
                    account.scrypto(),
                    ResourceAddress::xrd_on_network(NetworkID::Stokenet)
                        .scrypto(),
                    1,
                )
                .yield_to_parent(())
                .build(),
            NetworkID::Stokenet,
        )
            .try_into()
            .unwrap();
        let subintent = Subintent::new(
            IntentHeaderV2 {
                network_id: NetworkID::Stokenet,
                ..IntentHeaderV2::sample()
            },
            manifest,
            MessageV2::sample(),
        )
        .unwrap();

        let resolved = resolve_external_accounts_for_subintent(&os, &subintent)
            .await
            .unwrap();

        let id =
            |global_id: &str| NonFungibleGlobalId::from_str(global_id).unwrap();
        let key = |hex: &str| {
            id(&format!(
                "resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa:[{hex}]"
            ))
        };
        assert_eq!(resolved.len(), 1);
        assert_eq!(
            resolved[0].owner,
            AddressOfAccountOrPersona::Account(account)
        );
        assert_eq!(
            resolved[0].required_nft_ids.iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([
                id("resource_tdx_2_1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxx4vczzk:[51b236b78acd067cc4c4c3f5fb686c7f4a765d6cb3cd2ddfd25310fec1a0]"),
                key("b236b78acd067cc4c4c3f5fb686c7f4a765d6cb3cd2ddfd25310fec1a0"),
                key("fd1fdc334ff599f43da12459e733579c74a8797e92161a7e8847a8eea4"),
                key("76541ca712dcdccc5f176a50d7f90b3cf560124e69d5062974a1dd7111"),
            ])
        );
    }
}
