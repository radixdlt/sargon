use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceSpecifier {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal,
    },
    NonFungible {
        resource_address: ResourceAddress,
        ids: Vec<NonFungibleLocalId>,
    },
}

impl ResourceSpecifier {
    pub fn get_address(&self) -> ResourceAddress {
        match self {
            ResourceSpecifier::Fungible {
                resource_address,
                amount: _,
            } => *resource_address,
            ResourceSpecifier::NonFungible {
                resource_address,
                ids: _,
            } => *resource_address,
        }
    }
}

impl ResourceSpecifier {
    pub fn fungible(
        resource_address: impl Into<ResourceAddress>,
        amount: impl Into<Decimal>,
    ) -> Self {
        Self::Fungible {
            resource_address: resource_address.into(),
            amount: amount.into(),
        }
    }

    pub fn non_fungible(
        resource_address: impl Into<ResourceAddress>,
        ids: Vec<NonFungibleLocalId>,
    ) -> Self {
        Self::NonFungible {
            resource_address: resource_address.into(),
            ids,
        }
    }
}

impl From<(ScryptoResourceSpecifier, NetworkID)> for ResourceSpecifier {
    fn from(value: (ScryptoResourceSpecifier, NetworkID)) -> Self {
        let (scrypto_value, network_id) = value;
        match scrypto_value {
            ScryptoResourceSpecifier::Amount(resource_address, amount) => {
                Self::fungible((resource_address, network_id), amount)
            }
            ScryptoResourceSpecifier::Ids(resource_address, ids) => {
                Self::non_fungible(
                    (resource_address, network_id),
                    ids.into_iter().map(NonFungibleLocalId::from).collect(),
                )
            }
        }
    }
}

impl HasSampleValues for ResourceSpecifier {
    fn sample() -> Self {
        Self::fungible(ResourceAddress::sample(), 3)
    }

    fn sample_other() -> Self {
        Self::non_fungible(
            ResourceAddress::sample_other(),
            vec![NonFungibleLocalId::sample_other()],
        )
    }
}
