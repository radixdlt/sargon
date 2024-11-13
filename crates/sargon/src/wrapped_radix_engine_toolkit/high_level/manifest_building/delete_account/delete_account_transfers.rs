use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeleteAccountTransfers {
    pub recipient: AccountAddress,
    pub fungibles: Vec<FungibleResourcesCollectionItemGloballyAggregated>,
    pub non_fungibles:
        Vec<NonFungibleResourcesCollectionItemGloballyAggregated>,
}

impl DeleteAccountTransfers {
    pub fn new(
        recipient: AccountAddress,
        fungibles: Vec<FungibleResourcesCollectionItemGloballyAggregated>,
        non_fungibles: Vec<
            NonFungibleResourcesCollectionItemGloballyAggregated,
        >,
    ) -> DeleteAccountTransfers {
        DeleteAccountTransfers {
            recipient,
            fungibles,
            non_fungibles,
        }
    }
}

impl TryFrom<(FetchResourcesOutput, AccountAddress)>
    for DeleteAccountTransfers
{
    type Error = CommonError;
    fn try_from(value: (FetchResourcesOutput, AccountAddress)) -> Result<Self> {
        let (fetch_resources_output, recipient) = value;

        let fungibles: Vec<FungibleResourcesCollectionItemGloballyAggregated> =
            fetch_resources_output
                .fungibles
                .into_iter()
                .map(|item| {
                    item.as_global().cloned().ok_or(CommonError::EntityNotFound)
                })
                .collect::<Result<_, _>>()?;

        let non_fungibles: Vec<
            NonFungibleResourcesCollectionItemGloballyAggregated,
        > = fetch_resources_output
            .non_fungibles
            .into_iter()
            .map(|item| {
                item.as_global().cloned().ok_or(CommonError::EntityNotFound)
            })
            .collect::<Result<_, _>>()?;

        Ok(Self::new(recipient, fungibles, non_fungibles))
    }
}
