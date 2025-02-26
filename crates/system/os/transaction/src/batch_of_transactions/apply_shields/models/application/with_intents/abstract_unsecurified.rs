use crate::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct AbstractSecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent<
    E: IsEntity,
> {
    pub application: AbstractSecurityShieldApplicationForUnsecurifiedEntity<E>,
    pub transaction_intent: TransactionIntent,
}

impl<E: IsEntity>
    AbstractSecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent<
        E,
    >
{
    pub fn paying_account(&self) -> ApplicationInputPayingAccount {
        self.application.paying_account.clone()
    }

    pub fn entity_applying_shield(&self) -> AnyUnsecurifiedEntity {
        AnyUnsecurifiedEntity::new(
            self.application.entity_applying_shield().entity,
        )
        .expect("Is unsecurified")
    }

    pub fn transaction_intent(&self) -> TransactionIntent {
        self.transaction_intent.clone()
    }

    pub fn new(
        application: AbstractSecurityShieldApplicationForUnsecurifiedEntity<E>,
        transaction_intent: TransactionIntent,
    ) -> Self {
        Self {
            application,
            transaction_intent,
        }
    }
}
