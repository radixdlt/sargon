use crate::prelude::*;

/// A struct detailing the transfers for a given account to be deleted.
#[derive(Debug, PartialEq, Eq)]
pub struct DeleteAccountTransfers {
    pub recipient: AccountAddress,
    pub transfers: Vec<DeleteAccountTransfer>,
}

impl DeleteAccountTransfers {
    pub fn new(
        recipient: AccountAddress,
        transfers: Vec<DeleteAccountTransfer>,
    ) -> DeleteAccountTransfers {
        DeleteAccountTransfers {
            recipient,
            transfers,
        }
    }
}

impl TryFrom<(FetchResourcesOutput, AccountAddress)>
    for DeleteAccountTransfers
{
    type Error = CommonError;
    fn try_from(value: (FetchResourcesOutput, AccountAddress)) -> Result<Self> {
        let (fetch_resources_output, recipient) = value;

        // Convert fungibles
        let fungibles = fetch_resources_output
            .fungibles
            .clone()
            .into_iter()
            .map(DeleteAccountTransfer::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        // Convert non-fungibles
        let non_fungibles = fetch_resources_output
            .non_fungibles
            .into_iter()
            .map(DeleteAccountTransfer::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        // Merge in one collection
        let transfers = [fungibles, non_fungibles].concat();

        // Verify we don't exceed the maximum number of transfers per transaction.
        let total_weight = transfers
            .clone()
            .into_iter()
            .fold(0, |acc, x| acc + x.weight);
        if total_weight >= MAX_TRANSFERS_PER_TRANSACTION {
            return Err(CommonError::MaxTransfersPerTransactionReached {
                amount: total_weight,
            });
        }

        Ok(Self::new(recipient, transfers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeleteAccountTransfers;

    #[test]
    fn from_fetch_resources_output_and_recipient() {
        // Test the case where the total weight of the transfers is less than the maximum.
        let fungible = FungibleResourcesCollectionItem::sample();
        let non_fungible = NonFungibleResourcesCollectionItem::sample();
        let output = FetchResourcesOutput::new(
            vec![fungible.clone()],
            vec![non_fungible.clone()],
        );
        let recipient = AccountAddress::sample();

        let result = SUT::try_from((output, recipient)).unwrap();
        assert_eq!(result.recipient, recipient);
        assert_eq!(
            result.transfers,
            vec![
                fungible.clone().try_into().unwrap(),
                non_fungible.try_into().unwrap()
            ]
        );

        // Test the case where the total weight of the transfers is over the maximum.
        let non_fungible = NonFungibleResourcesCollectionItem::Global(
            NonFungibleResourcesCollectionItemGloballyAggregated::new(
                ResourceAddress::sample(),
                50,
            ),
        );

        let output = FetchResourcesOutput::new(
            vec![fungible.clone()],
            vec![non_fungible.clone()],
        );
        let result =
            SUT::try_from((output, recipient)).expect_err("Expected error");
        assert_eq!(
            result,
            CommonError::MaxTransfersPerTransactionReached { amount: 51 }
        );
    }
}
