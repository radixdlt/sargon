use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShieldApplicationInput {
    Unsecurified(ApplicationInputForUnsecurifiedEntity),
    Securified(ApplicationInputForSecurifiedEntity),
}

impl From<ApplicationInputForUnsecurifiedEntity> for ShieldApplicationInput {
    fn from(value: ApplicationInputForUnsecurifiedEntity) -> Self {
        Self::Unsecurified(value)
    }
}

impl From<ApplicationInputForSecurifiedEntity> for ShieldApplicationInput {
    fn from(value: ApplicationInputForSecurifiedEntity) -> Self {
        Self::Securified(value)
    }
}

impl From<ApplicationInputForUnsecurifiedAccount> for ShieldApplicationInput {
    fn from(value: ApplicationInputForUnsecurifiedAccount) -> Self {
        ApplicationInputForUnsecurifiedEntity::from(value).into()
    }
}
impl From<ApplicationInputForUnsecurifiedPersona> for ShieldApplicationInput {
    fn from(value: ApplicationInputForUnsecurifiedPersona) -> Self {
        ApplicationInputForUnsecurifiedEntity::from(value).into()
    }
}

impl From<ApplicationInputForSecurifiedAccount> for ShieldApplicationInput {
    fn from(value: ApplicationInputForSecurifiedAccount) -> Self {
        ApplicationInputForSecurifiedEntity::from(value).into()
    }
}
impl From<ApplicationInputForSecurifiedPersona> for ShieldApplicationInput {
    fn from(value: ApplicationInputForSecurifiedPersona) -> Self {
        ApplicationInputForSecurifiedEntity::from(value).into()
    }
}
