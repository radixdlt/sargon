use crate::prelude::*;

impl PerAssetFungibleTransfer {
    pub fn new(
        recipient: impl Into<AssetsTransfersRecipient>,
        use_try_deposit_or_abort: bool,
        amount: impl Into<Decimal192>,
    ) -> Self {
        Self {
            recipient: recipient.into(),
            use_try_deposit_or_abort,
            amount: amount.into(),
        }
    }

    pub fn amount(
        &self,
        divisibility: impl Into<Option<u8>>,
    ) -> ScryptoDecimal192 {
        self.amount.round(divisibility).into()
    }
}

impl From<(&AssetsTransfersRecipient, PerRecipientFungibleTransfer)>
    for PerAssetFungibleTransfer
{
    fn from(
        value: (&AssetsTransfersRecipient, PerRecipientFungibleTransfer),
    ) -> Self {
        let (recipient, transfer) = value;
        Self::new(
            recipient.clone(),
            transfer.use_try_deposit_or_abort,
            transfer.amount,
        )
    }
}

impl HasSampleValues for PerAssetFungibleTransfer {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
    }
}

impl PerAssetFungibleTransfer {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            AssetsTransfersRecipient::MyOwnAccount {
                value: Account::sample_mainnet_carol(),
            },
            true,
            Decimal192::from_str("237.13372718281828").unwrap(),
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(AssetsTransfersRecipient::ForeignAccount {
            value: AccountAddress::from_str("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69").unwrap()
        },
        true,
        Decimal192::from_str("987654.1234").unwrap())
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            AssetsTransfersRecipient::MyOwnAccount {
                value: Account::sample_stokenet_olivia(),
            },
            true,
            Decimal192::from_str("42.311415").unwrap(),
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(AssetsTransfersRecipient::ForeignAccount {
            value: AccountAddress::from_str("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk").unwrap() 
        },
        true,
        Decimal192::from_str("1337.2371828128").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PerAssetFungibleTransfer;

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
