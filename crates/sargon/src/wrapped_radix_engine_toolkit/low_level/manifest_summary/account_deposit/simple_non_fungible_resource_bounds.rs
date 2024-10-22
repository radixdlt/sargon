use crate::prelude::*;

/// Represents the bounds for a simple non-fungible resource, which can be either exact or not exact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimpleNonFungibleResourceBounds {
    Exact {
        amount: Decimal,
        certain_ids: Vec<NonFungibleLocalId>,
    },
    NotExact {
        certain_ids: Vec<NonFungibleLocalId>,
        lower_bound: LowerBound,
        upper_bound: UpperBound,
        allowed_ids: AllowedIds,
    },
}

impl SimpleNonFungibleResourceBounds {
    pub fn exact(
        amount: impl Into<Decimal>,
        certain_ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self::Exact {
            amount: amount.into(),
            certain_ids: certain_ids.into_iter().collect(),
        }
    }

    /// # Panics
    /// Panics if `lower_bound` is greater than `upper_bound`.
    pub fn not_exact(
        certain_ids: impl IntoIterator<Item = NonFungibleLocalId>,
        lower_bound: LowerBound,
        upper_bound: UpperBound,
        allowed_ids: AllowedIds,
    ) -> Self {
        assert!(
            lower_bound.get_amount() <= upper_bound.get_amount(),
            "Upper bound MUST be greater than or equal lower bound."
        );

        Self::NotExact {
            certain_ids: certain_ids.into_iter().collect(),
            lower_bound,
            upper_bound,
            allowed_ids,
        }
    }

    pub fn certain_ids(&self) -> IndexSet<NonFungibleLocalId> {
        match self {
            Self::Exact { certain_ids, .. } => {
                certain_ids.clone().into_iter().collect()
            }
            Self::NotExact { certain_ids, .. } => {
                certain_ids.clone().into_iter().collect()
            }
        }
    }
}

impl From<ScryptoSimpleNonFungibleResourceBounds>
    for SimpleNonFungibleResourceBounds
{
    fn from(value: ScryptoSimpleNonFungibleResourceBounds) -> Self {
        match value {
            ScryptoSimpleNonFungibleResourceBounds::Exact {
                amount,
                certain_ids,
            } => Self::exact(
                amount,
                certain_ids
                    .into_iter()
                    .map(NonFungibleLocalId::from)
                    .collect::<IndexSet<_>>(),
            ),
            ScryptoSimpleNonFungibleResourceBounds::NotExact {
                certain_ids,
                lower_bound,
                upper_bound,
                allowed_ids,
            } => Self::not_exact(
                certain_ids
                    .into_iter()
                    .map(NonFungibleLocalId::from)
                    .collect::<IndexSet<_>>(),
                LowerBound::from(lower_bound),
                UpperBound::from(upper_bound),
                AllowedIds::from(allowed_ids),
            ),
        }
    }
}

impl HasSampleValues for SimpleNonFungibleResourceBounds {
    fn sample() -> Self {
        Self::exact(
            150,
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
        )
    }

    fn sample_other() -> Self {
        Self::not_exact(
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
            LowerBound::sample(),
            UpperBound::sample(),
            AllowedIds::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SimpleNonFungibleResourceBounds;

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
    fn from_scrypto_exact() {
        let scrypto = ScryptoSimpleNonFungibleResourceBounds::Exact {
            amount: 150.into(),
            certain_ids: [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ]
            .into_iter()
            .map(ScryptoNonFungibleLocalId::from)
            .collect(),
        };
        assert_eq!(SUT::from(scrypto), SUT::sample());
    }

    #[test]
    fn from_scrypto_not_exact() {
        let scrypto = ScryptoSimpleNonFungibleResourceBounds::NotExact {
            certain_ids: [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ]
            .into_iter()
            .map(ScryptoNonFungibleLocalId::from)
            .collect(),
            lower_bound: ScryptoLowerBound::Inclusive(1.into()),
            upper_bound: ScryptoUpperBound::Inclusive(1.into()),
            allowed_ids: ScryptoAllowedIds::Any,
        };
        assert_eq!(SUT::from(scrypto), SUT::sample_other());
    }

    #[test]
    fn test_certain_ids_exact() {
        let exact = SUT::exact(
            100,
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
        );
        assert_eq!(
            exact.certain_ids(),
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other()
            ]
            .into_iter()
            .collect::<IndexSet<_>>()
        )
    }

    #[test]
    fn test_certain_ids_not_exact() {
        let not_exact = SUT::not_exact(
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
            LowerBound::sample(),
            UpperBound::sample(),
            AllowedIds::sample(),
        );
        assert_eq!(
            not_exact.certain_ids(),
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other()
            ]
            .into_iter()
            .collect::<IndexSet<_>>()
        )
    }

    #[test]
    #[should_panic(
        expected = "Upper bound MUST be greater than or equal lower bound."
    )]
    fn not_exact_should_panic_when_upper_bound_less_than_lower_bound() {
        SUT::not_exact(
            [],
            LowerBound::inclusive(2),
            UpperBound::inclusive(1),
            AllowedIds::sample(),
        );
    }
}
