use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount),
    Persona(SecurityShieldApplicationTransactionIntentsForSecurifiedPersona),
}

impl SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    pub fn paying_account(&self) -> ApplicationInputPayingAccount {
        match self {
            Self::Account(a) => a.paying_account(),
            Self::Persona(p) => p.paying_account(),
        }
    }
}

pub type SecurityShieldApplicationTransactionIntentsForSecurifiedAccount =
    SecurityShieldApplicationTransactionIntentsForSecurifiedEntityWithPayingAccount<Account>;

pub type SecurityShieldApplicationTransactionIntentsForSecurifiedPersona =
SecurityShieldApplicationTransactionIntentsForSecurifiedEntityWithPayingAccount<Persona>;

pub type SecurityShieldApplicationTransactionIntentsForSecurifiedEntityWithPayingAccount<
    E,
> = AbstractSecurityShieldApplicationForSecurifiedEntityWithIntent<
    SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<E>,
>;

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone> SecurityShieldApplicationTransactionIntentsForSecurifiedEntityWithPayingAccount<E> {
    pub fn paying_account(&self) -> ApplicationInputPayingAccount {
        self.entity.account_topping_up_xrd_vault_of_access_controller.clone()
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
                Self::Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount::new(a.entity, initiate_with_recovery_complete_with_primary, initiate_with_recovery_complete_with_confirmation, initiate_with_recovery_delayed_completion, initiate_with_primary_complete_with_confirmation, initiate_with_primary_delayed_completion))
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                Self::Persona(SecurityShieldApplicationTransactionIntentsForSecurifiedPersona::new(p.entity, initiate_with_recovery_complete_with_primary, initiate_with_recovery_complete_with_confirmation, initiate_with_recovery_delayed_completion, initiate_with_primary_complete_with_confirmation, initiate_with_primary_delayed_completion))
            }
        }
    }
}
