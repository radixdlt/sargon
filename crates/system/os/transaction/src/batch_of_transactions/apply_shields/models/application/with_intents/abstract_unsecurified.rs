use crate::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub struct AbstractSecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent<
    E: IsBaseEntity + std::hash::Hash + Eq + Clone,
> {
    pub application: AbstractSecurityShieldApplicationForUnsecurifiedEntity<E>,
    pub transaction_intent: TransactionIntent,
}

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone>
    AbstractSecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent<
        E,
    >
{
    pub fn paying_account(&self) -> ApplicationInputPayingAccount {
        self.application.paying_account.clone()
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
