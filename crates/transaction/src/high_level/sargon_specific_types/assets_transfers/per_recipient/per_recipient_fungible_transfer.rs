use crate::prelude::*;

impl PerRecipientFungibleTransfer {
    pub fn new(
        resource_address: ResourceAddress,
        amount: impl Into<Decimal192>,
        use_try_deposit_or_abort: bool,
        divisibility: impl Into<Option<u8>>,
    ) -> Self {
        Self {
            resource_address,
            amount: amount.into(),
            use_try_deposit_or_abort,
            divisibility: divisibility.into(),
        }
    }
}

impl PerRecipientFungibleTransfer {
    pub fn sample_mainnet() -> Self {
        Self::new(ResourceAddress::sample_mainnet_xrd(), 237, true, None)
    }

    pub fn sample_mainnet_other() -> Self {
        Self::new(ResourceAddress::sample_mainnet_candy(), 1337, true, 4)
    }

    pub fn sample_stokenet() -> Self {
        Self::new(ResourceAddress::sample_stokenet_xrd(), 42, false, None)
    }

    pub fn sample_stokenet_other() -> Self {
        Self::new(ResourceAddress::sample_stokenet_candy(), 3, true, 6)
    }
}

impl HasSampleValues for PerRecipientFungibleTransfer {
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
    type SUT = PerRecipientFungibleTransfer;

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
