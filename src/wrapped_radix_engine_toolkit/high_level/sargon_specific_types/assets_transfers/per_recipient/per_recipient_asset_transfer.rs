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

#[allow(unused)]
impl PerRecipientAssetTransfer {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            AssetsTransfersRecipient::sample_mainnet(),
            [
                PerRecipientFungibleTransfer::sample_mainnet(),
                PerRecipientFungibleTransfer::sample_mainnet_other(),
            ],
            [
                PerRecipientNonFungiblesTransfer::sample_mainnet(),
                PerRecipientNonFungiblesTransfer::sample_mainnet_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            AssetsTransfersRecipient::sample_mainnet_other(),
            [PerRecipientFungibleTransfer::sample_mainnet_other()],
            [PerRecipientNonFungiblesTransfer::sample_mainnet_other()],
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            AssetsTransfersRecipient::sample_stokenet(),
            [
                PerRecipientFungibleTransfer::sample_stokenet(),
                PerRecipientFungibleTransfer::sample_stokenet_other(),
            ],
            [
                PerRecipientNonFungiblesTransfer::sample_stokenet(),
                PerRecipientNonFungiblesTransfer::sample_stokenet_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            AssetsTransfersRecipient::sample_stokenet_other(),
            [PerRecipientFungibleTransfer::sample_stokenet_other()],
            [PerRecipientNonFungiblesTransfer::sample_stokenet_other()],
        )
    }
}

impl HasSampleValues for PerRecipientAssetTransfer {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
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
