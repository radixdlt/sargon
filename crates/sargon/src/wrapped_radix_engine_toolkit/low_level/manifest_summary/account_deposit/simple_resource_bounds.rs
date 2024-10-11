use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum SimpleResourceBounds {
    Fungible {
        bounds: SimpleFungibleResourceBounds,
    },
    NonFungible {
        bounds: SimpleNonFungibleResourceBounds,
    },
}

impl SimpleResourceBounds {
    pub fn fungible(bounds: SimpleFungibleResourceBounds) -> Self {
        Self::Fungible { bounds }
    }

    pub fn non_fungible(bounds: SimpleNonFungibleResourceBounds) -> Self {
        Self::NonFungible { bounds }
    }
}

impl From<ScryptoSimpleResourceBounds> for SimpleResourceBounds {
    fn from(value: ScryptoSimpleResourceBounds) -> Self {
        match value {
            ScryptoSimpleResourceBounds::Fungible(bounds) => Self::Fungible {
                bounds: bounds.into(),
            },
            ScryptoSimpleResourceBounds::NonFungible(bounds) => {
                Self::NonFungible {
                    bounds: bounds.into(),
                }
            }
        }
    }
}

impl HasSampleValues for SimpleResourceBounds {
    fn sample() -> Self {
        Self::fungible(SimpleFungibleResourceBounds::sample())
    }

    fn sample_other() -> Self {
        Self::non_fungible(SimpleNonFungibleResourceBounds::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SimpleResourceBounds;

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
    fn from_scrypto_fungible() {
        let scrypto = ScryptoSimpleResourceBounds::Fungible(
            ScryptoSimpleFungibleResourceBounds::Exact(Decimal::from(1).into()),
        );
        assert_eq!(SUT::from(scrypto), SUT::sample());
    }

    #[test]
    fn from_scrypto_non_fungible() {
        let scrypto = ScryptoSimpleResourceBounds::NonFungible(
            ScryptoSimpleNonFungibleResourceBounds::Exact {
                amount: Decimal::from(1).into(),
                certain_ids: vec![
                    NonFungibleLocalId::sample(),
                    NonFungibleLocalId::sample_other(),
                ]
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from)
                .collect(),
            },
        );
        assert_eq!(SUT::from(scrypto), SUT::sample_other());
    }
}
