use entity_for_display::EntityForDisplay;

use crate::prelude::*;

use super::signing_manager_dependencies::SigningManagerDependencies;

/// Implementation of complex signing flow laid out in this
/// [whimsical diagram][flow].
///
/// [flow]: https://whimsical.com/wallet-sargon-signing-flow-QFvU2NAVXFiX1VgNBuvj5g
#[derive(derive_more::Deref)]
pub(crate) struct SigningManager {
    #[deref]
    pub(super) dependencies: Immutable<SigningManagerDependencies>,

    /// The internal state of the SigningManager
    ///
    /// We start with `None` in ctor, and set it to `Some` in `sign_intent_sets`.
    /// We wanna init this SigninManager only with dependencies and not until
    /// later when we call `sign_intent_sets` we can set the state.
    pub(super) state: RwLock<SigningManagerState>,
}

// ===============
// === PRIVATE ===
// ===============

// === Private Set ===
impl SigningManager {
    fn try_updating_state<R>(
        &self,
        f: impl FnOnce(&mut SigningManagerState) -> Result<R>,
    ) -> Result<R> {
        let mut state = self.state.write().map_err(|_| CommonError::Unknown)?; // TODO specific error variant
        f(&mut state)
    }

    fn updating_state<R>(
        &self,
        f: impl FnOnce(&mut SigningManagerState) -> R,
    ) -> Result<R> {
        self.try_updating_state(|state| Ok(f(state)))
    }
}

// === Private Update State ===
impl SigningManager {
    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Recovery
    fn handle_recovery_outcome(
        &self,
        recovery_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(recovery_outcome.role, RoleKind::Recovery);
        self.updating_state(|state| {
            state.update_with_exercise_role_outcome(recovery_outcome);
        })?;
        Ok(())
    }

    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Confirmation
    fn handle_confirmation_outcome(
        &self,
        confirmation_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(confirmation_outcome.role, RoleKind::Confirmation);
        self.updating_state(|state| {
            state.update_with_exercise_role_outcome(confirmation_outcome);
        })?;
        Ok(())
    }

    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Primary
    fn handle_primary_outcome(
        &self,
        primary_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(primary_outcome.role, RoleKind::Primary);
        self.updating_state(|state| {
            state.update_with_exercise_role_outcome(primary_outcome);
        })?;
        Ok(())
    }
}

struct CrossRoleSkipOutcomeAnalyzerForManager {
    pub(super) proto_profile: Arc<dyn IsProtoProfile>,
    pub(super) signing_manager_state_snapshot: SigningManagerState,
}

/// This is a very special struct which is used by SigningManager
/// and mixes data from possibly two different shields!
/// The `time_until_delayed_confirmation_is_callable` is from the
/// **committed** shield, if any, else None, but the `metadata` is
/// from the provisional
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PotentiallyMixedSecurityStructureMetadata {
    /// Read out from the provisional config!
    pub metadata: SecurityStructureMetadata,

    /// Read of from the committed shield, if any.
    /// None if the entity was unsecurified
    pub time_until_delayed_confirmation_is_callable: Option<TimePeriod>,
}

impl CrossRoleSkipOutcomeAnalyzerForManager {
    fn new(
        proto_profile: Arc<dyn IsProtoProfile>,
        signing_manager_state_snapshot: SigningManagerState,
    ) -> Arc<Self> {
        Arc::new(Self {
            proto_profile,
            signing_manager_state_snapshot,
        })
    }

