use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
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
        amount: Decimal,
        certain_ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self::Exact {
            amount,
            certain_ids: certain_ids.into_iter().collect(),
        }
    }

    pub fn not_exact(
        certain_ids: impl IntoIterator<Item = NonFungibleLocalId>,
        lower_bound: LowerBound,
        upper_bound: UpperBound,
        allowed_ids: AllowedIds,
    ) -> Self {
        Self::NotExact {
            certain_ids: certain_ids.into_iter().collect(),
            lower_bound,
            upper_bound,
            allowed_ids,
        }
    }

    pub fn certain_ids(&self) -> Vec<NonFungibleLocalId> {
        match self {
            Self::Exact { certain_ids, .. } => certain_ids.clone(),
            Self::NotExact { certain_ids, .. } => certain_ids.clone(),
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
            } => Self::Exact {
                amount: amount.into(),
                certain_ids: certain_ids
                    .into_iter()
                    .map(NonFungibleLocalId::from)
                    .collect(),
            },
            ScryptoSimpleNonFungibleResourceBounds::NotExact {
                certain_ids,
                lower_bound,
                upper_bound,
                allowed_ids,
            } => Self::NotExact {
                certain_ids: certain_ids
                    .into_iter()
                    .map(NonFungibleLocalId::from)
                    .collect(),
                lower_bound: LowerBound::from(lower_bound),
                upper_bound: UpperBound::from(upper_bound),
                allowed_ids: AllowedIds::from(allowed_ids),
            },
        }
    }
}

impl HasSampleValues for SimpleNonFungibleResourceBounds {
    fn sample() -> Self {
        Self::exact(
            Decimal::from(150),
            vec![
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
        )
    }

    fn sample_other() -> Self {
        Self::not_exact(
            vec![
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
            amount: Decimal::from(150).into(),
            certain_ids: vec![
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
            certain_ids: vec![
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ]
            .into_iter()
            .map(ScryptoNonFungibleLocalId::from)
            .collect(),
            lower_bound: ScryptoLowerBound::Inclusive(Decimal::from(1).into()),
            upper_bound: ScryptoUpperBound::Inclusive(Decimal::from(1).into()),
            allowed_ids: ScryptoAllowedIds::Any,
        };
        assert_eq!(SUT::from(scrypto), SUT::sample_other());
    }
}
