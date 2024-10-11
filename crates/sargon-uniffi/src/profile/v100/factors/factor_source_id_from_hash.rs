use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;

/// FactorSourceID from the blake2b hash of the special HD public key derived at `CAP26::GetID`,
/// for a certain `FactorSourceKind`
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct FactorSourceIDFromHash {
    /// The kind of the FactorSource this ID refers to, typically `device` or `ledger`.
    pub kind: FactorSourceKind,

    /// The blake2b hash of the special HD public key derived at `CAP26::GetID`.
    pub body: Exactly32Bytes,
}

delegate_display_debug_into!(
    FactorSourceIDFromHash,
    InternalFactorSourceIDFromHash
);

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
        factor_source_kind.into_internal(),
        &mnemonic_with_passphrase.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn new_factor_source_id_from_hash_sample() -> FactorSourceIDFromHash {
    InternalFactorSourceIDFromHash::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_id_from_hash_sample_other() -> FactorSourceIDFromHash {
    InternalFactorSourceIDFromHash::sample_other().into()
}
