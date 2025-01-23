use crate::prelude::*;
use sargon::DerivationPath as InternalDerivationPath;

/// A derivation path on either supported schemes, either Babylon (CAP26) or Olympia (BIP44Like).
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum DerivationPath {
    Account { value: AccountPath },
    Identity { value: IdentityPath },
    Bip44Like { value: BIP44LikePath },
}

#[uniffi::export]
pub fn new_derivation_path_sample() -> DerivationPath {
    InternalDerivationPath::sample().into()
}

#[uniffi::export]
pub fn new_derivation_path_sample_other() -> DerivationPath {
    InternalDerivationPath::sample_other().into()
}

#[uniffi::export]
pub fn new_derivation_path_from_string(
    string: String,
) -> Result<DerivationPath> {
    InternalDerivationPath::from_str(&string).into_result()
}

#[uniffi::export]
pub fn derivation_path_to_hd_path(path: &DerivationPath) -> HDPath {
    path.into_internal().to_hd_path().clone().into()
}

#[uniffi::export]
pub fn derivation_path_to_string(path: &DerivationPath) -> String {
    path.into_internal().to_string()
}

/// String representation of the path using BIP32 notation.
/// In sargon, paths in the securified space are printed with the `S` notation after the index,
/// for readability purposes.
///
/// The notation `{i}S` means `{i + 2^30}H`, and since `H` means `+ 2^31` we can
/// verbosely express `{i}S` as `{i + 2^30 + 2^31} (without the H)
///
/// Such paths need to be on BIP32 notation meaning that
/// an index of `"{i}S"` => `"{i + 2^30}H"` when communication with other external APIs,
/// e.g. using Ledger hardware wallet or Arculus.
#[uniffi::export]
pub fn derivation_path_to_bip32_string(path: &DerivationPath) -> String {
    path.into_internal().to_cap43_string()
}
