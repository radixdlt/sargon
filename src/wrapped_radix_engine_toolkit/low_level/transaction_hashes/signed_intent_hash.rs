use crate::prelude::*;

impl HasSampleValues for SignedIntentHash {
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
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedIntentHash;

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
    fn to_string() {
        assert_eq!(SUT::sample().to_string(), "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl");
    }

    #[test]
    fn to_string_other() {
        assert_eq!(SUT::sample_other().to_string(), "signedintent_sim1r9k2z662xhqu8hccugxtrxwfn5ffemrw5chtqw3flhskmwyh7jcslkptg7");
    }

    #[test]
    fn from_str() {
        assert_eq!(SUT::sample(), "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl".parse::<SUT>().unwrap());
    }

    #[test]
    fn from_str_other() {
        assert_eq!(SUT::sample_other(), "signedintent_sim1r9k2z662xhqu8hccugxtrxwfn5ffemrw5chtqw3flhskmwyh7jcslkptg7".parse::<SUT>().unwrap());
    }
}
