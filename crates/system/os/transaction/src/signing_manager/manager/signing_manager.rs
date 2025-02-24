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

    fn for_display_by_address(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<(EntityForDisplay, Option<SecurityStructureMetadata>)> {
        let entity = self.proto_profile.entity_by_address(entity_address)?;
        let provisional_security_config = entity.get_provisional();
        let security_structure_of_factor_instances = provisional_security_config.map(|config| config.as_factor_instances_derived().cloned().ok_or(CommonError::ProvisionalConfigInWrongStateExpectedInstancesDerived)).transpose()?;
        let shield_id = security_structure_of_factor_instances.map(|s| s.id());
        let shield_metadata = shield_id
            .map(|id| self.proto_profile.shield_metadata_by_id(id))
            .transpose()?;
        let entity_for_display = EntityForDisplay::from(entity);
        Ok((entity_for_display, shield_metadata))
    }

    fn no_cross_role(
        &self,
        signable_id: TransactionIntentHash,
        skipped_factor_source_ids: IndexSet<FactorSourceIDFromHash>,
        petitions: Vec<PetitionForEntity<TransactionIntentHash>>,
    ) -> Option<InvalidTransactionIfNeglected<TransactionIntentHash>> {
        let invalid_for_display = petitions
            .into_iter()
            .filter_map(|p| {
                p.invalid_transaction_if_neglected_factors(
                    skipped_factor_source_ids.clone(),
                )
            })
            .map(|e| self.for_display_by_address(e))
            .collect::<Result<Vec<_>>>()
            .ok();

        let Some(invalid_for_display) = invalid_for_display else {
            return None;
        };

        if invalid_for_display.is_empty() {
            return None;
        }

        Some(InvalidTransactionIfNeglected {
            signable_id,
            entities_which_would_require_delayed_confirmation: vec![],
            entities_which_would_fail_auth: invalid_for_display
                .into_iter()
                .map(|(entity_for_display, shield_metadata)| {
                    InvalidTransactionForEntity::new(
                        entity_for_display,
                        shield_metadata,
                    )
                })
                .collect(),
        })
    }
}
impl CrossRoleSkipOutcomeAnalyzer<TransactionIntent>
    for CrossRoleSkipOutcomeAnalyzerForManager
{
    fn invalid_transaction_if_neglected_factors(
        &self,
        signable_id: TransactionIntentHash,
        skipped_factor_source_ids: IndexSet<FactorSourceIDFromHash>,
        petitions: Vec<PetitionForEntity<TransactionIntentHash>>,
    ) -> Option<InvalidTransactionIfNeglected<TransactionIntentHash>> {
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

        /*
        struct DelayedConfirmationForEntity {
            entity_for_display: EntityForDisplay,
            delay: TimePeriod,
            shield_for_display: ShieldForDisplay,
        }
        */

        /*
        struct InvalidTransactionForEntity {
            entity_for_display: EntityForDisplay,
            shield_for_display: Option<ShieldForDisplay>,
        }
        */
        let entities_which_would_require_delayed_confirmation: Vec<
            DelayedConfirmationForEntity,
        > = vec![];
        let entities_which_would_fail_auth: Vec<InvalidTransactionForEntity> =
            vec![];
        if entities_which_would_fail_auth.is_empty()
            && entities_which_would_require_delayed_confirmation.is_empty()
        {
            return None;
        }

        Some(InvalidTransactionIfNeglected {
            signable_id,
            entities_which_would_require_delayed_confirmation,
            entities_which_would_fail_auth,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SigningManager;

    #[actix_rt::test]
    async fn test() {
        // let sut = SUT::new(profile, interactor)
    }
}
