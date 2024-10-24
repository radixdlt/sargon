use crate::prelude::*;

/// Represents which ids are possible in a non-fungible balance.
///
/// `Any` represents that any id is possible. `Allowlist` represents that
/// any ids in the balance have to be in the allowlist.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllowedIds {
    Allowlist { ids: Vec<NonFungibleLocalId> },
    Any,
}

impl AllowedIds {
    pub fn allowlist(
        ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self::Allowlist {
            ids: ids.into_iter().collect(),
        }
    }

    pub fn any() -> Self {
        Self::Any
    }

    pub fn ids(&self) -> IndexSet<NonFungibleLocalId> {
        match self {
            AllowedIds::Allowlist { ids } => ids.clone().into_iter().collect(),
            AllowedIds::Any => IndexSet::new(),
        }
    }
}

impl From<ScryptoAllowedIds> for AllowedIds {
    fn from(value: ScryptoAllowedIds) -> Self {
        match value {
            ScryptoAllowedIds::Allowlist(ids) => Self::Allowlist {
                ids: ids.into_iter().map(NonFungibleLocalId::from).collect(),
            },
            ScryptoAllowedIds::Any => Self::Any,
        }
    }
}

impl HasSampleValues for AllowedIds {
    fn sample() -> Self {
        Self::allowlist([
            NonFungibleLocalId::sample(),
            NonFungibleLocalId::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AllowedIds;

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
    fn get_ids() {
        let ids = [
            NonFungibleLocalId::random(),
            NonFungibleLocalId::random(),
            NonFungibleLocalId::random(),
        ]
        .into_iter()
        .collect::<IndexSet<_>>();

        assert_eq!(SUT::allowlist(ids.clone()).ids(), ids);
        assert_eq!(SUT::any().ids(), IndexSet::new());
    }

    #[test]
    fn from_scrypto_allowlist() {
        let scrypto = ScryptoAllowedIds::Allowlist(
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ]
            .into_iter()
            .map(ScryptoNonFungibleLocalId::from)
            .collect::<IndexSet<_>>(),
        );
        assert_eq!(SUT::from(scrypto), SUT::sample());
    }

    #[test]
    fn from_scrypto_any() {
        let scrypto = ScryptoAllowedIds::Any;
        assert_eq!(SUT::from(scrypto), SUT::sample_other());
    }
}
