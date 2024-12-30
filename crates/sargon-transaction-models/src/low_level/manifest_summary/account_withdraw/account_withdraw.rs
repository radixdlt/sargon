use crate::prelude::*;

/// Represents a withdrawal from an account, either by amount or by specific IDs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountWithdraw {
    /// Withdraw a specific amount from the account.
    Amount {
        resource_address: ResourceAddress,
        amount: Decimal,
    },

    /// Withdraw specific IDs from the account.
    Ids {
        resource_address: ResourceAddress,
        ids: Vec<NonFungibleLocalId>,
    },
}

impl AccountWithdraw {
    pub fn get_address(&self) -> ResourceAddress {
        match self {
            AccountWithdraw::Amount {
                resource_address,
                amount: _,
            } => *resource_address,
            AccountWithdraw::Ids {
                resource_address,
                ids: _,
            } => *resource_address,
        }
    }
}

impl AccountWithdraw {
    pub fn amount(
        resource_address: impl Into<ResourceAddress>,
        amount: impl Into<Decimal>,
    ) -> Self {
        Self::Amount {
            resource_address: resource_address.into(),
            amount: amount.into(),
        }
    }

    pub fn ids(
        resource_address: impl Into<ResourceAddress>,
        ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self::Ids {
            resource_address: resource_address.into(),
            ids: ids.into_iter().collect(),
        }
    }
}

impl From<(ScryptoAccountWithdraw, NetworkID)> for AccountWithdraw {
    fn from(value: (ScryptoAccountWithdraw, NetworkID)) -> Self {
        let (scrypto_value, network_id) = value;
        match scrypto_value {
            ScryptoAccountWithdraw::Amount(resource_address, amount) => {
                Self::amount((resource_address, network_id), amount)
            }
            ScryptoAccountWithdraw::Ids(resource_address, ids) => Self::ids(
                (resource_address, network_id),
                ids.into_iter().map(NonFungibleLocalId::from),
            ),
        }
    }
}

impl From<AccountWithdraw> for ScryptoAccountWithdraw {
    fn from(value: AccountWithdraw) -> Self {
        match value {
            AccountWithdraw::Amount {
                resource_address,
                amount,
            } => ScryptoAccountWithdraw::Amount(
                resource_address.into(),
                amount.into(),
            ),
            AccountWithdraw::Ids {
                resource_address,
                ids,
            } => ScryptoAccountWithdraw::Ids(
                resource_address.into(),
                ids.into_iter()
                    .map(ScryptoNonFungibleLocalId::from)
                    .collect::<IndexSet<_>>(),
            ),
        }
    }
}

impl HasSampleValues for AccountWithdraw {
    fn sample() -> Self {
        Self::amount(ResourceAddress::sample_sim_xrd(), 330)
    }

    fn sample_other() -> Self {
        Self::ids(
            ResourceAddress::sample_other(),
            vec![NonFungibleLocalId::sample_other()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountWithdraw;

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
    fn get_address_amount() {
        let resource_address = ResourceAddress::sample_sim_xrd();
        let withdraw = SUT::amount(resource_address, Decimal::from(100));
        assert_eq!(withdraw.get_address(), resource_address);
    }

    #[test]
    fn get_address_ids() {
        let resource_address = ResourceAddress::sample_sim_xrd();
        let withdraw =
            SUT::ids(resource_address, vec![NonFungibleLocalId::sample()]);
        assert_eq!(withdraw.get_address(), resource_address);
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |sut: SUT, network_id: NetworkID| {
            SUT::from((ScryptoAccountWithdraw::from(sut.clone()), network_id))
        };
        roundtrip(SUT::sample(), NetworkID::Simulator);
        roundtrip(SUT::sample_other(), NetworkID::Mainnet);
    }
}
