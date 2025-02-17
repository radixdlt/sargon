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
            SigningFinishEarlyStrategy::default(),
            self.factor_sources_in_profile.clone(),
            adapter.transactions_with_petitions(),
            self.interactor.clone(),
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
        let intent_sets = self.get_intent_sets_to_sign_for_with_recovery_role();
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Recovery)
            .await?;
        self.handle_recovery_outcome(outcome)
    }

    /// Signs all relevant Intents of all relevant IntentSets
    /// with the Confirmation role.
    pub(super) async fn sign_intents_with_confirmation_role(
        &self,
    ) -> Result<()> {
        let intent_sets =
            self.get_intent_sets_to_sign_for_with_confirmation_role();
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Confirmation)
            .await?;
        self.handle_confirmation_outcome(outcome)
    }

    /// Signs all relevant Intents of all relevant IntentSets
    /// with the Primary role.
    pub(super) async fn sign_intents_with_primary_role(&self) -> Result<()> {
        let intent_sets = self.get_intent_sets_to_sign_for_with_primary_role();
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Primary)
            .await?;
        self.handle_primary_outcome(outcome)
    }

    pub(super) fn intermediary_outcome(
        &self,
    ) -> Result<SigningManagerIntermediaryOutcome> {
        let successfully_signed_intent_sets: Vec<SignedIntentSet> = vec![];
        let failed_intent_sets: Vec<SignedIntentSet> = vec![];
        todo!();
        Ok(SigningManagerIntermediaryOutcome::new(
            successfully_signed_intent_sets,
            failed_intent_sets,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type SUT = SigningManager;

    #[actix_rt::test]
    async fn test() {
        // let sut = SUT::new(profile, interactor)
    }
}
