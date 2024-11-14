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
