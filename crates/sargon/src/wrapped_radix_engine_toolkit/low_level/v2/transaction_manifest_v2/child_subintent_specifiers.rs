use crate::prelude::*;

/// Represents a collection of child subintents.
///
/// This struct is used to manage a list of `ChildSubintentSpecifier` instances, providing
/// methods for creation, conversion, and sample values for testing purposes.
///
/// # Fields
/// - `children`: A vector of `ChildSubintentSpecifier` instances.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildSubintentSpecifiers {
    pub children: Vec<ChildSubintentSpecifier>,
}

impl ChildSubintentSpecifiers {
    pub fn new(
        children: impl IntoIterator<Item = ChildSubintentSpecifier>,
    ) -> Self {
        Self {
            children: children.into_iter().collect(),
        }
    }
}

impl Default for ChildSubintentSpecifiers {
    fn default() -> Self {
        Self::new([])
    }
}

impl From<ChildSubintentSpecifiers>
    for IndexSet<ScryptoChildSubintentSpecifier>
{
    fn from(value: ChildSubintentSpecifiers) -> Self {
        value.children.into_iter().map(Into::into).collect()
    }
}

impl From<ChildSubintentSpecifiers> for ScryptoChildSubintentSpecifiers {
    fn from(value: ChildSubintentSpecifiers) -> Self {
        ScryptoChildSubintentSpecifiers {
            children: value
                .children
                .into_iter()
                .map(ScryptoChildSubintentSpecifier::from)
                .collect(),
        }
    }
}

impl From<(IndexSet<ScryptoChildSubintentSpecifier>, NetworkID)>
    for ChildSubintentSpecifiers
{
    fn from(
        value: (IndexSet<ScryptoChildSubintentSpecifier>, NetworkID),
    ) -> Self {
        Self::new(
            value
                .0
                .into_iter()
                .map(|c| (c, value.1).into())
                .collect::<Vec<_>>(),
        )
    }
}

impl ChildSubintentSpecifiers {
    pub(crate) fn empty() -> Self {
        Self::default()
    }
}

impl HasSampleValues for ChildSubintentSpecifiers {
    fn sample() -> Self {
        Self::new([ChildSubintentSpecifier::sample()])
    }

    fn sample_other() -> Self {
        Self::new([ChildSubintentSpecifier::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ChildSubintentSpecifiers;

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
            let scrypto: ScryptoChildSubintentSpecifiers = s.clone().into();
            SUT::from((scrypto.children, network_id))
        };
        assert_eq!(SUT::sample(), roundtrip(SUT::sample(), NetworkID::Mainnet));
        assert_eq!(
            SUT::sample_other(),
            roundtrip(SUT::sample_other(), NetworkID::Simulator)
        );
    }
}
