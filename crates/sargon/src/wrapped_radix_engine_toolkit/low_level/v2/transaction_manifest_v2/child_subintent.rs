use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildSubintentSpecifier {
    pub hash: SubintentHash,
}

impl ChildSubintentSpecifier {
    pub fn new(hash: SubintentHash) -> Self {
        Self { hash }
    }
}

impl From<ChildSubintentSpecifier> for ScryptoChildSubintentSpecifier {
    fn from(value: ChildSubintentSpecifier) -> Self {
        ScryptoChildSubintentSpecifier {
            hash: ScryptoSubintentHash(value.hash.hash.into()),
        }
    }
}

impl From<(ScryptoChildSubintentSpecifier, NetworkID)>
    for ChildSubintentSpecifier
{
    fn from(value: (ScryptoChildSubintentSpecifier, NetworkID)) -> Self {
        Self::new((value.0.hash, value.1).into())
    }
}

impl From<(ScryptoSubintentHash, NetworkID)> for SubintentHash {
    fn from(value: (ScryptoSubintentHash, NetworkID)) -> Self {
        Self::from_scrypto(value.0, value.1)
    }
}

impl From<SubintentHash> for ScryptoSubintentHash {
    fn from(value: SubintentHash) -> Self {
        Self(ScryptoHash::from(value.hash))
    }
}

impl HasSampleValues for ChildSubintentSpecifier {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ChildSubintentSpecifier;

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
    fn to_from_scrypto() {
        let roundtrip = |s: SUT, network_id: NetworkID| {
            let scrypto: ScryptoChildSubintentSpecifier = s.clone().into();
            SUT::from((scrypto, network_id))
        };
        assert_eq!(SUT::sample(), roundtrip(SUT::sample(), NetworkID::Mainnet));
        assert_eq!(
            SUT::sample_other(),
            roundtrip(SUT::sample_other(), NetworkID::Simulator)
        );
    }
}
