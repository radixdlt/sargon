use std::sync::RwLockReadGuard;

use crate::prelude::*;

/// Implementation of complex signing flow laid out in this
/// [whimsical diagram][flow].
///
/// [flow]: https://whimsical.com/wallet-sargon-signing-flow-QFvU2NAVXFiX1VgNBuvj5g
pub(crate) struct SigningManager {
    /// FactorSources in Profile
    factor_sources_in_profile: IndexSet<FactorSource>,
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,

    /// The internal state of the SigningManager
    ///
    /// We start with `None` in ctor, and set it to `Some` in `sign_intent_sets`.
    /// We wanna init this SigninManager only with dependencies and not until
    /// later when we call `sign_intent_sets` we can set the state.
    state: RwLock<SigningManagerState>,
}

// ==============
// === PUBLIC ===
// ==============
impl SigningManager {
    pub fn new(
        factor_sources_in_profile: IndexSet<FactorSource>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        intent_sets: impl IntoIterator<
            Item = SecurityShieldApplicationWithTransactionIntents,
        >,
    ) -> Self {
        let state = SigningManagerState::new(intent_sets);
        Self {
            factor_sources_in_profile,
            interactor,
            state: RwLock::new(state),
        }
    }

    /// A "TransactionIntent Set" is a "group" of TransactionsIntents having manifest per variant
    /// of [`RolesExercisableInTransactionManifestCombination`]. For manifests
    /// securifying an unsecurified entity the set will have only one intent.
    ///
    /// From each set we should only submit one to the Ledger, and that is the
    /// "best one" of those which was signed. Successfully signed intent which
    /// can exercise the Confirmation role are better than those using delay completion (
    /// time).
    ///
    /// We are performing 4 passes to the SignaturesCollector, first
    /// using Recovery role, then Confirmation role, then Primary role for
    /// the entities applying the shield, and lastly we sign for the fee payers
    /// using Primary role.
    pub async fn sign_intent_sets(&self) -> Result<SigningManagerOutcome> {
        // Start with Recovery role
        self.sign_intents_with_recovery_role().await?;

        // Then we sign for the Confirmation role
        self.sign_intents_with_confirmation_role().await?;

        // Then we sign for the Primary role
        self.sign_intents_with_primary_role().await?;

        // Try to get the intermediary outcome
        // We have not signed for with all entities
        // applying the shield.
        let signed_for_with_entities_applying_shield =
            self.intermediary_outcome()?;
        // Get the best ones
        let best_signed_intent = signed_for_with_entities_applying_shield
            .get_best_signed_intents()?;

        // Sign with fee payer
        self.sign_for_fee_payers(best_signed_intent).await
    }
}

// ===============
// === PRIVATE ===
// ===============
impl SigningManager {
    /// # Throws
    /// An error thrown means abort the whole process.
    async fn sign_intent_sets_with_role(
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

    fn _get_state(&self) -> RwLockReadGuard<'_, SigningManagerState> {
        self.state.read().unwrap()
    }

    fn get_intent_sets_to_sign_for_with_role_of_kind(
        &self,
        role_kind: RoleKind,
    ) -> Vec<IntentSetToSign> {
        let state = self._get_state();
        state
            .per_set_state
            .values()
            .filter_map(|s| IntentSetToSign::maybe_from(s, role_kind))
            .collect_vec()
    }

    fn get_intent_sets_to_sign_for_with_recovery_role(
        &self,
    ) -> Vec<IntentSetToSign> {
        self.get_intent_sets_to_sign_for_with_role_of_kind(RoleKind::Recovery)
    }

    fn get_intent_sets_to_sign_for_with_confirmation_role(
        &self,
    ) -> Vec<IntentSetToSign> {
        self.get_intent_sets_to_sign_for_with_role_of_kind(
            RoleKind::Confirmation,
        )
    }

    fn get_intent_sets_to_sign_for_with_primary_role(
        &self,
    ) -> Vec<IntentSetToSign> {
        self.get_intent_sets_to_sign_for_with_role_of_kind(RoleKind::Primary)
    }

    async fn sign_intents_with_recovery_role(&self) -> Result<()> {
        let intent_sets = self.get_intent_sets_to_sign_for_with_recovery_role();
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Recovery)
            .await?;
        self.handle_recovery_outcome(outcome)
    }

    async fn sign_intents_with_confirmation_role(&self) -> Result<()> {
        let intent_sets =
            self.get_intent_sets_to_sign_for_with_confirmation_role();
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Confirmation)
            .await?;
        self.handle_confirmation_outcome(outcome)
    }

    async fn sign_intents_with_primary_role(&self) -> Result<()> {
        let intent_sets = self.get_intent_sets_to_sign_for_with_primary_role();
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Primary)
            .await?;
        self.handle_primary_outcome(outcome)
    }

    async fn sign_for_fee_payers(
        &self,
        signed_intents: Vec<SignedIntentWithContext>,
    ) -> Result<SigningManagerOutcome> {
        let role_kind = RoleKind::Primary;
        let payer_by_tx_id = |intent_set_id: IntentSetID,
                              txid: TransactionIntentHash|
         -> Result<Account> {
            let state = self._get_state();
            let s = state.per_set_state.get(&intent_set_id).unwrap();
            let txids = s.internal_state.transaction_intent_hashes();
            assert!(txids.contains(&txid));
            Ok(s.internal_state.paying_account())
        };

        // We are NOT signing intent SETs but we piggy back
        // on the existing code above, and inlay a single intent into a set
        // to be able to use the same code.
        let intent_sets = signed_intents
            .iter()
            .map(|si| {
                let intent_set_id = si.context.intent_set_id;
                let txid = si.signed_intent.intent.transaction_intent_hash();
                let entity = payer_by_tx_id(intent_set_id, txid)?;
                Ok(IntentSetToSign::single_intent(
                    IntentSetID::new(),
                    role_kind,
                    IntentVariant::new(None, si.signed_intent.intent.clone()),
                    entity.into(),
                ))
            })
            .collect::<Result<Vec<IntentSetToSign>>>()?;

        let mut signed_intents = signed_intents
            .into_iter()
            .map(|si| (si.context, si.signed_intent))
            .collect::<IndexMap<EntitySigningContext, SignedIntent>>();

        let exercise_role_outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Primary)
            .await?;

        assert!(exercise_role_outcome.entities_not_signed_for.is_empty());

        let signed_with_payers = exercise_role_outcome.entities_signed_for;
        signed_with_payers
            .0
            .into_iter()
            .for_each(|signed_with_payer| {
                let signed_intent = signed_intents
                    .get_mut(&signed_with_payer.context)
                    .expect("Should have signed intent");
                signed_intent.add_fee_payer_signatures(
                    signed_with_payer.intent_signatures(),
                );
            });

        Ok(SigningManagerOutcome(
            signed_intents.values().cloned().collect_vec(),
        ))
    }

    fn intermediary_outcome(
        &self,
    ) -> Result<SigningManagerIntermediaryOutcome> {
        let mut state = self.state.write().map_err(|_| CommonError::Unknown)?; // TODO specific error variant
        todo!()
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
