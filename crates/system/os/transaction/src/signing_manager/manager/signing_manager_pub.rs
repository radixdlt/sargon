use crate::prelude::*;

use super::signing_manager_dependencies::SigningManagerDependencies;

// ==============
// === PUBLIC ===
// ==============
impl SigningManager {
    pub(crate) fn new(
        proto_profile: Arc<dyn IsProtoProfile>,
        sign_interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        saver_of_intents_to_confirm_after_delay: SaveIntentsToConfirmAfterDelayClient,
        intent_sets: impl IntoIterator<
            Item = SecurityShieldApplicationWithTransactionIntents,
        >,
    ) -> Self {
        let state = SigningManagerState::new(intent_sets);
        Self {
            dependencies: SigningManagerDependencies::new(
                proto_profile,
                sign_interactor,
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
        println!("🛡️ Signing with Recovery");
        self.sign_intents_with_recovery_role().await?;

        // Then we sign for the Confirmation role
        println!("🛡️ Signing with Confirmation");
        self.sign_intents_with_confirmation_role().await?;

        // Then we sign for the Primary role, might not be needed. It is ALWAYS
        // needed if any of the entities we are signing for is unsecurified though,
        // since unsecurified entities only have Primary Role.
        println!("🛡️ Signing with Primary if needed");
        self.sign_intents_with_primary_role_if_needed().await?;

        // Try to get the intermediary outcome - containing many variants
        // per IntentSet.
        let signed_for_with_entities_applying_shield =
            self.intermediary_outcome()?;

        // Get the best ones - before we sign with fee payer. No need to sign
        // intents we are not going to submit.
        let best_signed_intent = signed_for_with_entities_applying_shield
            .get_best_signed_intents()?;

        // println!(
        //     "🍭 best signed - set id: {:?}",
        //     best_signed_intent
        //         .iter()
        //         .map(|si| si.intent_set_id())
        //         .collect_vec()
        // );

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
