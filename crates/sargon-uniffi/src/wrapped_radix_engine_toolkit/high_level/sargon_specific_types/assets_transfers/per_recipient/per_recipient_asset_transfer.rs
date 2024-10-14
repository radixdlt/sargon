use crate::prelude::*;
use sargon::PerRecipientAssetTransfer as InternalPerRecipientAssetTransfer;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct PerRecipientAssetTransfer {
    pub recipient: AccountOrAddressOf,
    pub fungibles: Vec<PerRecipientFungibleTransfer>,
    pub non_fungibles: Vec<PerRecipientNonFungibleTransfer>,
}
