use crate::prelude::*;
use sargon::FactorSourceIDFromAddress as InternalFactorSourceIDFromAddress;

/// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct FactorSourceIDFromAddress {
    /// The kind of the FactorSource this ID refers to, typically `trustedContact`.
    pub kind: FactorSourceKind,

    /// An account address which the FactorSource this ID refers uses/needs.
    pub body: AccountAddress,
}

impl From<InternalFactorSourceIDFromAddress> for FactorSourceIDFromAddress {
    fn from(factor_source_id: InternalFactorSourceIDFromAddress) -> Self {
        Self {
            kind: factor_source_id.kind.into(),
            body: factor_source_id.body.into(),
        }
    }
}

impl Into<InternalFactorSourceIDFromAddress> for FactorSourceIDFromAddress {
    fn into(self) -> InternalFactorSourceIDFromAddress {
        InternalFactorSourceIDFromAddress {
            kind: self.kind.into(),
            body: self.body.into(),
        }
    }
}

json_data_convertible!(FactorSourceIDFromAddress);

#[uniffi::export]
pub fn factor_source_id_from_address_to_string(
    factor_source_id: &FactorSourceIDFromAddress,
) -> String {
    factor_source_id.into_internal().to_string()
}

#[uniffi::export]
pub fn new_factor_source_id_from_address_sample() -> FactorSourceIDFromAddress {
    InternalFactorSourceIDFromAddress::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_id_from_address_sample_other(
) -> FactorSourceIDFromAddress {
    InternalFactorSourceIDFromAddress::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceIDFromAddress;

    #[test]
    fn test_factor_source_id_from_address_to_string() {
        assert_eq!(
            factor_source_id_from_address_to_string(&SUT::sample()),
            SUT::sample().to_string()
        );
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_id_from_address_sample(),
                new_factor_source_id_from_address_sample_other(),
                // duplicates should get removed
                new_factor_source_id_from_address_sample(),
                new_factor_source_id_from_address_sample_other(),
            ])
            .len(),
            2
        );
    }
}
