use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopTimedRecoveryIntentKind {
    StopAndCancel,
    Stop,
}

pub struct AccessControllerStopTimedRecoveryIntentsBuilder {
    base_intent: TransactionIntent,
    lock_fee_data: LockFeeData,
    securified_entity: AnySecurifiedEntity,
}

impl AccessControllerStopTimedRecoveryIntentsBuilder {
    pub fn new(
        base_intent: TransactionIntent,
        lock_fee_data: LockFeeData,
        securified_entity: AnySecurifiedEntity,
    ) -> Self {
        Self {
            base_intent,
            lock_fee_data,
            securified_entity,
        }
    }
}

impl AccessControllerStopTimedRecoveryIntentsBuilder {
    pub fn build(&self) -> Result<AccessControllerStopTimedRecoveryIntents> {
        let stop_with_recovery =
            self.signable_for_role_kind(RoleKind::Recovery)?;
        let stop_with_primary = self.signable_for_role_kind(RoleKind::Primary)?;
        let stop_with_confirmation = self.signable_for_role_kind(RoleKind::Confirmation)?;

        Ok(AccessControllerStopTimedRecoveryIntents::new(
            stop_with_recovery,
            stop_with_primary,
            stop_with_confirmation
        ))
    }

    fn signable_for_role_kind(
        &self,
        role_kind: RoleKind,
    ) -> Result<SignableWithEntities<TransactionIntent>> {
        let manifest = match role_kind {
            RoleKind::Recovery => {
                TransactionManifest::stop_and_cancel_timed_recovery(
                    self.securified_entity.clone(),
                )
            }
            RoleKind::Primary | RoleKind::Confirmation => {
                TransactionManifest::stop_timed_recovery(
                    self.securified_entity.clone(),
                )
            }
        };

        let manifest_with_fee =
            manifest.modify_add_lock_fee(self.lock_fee_data.clone())?;

        let intent = TransactionIntent::new(
            self.base_intent.header,
            manifest_with_fee,
            self.base_intent.message.clone(),
        )?;

        Ok(SignableWithEntities::with(
            intent,
            vec![self.securified_entity.entity.clone()],
        ))
    }
}

pub struct AccessControllerStopTimedRecoveryIntents {
    pub stop_with_recovery: SignableWithEntities<TransactionIntent>,
    pub stop_with_primary: SignableWithEntities<TransactionIntent>,
    pub stop_with_confirmation: SignableWithEntities<TransactionIntent>,
}

impl AccessControllerStopTimedRecoveryIntents {
    pub fn new(
        stop_with_recovery: SignableWithEntities<TransactionIntent>,
        stop_with_primary: SignableWithEntities<TransactionIntent>,
        stop_with_confirmation: SignableWithEntities<TransactionIntent>,
    ) -> Self {
        Self {
            stop_with_recovery,
            stop_with_primary,
            stop_with_confirmation,
        }
    }

    pub fn all_signables(
        &self,
    ) -> IdentifiedVecOf<SignableWithEntities<TransactionIntent>> {
        IdentifiedVecOf::from(vec![
            self.stop_with_recovery.clone(),
            self.stop_with_primary.clone(),
            self.stop_with_confirmation.clone()
        ])
    }

    pub fn role_kind_for_intent_hash(
        &self,
        intent_hash: &TransactionIntentHash
    ) -> RoleKind {
        if self.stop_with_recovery.id == *intent_hash {
            RoleKind::Recovery
        } else if self.stop_with_primary.id == *intent_hash {
            RoleKind::Primary
        } else {
            RoleKind::Confirmation
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = AccessControllerStopTimedRecoveryIntentsBuilder;

//     #[test]
//     fn builds_both_intents() {
//         let base_intent = TransactionIntent::sample();
//         let lock_fee_data = LockFeeData::new_with_unsecurified_fee_payer(
//             AccountAddress::sample(),
//             Decimal192::one(),
//         );
//         let securified_entity = AnySecurifiedEntity::sample_account();

//         let builder =
//             SUT::new(base_intent, lock_fee_data, securified_entity.clone());

//         let intents = builder.build().expect("intents");
//         assert_ne!(intents.stop_and_cancel.id, intents.stop.id);

//         let all_signables = intents.all_signables();
//         assert_eq!(all_signables.len(), 2);
//     }
// }
