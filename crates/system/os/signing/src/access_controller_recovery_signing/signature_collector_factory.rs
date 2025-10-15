use crate::prelude::*;

pub struct SignaturesCollectorFactory {
    finish_early_strategy: SigningFinishEarlyStrategy,
    profile_factor_sources: IndexSet<FactorSource>,
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    recovery_intents: AccessControllerRecoveryIntents,
}

impl SignaturesCollectorFactory {
    pub fn new(
        finish_early_strategy: SigningFinishEarlyStrategy,
        profile_factor_sources: IndexSet<FactorSource>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        recovery_intents: AccessControllerRecoveryIntents,
    ) -> Self {
        Self {
            finish_early_strategy,
            profile_factor_sources,
            interactor,
            recovery_intents,
        }
    }

    pub fn intent_for_hash(
        &self,
        hash: &TransactionIntentHash,
    ) -> Option<TransactionIntent> {
        self.recovery_intents
            .all_signables()
            .into_iter()
            .find(|signable| signable.id == *hash)
            .map(|signable| signable.signable.clone())
    }

    pub fn intent_hash_of_timed_recovery(&self) -> TransactionIntentHash {
        self.recovery_intents
            .initiate_with_recovery_delayed_completion
            .id()
    }
}

impl SignaturesCollectorFactory {
    pub fn initiate_recovery_with_recovery_sign_with_recovery_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Recovery,
            RoleKind::Recovery,
        )
    }

    pub fn initiate_recovery_with_recovery_sign_with_confirmation_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Recovery,
            RoleKind::Confirmation,
        )
    }

    pub fn initiate_recovery_with_recovery_sign_with_primary_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Recovery,
            RoleKind::Primary,
        )
    }

    pub fn initiate_recovery_with_primary_sign_with_primary_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Primary,
            RoleKind::Primary,
        )
    }

    pub fn initiate_recovery_with_primary_sign_with_confirmation_role_collector(
        &self,
    ) -> SignaturesCollector<TransactionIntent> {
        self.signature_collector_for_recovery_signing(
            RoleKind::Primary,
            RoleKind::Confirmation,
        )
    }

    // Sign for the fee payer? TBD

    fn signature_collector_for_recovery_signing(
        &self,
        recovery_proposer_kind: RoleKind,
        signing_kind: RoleKind,
    ) -> SignaturesCollector<TransactionIntent> {
        SignaturesCollector::with(
            self.finish_early_strategy.clone(),
            self.profile_factor_sources.clone(),
            self.transaction_intents_for_recovery_signing(
                recovery_proposer_kind,
                signing_kind,
            ),
            self.interactor.clone(),
            SigningPurpose::SignTX {
                role_kind: signing_kind,
            },
        )
    }
}

impl SignaturesCollectorFactory {
    fn transaction_intents_for_recovery_signing(
        &self,
        recovery_proposer_kind: RoleKind,
        signing_kind: RoleKind,
    ) -> IdentifiedVecOf<SignableWithEntities<TransactionIntent>> {
        match recovery_proposer_kind {
            RoleKind::Recovery => match signing_kind {
                RoleKind::Recovery => IdentifiedVecOf::from(vec![
                    self.recovery_intents
                        .initiate_with_recovery_complete_with_confirmation
                        .clone(),
                    self.recovery_intents
                        .initiate_with_recovery_complete_with_primary
                        .clone(),
                    self.recovery_intents
                        .initiate_with_recovery_delayed_completion
                        .clone(),
                ]),
                RoleKind::Primary => IdentifiedVecOf::from(vec![self
                    .recovery_intents
                    .initiate_with_recovery_complete_with_primary
                    .clone()]),
                RoleKind::Confirmation => IdentifiedVecOf::from(vec![self
                    .recovery_intents
                    .initiate_with_recovery_complete_with_confirmation
                    .clone()]),
            },
            RoleKind::Primary => IdentifiedVecOf::from(vec![self
                .recovery_intents
                .initiate_with_primary_complete_with_confirmation
                .clone()]),
            RoleKind::Confirmation => {
                panic!("Confirmation role cannot iniate recovery")
            }
        }
    }
}
