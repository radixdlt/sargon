use crate::prelude::*;


// ========================
// UNSECURIFIED
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputForUnsecurifiedEntity {
    Account(ApplicationInputForUnsecurifiedAccount),
    Persona(ApplicationInputForUnsecurifiedPersona),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForUnsecurifiedAccount {
    pub entity_input: UnsecurifiedAccountEntityInput,
    pub maybe_paying_account: Option<ApplicationInputPayingAccount>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForUnsecurifiedPersona {
    pub entity_input: UnsecurifiedPersona,
    pub maybe_paying_account: ApplicationInputPayingAccount,
}


// ========================
// ENTITY INPUT
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsecurifiedAccountEntityInput {
    /// The entity applying the shield
    pub unsecurified_entity: UnsecurifiedAccount,

    /// XRD balance of the entity applying the shield
    pub xrd_balance_of_account: Decimal,
}
