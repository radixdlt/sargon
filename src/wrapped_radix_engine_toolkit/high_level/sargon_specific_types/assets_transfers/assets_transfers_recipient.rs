use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
#[allow(clippy::large_enum_variant)] // we cannot Box<Account>, since Box is not UniFFI compatible.
pub enum AssetsTransfersRecipient {
    MyOwnAccount { value: Account },
    ForeignAccount { value: AccountAddress },
}

impl From<Account> for AssetsTransfersRecipient {
    fn from(value: Account) -> Self {
        Self::MyOwnAccount { value }
    }
}

impl From<AccountAddress> for AssetsTransfersRecipient {
    fn from(value: AccountAddress) -> Self {
        Self::ForeignAccount { value }
    }
}

impl AssetsTransfersRecipient {
    pub fn account_address(&self) -> &AccountAddress {
        match self {
            AssetsTransfersRecipient::MyOwnAccount { value } => &value.address,
            AssetsTransfersRecipient::ForeignAccount { value } => value,
        }
    }
}

impl AssetsTransfersRecipient {
    pub(crate) fn sample_mainnet() -> Self {
        Self::MyOwnAccount {
            value: Account::sample_mainnet_bob(),
        }
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::ForeignAccount {
            value: AccountAddress::sample_mainnet_other(),
        }
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::MyOwnAccount {
            value: Account::sample_stokenet_nadia(),
        }
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::ForeignAccount {
            value: AccountAddress::sample_stokenet_other(),
        }
    }
}

impl HasSampleValues for AssetsTransfersRecipient {
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
    type SUT = AssetsTransfersRecipient;

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
