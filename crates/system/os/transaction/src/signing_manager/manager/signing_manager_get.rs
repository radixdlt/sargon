use std::sync::RwLockReadGuard;

use crate::prelude::*;

// === Non-pub Get ===
impl SigningManager {
    pub(super) fn get_intents_to_confirm_after_delay(
        &self,
        best_signed_intent: &[SignedIntentWithContext],
    ) -> Result<IndexSet<IntentVariantToConfirmAfterDelay>> {
        best_signed_intent
            .iter()
            .filter_map(|s| s.as_confirm_after_delay_variant())
            .collect::<Result<IndexSet<IntentVariantToConfirmAfterDelay>>>()
    }
}
impl SigningManager {
    pub(super) fn _get_state(
        &self,
    ) -> RwLockReadGuard<'_, SigningManagerState> {
        self.state.read().unwrap()
    }

    pub(super) fn get_intent_sets_to_sign_for_with_role_of_kind(
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

    pub(super) fn get_intent_sets_to_sign_for_with_recovery_role(
        &self,
    ) -> Vec<IntentSetToSign> {
        self.get_intent_sets_to_sign_for_with_role_of_kind(RoleKind::Recovery)
    }

    pub(super) fn get_intent_sets_to_sign_for_with_confirmation_role(
        &self,
    ) -> Vec<IntentSetToSign> {
        self.get_intent_sets_to_sign_for_with_role_of_kind(
            RoleKind::Confirmation,
        )
    }

    pub(super) fn get_intent_sets_to_sign_for_with_primary_role(
        &self,
    ) -> Vec<IntentSetToSign> {
        self.get_intent_sets_to_sign_for_with_role_of_kind(RoleKind::Primary)
    }
}
