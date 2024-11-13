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
