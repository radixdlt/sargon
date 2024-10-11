use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ChildIntents {
    pub children: Vec<ChildSubintent>,
}

impl From<ChildIntents> for Vec<ScryptoChildSubintent> {
    fn from(value: ChildIntents) -> Self {
        value.children.into_iter().map(Into::into).collect()
    }
}

impl From<ChildIntents> for ScryptoChildIntents {
    fn from(value: ChildIntents) -> Self {
        ScryptoChildIntents {
            children: value.into(),
        }
    }
}

impl From<(Vec<ScryptoChildSubintent>, NetworkID)> for ChildIntents {
    fn from(value: (Vec<ScryptoChildSubintent>, NetworkID)) -> Self {
        Self {
            children: value
                .0
                .into_iter()
                .map(|c| (c, value.1).into())
                .collect(),
        }
    }
}

impl ChildIntents {
    pub(crate) fn empty() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl HasSampleValues for ChildIntents {
    fn sample() -> Self {
        Self {
            children: vec![ChildSubintent::sample()],
        }
    }

    fn sample_other() -> Self {
        Self {
            children: vec![ChildSubintent::sample_other()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ChildIntents;

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
    fn empty() {
        let empty = SUT::empty();
        assert!(empty.children.is_empty());
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT, network_id: NetworkID| {
            let scrypto: ScryptoChildIntents = s.clone().into();
            SUT::from((scrypto.children, network_id))
        };
        assert_eq!(SUT::sample(), roundtrip(SUT::sample(), NetworkID::Mainnet));
        assert_eq!(
            SUT::sample_other(),
            roundtrip(SUT::sample_other(), NetworkID::Simulator)
        );
    }
}