use crate::prelude::*;

/// A trait bridging Account and Persona.
pub trait IsEntity:
    IsEntityWithoutConcreteTypes
    + TryFrom<AccountOrPersona, Error = CommonError>
    + Into<AccountOrPersona>
{
}

impl IsEntity for Account {}
impl IsEntity for Persona {}
