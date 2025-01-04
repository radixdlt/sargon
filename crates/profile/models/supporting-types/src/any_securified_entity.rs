use crate::prelude::*;

pub type AnySecurifiedEntity = AbstractSecurifiedEntity<AccountOrPersona>;

impl TryFrom<AccountOrPersona> for AnySecurifiedEntity {
    type Error = CommonError;

    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Self::new(value)
    }
}
