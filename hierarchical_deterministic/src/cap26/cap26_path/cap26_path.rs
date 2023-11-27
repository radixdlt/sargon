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

impl CAP26Path {
    pub fn placeholder_account() -> Self {
        Self::AccountPath(AccountPath::placeholder())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cap26::cap26_path::paths::{account_path::AccountPath, getid_path::GetIDPath},
        derivation::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme},
    };

    use super::CAP26Path;

    #[test]
    fn scheme_account_path() {
        assert_eq!(
            CAP26Path::placeholder_account().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn scheme_getid_path() {
        assert_eq!(
            CAP26Path::GetID(GetIDPath::default()).scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn hdpath_account_path() {
        assert_eq!(
            CAP26Path::placeholder_account().hd_path(),
            AccountPath::placeholder().hd_path()
        );
    }

    #[test]
    fn hdpath_getid_path() {
        assert_eq!(
            CAP26Path::GetID(GetIDPath::default()).hd_path(),
            GetIDPath::default().hd_path()
        );
    }
}
