use crate::prelude::*;

impl PerRecipientNonFungiblesTransfer {
    pub fn new(
        resource_address: impl Into<ResourceAddress>,
        use_try_deposit_or_abort: bool,
        local_ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self {
            resource_address: resource_address.into(),
            use_try_deposit_or_abort,
            local_ids: local_ids.into_iter().collect_vec(),
        }
    }
}

impl PerRecipientNonFungiblesTransfer {
    pub fn sample_mainnet() -> Self {
        Self::new(
            ResourceAddress::sample_mainnet_xrd(),
            true,
            [
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
            ],
        )
    }

    pub fn sample_mainnet_other() -> Self {
        Self::new(
            ResourceAddress::sample_mainnet_candy(),
            true,
            [
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
            ],
        )
    }

    pub fn sample_stokenet() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_xrd(),
            false,
            [
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
            ],
        )
    }

    pub fn sample_stokenet_other() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_candy(),
            true,
            [
                NonFungibleLocalId::integer(1),
                NonFungibleLocalId::integer(2),
            ],
        )
    }
}

impl HasSampleValues for PerRecipientNonFungiblesTransfer {
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
    type SUT = PerRecipientNonFungiblesTransfer;

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
