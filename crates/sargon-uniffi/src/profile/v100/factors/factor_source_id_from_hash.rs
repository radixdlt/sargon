use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;

/// FactorSourceID from the blake2b hash of the special HD public key derived at `CAP26::GetID`,
/// for a certain `FactorSourceKind`
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct FactorSourceIDFromHash {
    /// The kind of the FactorSource this ID refers to, typically `device` or `ledger`.
    pub kind: FactorSourceKind,

    /// The blake2b hash of the special HD public key derived at `CAP26::GetID`.
    pub body: Exactly32Bytes,
}

delegate_display_debug_into!(FactorSourceIDFromHash, InternalFactorSourceIDFromHash);

impl From<InternalFactorSourceIDFromHash> for FactorSourceIDFromHash {
    fn from(value: InternalFactorSourceIDFromHash) -> Self {
        Self {
            kind: value.kind.into(),
            body: value.body.into(),
        }
    }
}

impl Into<InternalFactorSourceIDFromHash> for FactorSourceIDFromHash {
    fn into(self) -> InternalFactorSourceIDFromHash {
        InternalFactorSourceIDFromHash {
            kind: self.kind.into(),
            body: self.body.into(),
        }
    }
}

json_data_convertible!(FactorSourceIDFromHash);

#[uniffi::export]
pub fn factor_source_id_from_hash_to_string(
    factor_source_id: &FactorSourceIDFromHash,
) -> String {
    factor_source_id.into_internal().to_string()
}

#[uniffi::export]
pub fn new_factor_source_id_from_hash_from_mnemonic_with_passphrase(
    factor_source_kind: FactorSourceKind,
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
) -> FactorSourceIDFromHash {
    InternalFactorSourceIDFromHash::from_mnemonic_with_passphrase(
        factor_source_kind.into(),
        &mnemonic_with_passphrase.into(),
    ).into()
}

#[uniffi::export]
pub fn new_factor_source_id_from_hash_sample() -> FactorSourceIDFromHash {
    InternalFactorSourceIDFromHash::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_id_from_hash_sample_other() -> FactorSourceIDFromHash {
    InternalFactorSourceIDFromHash::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceIDFromHash;

    #[test]
    fn test_factor_source_id_from_hash_to_string() {
        assert_eq!(
            factor_source_id_from_hash_to_string(&SUT::sample()),
            SUT::sample().to_string()
        );
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_id_from_hash_sample(),
                new_factor_source_id_from_hash_sample_other(),
                // duplicates should get removed
                new_factor_source_id_from_hash_sample(),
                new_factor_source_id_from_hash_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_factor_source_id_from_hash_from_mnemonic_with_passphrase() {
        let kind = FactorSourceKind::sample();
        let sut = new_factor_source_id_from_hash_from_mnemonic_with_passphrase(
            kind,
            &MnemonicWithPassphrase::sample(),
        );
        assert_eq!(sut.kind, kind)
    }
}
