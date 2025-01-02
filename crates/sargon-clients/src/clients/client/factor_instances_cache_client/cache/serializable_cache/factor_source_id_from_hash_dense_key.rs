use crate::prelude::*;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Hash,
    derive_more::Display,
    derive_more::FromStr,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub struct FactorSourceIDFromHashDenseKey(FactorSourceIDFromHash);

impl From<FactorSourceIDFromHash> for FactorSourceIDFromHashDenseKey {
    fn from(value: FactorSourceIDFromHash) -> Self {
        Self(value)
    }
}

impl From<FactorSourceIDFromHashDenseKey> for FactorSourceIDFromHash {
    fn from(value: FactorSourceIDFromHashDenseKey) -> Self {
        value.0
    }
}
