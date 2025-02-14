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
    pub fn paying_account(&self) -> ApplicationInputPayingAccount {
        match self {
            Self::Account(a) => a.paying_account(),
            Self::Persona(p) => p.paying_account(),
        }
    }
    pub fn entity_applying_shield(&self) -> AnyUnsecurifiedEntity {
        match self {
            Self::Account(a) => a.entity_applying_shield(),
            Self::Persona(p) => p.entity_applying_shield(),
        }
    }

    pub fn transaction_intent(&self) -> TransactionIntent {
        match self {
            Self::Account(a) => a.transaction_intent(),
            Self::Persona(p) => p.transaction_intent(),
        }
    }
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
