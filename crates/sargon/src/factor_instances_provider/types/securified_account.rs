use crate::prelude::*;

/// The `SecuredEntityControl`, address and possibly third party deposit state of some
/// Securified entity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecurifiedAccount {
    display_name: DisplayName,
    /// The address which is verified to match the `veci`
    account_address: AccountAddress,
    securified_entity_control: SecuredEntityControl,
}
impl HasEntityKind for SecurifiedAccount {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
    }
}
impl IsSecurifiedEntity for SecurifiedAccount {
    type BaseEntity = Account;
    fn securified_entity_control(&self) -> SecuredEntityControl {
        self.securified_entity_control.clone()
    }

    fn new(
        name: impl Into<DisplayName>,
        address: AccountAddress,
        securified_entity_control: SecuredEntityControl,
    ) -> Self {
        Self {
            display_name: name.into(),
            account_address: address,
            securified_entity_control,
        }
    }
}

impl IsNetworkAware for SecurifiedAccount {
    fn network_id(&self) -> NetworkID {
        self.account_address.network_id()
    }
}

impl TryFrom<Account> for SecurifiedAccount {
    type Error = CommonError;
    fn try_from(value: Account) -> Result<Self> {
        let securified_entity_control =
            value.security_state.as_securified().cloned().ok_or(
                CommonError::AccountNotSecurified {
                    address: value.address.to_string(),
                },
            )?;
        Ok(SecurifiedAccount::new(
            value.display_name.clone(),
            value.address.clone(),
            securified_entity_control,
        ))
    }
}

impl TryFrom<AccountOrPersona> for SecurifiedAccount {
    type Error = CommonError;
    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Account::try_from(value).and_then(SecurifiedAccount::try_from)
    }
}
impl SecurifiedAccount {
    pub fn address(&self) -> AccountAddress {
        self.account_address.clone()
    }
    pub fn securified_entity_control(&self) -> SecuredEntityControl {
        self.securified_entity_control.clone()
    }
}

impl HasSampleValues for SecurifiedAccount {
    fn sample() -> Self {
        // Self::new(
        //     "SecurifiedAccount",
        //     AccountAddress::sample(),
        //     SecuredEntityControl::sample(),
        //     None,
        // )
        todo!()
    }
    fn sample_other() -> Self {
        // Self::new(
        //     "SecurifiedAccount Other",
        //     AccountAddress::sample_other(),
        //     SecuredEntityControl::sample_other(),
        //     None,
        // )
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = SecurifiedAccount;
    // #[test]
    // fn equality() {
    //     assert_eq!(Sut::sample(), Sut::sample());
    //     assert_eq!(Sut::sample_other(), Sut::sample_other());
    // }
    // #[test]
    // fn inequality() {
    //     assert_ne!(Sut::sample(), Sut::sample_other());
    // }
    // #[test]
    // fn third_party_dep() {
    //     let test = |dep: DepositRule| {
    //         let sut = Sut::new(
    //             "name",
    //             AccountAddress::sample_0(),
    //             SecuredEntityControl::sample(),
    //             dep,
    //         );
    //         assert_eq!(sut.third_party_deposit(), Some(dep));
    //     };
    //     test(DepositRule::DenyAll);
    //     test(DepositRule::AllowAll);
    //     test(DepositRule::AllowKnown);
    // }
}
