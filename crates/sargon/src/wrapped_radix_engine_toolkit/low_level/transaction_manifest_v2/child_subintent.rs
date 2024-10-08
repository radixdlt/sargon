use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ChildSubintent {
    pub hash: SubintentHash,
}

impl From<ChildSubintent> for ScryptoChildSubintent {
    fn from(value: ChildSubintent) -> Self {
        ScryptoChildSubintent {
            hash: ScryptoSubintentHash {
                0: value.hash.hash.into(),
            },
        }
    }
}

impl From<(ScryptoChildSubintent, NetworkID)> for ChildSubintent {
    fn from(value: (ScryptoChildSubintent, NetworkID)) -> Self {
        Self {
            hash: (value.0.hash, value.1).into(),
        }
    }
}

impl From<(ScryptoSubintentHash, NetworkID)> for SubintentHash {
    fn from(value: (ScryptoSubintentHash, NetworkID)) -> Self {
        Self::from_scrypto(value.0, value.1)
    }
}

impl HasSampleValues for ChildSubintent {
    fn sample() -> Self {
        Self {
            hash: SubintentHash::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            hash: SubintentHash::sample_other(),
        }
    }
}
