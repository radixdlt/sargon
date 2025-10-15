use crate::prelude::*;

pub struct AccessControllerRecoveryIntentsBuilder {
    base_intent: TransactionIntent,
    // The lock fee data for the above intent
    lock_fee_data: LockFeeData,
    securified_entity: AnySecurifiedEntity,
    proposed_security_structure: SecurityStructureOfFactorInstances,
}

impl AccessControllerRecoveryIntentsBuilder {
    pub fn new(
        base_intent: TransactionIntent,
        lock_fee_data: LockFeeData,
        securified_entity: AnySecurifiedEntity,
        proposed_security_structure: SecurityStructureOfFactorInstances,
    ) -> Self {
        Self {
            base_intent,
            lock_fee_data,
            securified_entity,
            proposed_security_structure,
        }
    }
}

impl AccessControllerRecoveryIntentsBuilder {
    pub fn build(&self) -> Result<AccessControllerRecoveryIntents> {
        Ok(
            AccessControllerRecoveryIntents::new(
            self.signable_for_role_combination(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?,
            self.signable_for_role_combination(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?,
            self.signable_for_role_combination(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?,
            self.signable_for_role_combination(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?,
            )
        )
    }

    fn signable_for_role_combination(
        &self,
        role_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Result<SignableWithEntities<TransactionIntent>> {
        let mut manifest =
            TransactionManifest::apply_security_shield_for_securified_entity(
                self.securified_entity.clone(),
                self.proposed_security_structure.clone(),
                role_combination,
            );

        manifest.modify_add_lock_fee(self.lock_fee_data.clone())?;

        let intent = TransactionIntent::new(
            self.base_intent.header,
            manifest,
            self.base_intent.message.clone(),
        )?;

        Ok(SignableWithEntities::with(
            intent,
            vec![self.securified_entity.entity.clone()],
        ))
    }
}

pub struct AccessControllerRecoveryIntents {
    pub initiate_with_recovery_complete_with_confirmation:
        SignableWithEntities<TransactionIntent>,
    pub initiate_with_recovery_complete_with_primary:
        SignableWithEntities<TransactionIntent>,
    pub initiate_with_recovery_delayed_completion:
        SignableWithEntities<TransactionIntent>,
    pub initiate_with_primary_complete_with_confirmation:
        SignableWithEntities<TransactionIntent>,
}

impl AccessControllerRecoveryIntents {
    pub fn new(
        initiate_with_recovery_complete_with_confirmation: SignableWithEntities<
            TransactionIntent,
        >,
        initiate_with_recovery_complete_with_primary: SignableWithEntities<
            TransactionIntent,
        >,
        initiate_with_recovery_delayed_completion: SignableWithEntities<
            TransactionIntent,
        >,
        initiate_with_primary_complete_with_confirmation: SignableWithEntities<
            TransactionIntent,
        >,
    ) -> Self {
        Self {
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
        }
    }

    pub fn all_signables(
        &self,
    ) -> IdentifiedVecOf<SignableWithEntities<TransactionIntent>> {
        IdentifiedVecOf::from(vec![
            self.initiate_with_primary_complete_with_confirmation
                .clone(),
            self.initiate_with_recovery_complete_with_primary.clone(),
            self.initiate_with_recovery_delayed_completion.clone(),
            self.initiate_with_primary_complete_with_confirmation
                .clone(),
        ])
    }
}
