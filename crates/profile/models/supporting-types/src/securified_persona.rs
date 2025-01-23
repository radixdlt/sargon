use crate::prelude::*;

pub type SecurifiedPersona = AbstractSecurifiedEntity<Persona>;

impl SecurifiedPersona {
    pub fn erase_to_any(&self) -> AnySecurifiedEntity {
        AnySecurifiedEntity::new(AccountOrPersona::from(self.entity.clone()))
            .unwrap()
    }
}

impl From<SecurifiedPersona> for AnySecurifiedEntity {
    fn from(value: SecurifiedPersona) -> Self {
        value.erase_to_any()
    }
}

impl HasEntityKind for SecurifiedPersona {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
    }
}

impl TryFrom<AccountOrPersona> for SecurifiedPersona {
    type Error = CommonError;

    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Persona::try_from(value).and_then(Self::new)
    }
}

impl TryFrom<AnySecurifiedEntity> for SecurifiedPersona {
    type Error = CommonError;

    fn try_from(value: AnySecurifiedEntity) -> Result<Self> {
        match value.entity {
            AccountOrPersona::PersonaEntity(persona) => Self::new(persona),
            AccountOrPersona::AccountEntity(_) => {
                Err(CommonError::ExpectedPersonaButGotAccount {
                    address: value.entity.address().to_string(),
                })
            }
        }
    }
}
