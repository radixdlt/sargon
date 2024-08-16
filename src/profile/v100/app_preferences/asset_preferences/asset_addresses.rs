use crate::prelude::*;

decl_identified_vec_of!(AssetAddresses, AssetAddress);

impl HasSampleValues for AssetAddresses {
    fn sample() -> Self {
        Self::from_iter([AssetAddress::sample(), AssetAddress::sample_other()])
    }

    fn sample_other() -> Self {
        Self::from_iter([AssetAddress::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssetAddresses;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
