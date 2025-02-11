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
        without: impl Into<SecurityShieldApplicationForUnsecurifiedEntity>,
        intent: TransactionIntent,
    ) -> Self {
        let without = without.into();
        match without {
            SecurityShieldApplicationForUnsecurifiedEntity::Account(a) =>
                Self::Account(SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent::new(a, intent)),
            SecurityShieldApplicationForUnsecurifiedEntity::Persona(p) => Self::Persona(SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent::new(p, intent)),
        }
    }
}

pub type SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent =
    AbstractSecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent<
        Account,
    >;

pub type SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent =
    AbstractSecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent<
        Persona,
    >;
