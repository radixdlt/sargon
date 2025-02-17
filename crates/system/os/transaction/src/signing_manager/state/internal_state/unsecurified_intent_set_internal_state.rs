use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct UnsecurifiedIntentSetInternalState {
    account_paying_for_transaction: Immutable<ApplicationInputPayingAccount>,
    pub(crate) entity_applying_shield: Immutable<AnyUnsecurifiedEntity>,
    pub(crate) transaction_intent: Immutable<TransactionIntent>,

    signatures: IntentVariantSignaturesForRoleState,
}

impl UnsecurifiedIntentSetInternalState {
    pub(crate) fn paying_account(&self) -> Account {
        self.account_paying_for_transaction.account()
    }

    pub(crate) fn transaction_intent_hash(&self) -> TransactionIntentHash {
        self.transaction_intent.transaction_intent_hash()
    }

    pub(crate) fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(intent_with_signatures.intent, *self.transaction_intent);
        assert_eq!(
            intent_with_signatures.entity.address(),
            self.entity_applying_shield.address()
        );

        self.signatures
            .update_with_intent_with_signatures(intent_with_signatures);
    }
    fn new(
        account_paying_for_transaction: impl Into<
            Immutable<ApplicationInputPayingAccount>,
        >,
        entity_applying_shield: impl Into<Immutable<AnyUnsecurifiedEntity>>,
        transaction_intent: impl Into<Immutable<TransactionIntent>>,
    ) -> Self {
        Self {
            account_paying_for_transaction: account_paying_for_transaction
                .into(),
            entity_applying_shield: entity_applying_shield.into(),
            transaction_intent: transaction_intent.into(),
            // For unsecurified entities we only have Primary role.
            signatures: IntentVariantSignaturesForRoleState::new(
                RoleKind::Primary,
            ),
        }
    }
}
impl From<SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent>
    for UnsecurifiedIntentSetInternalState
{
    fn from(
        application_with_intent: SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent,
    ) -> Self {
        Self::new(
            application_with_intent.paying_account(),
            application_with_intent.entity_applying_shield(),
            application_with_intent.transaction_intent(),
        )
    }
}
