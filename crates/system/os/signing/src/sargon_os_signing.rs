use crate::prelude::*;
use profile_logic::prelude::ProfileCurrentNetwork;

#[async_trait::async_trait]
pub trait OsSigning {
    async fn sign<S: Signable>(
        &self,
        signable: S,
        sign_interactor: Arc<dyn SignInteractor<S>>,
        purpose: SigningPurpose,
    ) -> Result<S::Signed>;

    async fn sign_auth(
        &self,
        auth_intent: AuthIntent,
    ) -> Result<SignedAuthIntent>;

    async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        execution_summary: ExecutionSummary,
    ) -> Result<SignedIntent>;

    async fn sign_subintent(
        &self,
        subintent: Subintent,
        role_kind: RoleKind,
    ) -> Result<SignedSubintent>;
}

// ==================
// Sign Signables
// ==================
#[async_trait::async_trait]
impl OsSigning for SargonOS {
    async fn sign_auth(
        &self,
        auth_intent: AuthIntent,
    ) -> Result<SignedAuthIntent> {
        self.sign(
            auth_intent.clone(),
            self.sign_auth_interactor(),
            SigningPurpose::ROLA,
        )
        .await
    }

    async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        execution_summary: ExecutionSummary,
    ) -> Result<SignedIntent> {
        match execution_summary.detailed_classification {
            Some(DetailedManifestClass::AccessControllerRecovery {
                ac_addresses,
            }) => {
                return sign_access_controller_recovery_transaction(
                    self,
                    transaction_intent,
                    ac_addresses[0],
                )
                .await;
            }
            Some(
                DetailedManifestClass::AccessControllerStopTimedRecovery {
                    ac_addresses,
                },
            ) => {
                return sign_access_controller_stop_timed_recovery_transaction(
                    self,
                    transaction_intent,
                    ac_addresses[0],
                )
                .await;
            }
            Some(DetailedManifestClass::SecurifyEntity { entities }) => {
                let entity_address = entities.first().unwrap();
                return sign_entity_securify(
                    self,
                    transaction_intent,
                    *entity_address,
                )
                .await;
            }
            _ => {
                self.sign(
                    transaction_intent.clone(),
                    self.sign_transactions_interactor(),
                    SigningPurpose::sign_transaction(RoleKind::Primary),
                )
                .await
            }
        }
    }

    async fn sign_subintent(
        &self,
        subintent: Subintent,
        role_kind: RoleKind,
    ) -> Result<SignedSubintent> {
        info!(
            "External signing: begin for subintent {:?} (role: {:?})",
            subintent.get_id(),
            role_kind
        );
        let profile = &self.profile()?;

        let collector = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            vec![subintent.clone()],
            self.sign_subintents_interactor(),
            profile,
            SigningPurpose::sign_transaction(role_kind),
        )?;

        let outcome = collector.collect_signatures().await?;
        if !outcome.successful() {
            return Err(
                CommonError::SigningFailedTooManyFactorSourcesNeglected,
            );
        }

        let mut signatures =
            outcome.signatures_of_successful_transactions();

        let external_accounts =
            resolve_external_accounts_for_subintent(self, &subintent).await?;
        info!(
            "External signing: external accounts resolved: {}",
            external_accounts.len()
        );
        let external_signatures = collect_external_signatures(
            subintent.clone(),
            external_accounts,
            self,
            self.sign_subintents_interactor(),
        )
        .await?;
        info!(
            "External signing: collected {} external signatures",
            external_signatures.len()
        );
        signatures.extend(external_signatures);

        subintent.signed(signatures)
    }

    async fn sign<S: Signable>(
        &self,
        signable: S,
        sign_interactor: Arc<dyn SignInteractor<S>>,
        purpose: SigningPurpose,
    ) -> Result<S::Signed> {
        let profile = &self.profile()?;

        let collector = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            vec![signable.clone()],
            sign_interactor,
            profile,
            purpose,
        )?;

        let outcome = collector.collect_signatures().await?;

        if outcome.successful() {
            let signatures = outcome.signatures_of_successful_transactions();
            signable.signed(signatures)
        } else {
            Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
        }
    }
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
                instances.push(
                    mfafi
                        .factor_instance
                        .try_as_hd_factor_instances()?,
                );
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

