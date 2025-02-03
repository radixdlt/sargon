use crate::prelude::*;


// ========================
// SECURIFIED
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputForSecurifiedEntity {
    Account(ApplicationInputForSecurifiedAccount),
    Persona(ApplicationInputForSecurifiedPersona),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForSecurifiedAccount {
    pub entity_input: SecurifiedAccountEntityInput,
    pub maybe_paying_account: Option<ApplicationInputPayingAccount>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForSecurifiedPersona {
    pub entity_input: SecurifiedPersonaEntityInput,
    pub maybe_paying_account: Option<ApplicationInputPayingAccount>,
}


// ========================
// ENTITY INPUT
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurifiedAccountEntityInput {
    /// The entity applying the shield
    pub securifed_entity: SecurifiedAccount,

    /// XRD balance of the AccessControllers of `securifed_entity`'s XRD Vault
    pub xrd_balance_of_access_controller: Decimal,

    /// XRD balance of the entity applying the shield
    pub xrd_balance_of_account: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurifiedPersonaEntityInput {
    /// The entity applying the shield
    pub securifed_entity: SecurifiedPersona,

    /// XRD balance of the AccessControllers of `securifed_entity`'s XRD Vault
    pub xrd_balance_of_access_controller: Decimal,
}
