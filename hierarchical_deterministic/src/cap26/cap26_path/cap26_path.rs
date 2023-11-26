use serde::{Deserialize, Serialize};

use crate::{
    bip32::hd_path::HDPath,
    derivation::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme},
};

use super::paths::{account_path::AccountPath, getid_path::GetIDPath};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "discriminator", content = "value")]
pub enum CAP26Path {
    GetID(GetIDPath),
    AccountPath(AccountPath),
}

impl Derivation for CAP26Path {
    fn hd_path(&self) -> &HDPath {
        match self {
            CAP26Path::AccountPath(path) => path.hd_path(),
            CAP26Path::GetID(path) => path.hd_path(),
        }
    }
    fn scheme(&self) -> DerivationPathScheme {
        match self {
            CAP26Path::AccountPath(p) => p.scheme(),
            CAP26Path::GetID(p) => p.scheme(),
        }
    }
}
