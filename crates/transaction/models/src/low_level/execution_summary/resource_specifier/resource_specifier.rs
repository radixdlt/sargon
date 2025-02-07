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

impl TryFrom<(RetManifestResourceSpecifier, NetworkID)> for ResourceSpecifier {
    type Error = CommonError;

    fn try_from(
        (scrypto_value, n): (RetManifestResourceSpecifier, NetworkID),
    ) -> Result<Self, Self::Error> {
        match scrypto_value {
            RetManifestResourceSpecifier::Amount(resource_address, amount) => {
                Ok(Self::fungible(
                    ResourceAddress::try_from((resource_address, n))?,
                    amount,
                ))
            }
            RetManifestResourceSpecifier::Ids(resource_address, ids) => {
                Ok(Self::non_fungible(
                    ResourceAddress::try_from((resource_address, n))?,
                    ids.into_iter().map(NonFungibleLocalId::from).collect(),
                ))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceSpecifier;

    #[test]
    fn get_address() {
        let address = ResourceAddress::sample();
        let ids = vec![NonFungibleLocalId::sample()];
        let sut = SUT::non_fungible(address, ids.clone());

        assert_eq!(sut.get_address(), address);
    }

    #[test]
    fn try_from_manifest_resource_specifier_ids() {
        let address = ResourceAddress::sample();
        let ids = vec![NonFungibleLocalId::sample()];
        let sut = SUT::non_fungible(address, ids.clone());

        let ret = RetManifestResourceSpecifier::Ids(
            ScryptoManifestResourceAddress::Static(
                ScryptoResourceAddress::try_from(address.scrypto()).unwrap(),
            ),
            ids.into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect(),
        );

        let result = SUT::try_from((ret, NetworkID::Mainnet));
        assert_eq!(result.unwrap(), sut);
    }

    #[test]
    fn try_from_manifest_resource_specifier_amount() {
        let address = ResourceAddress::sample();
        let amount = 3.into();
        let sut = SUT::fungible(address, amount);

        let ret = RetManifestResourceSpecifier::Amount(
            ScryptoManifestResourceAddress::Static(
                ScryptoResourceAddress::try_from(address.scrypto()).unwrap(),
            ),
            amount,
        );

        let result = SUT::try_from((ret, NetworkID::Mainnet));
        assert_eq!(result.unwrap(), sut);
    }
}
