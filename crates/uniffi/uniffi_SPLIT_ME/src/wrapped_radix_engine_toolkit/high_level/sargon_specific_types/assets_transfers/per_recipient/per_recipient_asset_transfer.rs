use crate::prelude::*;
use sargon::PerRecipientAssetTransfer as InternalPerRecipientAssetTransfer;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct PerRecipientAssetTransfer {
    pub recipient: TransferRecipient,
    pub fungibles: Vec<PerRecipientFungibleTransfer>,
    pub non_fungibles: Vec<PerRecipientNonFungibleTransfer>,
}
