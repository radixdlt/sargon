use serde::{Deserialize, Serialize};

use crate::{
    bip32::hd_path::HDPath,
    bip44::bip44_like_path::BIP44LikePath,
    cap26::cap26_path::{cap26_path::CAP26Path, paths::account_path::AccountPath},
};

use super::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "discriminator", content = "value")]
pub enum DerivationPath {
    CAP26(CAP26Path),
    BIP44Like(BIP44LikePath),
}

impl DerivationPath {
    pub fn placeholder() -> Self {
        Self::CAP26(CAP26Path::AccountPath(AccountPath::placeholder()))
    }
}

impl Derivation for DerivationPath {
    fn hd_path(&self) -> &HDPath {
        match self {
            DerivationPath::CAP26(path) => path.hd_path(),
            DerivationPath::BIP44Like(path) => path.hd_path(),
        }
    }
    fn scheme(&self) -> DerivationPathScheme {
        match self {
            DerivationPath::CAP26(p) => p.scheme(),
            DerivationPath::BIP44Like(p) => p.scheme(),
        }
    }
}
