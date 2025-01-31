use crate::{
    create_application_for_securified_entity_with_intents, prelude::*,
};

create_application_for_securified_entity_with_intents! {
    SecurityShieldApplicationTransactionIntentsForSecurifiedPersona,
    /// The persona we are applyying the shield for
    /// and the account that will pay the topping up of the AccessControllers XRD vault.
    persona_with_paying_account: SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount
}

create_application_for_securified_entity_with_intents! {
    SecurityShieldApplicationTransactionIntentsForSecurifiedAccount,
    /// The account we are applying the shield for and an optional other payer
    account_with_optional_paying_account: SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount
}

#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount),
    Persona(SecurityShieldApplicationTransactionIntentsForSecurifiedPersona),
}

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

impl SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    pub fn with_intents(
        without: SecurityShieldApplicationForSecurifiedEntity,
        initiate_with_recovery_complete_with_primary: TransactionIntent,
        initiate_with_recovery_complete_with_confirmation: TransactionIntent,
        initiate_with_recovery_delayed_completion: TransactionIntent,
        initiate_with_primary_complete_with_confirmation: TransactionIntent,
        initiate_with_primary_delayed_completion: TransactionIntent,
    ) -> Self {
        match without {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                Self::Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount::new(a.account_with_optional_paying_account, initiate_with_recovery_complete_with_primary, initiate_with_recovery_complete_with_confirmation, initiate_with_recovery_delayed_completion, initiate_with_primary_complete_with_confirmation, initiate_with_primary_delayed_completion))
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                Self::Persona(SecurityShieldApplicationTransactionIntentsForSecurifiedPersona::new(p.persona_with_paying_account, initiate_with_recovery_complete_with_primary, initiate_with_recovery_complete_with_confirmation, initiate_with_recovery_delayed_completion, initiate_with_primary_complete_with_confirmation, initiate_with_primary_delayed_completion))
            }
        }
    }
}
