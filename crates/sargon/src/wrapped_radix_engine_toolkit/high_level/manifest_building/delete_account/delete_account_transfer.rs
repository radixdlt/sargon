use crate::prelude::*;

/// A struct detailing a transfer for a given resource belonging to an account to be deleted.
/// Since every resource in the account is going to be deleted, the amount is the total amount
/// of the resource in the account (and we don't need to specify ids for non-fungibles).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeleteAccountTransfer {
    pub resource_address: ScryptoGlobalAddress,
    pub amount: ScryptoDecimal192,
}

impl DeleteAccountTransfer {
    fn new(
        resource_address: ScryptoGlobalAddress,
        amount: ScryptoDecimal192,
    ) -> DeleteAccountTransfer {
        DeleteAccountTransfer {
            resource_address,
            amount,
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
            Self::new(value.resource_address.scrypto(), value.amount.into());
        Ok(result)
    }
}

impl TryFrom<NonFungibleResourcesCollectionItem> for DeleteAccountTransfer {
    type Error = CommonError;
    fn try_from(value: NonFungibleResourcesCollectionItem) -> Result<Self> {
        let value = value
            .as_global()
            .ok_or(CommonError::UnexpectedCollectionItemAggregation)?;
        let result =
            Self::new(value.resource_address.scrypto(), value.amount.into());
        Ok(result)
    }
}

impl HasSampleValues for DeleteAccountTransfer {
    fn sample() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_xrd().into(),
            Decimal192::sample().into(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_nft_abandon().into(),
            Decimal192::five().into(),
        )
    }
}
