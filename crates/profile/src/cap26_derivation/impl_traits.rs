use crate::prelude::*;

impl Derivation for BIP44LikePath {
    fn curve(&self) -> SLIP10Curve {
        self.scheme().curve()
    }

    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::BIP44Like {
            value: self.clone(),
        }
    }
    fn hd_path(&self) -> &HDPath {
        &self.path
    }
}

#[uniffi::export]
pub fn bip44_like_path_get_address_index(path: &BIP44LikePath) -> HDPathValue {
    path.last_component().index()
}

impl BIP44LikePath {
    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Bip44Olympia
    }
}

pub trait HDPrivateKeyDeriving {
    fn derive_private_key<D>(
        &self,
        derivation: &D,
    ) -> HierarchicalDeterministicPrivateKey
    where
        D: Derivation;
}

impl HDPrivateKeyDeriving for BIP39Seed {
    fn derive_private_key<D>(
        &self,
        derivation: &D,
    ) -> HierarchicalDeterministicPrivateKey
    where
        D: Derivation,
    {
        match derivation.curve() {
            SLIP10Curve::Curve25519 => {
                let key = self.derive_ed25519_private_key(derivation.hd_path());
                HierarchicalDeterministicPrivateKey::new(
                    key.into(),
                    derivation.derivation_path(),
                )
            }
            SLIP10Curve::Secp256k1 => {
                let key =
                    self.derive_secp256k1_private_key(derivation.hd_path());
                HierarchicalDeterministicPrivateKey::new(
                    key.into(),
                    derivation.derivation_path(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bip44_like_path_get_address_index() {
        assert_eq!(bip44_like_path_get_address_index(&SUT::sample_other()), 1)
    }
}
