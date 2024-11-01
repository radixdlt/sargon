use crate::prelude::*;

/// Represents the bounds for a simple resource, which can be either fungible or non-fungible.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimpleResourceBounds {
    Fungible {
        resource_address: ResourceAddress,
        bounds: SimpleCountedResourceBounds,
    },
    NonFungible {
        resource_address: ResourceAddress,
        bounds: SimpleNonFungibleResourceBounds,
    },
}

impl SimpleResourceBounds {
    pub fn fungible(
        resource_address: impl Into<ResourceAddress>,
        bounds: SimpleCountedResourceBounds,
    ) -> Self {
        Self::Fungible {
            resource_address: resource_address.into(),
            bounds,
        }
    }

    pub fn non_fungible(
        resource_address: impl Into<ResourceAddress>,
        bounds: SimpleNonFungibleResourceBounds,
    ) -> Self {
        Self::NonFungible {
            resource_address: resource_address.into(),
            bounds,
        }
    }

    pub fn exact_fungible(
        resource_address: ResourceAddress,
        amount: impl Into<Decimal>,
    ) -> Self {
        Self::fungible(
            resource_address,
            SimpleCountedResourceBounds::exact(amount.into()),
        )
    }
}

impl From<(ResourceAddress, ScryptoSimpleResourceBounds)>
    for SimpleResourceBounds
{
    fn from(value: (ResourceAddress, ScryptoSimpleResourceBounds)) -> Self {
        let (resource_address, bounds) = value;
        match bounds {
            ScryptoSimpleResourceBounds::Fungible(bounds) => {
                Self::fungible(resource_address, bounds.into())
            }
            ScryptoSimpleResourceBounds::NonFungible(bounds) => {
                Self::non_fungible(resource_address, bounds.into())
            }
        }
    }
}

impl HasSampleValues for SimpleResourceBounds {
    fn sample() -> Self {
        Self::fungible(
            ResourceAddress::sample(),
            SimpleCountedResourceBounds::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::non_fungible(
            ResourceAddress::sample_other(),
            SimpleNonFungibleResourceBounds::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SimpleResourceBounds;

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
    fn from_scrypto_fungible() {
        let scrypto = ScryptoSimpleResourceBounds::Fungible(
            ScryptoSimpleFungibleResourceBounds::Exact(
                Decimal::from(1337).into(),
            ),
        );
        assert_eq!(
            SUT::from((ResourceAddress::sample(), scrypto)),
            SUT::sample()
        );
    }

    #[test]
    fn from_scrypto_non_fungible() {
        let scrypto = ScryptoSimpleResourceBounds::NonFungible(
            ScryptoSimpleNonFungibleResourceBounds::Exact {
                amount: Decimal::from(150).into(),
                certain_ids: vec![
                    NonFungibleLocalId::sample(),
                    NonFungibleLocalId::sample_other(),
                ]
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect(),
            },
        );
        assert_eq!(
            SUT::from((ResourceAddress::sample_other(), scrypto)),
            SUT::sample_other()
        );
    }
}
