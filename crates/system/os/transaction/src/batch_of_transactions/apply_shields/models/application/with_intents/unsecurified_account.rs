use crate::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub struct SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent
{
    pub application: SecurityShieldApplicationForUnsecurifiedAccount,
    pub transaction_intent: TransactionIntent,
}
impl SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent {
    pub fn new(
        application: SecurityShieldApplicationForUnsecurifiedAccount,
        transaction_intent: TransactionIntent,
    ) -> Self {
        Self {
            application,
            transaction_intent,
        }
    }
}
