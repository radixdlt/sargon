use serde::{Deserialize, Serialize};

use crate::{
    bip32::hd_path::HDPath,
    bip44::bip44_like_path::BIP44LikePath,
    cap26::cap26_path::{cap26_path::CAP26Path, paths::account_path::AccountPath},
};

use super::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme};

/// A derivation path on either supported schemes, either Babylon (CAP26) or Olympia (BIP44Like).
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

impl DerivationPath {
    pub fn placeholder_cap26() -> Self {
        DerivationPath::CAP26(CAP26Path::placeholder_account())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bip44::bip44_like_path::BIP44LikePath,
        cap26::cap26_path::paths::account_path::AccountPath,
        derivation::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme},
    };

    use super::DerivationPath;
    #[test]
    fn cap26_scheme() {
        assert_eq!(
            DerivationPath::placeholder_cap26().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn cap26_hdpath() {
        assert_eq!(
            DerivationPath::placeholder_cap26().hd_path(),
            AccountPath::placeholder().hd_path()
        );
    }

    #[test]
    fn bip44like_scheme() {
        assert_eq!(
            DerivationPath::BIP44Like(BIP44LikePath::new(0)).scheme(),
            DerivationPathScheme::Bip44Olympia
        );
    }

    #[test]
    fn bip44like_hdpath() {
        assert_eq!(
            DerivationPath::BIP44Like(BIP44LikePath::new(0)).hd_path(),
            BIP44LikePath::new(0).hd_path()
        );
    }
}
