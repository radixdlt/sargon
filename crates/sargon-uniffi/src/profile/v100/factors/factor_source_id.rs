use crate::prelude::*;
use sargon::FactorSourceID as InternalFactorSourceID;

/// A unique and stable identifier of a FactorSource, e.g. a
/// DeviceFactorSource being a mnemonic securely stored in a
/// device (phone), where the ID of it is the hash of a special
/// key derived near the root of it.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum FactorSourceID {
    /// FactorSourceID from the blake2b hash of the special HD public key derived at `CAP26::GetID`,
    /// for a certain `FactorSourceKind`
    Hash {
        value: FactorSourceIDFromHash,
    },

    /// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
    Address {
        value: FactorSourceIDFromAddress,
    },
}

impl From<InternalFactorSourceID> for FactorSourceID {
    fn from(value: InternalFactorSourceID) -> Self {
        match value {
            InternalFactorSourceID::Hash { value } => FactorSourceID::Hash {
                value: value.into(),
            },
            InternalFactorSourceID::Address { value } => FactorSourceID::Address {
                value: value.into(),
            },
        }
    }
}

impl Into<InternalFactorSourceID> for FactorSourceID {
    fn into(self) -> InternalFactorSourceID {
        match self {
            FactorSourceID::Hash { value } => InternalFactorSourceID::Hash {
                value: value.into(),
            },
            FactorSourceID::Address { value } => InternalFactorSourceID::Address {
                value: value.into(),
            },
        }
    }
}

#[uniffi::export]
pub fn factor_source_id_to_string(factor_source_id: &FactorSourceID) -> String {
    factor_source_id.into_internal().to_string()
}

#[uniffi::export]
pub fn new_factor_source_id_sample() -> FactorSourceID {
    InternalFactorSourceID::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_id_sample_other() -> FactorSourceID {
    InternalFactorSourceID::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceID;

    #[test]
    fn test_factor_source_id_to_string() {
        assert_eq!(
            factor_source_id_to_string(&SUT::sample()),
            SUT::sample().to_string()
        );
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_id_sample(),
                new_factor_source_id_sample_other(),
                // duplicates should get removed
                new_factor_source_id_sample(),
                new_factor_source_id_sample_other(),
            ])
            .len(),
            2
        );
    }
}
