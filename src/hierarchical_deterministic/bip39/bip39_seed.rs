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
        self.components
            .iter()
            .map(|c| c.value)
            .map(|v| IotaSlip10PathComponent::try_from(v).expect("Should work"))
            .collect_vec()
    }
}

use crypto::{
    keys::slip10::{self as IotaSlip10, Hardened as IotaSlip10PathComponent},
    signatures::ed25519 as IotaSlip10Ed25519,
    signatures::secp256k1_ecdsa as IotaSlip10Secp256k1,
};

impl BIP39Seed {
    fn derive_slip10_private_key<K, I>(&self, chain: I) -> IotaSlip10::Slip10<K>
    where
        K: IotaSlip10::IsSecretKey
            + IotaSlip10::WithSegment<<I as Iterator>::Item>,
        I: Iterator,
        <I as Iterator>::Item: IotaSlip10::Segment,
    {
        let iota_seed = IotaSlip10::Seed::from_bytes(&*self.secret_magic.0);
        iota_seed.derive(chain)
        // `IotaSlip10::Seed` implements `ZeroizeOnDrop` so should now be zeroized.
    }

    fn derive_ed25519_private_key(&self, path: &HDPath) -> Ed25519PrivateKey {
        let ck = self
            .derive_slip10_private_key::<IotaSlip10Ed25519::SecretKey, _>(
                path.hardened_chain().into_iter(),
            );
        Ed25519PrivateKey::from_bytes(ck.secret_key().as_slice())
            .expect("Valid Ed25519PrivateKey bytes")
        // `IotaSlip10Ed25519::SecretKey` implements `ZeroizeOnDrop` so should now be zeroized.
    }

    pub(crate) fn derive_secp256k1_private_key(
        &self,
        path: &HDPath,
    ) -> Secp256k1PrivateKey {
        let ck = self
            .derive_slip10_private_key::<IotaSlip10Secp256k1::SecretKey, _>(
                path.components.iter().cloned().map(|c| c.value),
            );
        Secp256k1PrivateKey::from_bytes(&*ck.secret_key().to_bytes())
            .expect("Valid Secp256k1PrivateKey bytes")
        // `IotaSlip10Ed25519::SecretKey` implements `ZeroizeOnDrop` so should now be zeroized.
    }

    pub fn derive_private_key<D>(
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
