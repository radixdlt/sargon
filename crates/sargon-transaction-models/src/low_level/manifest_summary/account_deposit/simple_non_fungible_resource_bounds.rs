use uuid::fmt::Simple;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleNonFungibleResourceBounds {
    /// The IDs of the non_fungible resources that are certain to be deposited.
    pub certain_ids: Vec<NonFungibleLocalId>,
    /// The additional amount of non_fungible resources that may be deposited.
    pub additional_amount: Option<SimpleCountedResourceBounds>,
}

impl SimpleNonFungibleResourceBounds {
    pub fn new(
        certain_ids: impl IntoIterator<Item = NonFungibleLocalId>,
        additional_amount: Option<SimpleCountedResourceBounds>,
    ) -> Self {
        Self {
            certain_ids: certain_ids.into_iter().collect(),
            additional_amount,
        }
    }
}

impl From<ScryptoSimpleNonFungibleResourceBounds>
    for SimpleNonFungibleResourceBounds
{
    fn from(value: ScryptoSimpleNonFungibleResourceBounds) -> Self {
        match value {
            ScryptoSimpleNonFungibleResourceBounds::Exact {
                amount: _,
                certain_ids,
            } => Self::new(
                certain_ids
                    .into_iter()
                    .map(NonFungibleLocalId::from)
                    .collect::<IndexSet<_>>(),
                None,
            ),
            ScryptoSimpleNonFungibleResourceBounds::NotExact {
                certain_ids,
                lower_bound,
                upper_bound,
                allowed_ids: _,
            } => Self::new(
                certain_ids
                    .clone()
                    .into_iter()
                    .map(NonFungibleLocalId::from)
                    .collect::<IndexSet<_>>(),
                Some(SimpleCountedResourceBounds::from((
                    certain_ids.len(),
                    lower_bound,
                    upper_bound,
                ))),
            ),
        }
    }
}

/// Converts the ScryptoSimpleNonFungibleResourceBounds::NotExact to SimpleCountedResourceBounds.
/// The following invariants are guaranteed by scrypto:
/// - certain_ids.len() <= lower_bound_inclusive <= upper_bound_inclusive
/// - certain_ids.len() is included in the bounds
/// - if certain_ids.len() == upper_bound_inclusive, then the bounds are exact, and it will be represented as ScryptoSimpleNonFungibleResourceBounds::Exact.
impl From<(usize, ScryptoLowerBound, ScryptoUpperBound)>
    for SimpleCountedResourceBounds
{
    fn from(val: (usize, ScryptoLowerBound, ScryptoUpperBound)) -> Self {
        let (certain_ids_size, lower_bound, upper_bound) = val;

        // Adjust the bound values to account for the certain IDs count.
        fn adjusted_bound_value(
            bound_value: ScryptoDecimal192,
            certain_ids_size: usize,
        ) -> Decimal {
            Decimal::from(
                u64::try_from(bound_value).unwrap() - certain_ids_size as u64,
            )
        }

        match (lower_bound, upper_bound) {
            (ScryptoLowerBound::NonZero, ScryptoUpperBound::Unbounded) => {
                SimpleCountedResourceBounds::UnknownAmount
            }
            (
                ScryptoLowerBound::NonZero,
                ScryptoUpperBound::Inclusive(upper_bound),
            ) => SimpleCountedResourceBounds::AtMost {
                amount: adjusted_bound_value(upper_bound, certain_ids_size),
            },
            (
                ScryptoLowerBound::Inclusive(lower_bound),
                ScryptoUpperBound::Unbounded,
            ) => {
                let adjusted_lower_bound_value =
                    adjusted_bound_value(lower_bound, certain_ids_size);
                if adjusted_lower_bound_value.is_zero() {
                    SimpleCountedResourceBounds::UnknownAmount
                } else {
                    SimpleCountedResourceBounds::AtLeast {
                        amount: adjusted_lower_bound_value,
                    }
                }
            }
            (
                ScryptoLowerBound::Inclusive(lower_bound),
                ScryptoUpperBound::Inclusive(upper_bound),
            ) => {
                let adjusted_lower_bound_value =
                    adjusted_bound_value(lower_bound, certain_ids_size);
                let adjusted_upper_bound_value =
                    adjusted_bound_value(upper_bound, certain_ids_size);

                if adjusted_lower_bound_value.is_zero() {
                    SimpleCountedResourceBounds::AtMost {
                        amount: adjusted_upper_bound_value,
                    }
                } else if adjusted_lower_bound_value
                    == adjusted_upper_bound_value
                {
                    SimpleCountedResourceBounds::Exact {
                        amount: adjusted_lower_bound_value,
                    }
                } else {
                    SimpleCountedResourceBounds::Between {
                        min_amount: adjusted_lower_bound_value,
                        max_amount: adjusted_upper_bound_value,
                    }
                }
            }
        }
    }
}

