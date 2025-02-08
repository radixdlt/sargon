use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent {
    Account(
        SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent,
    ),
    Persona(
        SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent,
    ),
}

impl SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent {
    pub fn with_intent(
        without: SecurityShieldApplicationForUnsecurifiedEntity,
        intent: TransactionIntent,
    ) -> Self {
        match without {
            SecurityShieldApplicationForUnsecurifiedEntity::Account(a) => {
                Self::Account(SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent::new(a, intent))
            }
            SecurityShieldApplicationForUnsecurifiedEntity::Persona(p) => {
                Self::Persona(SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent::new(p, intent))
            }
        }
    }
}
