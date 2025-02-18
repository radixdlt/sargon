use crate::prelude::*;

use super::signing_manager_dependencies::SigningManagerDependencies;

// ==============
// === PUBLIC ===
// ==============
impl SigningManager {
    pub(crate) fn new(
        factor_sources_in_profile: IndexSet<FactorSource>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        saver_of_intents_to_confirm_after_delay: SaveIntentsToConfirmAfterDelayClient,
        intent_sets: impl IntoIterator<
            Item = SecurityShieldApplicationWithTransactionIntents,
        >,
    ) -> Self {
        let state = SigningManagerState::new(intent_sets);
        Self {
            dependencies: SigningManagerDependencies::new(
                factor_sources_in_profile,
                interactor,
                saver_of_intents_to_confirm_after_delay,
            )
            .into(),
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
    pub(crate) async fn sign_intent_sets(
        &self,
    ) -> Result<SigningManagerOutcome> {
        // Start with Recovery role
        self.sign_intents_with_recovery_role().await?;

        // Then we sign for the Confirmation role
        self.sign_intents_with_confirmation_role().await?;

        // Then we sign for the Primary role
        self.sign_intents_with_primary_role().await?;

        // Try to get the intermediary outcome - containing many variants
        // per IntentSet.
        let signed_for_with_entities_applying_shield =
            self.intermediary_outcome()?;

        // Get the best ones - before we sign with fee payer. No need to sign
        // intents we are not going to submit.
        let best_signed_intent = signed_for_with_entities_applying_shield
            .get_best_signed_intents()?;

        let intents_to_confirm_after_delay =
            self.get_intents_to_confirm_after_delay(&best_signed_intent)?;

        self.saver_of_intents_to_confirm_after_delay
            .save_intents_to_confirm_after_delay(intents_to_confirm_after_delay)
            .await?;

        // Sign with fee payer - we only need to sign the best ones with
        // the fee payer.
        self.sign_for_fee_payers(best_signed_intent).await
    }
}
