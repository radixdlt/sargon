use crate::prelude::*;
use sargon::Derivation;
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
