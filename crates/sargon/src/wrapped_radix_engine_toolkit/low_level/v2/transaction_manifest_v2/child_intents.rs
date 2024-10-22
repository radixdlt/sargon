use crate::prelude::*;

/// Represents a collection of child subintents.
///
/// This struct is used to manage a list of `ChildSubintent` instances, providing
/// methods for creation, conversion, and sample values for testing purposes.
///
/// # Fields
/// - `children`: A vector of `ChildSubintent` instances.
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ChildIntents {
    pub children: Vec<ChildSubintent>,
}

impl ChildIntents {
    pub fn new(children: impl IntoIterator<Item = ChildSubintent>) -> Self {
        Self {
            children: children.into_iter().collect(),
        }
    }
}

impl Default for ChildIntents {
    fn default() -> Self {
        Self::new([])
    }
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
        Self::new(
            value
                .0
                .into_iter()
                .map(|c| (c, value.1).into())
                .collect::<Vec<_>>(),
        )
    }
}

impl ChildIntents {
    pub(crate) fn empty() -> Self {
        Self::default()
    }
}

impl HasSampleValues for ChildIntents {
    fn sample() -> Self {
        Self::new([ChildSubintent::sample()])
    }

    fn sample_other() -> Self {
        Self::new([ChildSubintent::sample_other()])
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
