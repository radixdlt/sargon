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

        let fungibles = fetch_resources_output
            .fungibles
            .clone()
            .into_iter()
            .map(DeleteAccountTransfer::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let non_fungibles = fetch_resources_output
            .non_fungibles
            .into_iter()
            .map(DeleteAccountTransfer::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let transfers = [fungibles, non_fungibles].concat();

        Ok(Self::new(recipient, transfers))
    }
}
