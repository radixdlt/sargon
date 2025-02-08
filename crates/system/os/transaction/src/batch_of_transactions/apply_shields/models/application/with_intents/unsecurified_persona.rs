use crate::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub struct SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent
{
    pub application: SecurityShieldApplicationForUnsecurifiedPersona,
    pub transaction_intent: TransactionIntent,
}
impl SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent {
    pub fn new(
        application: SecurityShieldApplicationForUnsecurifiedPersona,
        transaction_intent: TransactionIntent,
    ) -> Self {
        Self {
            application,
            transaction_intent,
        }
    }
}
