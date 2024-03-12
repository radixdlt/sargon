use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerRecipientAssetTransfer {
    pub recipient: AssetsTransfersRecipient,
    pub fungibles: Vec<PerRecipientFungibleTransfer>,
    pub non_fungibles: Vec<PerRecipientNonFungiblesTransfer>,
}

impl PerRecipientAssetTransfer {
    pub fn new(
        recipient: impl Into<AssetsTransfersRecipient>,
        fungibles: impl IntoIterator<Item = PerRecipientFungibleTransfer>,
        non_fungibles: impl IntoIterator<Item = PerRecipientNonFungiblesTransfer>,
    ) -> Self {
        Self {
            recipient: recipient.into(),
            fungibles: fungibles.into_iter().collect_vec(),
            non_fungibles: non_fungibles.into_iter().collect_vec(),
        }
    }
}