impl HasSampleValues for SimpleNonFungibleResourceBounds {
    fn sample() -> Self {
        Self::new(
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
            None,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
            Some(SimpleCountedResourceBounds::between(3, 5)),
        )
    }
}

#[cfg(test)]
mod tests {
    use sbor::prelude::indexmap::IndexSet;

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
        let certain_ids = vec![
            NonFungibleLocalId::sample(),
            NonFungibleLocalId::sample_other(),
        ];

        let scrypto = ScryptoSimpleNonFungibleResourceBounds::Exact {
            amount: Decimal::from(2).into(),
            certain_ids: certain_ids
                .clone()
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect(),
        };

        assert_eq!(SUT::from(scrypto), SUT::new(certain_ids, None));
    }

    #[test]
    fn from_scrypto_non_exact_no_certain_ids() {
        fn assert_bounds(
            lower_bound: ScryptoLowerBound,
            upper_bound: ScryptoUpperBound,
            expected: SimpleCountedResourceBounds,
        ) {
            let scrypto = ScryptoSimpleNonFungibleResourceBounds::NotExact {
                certain_ids: IndexSet::new(),
                lower_bound,
                upper_bound,
                allowed_ids: ScryptoAllowedIds::Any,
            };

            assert_eq!(
                SUT::from(scrypto),
                SUT::new(Vec::new(), Some(expected))
            );
        }

        assert_bounds(
            ScryptoLowerBound::NonZero,
            ScryptoUpperBound::Unbounded,
            SimpleCountedResourceBounds::unknown_amount(),
        );

        assert_bounds(
            ScryptoLowerBound::NonZero,
            ScryptoUpperBound::Inclusive(Decimal::from(5).into()),
            SimpleCountedResourceBounds::at_most(5),
        );

        assert_bounds(
            ScryptoLowerBound::Inclusive(Decimal::from(3).into()),
            ScryptoUpperBound::Unbounded,
            SimpleCountedResourceBounds::at_least(3),
        );

        assert_bounds(
            ScryptoLowerBound::Inclusive(Decimal::from(3).into()),
            ScryptoUpperBound::Inclusive(Decimal::from(5).into()),
            SimpleCountedResourceBounds::between(3, 5),
        );

        assert_bounds(
            ScryptoLowerBound::Inclusive(Decimal::from(3).into()),
            ScryptoUpperBound::Inclusive(Decimal::from(3).into()),
            SimpleCountedResourceBounds::exact(3),
        );
    }

    #[test]
    fn from_scrypto_non_exact_with_certain_ids() {
        let certain_ids = vec![
            NonFungibleLocalId::sample(),
            NonFungibleLocalId::sample_other(),
        ];

        let scrypto_certain_ids: IndexSet<ScryptoNonFungibleLocalId> =
            certain_ids
                .clone()
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect();

        let assert_bounds =
            |lower_bound: ScryptoLowerBound,
             upper_bound: ScryptoUpperBound,
             expected: SimpleCountedResourceBounds| {
                let scrypto =
                    ScryptoSimpleNonFungibleResourceBounds::NotExact {
                        certain_ids: scrypto_certain_ids.clone(),
                        lower_bound,
                        upper_bound,
                        allowed_ids: ScryptoAllowedIds::Any,
                    };

                assert_eq!(
                    SUT::from(scrypto),
                    SUT::new(certain_ids.clone(), Some(expected))
                );
            };

        assert_bounds(
            ScryptoLowerBound::at_least(certain_ids.len().into()),
            ScryptoUpperBound::Unbounded,
            SimpleCountedResourceBounds::unknown_amount(),
        );

        assert_bounds(
            ScryptoLowerBound::at_least((certain_ids.len() + 1).into()),
            ScryptoUpperBound::Unbounded,
            SimpleCountedResourceBounds::at_least(1),
        );

        assert_bounds(
            ScryptoLowerBound::at_least((certain_ids.len() + 2).into()),
            ScryptoUpperBound::at_most((certain_ids.len() + 2).into()),
            SimpleCountedResourceBounds::exact(2),
        );

        assert_bounds(
            ScryptoLowerBound::at_least((certain_ids.len() + 3).into()),
            ScryptoUpperBound::at_most((certain_ids.len() + 5).into()),
            SimpleCountedResourceBounds::between(3, 5),
        );
    }
}
