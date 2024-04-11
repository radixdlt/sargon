use crate::prelude::*;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct BIP39Seed([u8; 64]);

impl BIP39Seed {
    pub(crate) fn new(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }
}

use crypto::{
    keys::slip10::{self as IotaSlip10, Hardened as IotaSlip10PathComponent},
    signatures::ed25519 as IotaSlip10Ed25519,
    signatures::secp256k1_ecdsa as IotaSlip10Secp256k1,
};

impl HDPath {
    fn hardened_chain(&self) -> Vec<IotaSlip10PathComponent> {
        self.components
            .iter()
            .map(|c| c.value)
            .map(|v| IotaSlip10PathComponent::try_from(v).expect("Should work"))
            .collect_vec()
    }
}

impl BIP39Seed {
    fn derive_slip10_private_key<K, I>(&self, chain: I) -> IotaSlip10::Slip10<K>
    where
        K: IotaSlip10::IsSecretKey
            + IotaSlip10::WithSegment<<I as Iterator>::Item>,
        I: Iterator,
        <I as Iterator>::Item: IotaSlip10::Segment,
    {
        let iota_seed = IotaSlip10::Seed::from_bytes(&self.0);
        iota_seed.derive(chain)
    }

    fn derive_ed25519_private_key(&self, path: &HDPath) -> Ed25519PrivateKey {
        let ck = self
            .derive_slip10_private_key::<IotaSlip10Ed25519::SecretKey, _>(
                path.hardened_chain().into_iter(),
            );
        Ed25519PrivateKey::from_bytes(ck.secret_key().as_slice())
            .expect("Valid Ed25519PrivateKey bytes")
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
