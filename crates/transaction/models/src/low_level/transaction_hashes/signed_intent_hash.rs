use crate::prelude::*;

impl HasSampleValues for SignedTransactionIntentHash {
    fn sample() -> Self {
        Self::new(Hash::sample(), NetworkID::Mainnet)
    }

    fn sample_other() -> Self {
        Self::new(Hash::sample_other(), NetworkID::Simulator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedTransactionIntentHash;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::sample()),
            SUT::sample().bech32_encoded_tx_id.to_string()
        );
        assert_eq!(format!("{}", SUT::sample()), "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl");
        assert_eq!(format!("{}", SUT::sample_other()), "signedintent_sim1v7wlh0dpd5lev6w6s4f2kev562cygmgrm9kqw6swe8w92r4yr7ks7uynkj");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::sample()), "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl");
    }
}
