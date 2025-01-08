use crate::prelude::*;

pub type SecurifiedPersona = AbstractSecurifiedEntity<Persona>;

impl SecurifiedPersona {
    pub fn erase_to_any(&self) -> AnySecurifiedEntity {
        AnySecurifiedEntity::new(AccountOrPersona::from(self.entity.clone()))
            .unwrap()
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
