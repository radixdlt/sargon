use crate::prelude::*;

pub type AnySecurifiedEntity = AbstractSecurifiedEntity<AccountOrPersona>;

impl TryFrom<AccountOrPersona> for AnySecurifiedEntity {
    type Error = CommonError;

    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Self::new(value)
    }
}

impl AnySecurifiedEntity {
    pub fn commit_provisional(&mut self) -> Result<()> {
        self.securified_entity_control.commit_provisional()?;
        self.entity.set_security_state(EntitySecurityState::Securified { value: self.securified_entity_control.clone() })
    }
}

impl AnySecurifiedEntity {
    pub fn sample_account() -> Self {
        SecurifiedAccount::sample().into()
    }

    pub fn sample_account_other() -> Self {
        SecurifiedAccount::sample_other().into()
    }

    pub fn sample_persona() -> Self {
        SecurifiedPersona::sample().into()
    }

    pub fn sample_persona_other() -> Self {
        SecurifiedPersona::sample_other().into()
    }
}

impl HasSampleValues for AnySecurifiedEntity {
    fn sample() -> Self {
        Self::sample_account()
    }

    fn sample_other() -> Self {
        Self::sample_persona()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AnySecurifiedEntity;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample_account(),
                SUT::sample_account_other(),
                SUT::sample_persona(),
                SUT::sample_persona_other(),
                // Duplicates should be removed
                SUT::sample_account(),
                SUT::sample_account_other(),
                SUT::sample_persona(),
                SUT::sample_persona_other(),
            ])
            .len(),
            4
        );
    }
}
