use crate::prelude::*;

/// A struct detailing the transfers for a given account to be deleted.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeleteAccountTransfers {
    /// The recipient account to which the resources are going to be transferred.
    pub recipient: AccountAddress,

    /// The transfers to be made.
    pub transfers: Vec<DeleteAccountTransfer>,

    /// The resources that cannot be transferred
    pub non_transferable_resources: Vec<ResourceAddress>,
}

impl DeleteAccountTransfers {
    pub fn new(
        recipient: AccountAddress,
        transfers: Vec<DeleteAccountTransfer>,
        non_transferable_resources: Vec<ResourceAddress>,
    ) -> DeleteAccountTransfers {
        DeleteAccountTransfers {
            recipient,
            transfers,
            non_transferable_resources,
        }
    }
}

impl TryFrom<(FetchTransferableResourcesOutput, AccountAddress)>
    for DeleteAccountTransfers
{
    type Error = CommonError;
    fn try_from(
        value: (FetchTransferableResourcesOutput, AccountAddress),
    ) -> Result<Self> {
        let (output, recipient) = value;

        // Convert fungibles
        let fungibles = output
            .fungibles
            .clone()
            .into_iter()
            .map(DeleteAccountTransfer::from)
            .collect::<Vec<_>>();

        // Convert non-fungibles
        let non_fungibles = output
            .non_fungibles
            .into_iter()
            .map(DeleteAccountTransfer::from)
            .collect::<Vec<_>>();

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

        Ok(Self::new(
            recipient,
            transfers,
            output.non_transferable_resources,
        ))
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
        let fungible =
            FungibleResourcesCollectionItemGloballyAggregated::sample();
        let non_fungible =
            NonFungibleResourcesCollectionItemGloballyAggregated::sample();
        let non_transferable_resources = vec![ResourceAddress::sample()];
        let output = FetchTransferableResourcesOutput::new(
            vec![fungible.clone()],
            vec![non_fungible.clone()],
            non_transferable_resources.clone(),
        );
        let recipient = AccountAddress::sample();

        let result = SUT::try_from((output, recipient)).unwrap();
        assert_eq!(result.recipient, recipient);
        assert_eq!(
            result.transfers,
            vec![
                fungible.clone().into(),
                non_fungible.into()
            ]
        );
        assert_eq!(
            result.non_transferable_resources,
            non_transferable_resources
        );

        // Test the case where the total weight of the transfers is over the maximum.
        let non_fungible =
            NonFungibleResourcesCollectionItemGloballyAggregated::new(
                ResourceAddress::sample(),
                50,
            );

        let output = FetchTransferableResourcesOutput::new(
            vec![fungible.clone()],
            vec![non_fungible.clone()],
            vec![],
        );
        let result =
            SUT::try_from((output, recipient)).expect_err("Expected error");
        assert_eq!(
            result,
            CommonError::MaxTransfersPerTransactionReached { amount: 51 }
        );
    }
}
