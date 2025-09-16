use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum TransferRecipient {
    ProfileAccount { value: AccountForDisplay },
    AddressOfExternalAccount { value: AccountAddress },
    RnsDomain { value: RnsDomainConfiguredReceiver },
}

impl TransferRecipient {
    pub fn account_address(&self) -> &AccountAddress {
        match self {
            TransferRecipient::ProfileAccount { value } => &value.address,
            TransferRecipient::AddressOfExternalAccount { value } => value,
            TransferRecipient::RnsDomain { value } => &value.receiver,
        }
    }
}

impl TransferRecipient {
    pub(crate) fn sample_mainnet() -> Self {
        Self::ProfileAccount {
            value: AccountForDisplay::new(AccountAddress::from_str("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7").unwrap(), 
            DisplayName::sample(), AppearanceID::sample()
        ),
        }
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::AddressOfExternalAccount {
            value: AccountAddress::sample_mainnet_other(),
        }
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::ProfileAccount {
            value: AccountForDisplay::new(AccountAddress::from_str("account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m").unwrap(), DisplayName::sample(), AppearanceID::sample())
        }
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::AddressOfExternalAccount {
            value: AccountAddress::sample_stokenet_other(),
        }
    }
}

impl HasSampleValues for TransferRecipient {
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
    type SUT = TransferRecipient;

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

    // #[test]
    // fn from_account() {
    //     let acc = Account::sample();
    //     let exp = &acc.clone().address;
    //     assert_eq!(SUT::from(acc).account_address(), exp)
    // }

    // #[test]
    // fn from_address() {
    //     let exp = &AccountAddress::sample();
    //     assert_eq!(SUT::from(*exp).account_address(), exp)
    // }
}
