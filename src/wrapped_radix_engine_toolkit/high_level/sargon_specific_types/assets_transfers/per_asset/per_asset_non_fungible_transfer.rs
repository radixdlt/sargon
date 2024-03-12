use crate::prelude::*;

impl PerAssetNonFungibleTransfer {
    pub fn new(
        recipient: impl Into<AssetsTransfersRecipient>,
        use_try_deposit_or_abort: bool,
        non_fungible_local_ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self {
            recipient: recipient.into(),
            use_try_deposit_or_abort,
            non_fungible_local_ids: non_fungible_local_ids
                .into_iter()
                .collect_vec(),
        }
    }

    pub fn local_ids(&self) -> Vec<ScryptoNonFungibleLocalId> {
        self.non_fungible_local_ids
            .clone()
            .into_iter()
            .map(ScryptoNonFungibleLocalId::from)
            .collect_vec()
    }
}

impl From<(&AssetsTransfersRecipient, PerRecipientNonFungiblesTransfer)>
    for PerAssetNonFungibleTransfer
{
    fn from(
        value: (&AssetsTransfersRecipient, PerRecipientNonFungiblesTransfer),
    ) -> Self {
        let (recipient, non_fungibles) = value;
        Self::new(
            recipient.clone(),
            non_fungibles.use_try_deposit_or_abort,
            non_fungibles.local_ids,
        )
    }
}

impl PerAssetNonFungibleTransfer {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            AssetsTransfersRecipient::MyOwnAccount {
                value: Account::sample_mainnet_carol(),
            },
            true,
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            AssetsTransfersRecipient::ForeignAccount {
                value: AccountAddress::from_str("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69").unwrap() 
            },
        true,
        [NonFungibleLocalId::sample_other()]
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            AssetsTransfersRecipient::MyOwnAccount {
                value: Account::sample_stokenet_carol(),
            },
            true,
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            AssetsTransfersRecipient::ForeignAccount {
                value: AccountAddress::from_str("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk").unwrap() 
            },
        true,
        [NonFungibleLocalId::sample_other()]
        )
    }
}

impl HasSampleValues for PerAssetNonFungibleTransfer {
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
    type SUT = PerAssetNonFungibleTransfer;

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
