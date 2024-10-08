use crate::prelude::*;
use sargon::PerRecipientAssetTransfer as InternalPerRecipientAssetTransfer;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerRecipientAssetTransfer {
    pub recipient: AccountOrAddressOf,
    pub fungibles: Vec<PerRecipientFungibleTransfer>,
    pub non_fungibles: Vec<PerRecipientNonFungiblesTransfer>,
}

impl From<InternalPerRecipientAssetTransfer> for PerRecipientAssetTransfer {
    fn from(per_recipient_asset_transfer: InternalPerRecipientAssetTransfer) -> Self {
        Self {
            recipient: per_recipient_asset_transfer.recipient.into(),
            fungibles: per_recipient_asset_transfer
                .fungibles
                .into_vec(),
            non_fungibles: per_recipient_asset_transfer
                .non_fungibles
                .into_vec(),
        }
    }
}

impl Into<InternalPerRecipientAssetTransfer> for PerRecipientAssetTransfer {
    fn into(self) -> InternalPerRecipientAssetTransfer {
        InternalPerRecipientAssetTransfer {
            recipient: self.recipient.into(),
            fungibles: self.fungibles.into_internal_vec(),
            non_fungibles: self.non_fungibles.into_internal_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PerRecipientAssetTransfer;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample_mainnet(),
                SUT::sample_mainnet_other(),
                SUT::sample_stokenet(),
                SUT::sample_stokenet_other(),
                // duplicates should be removed
                SUT::sample_mainnet(),
                SUT::sample_mainnet_other(),
                SUT::sample_stokenet(),
                SUT::sample_stokenet_other(),
            ])
            .len(),
            4
        )
    }
}
