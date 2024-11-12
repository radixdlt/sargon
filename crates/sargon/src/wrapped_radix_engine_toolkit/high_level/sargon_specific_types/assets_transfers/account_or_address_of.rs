use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum AccountOrAddressOf {
    ProfileAccount { value: Account },
    AddressOfExternalAccount { value: AccountAddress },
}

impl From<Account> for AccountOrAddressOf {
    fn from(value: Account) -> Self {
        Self::ProfileAccount { value }
    }
}

impl From<AccountAddress> for AccountOrAddressOf {
    fn from(value: AccountAddress) -> Self {
        Self::AddressOfExternalAccount { value }
    }
}

impl AccountOrAddressOf {
    pub fn account_address(&self) -> &AccountAddress {
        match self {
            AccountOrAddressOf::ProfileAccount { value } => &value.address,
            AccountOrAddressOf::AddressOfExternalAccount { value } => value,
        }
    }
}

impl AccountOrAddressOf {
    pub(crate) fn sample_mainnet() -> Self {
        Self::ProfileAccount {
            value: Account::sample_mainnet_bob(),
        }
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::AddressOfExternalAccount {
            value: AccountAddress::sample_mainnet_other(),
        }
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::ProfileAccount {
            value: Account::sample_stokenet_nadia(),
        }
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::AddressOfExternalAccount {
            value: AccountAddress::sample_stokenet_other(),
        }
    }
}

impl HasSampleValues for AccountOrAddressOf {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountOrAddressOf;

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
                SUT::sample_mainnet(),
                SUT::sample_mainnet_other(),
                SUT::sample_stokenet(),
                SUT::sample_stokenet_other(),
                // duplicates should be removed
                SUT::sample_mainnet(),
                SUT::sample_mainnet_other(),
                SUT::sample_stokenet(),
                SUT::sample_stokenet_other(),
            ])
            .len(),
            4
        )
    }

    #[test]
    fn from_account() {
        let acc = Account::sample();
        let exp = &acc.clone().address;
        assert_eq!(SUT::from(acc).account_address(), exp)
    }

    #[test]
    fn from_address() {
        let exp = &AccountAddress::sample();
        assert_eq!(SUT::from(*exp).account_address(), exp)
    }
}
