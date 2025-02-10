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
