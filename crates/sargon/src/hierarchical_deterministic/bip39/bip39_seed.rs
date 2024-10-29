use crate::{decl_secret_bytes, prelude::*};

decl_secret_bytes!(
    /// A BIP39 seed for hierarchal deterministic wallets, as per the [BIP39 standard][doc].
    ///
    /// We typically obtain this by calling [`to_seed` on `MnemonicWithPassphrase`][MnemonicWithPassphrase::to_seed].
    ///
    /// [doc]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki#user-content-From_mnemonic_to_seed
    BIP39Seed,
    64
);

impl HDPath {
    fn hardened_chain(&self) -> Vec<IotaSlip10PathComponent> {
        self.components()
            .iter()
            .map(|c| c.map_to_global_key_space())
            .map(|v| IotaSlip10PathComponent::try_from(v).expect("Should work"))
            .collect::<Vec<IotaSlip10PathComponent>>()
    }
}

use crypto::{
    keys::slip10::{self as IotaSlip10, Hardened as IotaSlip10PathComponent},
    signatures::ed25519 as IotaSlip10Ed25519,
    signatures::secp256k1_ecdsa as IotaSlip10Secp256k1,
};

impl BIP39Seed {
    fn _derive_slip10_private_key<K, I>(
        &self,
        chain: I,
    ) -> IotaSlip10::Slip10<K>
    where
        K: IotaSlip10::IsSecretKey
            + IotaSlip10::WithSegment<<I as Iterator>::Item>,
        I: Iterator,
        <I as Iterator>::Item: IotaSlip10::Segment,
    {
        let iota_seed = IotaSlip10::Seed::from_bytes(&*self.0);
        iota_seed.derive(chain)
    }

    fn _derive_ed25519_private_key(
        &self,
        path: &HDPath,
    ) -> IotaSlip10Ed25519::SecretKey {
        self._derive_slip10_private_key::<_, _>(
            path.hardened_chain().into_iter(),
        )
        .secret_key()
    }

    fn _derive_secp256k1_private_key(
        &self,
        path: &HDPath,
    ) -> IotaSlip10Secp256k1::SecretKey {
        self._derive_slip10_private_key::<_, _>(
            path.components()
                .iter()
                .cloned()
                .map(|c| c.map_to_global_key_space()),
        )
        .secret_key()
    }

    pub fn derive_secp256k1_private_key(
        &self,
        hd_path: impl Into<HDPath>,
    ) -> Secp256k1PrivateKey {
        let inner = self._derive_secp256k1_private_key(&hd_path.into());
        Secp256k1PrivateKey::from_bytes(&*inner.to_bytes())
            .expect("Valid Secp256k1PrivateKey bytes")
        // `IotaSlip10Ed25519::SecretKey` implements `ZeroizeOnDrop` so should now be zeroized.
    }

    pub fn derive_ed25519_private_key(
        &self,
        hd_path: impl Into<HDPath>,
    ) -> Ed25519PrivateKey {
        let inner = self._derive_ed25519_private_key(&hd_path.into());
        Ed25519PrivateKey::from_bytes(inner.as_slice())
            .expect("Valid Ed25519PrivateKey bytes")
        // `IotaSlip10Ed25519::SecretKey` implements `ZeroizeOnDrop` so should now be zeroized.
    }

    pub fn derive_private_key_curve(
        &self,
        curve: SLIP10Curve,
        path: impl Into<HDPath>,
    ) -> PrivateKey {
        match curve {
            SLIP10Curve::Curve25519 => {
                self.derive_ed25519_private_key(path).into()
            }
            SLIP10Curve::Secp256k1 => {
                self.derive_secp256k1_private_key(path).into()
            }
        }
    }

    pub fn derive_private_key<D>(
        &self,
        derivation: &D,
    ) -> HierarchicalDeterministicPrivateKey
    where
        D: Derivation,
    {
        let key = self.derive_private_key_curve(
            derivation.curve(),
            derivation.derivation_path(),
        );

        HierarchicalDeterministicPrivateKey::new(
            key,
            derivation.derivation_path(),
        )
    }
}

pub trait Derivation:
    Clone + Into<DerivationPath> + HasDerivationPathSchemeObjectSafe
{
    fn derivation_path(&self) -> DerivationPath {
        self.clone().into()
    }
}
impl<T: Clone + Into<DerivationPath> + HasDerivationPathSchemeObjectSafe>
    Derivation for T
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39Seed;

    #[test]
    fn zeroize() {
        let mut sut: SUT = MnemonicWithPassphrase::sample().to_seed();
        assert!(!sut.is_zeroized());
        sut.zeroize();
        assert!(sut.is_zeroized());
    }
}
