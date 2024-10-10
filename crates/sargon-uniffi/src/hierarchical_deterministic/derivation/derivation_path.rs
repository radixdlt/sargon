use crate::prelude::*;
use sargon::DerivationPath as InternalDerivationPath;

/// A derivation path on either supported schemes, either Babylon (CAP26) or Olympia (BIP44Like).
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
    uniffi::Enum,
)]
pub enum DerivationPath {
    CAP26 { value: CAP26Path },
    BIP44Like { value: BIP44LikePath },
}

impl From<InternalDerivationPath> for DerivationPath {
    fn from(value: InternalDerivationPath) -> Self {
        match value {
            InternalDerivationPath::CAP26 { value } => Self::CAP26 {
                value: value.into(),
            },
            InternalDerivationPath::BIP44Like { value } => Self::BIP44Like {
                value: value.into(),
            },
        }
    }
}

impl Into<InternalDerivationPath> for DerivationPath {
    fn into(self) -> InternalDerivationPath {
        match self {
            DerivationPath::CAP26 { value } => InternalDerivationPath::CAP26 { value: value.into() },
            DerivationPath::BIP44Like { value } => {
                InternalDerivationPath::BIP44Like { value: value.into() }
            }
        }
    }
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
    InternalDerivationPath::from_str(&string).map_result()
}

#[uniffi::export]
pub fn derivation_path_to_hd_path(path: &DerivationPath) -> HDPath {
    match path {
        DerivationPath::CAP26 { value } => value.path.clone().into(),
        DerivationPath::BIP44Like { value } => value.path.clone().into(),
        
    }
}

#[uniffi::export]
pub fn derivation_path_to_string(path: &DerivationPath) -> String {
    path.into_internal().to_string()
}