async fn resolve_external_accounts_for_subintent(
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
                    .map(|address| {
                        AddressOfAccountOrPersona::Account(*address)
                    })
                    .collect_vec(),
            )
            .await?;

        debug!(
            "Resolved badge owners for external accounts: {:?}",
            badge_owners
        );
        let account_role_assignments =
            fetch_role_assignments_for_addresses(
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
            let owner_address =
                AddressOfAccountOrPersona::Account(address);
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
                if let Some(assignments) =
                    access_controller_role_assignments.get(&Address::from(
                        access_controller_address,
                    ))
                {
                    debug!(
                        "Access controller role assignments for {}: {:?}",
                        access_controller_address, assignments
                    );
                    for rule in
                        access_rules_from_access_controller(assignments)
                    {
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

async fn fetch_role_assignments_for_addresses(
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
            let item = response.items.iter().find(|item| {
                item.address == address
            }).ok_or(CommonError::GWMissingResponseItem {
                item: "StateEntityDetailsResponseItem".to_owned(),
            })?;

            let details = item.details.as_ref().ok_or(CommonError::Unknown {
                error_message: format!(
                    "Missing entity details for address {}",
                    address
                ),
            })?;
            let role_assignments =
                role_assignments_from_details(details).ok_or(
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

fn role_assignments_from_details(
    details: &StateEntityDetailsResponseItemDetails,
) -> Option<ComponentEntityRoleAssignments> {
    match details {
        StateEntityDetailsResponseItemDetails::FungibleResource(details) => {
            Some(details.role_assignments.clone())
        }
        StateEntityDetailsResponseItemDetails::NonFungibleResource(
            details,
        ) => Some(details.role_assignments.clone()),
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

fn access_rule_from_explicit_rule(rule: &ExplicitRule) -> Option<AccessRule> {
    Some(rule.clone())
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

fn is_access_controller_role_key(role_key: &RoleKey) -> bool {
    role_key.module == ObjectModuleId::Main
        && matches!(
            role_key.name.as_str(),
            "primary" | "recovery" | "confirmation"
        )
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    fn default_sign_transaction_args() -> (ExecutionSummary, LockFeeData) {
        (
            ExecutionSummary::sample(),
            LockFeeData::new_with_unsecurified_fee_payer(
                AccountAddress::sample(),
                Decimal192::one(),
            ),
        )
    }

    #[test]
    fn access_controller_role_key_filtering() {
        let primary = RoleKey::new("primary".to_string(), ObjectModuleId::Main);
        let recovery =
            RoleKey::new("recovery".to_string(), ObjectModuleId::Main);
        let confirmation = RoleKey::new(
            "confirmation".to_string(),
            ObjectModuleId::Main,
        );
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
    fn access_rules_from_access_controller_collects_primary_roles() {
        let owner_rule = AccessRule::Protected {
            access_rule: CompositeRequirement::AnyOf { access_rules: vec![] },
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

    #[test]
    fn role_assignments_from_details_component() {
        let assignments = ComponentEntityRoleAssignments::sample();
        let details = StateEntityDetailsResponseItemDetails::Component(
            StateEntityDetailsResponseComponentDetails::new(assignments.clone()),
        );

        assert_eq!(
            role_assignments_from_details(&details),
            Some(assignments)
        );

        let vault_details = StateEntityDetailsResponseItemDetails::FungibleVault;
        assert!(role_assignments_from_details(&vault_details).is_none());
    }

    #[actix_rt::test]
    async fn factor_instances_for_nfts_uses_profile_mfa_instances() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let network = profile.current_network().unwrap();
        let first_mfa = network.mfa_factor_instances.iter().next().unwrap();
        let required_id = first_mfa
            .factor_instance
            .badge
            .try_non_fungible_global_id()
            .unwrap();

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

    #[actix_rt::test]
    async fn test_sign_auth_success() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;
        let all_accounts = profile.accounts_on_current_network().unwrap();
        let account = all_accounts.first().unwrap();
        let nonce = DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::new(
            WalletInteractionVersion::current(),
            NetworkID::Mainnet,
            "https://example.com",
            DappDefinitionAddress::sample(),
        );
        let auth_intent = AuthIntent::new_from_request(
            nonce,
            metadata,
            [AddressOfAccountOrPersona::Account(account.address)],
        )
        .unwrap();

        let signed = sut.sign_auth(auth_intent.clone()).await.unwrap();

        let signature_with_public_key = signed
            .intent_signatures_per_owner
            .values()
            .collect_vec()
            .first()
            .unwrap()
            .0;

        assert!(signature_with_public_key
            .is_valid_for_hash(&auth_intent.auth_intent_hash().hash()))
    }

    #[actix_rt::test]
    async fn test_sign_auth_failure() {
        let profile = Profile::sample();

        let sut =
            boot(Some(profile.clone()), Some(SigningFailure::UserRejected))
                .await;

        let all_accounts = profile.accounts_on_current_network().unwrap();
        let account = all_accounts.first().unwrap();
        let nonce = DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::new(
            WalletInteractionVersion::current(),
            NetworkID::Mainnet,
            "https://example.com",
            DappDefinitionAddress::sample(),
        );

        let auth_intent = AuthIntent::new_from_request(
            nonce,
            metadata,
            vec![AddressOfAccountOrPersona::Account(account.address)],
        )
        .unwrap();

        let result = sut.sign_auth(auth_intent).await;

        assert_eq!(result, Err(CommonError::HostInteractionAborted))
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_success() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let (signable, entities) = get_signable_with_entities::<
            TransactionIntent,
        >(&sut.profile().unwrap());

        let signed = sut
            .sign_transaction(signable.clone(), ExecutionSummary::sample())
            .await
            .unwrap();

        assert_eq!(signable, signed.intent);
        assert_eq!(entities.len(), signed.intent_signatures.signatures.len());
    }

    #[actix_rt::test]
    async fn test_sign_subintent_success() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let (signable, entities) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let signed = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await
            .unwrap();

        assert_eq!(signable, signed.subintent);
        assert_eq!(
            entities.len(),
            signed.subintent_signatures.signatures.len()
        );
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_only_with_irrelevant_entity() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let irrelevant_account = Account::sample_mainnet_third();
        let transaction = TransactionIntent::sample_entities_requiring_auth(
            vec![&irrelevant_account],
            vec![],
        );

        let outcome = sut
            .sign_transaction(transaction, ExecutionSummary::sample())
            .await
            .unwrap();

        assert_eq!(outcome.intent_signatures.signatures.len(), 0);
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_containing_irrelevant_entity() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let irrelevant_account = Account::sample_mainnet_third();
        let relevant_account = Account::sample_mainnet();
        let transaction = TransactionIntent::sample_entities_requiring_auth(
            vec![&irrelevant_account, &relevant_account],
            vec![],
        );

        let outcome = sut
            .sign_transaction(transaction, ExecutionSummary::sample())
            .await
            .unwrap();

        assert_eq!(outcome.intent_signatures.signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_rejected_due_to_all_factors_neglected(
    ) {
        let profile = Profile::sample();
        let sut = boot(
            Some(profile.clone()),
            Some(SigningFailure::FailingFactorSources(vec![
                profile.device_factor_sources().first().unwrap().id,
            ])),
        )
        .await;

        let (signable, _) = get_signable_with_entities::<TransactionIntent>(
            &sut.profile().unwrap(),
        );

        let outcome = sut
            .sign_transaction(signable.clone(), ExecutionSummary::sample())
            .await;

        assert_eq!(
            outcome,
            Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
        );
    }

    #[actix_rt::test]
    async fn test_sign_transaction_prudent_user_skips_factor() {
        let device = FactorSource::sample_at(0);
        let account_device = Account::sample_unsecurified_mainnet(
            "Device",
            HierarchicalDeterministicFactorInstance::new_for_entity(
                device.clone().as_device().unwrap().id,
                CAP26EntityKind::Account,
                Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
            ),
        );
        let ledger1 = FactorSource::sample_at(1);
        let account_ledger1 = Account::sample_unsecurified_mainnet(
            "Ledger1",
            HierarchicalDeterministicFactorInstance::new_for_entity(
                ledger1.clone().as_ledger().unwrap().id,
                CAP26EntityKind::Account,
                Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
            ),
        );
        let ledger2 = FactorSource::sample_at(2);
        let account_ledger2 = Account::sample_unsecurified_mainnet(
            "Ledger2",
            HierarchicalDeterministicFactorInstance::new_for_entity(
                ledger2.clone().as_ledger().unwrap().id,
                CAP26EntityKind::Account,
                Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
            ),
        );

        let profile = Profile::with(
            Header::sample(),
            FactorSources::from_iter([
                device.clone(),
                ledger1.clone(),
                ledger2.clone(),
            ]),
            AppPreferences::default(),
            ProfileNetworks::just(ProfileNetwork::new(
                NetworkID::Mainnet,
                [
                    account_device.clone(),
                    account_ledger1.clone(),
                    account_ledger2.clone(),
                ],
                [],
                AuthorizedDapps::new(),
                ResourcePreferences::new(),
                MFAFactorInstances::new(),
            )),
        );

        let sut = boot(
            Some(profile.clone()),
            Some(SigningFailure::SkippingFactorSources(vec![
                ledger1.as_ledger().unwrap().id,
            ])),
        )
        .await;

        let signable =
            TransactionIntent::sample_entity_addresses_requiring_auth(
                [
                    account_device.address,
                    account_ledger1.address,
                    account_ledger2.address,
                ],
                [],
            );

        let outcome = sut
            .sign_transaction(signable.clone(), ExecutionSummary::sample())
            .await;

        assert_eq!(
            outcome,
            Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
        );
    }

    #[actix_rt::test]
    async fn test_sign_transaction_subintent_rejected_due_to_all_factors_neglected(
    ) {
        let profile = Profile::sample();
        let sut = boot(
            Some(profile.clone()),
            Some(SigningFailure::FailingFactorSources(vec![
                profile.device_factor_sources().first().unwrap().id,
            ])),
        )
        .await;

        let (signable, _) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let outcome = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await;

        assert_eq!(
            outcome,
            Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
        );
    }

    #[actix_rt::test]
    async fn test_sign_fail_due_to_profile() {
        let test_drivers = Drivers::test();
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        let sut =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

        let transaction =
            TransactionIntent::sample_entity_addresses_requiring_auth(
                vec![AccountAddress::sample_mainnet()],
                vec![],
            );

        let (execution_summary, lock_fee_data) =
            default_sign_transaction_args();

        let outcome = sut
            .sign_transaction(transaction, ExecutionSummary::sample())
            .await;

        assert_eq!(
            outcome,
            Err(CommonError::ProfileStateNotLoaded {
                current_state: ProfileState::None.to_string()
            })
        );
    }

    #[actix_rt::test]
    async fn test_sign_fail_due_to_user_rejecting() {
        let profile = Profile::sample();
        let sut =
            boot(Some(profile.clone()), Some(SigningFailure::UserRejected))
                .await;

        let (signable, _) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let outcome = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await;

        assert_eq!(outcome, Err(CommonError::HostInteractionAborted));
    }

    async fn boot(
        profile: Option<Profile>,
        maybe_signing_failure: Option<SigningFailure>,
    ) -> Arc<SUT> {
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
        SUT::boot_with_clients_and_interactor(clients, interactors).await
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

    fn get_signable_with_entities<
        S: Signable + ProvidesSamplesByBuildingManifest,
    >(
        profile: &Profile,
    ) -> (S, Vec<impl IsEntityAddress>) {
        let accounts_addresses_involved = profile
            .accounts_on_current_network()
            .unwrap()
            .iter()
            .map(|a| a.address)
            .collect_vec();

        (
            S::sample_entity_addresses_requiring_auth(
                accounts_addresses_involved.clone(),
                [],
            ),
            accounts_addresses_involved,
        )
    }

    enum SigningFailure {
        FailingFactorSources(Vec<FactorSourceIDFromHash>),
        SkippingFactorSources(Vec<FactorSourceIDFromHash>),
        UserRejected,
    }
}
