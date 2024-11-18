use crate::prelude::*;

pub type SecurifiedAccount = AbstractSecurifiedEntity<Account>;

impl SecurifiedAccount {
    pub fn erase_to_any(&self) -> AnySecurifiedEntity {
        AnySecurifiedEntity::new(AccountOrPersona::from(self.entity.clone()))
            .unwrap()
    }
}

impl HasEntityKind for SecurifiedAccount {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
    }
}

impl TryFrom<AccountOrPersona> for SecurifiedAccount {
    type Error = CommonError;

    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Account::try_from(value).and_then(Self::new)
    }
}

impl HasSampleValues for SecurifiedAccount {
    fn sample() -> Self {
        Self::new(Account::sample_at(2)).unwrap()
    }

    fn sample_other() -> Self {
        Self::new(Account::sample_at(3)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurifiedAccount;

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
