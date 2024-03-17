use crate::prelude::*;

impl HasSampleValues for SignedIntentHash {
    fn sample() -> Self {
        Self::new(Hash::sample(), NetworkID::Mainnet)
    }

    fn sample_other() -> Self {
        Self::new(Hash::sample_other(), NetworkID::Simulator)
    }
}
