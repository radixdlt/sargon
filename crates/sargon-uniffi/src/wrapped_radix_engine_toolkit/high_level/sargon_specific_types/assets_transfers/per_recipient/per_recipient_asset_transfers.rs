use crate::prelude::*;
use sargon::PerRecipientAssetTransfers as InternalPerRecipientAssetTransfers;

#[derive(Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct PerRecipientAssetTransfers {
    pub address_of_sender: AccountAddress,
    pub transfers: Vec<PerRecipientAssetTransfer>,
}

impl From<InternalPerRecipientAssetTransfers> for PerRecipientAssetTransfers {
    fn from(per_recipient_asset_transfers: InternalPerRecipientAssetTransfers) -> Self {
        Self {
            address_of_sender: per_recipient_asset_transfers.address_of_sender.into(),
            transfers: per_recipient_asset_transfers
                .transfers
                .into_vec()
        }
    }
}

impl Into<InternalPerRecipientAssetTransfers> for PerRecipientAssetTransfers {
    fn into(self) -> InternalPerRecipientAssetTransfers {
        InternalPerRecipientAssetTransfers {
            address_of_sender: self.address_of_sender.into(),
            transfers: self.transfers.into_internal_vec(),
        }
    }
}


