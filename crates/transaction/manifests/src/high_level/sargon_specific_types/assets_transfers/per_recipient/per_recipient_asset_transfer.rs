use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PerRecipientAssetTransfer {
    pub recipient: TransferRecipient,
    pub fungibles: Vec<PerRecipientFungibleTransfer>,
    pub non_fungibles: Vec<PerRecipientNonFungibleTransfer>,
}

impl PerRecipientAssetTransfer {
    pub fn new(
        recipient: impl Into<TransferRecipient>,
        fungibles: impl IntoIterator<Item = PerRecipientFungibleTransfer>,
        non_fungibles: impl IntoIterator<Item = PerRecipientNonFungibleTransfer>,
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
            TransferRecipient::sample_mainnet(),
            [
                PerRecipientFungibleTransfer::sample_mainnet(),
                PerRecipientFungibleTransfer::sample_mainnet_other(),
            ],
            [
                PerRecipientNonFungibleTransfer::sample_mainnet(),
                PerRecipientNonFungibleTransfer::sample_mainnet_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            TransferRecipient::sample_mainnet_other(),
            [PerRecipientFungibleTransfer::sample_mainnet_other()],
            [PerRecipientNonFungibleTransfer::sample_mainnet_other()],
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            TransferRecipient::sample_stokenet(),
            [
                PerRecipientFungibleTransfer::sample_stokenet(),
                PerRecipientFungibleTransfer::sample_stokenet_other(),
            ],
            [
                PerRecipientNonFungibleTransfer::sample_stokenet(),
                PerRecipientNonFungibleTransfer::sample_stokenet_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            TransferRecipient::sample_stokenet_other(),
            [PerRecipientFungibleTransfer::sample_stokenet_other()],
            [PerRecipientNonFungibleTransfer::sample_stokenet_other()],
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