    /// N.B. We read the metadata of the provisional shield, and the
    /// time_until_delayed_confirmation_is_callable from the committed shield
    fn for_display_by_address(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<(
        EntityForDisplay,
        Option<PotentiallyMixedSecurityStructureMetadata>,
    )> {
        let entity = self.proto_profile.entity_by_address(entity_address)?;

        let id_of_committed_shield: Option<SecurityStructureID> = {
            match entity.security_state() {
                EntitySecurityState::Securified { value } => {
                    Some(value.security_structure.id())
                }
                EntitySecurityState::Unsecured { .. } => None,
            }
        };

        // Provisional is used to show the name of the shield
        // we are applying.
        let id_of_provisional = {
            let provisional_security_config = entity.get_provisional();

            let security_structure_of_factor_instances = provisional_security_config
                .map(|config| config
                    .as_factor_instances_derived()
                    .cloned()
                    .ok_or(CommonError::ProvisionalConfigInWrongStateExpectedInstancesDerived)
                )
                .transpose()?;

            security_structure_of_factor_instances.map(|s| s.id())
        };

        // We need to know how long time use need to wait for delayed completion
        // if they cannot quick confirm, that is the value on the committed shield,
        // not the provisional (since it is not in use yet).
        let potentially_mixed_security_structure_metadata: Option<_> =
            || -> Result<Option<PotentiallyMixedSecurityStructureMetadata>> {
                let Some(id_of_provisional) = id_of_provisional else {
                    // Hmm what if `id_of_committed_shield` is None? we dont care about
                    // that?
                    return Ok(None);
                };
                let provisional = self
                    .proto_profile
                    .shield_metadata_by_id(id_of_provisional)?;

                let committed = id_of_committed_shield
                    .map(|id| self.proto_profile.shield_metadata_by_id(id))
                    .transpose()?;

                Ok(Some(PotentiallyMixedSecurityStructureMetadata {
                    metadata: provisional.metadata,
                    time_until_delayed_confirmation_is_callable: committed
                        .map(|c| c.time_until_delayed_confirmation_is_callable),
                }))
            }()?;

        let entity_for_display = EntityForDisplay::from(entity);

        Ok((
            entity_for_display,
            potentially_mixed_security_structure_metadata,
        ))
    }

    fn no_cross_role(
        &self,
        signable_id: TransactionIntentHash,
        skipped_factor_source_ids: IndexSet<FactorSourceIDFromHash>,
        petitions: Vec<PetitionForEntity<TransactionIntentHash>>,
    ) -> Result<Option<InvalidTransactionIfNeglected<TransactionIntentHash>>>
    {
        let invalid_for_display = petitions
            .into_iter()
            .filter_map(|p| {
                p.invalid_transaction_if_neglected_factors(
                    skipped_factor_source_ids.clone(),
                )
            })
            .map(|e| self.for_display_by_address(e))
            .collect::<Result<Vec<_>>>()?;

        if invalid_for_display.is_empty() {
            return Ok(None);
        }

        Ok(Some(InvalidTransactionIfNeglected {
            signable_id,
            entities_which_would_require_delayed_confirmation: vec![],
            entities_which_would_fail_auth: invalid_for_display
                .into_iter()
                .map(|(entity_for_display, shield_metadata)| {
                    InvalidTransactionForEntity::new(
                        entity_for_display,
                        shield_metadata.map(|s| s.metadata),
                    )
                })
                .collect(),
        }))
    }

    fn delayed_confirmation_for_entity(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<DelayedConfirmationForEntity> {
        let (entity_for_display, shields_info) =
            self.for_display_by_address(address)?;
        let shields_info = shields_info.ok_or(CommonError::Unknown)?; // TODO specific error
        let delayed = DelayedConfirmationForEntity::new(
            entity_for_display,
            shields_info
                .time_until_delayed_confirmation_is_callable
                .ok_or(CommonError::Unknown)?, // TODO specific error
            shields_info.metadata,
        );
        Ok(delayed)
    }
}

impl CrossRoleSkipOutcomeAnalyzer<TransactionIntent>
    for CrossRoleSkipOutcomeAnalyzerForManager
{
    /// This method cares about which Role is being exercised,
    /// and the state `self.signing_manager_state_snapshot` is in
    /// when the method is called.
    ///
    /// We will try to find out which `RolesExercisableInTransactionManifestCombination`
    /// we can end up with per role if we where to neglect (skip) a factor.
    ///
    /// If it is the last role we are exercising, i.e. Primary, and we cannot
    /// end up with a valid `RolesExercisableInTransactionManifestCombination`
    /// then we will return `InvalidTransactionForEntity` for that entity (and
    /// put it inside the `entities_which_would_fail_auth` vec)
    ///
    /// We will return `DelayedConfirmationForEntity` (and put it inside
    /// the `entities_which_would_require_delayed_confirmation` vec) if we
    /// can only end up with a valid `RolesExercisableInTransactionManifestCombination`
    /// for which we can only use Delayed Confirmation.
    fn invalid_transaction_if_neglected_factors(
        &self,
        signable_id: TransactionIntentHash,
        skipped_factor_source_ids: IndexSet<FactorSourceIDFromHash>,
        petitions: Vec<PetitionForEntity<TransactionIntentHash>>,
    ) -> Result<Option<InvalidTransactionIfNeglected<TransactionIntentHash>>>
    {
        let Some(current_role) =
            self.signing_manager_state_snapshot.current_role
        else {
            // Signing with Fee payers
            return self.no_cross_role(
                signable_id,
                skipped_factor_source_ids,
                petitions,
            );
        };

        let mut entities_which_would_require_delayed_confirmation: Vec<
            DelayedConfirmationForEntity,
        > = vec![];

        match current_role {
            RoleKind::Recovery => {
                // We can always try Confirmation+Primary role for
                // secuirfied entities, so we don't need to check
                // for that here.
                // And for unsecurified entities we can only use
                // Primary role, which we have not gotten to yet,
                // since Primary is the last role we are exercising.
                return Ok(None);
            }
            RoleKind::Confirmation => {
                let entities_not_signed_for_with_recovery = self
                    .signing_manager_state_snapshot
                    .entities_not_signed_for_at_all();

                let new = entities_not_signed_for_with_recovery
                    .into_iter()
                    // we only care about securified entities - unsecurified entities can only
                    // exercise Primary, and we have not come to Primary role yet.
                    .filter(|e| e.is_securified())
                    .map(|e| self.delayed_confirmation_for_entity(e.address()))
                    .collect::<Result<Vec<DelayedConfirmationForEntity>>>()?;

                entities_which_would_require_delayed_confirmation.extend(new)
            }
            RoleKind::Primary => {
                todo!("impl me")
            }
        }

        let entities_which_would_fail_auth: Vec<InvalidTransactionForEntity> =
            vec![];
        if entities_which_would_fail_auth.is_empty()
            && entities_which_would_require_delayed_confirmation.is_empty()
        {
            return Ok(None);
        }

        Ok(Some(InvalidTransactionIfNeglected {
            signable_id,
            entities_which_would_require_delayed_confirmation,
            entities_which_would_fail_auth,
        }))
    }
}

// Shared Sign Code
impl SigningManager {
    /// # Throws
    /// An error thrown means abort the whole process.
    pub(super) async fn sign_intent_sets_with_role(
        &self,
        intent_sets: Vec<IntentSetToSign>,
        role_kind: RoleKind,
    ) -> Result<ExerciseRoleOutcome> {
        let purpose = SigningPurpose::SignTX { role_kind };

        let adapter =
            ManagerCollectorEphemeralAdapter::new(role_kind, intent_sets);

        let collector = SignaturesCollector::with(
            SigningFinishEarlyStrategy::new(
                WhenAllTransactionsAreValid::r#continue(),
                WhenSomeTransactionIsInvalid::r#continue(),
            ),
            self.proto_profile.factor_sources(),
            adapter.transactions_with_petitions(),
            self.interactor.clone(),
            CrossRoleSkipOutcomeAnalyzerForManager::new(
                self.dependencies.proto_profile.clone(),
                (*self._get_state()).clone(),
            ),
            purpose,
        );

        // Failure is not something we handle, it means the whole process should
        // be aborted by user
        let outcome = collector.collect_signatures().await?;

        // Map output of SignaturesCollector to models our internal state can use
        adapter.exercise_role_outcome(outcome)
    }
}

impl SigningManager {
    /// Signs all relevant Intents of all relevant IntentSets
    /// with the Recovery role.
    pub(super) async fn sign_intents_with_recovery_role(&self) -> Result<()> {
        let role = RoleKind::Recovery;
        self.updating_state(|state| {
            state.current_role = Some(role);
        })?;
        let intent_sets = self.get_intent_sets_to_sign_for_with_recovery_role();
        let outcome =
            self.sign_intent_sets_with_role(intent_sets, role).await?;
        self.handle_recovery_outcome(outcome)
    }

    /// Signs all relevant Intents of all relevant IntentSets
    /// with the Confirmation role.
    pub(super) async fn sign_intents_with_confirmation_role(
        &self,
    ) -> Result<()> {
        let role = RoleKind::Confirmation;
        self.updating_state(|state| {
            state.current_role = Some(role);
        })?;
        let intent_sets =
            self.get_intent_sets_to_sign_for_with_confirmation_role();
        let outcome =
            self.sign_intent_sets_with_role(intent_sets, role).await?;
        self.handle_confirmation_outcome(outcome)
    }

    /// Signs all relevant Intents of all relevant IntentSets
    /// with the Primary role.
    ///
    /// Might not be needed at all to sign with the primary role - since
    /// if user has exercised Recovery and Confirmation roles for all entities
    /// then we are done and can proceed to the next step (signing with fee payers).
    pub(super) async fn sign_intents_with_primary_role_if_needed(
        &self,
    ) -> Result<()> {
        if self.is_meaningless_to_exercise_primary() {
            return Ok(());
        }
        let role = RoleKind::Primary;
        self.updating_state(|state| {
            state.current_role = Some(role);
        })?;
        let intent_sets = self.get_intent_sets_to_sign_for_with_primary_role();

        let outcome =
            self.sign_intent_sets_with_role(intent_sets, role).await?;
        self.handle_primary_outcome(outcome)?;

        // Clear current role before signing with fee payers.
        self.updating_state(|state| state.current_role = None)?;

        Ok(())
    }

    pub(super) fn intermediary_outcome(
        &self,
    ) -> Result<SigningManagerIntermediaryOutcome> {
        let successfully_signed_intent_sets: Vec<SignedIntentSet> =
            self.get_signed_intent_sets()?;

        // TODO: need this?
        let failed_intent_sets: Vec<SignedIntentSet> = vec![];

        Ok(SigningManagerIntermediaryOutcome::new(
            successfully_signed_intent_sets,
            failed_intent_sets,
        ))
    }
}
