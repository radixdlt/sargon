use crate::prelude::*;

/// A struct detailing a transfer for a given resource belonging to an account to be deleted.
/// Since every resource in the account is going to be deleted, the amount is the total amount
/// of the resource in the account (and we don't need to specify ids for non-fungibles).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeleteAccountTransfer {
    /// The address of the resource to transfer.
    pub resource_address: ScryptoGlobalAddress,

    /// The amount to transfer.
    pub amount: ScryptoDecimal192,

    /// The weight of this transfer in a transaction, so that we don't exceed the maximum.
    /// The weight of a fungible transfer is 1, regardless of the amount, while the weight of a
    /// non_fungible transfer is the amount of items.
    pub weight: u64,
}

impl DeleteAccountTransfer {
    fn new(
        resource_address: ScryptoGlobalAddress,
        amount: ScryptoDecimal192,
        weight: u64,
    ) -> DeleteAccountTransfer {
        DeleteAccountTransfer {
            resource_address,
            amount,
            weight,
        }
    }
}

impl TryFrom<FungibleResourcesCollectionItem> for DeleteAccountTransfer {
    type Error = CommonError;
    fn try_from(value: FungibleResourcesCollectionItem) -> Result<Self> {
        let value = value
            .as_global()
            .ok_or(CommonError::UnexpectedCollectionItemAggregation)?;
        let result =
            Self::new(value.resource_address.scrypto(), value.amount.into(), 1);
        Ok(result)
    }
}

impl TryFrom<NonFungibleResourcesCollectionItem> for DeleteAccountTransfer {
    type Error = CommonError;
    fn try_from(value: NonFungibleResourcesCollectionItem) -> Result<Self> {
        let value = value
            .as_global()
            .ok_or(CommonError::UnexpectedCollectionItemAggregation)?;
        let result = Self::new(
            value.resource_address.scrypto(),
            value.amount.into(),
            value.amount,
        );
        Ok(result)
    }
}

impl HasSampleValues for DeleteAccountTransfer {
    fn sample() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_xrd().into(),
            Decimal192::sample().into(),
            1,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_nft_abandon().into(),
            Decimal192::five().into(),
            5,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeleteAccountTransfer;

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
    fn from_fungible_resources_collection_item() {
        let resource_address = ResourceAddress::sample();
        let amount = Decimal192::sample();
        let item = FungibleResourcesCollectionItem::Global(
            FungibleResourcesCollectionItemGloballyAggregated::new(
                resource_address,
                amount,
            ),
        );
        let result = DeleteAccountTransfer::try_from(item).unwrap();
        assert_eq!(result.resource_address, resource_address.scrypto());
        assert_eq!(result.amount, amount.into());
        assert_eq!(result.weight, 1);

        // TODO: Add test that returns `UnexpectedCollectionItemAggregation` whenever we support `FungibleResourcesCollectionItem::Vault`
    }

    #[test]
    fn from_non_fungible_resources_collection_item() {
        let resource_address = ResourceAddress::sample();
        let amount = 5;
        let item = NonFungibleResourcesCollectionItem::Global(
            NonFungibleResourcesCollectionItemGloballyAggregated::new(
                resource_address,
                amount,
            ),
        );
        let result = DeleteAccountTransfer::try_from(item).unwrap();
        assert_eq!(result.resource_address, resource_address.scrypto());
        assert_eq!(result.amount, amount.into());
        assert_eq!(result.weight, 5);

        // TODO: Add test that returns `UnexpectedCollectionItemAggregation` whenever we support `NonFungibleResourcesCollectionItem::Vault`
    }
}
