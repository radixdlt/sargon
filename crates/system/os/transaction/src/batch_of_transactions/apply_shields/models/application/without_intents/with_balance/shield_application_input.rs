use crate::prelude::*;

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShieldApplicationInput {
    Unsecurified(ApplicationInputForUnsecurifiedEntity),
    Securified(ApplicationInputForSecurifiedEntity),
}

impl ShieldApplicationInput {
    pub fn fee_tip_percentage(&self) -> Option<u16> {
        match self {
            Self::Unsecurified(a) => a.fee_tip_percentage(),
            Self::Securified(a) => a.fee_tip_percentage(),
        }
    }
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
