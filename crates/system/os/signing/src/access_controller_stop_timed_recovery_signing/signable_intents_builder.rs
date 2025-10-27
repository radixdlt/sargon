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
        let stop_and_cancel =
            self.signable_for_kind(StopTimedRecoveryIntentKind::StopAndCancel)?;
        let stop = self.signable_for_kind(StopTimedRecoveryIntentKind::Stop)?;

        Ok(AccessControllerStopTimedRecoveryIntents::new(
            stop_and_cancel,
            stop,
        ))
    }

    fn signable_for_kind(
        &self,
        kind: StopTimedRecoveryIntentKind,
    ) -> Result<SignableWithEntities<TransactionIntent>> {
        let manifest = match kind {
            StopTimedRecoveryIntentKind::StopAndCancel => {
                TransactionManifest::stop_and_cancel_timed_recovery(
                    self.securified_entity.clone(),
                )
            }
            StopTimedRecoveryIntentKind::Stop => {
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
    pub stop_and_cancel: SignableWithEntities<TransactionIntent>,
    pub stop: SignableWithEntities<TransactionIntent>,
}

impl AccessControllerStopTimedRecoveryIntents {
    pub fn new(
        stop_and_cancel: SignableWithEntities<TransactionIntent>,
        stop: SignableWithEntities<TransactionIntent>,
    ) -> Self {
        Self {
            stop_and_cancel,
            stop,
        }
    }

    pub fn all_signables(
        &self,
    ) -> IdentifiedVecOf<SignableWithEntities<TransactionIntent>> {
        IdentifiedVecOf::from(vec![
            self.stop_and_cancel.clone(),
            self.stop.clone(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerStopTimedRecoveryIntentsBuilder;

    #[test]
    fn builds_both_intents() {
        let base_intent = TransactionIntent::sample();
        let lock_fee_data = LockFeeData::new_with_unsecurified_fee_payer(
            AccountAddress::sample(),
            Decimal192::one(),
        );
        let securified_entity = AnySecurifiedEntity::sample_account();

        let builder =
            SUT::new(base_intent, lock_fee_data, securified_entity.clone());

        let intents = builder.build().expect("intents");
        assert_ne!(intents.stop_and_cancel.id, intents.stop.id);

        let all_signables = intents.all_signables();
        assert_eq!(all_signables.len(), 2);
    }
}
