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

#[uniffi::export]
fn new_vec_of_factor_source_id_from_hash_from_json(
    json_bytes: BagOfBytes,
) -> Result<Vec<FactorSourceIDFromHash>> {
    sargon::new_vec_of_factor_source_id_from_hash_from_json(
        json_bytes.into_internal(),
    )
    .into_iter_result()
}

#[uniffi::export]
fn vec_of_factor_source_id_from_hash_to_json(
    ids: Vec<FactorSourceIDFromHash>,
) -> Result<BagOfBytes> {
    sargon::vec_of_factor_source_id_from_hash_to_json(ids.into_internal())
        .into_result()
}
