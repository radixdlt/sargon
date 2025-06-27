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

impl From<SecurifiedPersona> for Persona {
    fn from(value: SecurifiedPersona) -> Self {
        value.entity
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

impl TryFrom<Persona> for SecurifiedPersona {
    type Error = CommonError;

    fn try_from(value: Persona) -> Result<Self> {
        Self::new(value)
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

impl HasSampleValues for SecurifiedPersona {
    fn sample() -> Self {
        Self::new(Persona::sample_at(2)).unwrap()
    }

    fn sample_other() -> Self {
        Self::new(Persona::sample_at(3)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurifiedPersona;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
