use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount),
    Persona(SecurityShieldApplicationTransactionIntentsForSecurifiedPersona),
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
