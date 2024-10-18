use crate::prelude::*;
use sargon::PerRecipientAssetTransfers as InternalPerRecipientAssetTransfers;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct PerRecipientAssetTransfers {
    pub address_of_sender: AccountAddress,
    pub transfers: Vec<PerRecipientAssetTransfer>,
}
