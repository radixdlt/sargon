use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum AccountWithdraw {
    Amount {
        resource_address: ResourceAddress,
        amount: Decimal,
    },
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
        ids: Vec<NonFungibleLocalId>,
    ) -> Self {
        Self::Ids {
            resource_address: resource_address.into(),
            ids,
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
                ids.into_iter().map(NonFungibleLocalId::from).collect(),
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
