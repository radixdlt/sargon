use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount),
    Persona(SecurityShieldApplicationTransactionIntentsForSecurifiedPersona),
}

pub type AbstractSecurityShieldApplicationTransactionIntentsForSecurifiedEntity<
    Entity,
> = AbstractSecurityShieldApplicationForSecurifiedEntityWithIntent<
    SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<Entity>,
>;

pub type SecurityShieldApplicationTransactionIntentsForSecurifiedAccount =
    AbstractSecurityShieldApplicationTransactionIntentsForSecurifiedEntity<
        Account,
    >;

pub type SecurityShieldApplicationTransactionIntentsForSecurifiedPersona =
    AbstractSecurityShieldApplicationTransactionIntentsForSecurifiedEntity<
        Persona,
    >;

impl<Entity: IsEntity>
    AbstractSecurityShieldApplicationTransactionIntentsForSecurifiedEntity<
        Entity,
    >
{
    pub fn paying_account(&self) -> ApplicationInputPayingAccount {
        self.entity
            .account_topping_up_xrd_vault_of_access_controller
            .clone()
    }

    pub fn entity_applying_shield(&self) -> AnySecurifiedEntity {
        AnySecurifiedEntity::new(self.entity.entity.entity.clone().into())
            .expect("is Securified")
    }
}

impl SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    pub fn paying_account(&self) -> ApplicationInputPayingAccount {
        match self {
            Self::Account(a) => a.paying_account(),
            Self::Persona(p) => p.paying_account(),
        }
    }

    pub fn entity_applying_shield(&self) -> AnySecurifiedEntity {
        match self {
            Self::Account(a) => a.entity_applying_shield(),
            Self::Persona(p) => p.entity_applying_shield(),
        }
    }

    pub fn initiate_with_recovery_complete_with_primary(
        &self,
    ) -> TransactionIntent {
        match self {
            Self::Account(a) => {
                a.initiate_with_recovery_complete_with_primary()
            }
            Self::Persona(p) => {
                p.initiate_with_recovery_complete_with_primary()
            }
        }
    }
    pub fn initiate_with_recovery_complete_with_confirmation(
        &self,
    ) -> TransactionIntent {
        match self {
            Self::Account(a) => {
                a.initiate_with_recovery_complete_with_confirmation()
            }
            Self::Persona(p) => {
                p.initiate_with_recovery_complete_with_confirmation()
            }
        }
    }
    pub fn initiate_with_recovery_delayed_completion(
        &self,
    ) -> TransactionIntent {
        match self {
            Self::Account(a) => a.initiate_with_recovery_delayed_completion(),
            Self::Persona(p) => p.initiate_with_recovery_delayed_completion(),
        }
    }
    pub fn initiate_with_primary_complete_with_confirmation(
        &self,
    ) -> TransactionIntent {
        match self {
            Self::Account(a) => {
                a.initiate_with_primary_complete_with_confirmation()
            }
            Self::Persona(p) => {
                p.initiate_with_primary_complete_with_confirmation()
            }
        }
    }
    pub fn initiate_with_primary_delayed_completion(
        &self,
    ) -> TransactionIntent {
        match self {
            Self::Account(a) => a.initiate_with_primary_delayed_completion(),
            Self::Persona(p) => p.initiate_with_primary_delayed_completion(),
        }
    }

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
                Self::Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount::new(
                    a.entity,
                    initiate_with_recovery_complete_with_primary,
                    initiate_with_recovery_complete_with_confirmation,
                    initiate_with_recovery_delayed_completion,
                    initiate_with_primary_complete_with_confirmation,
                    initiate_with_primary_delayed_completion
                )
            )
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                Self::Persona(
                    SecurityShieldApplicationTransactionIntentsForSecurifiedPersona::new(
                        p.entity,
                        initiate_with_recovery_complete_with_primary,
                        initiate_with_recovery_complete_with_confirmation,
                        initiate_with_recovery_delayed_completion,
                        initiate_with_primary_complete_with_confirmation,
                        initiate_with_primary_delayed_completion
                    )
                )
            }
        }
    }
}
