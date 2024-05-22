use crate::prelude::*;

use crypto::{
    keys::slip10::{self as IotaSlip10, Hardened as IotaSlip10PathComponent},
    signatures::ed25519 as IotaSlip10Ed25519,
    signatures::secp256k1_ecdsa as IotaSlip10Secp256k1,
};

uniffi::custom_newtype!(BIP39Seed, Exactly64Bytes);

/// A BIP39 seed for hierarchal deterministic wallets, as per the [BIP39 standard][doc].
///
/// We typically obtain this by calling [`to_seed` on `MnemonicWithPassphrase`][MnemonicWithPassphrase::to_seed].
///
/// [doc]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki#user-content-From_mnemonic_to_seed
#[derive(
    Zeroize,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    Hash,
)]
#[serde(transparent)]
#[display("<OBFUSCATED>")]
pub struct BIP39Seed(pub Exactly64Bytes);

impl BIP39Seed {
    pub fn is_zeroized(&self) -> bool {
        self.0.as_ref() == [0; 64]
    }
}

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
        let iota_seed = IotaSlip10::Seed::from_bytes(self.0.as_ref());
        iota_seed.derive(chain)
        // `IotaSlip10::Seed` implements `ZeroizeOnDrop` so should now be zeroized.
    }

    pub fn derive_ed25519_private_key(
        &self,
        path: &HDPath,
    ) -> Ed25519PrivateKey {
        let ck = self
            .derive_slip10_private_key::<IotaSlip10Ed25519::SecretKey, _>(
                path.hardened_chain().into_iter(),
            );
        Ed25519PrivateKey::from_bytes(ck.secret_key().as_slice())
            .expect("Valid Ed25519PrivateKey bytes")
        // `IotaSlip10Ed25519::SecretKey` implements `ZeroizeOnDrop` so should now be zeroized.
    }

    pub fn derive_secp256k1_private_key(
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
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = BIP39Seed;

//     #[test]
//     fn manual_uniffi_conversion() {
//         let bytes = Exactly64Bytes::sample();
//         let builtin: BagOfBytes = bytes.clone().as_ref().into();
//         let sut = new_b_i_p39_seed_from_bytes(builtin.clone()).unwrap();
//         let rust_side = sut.secret_magic;

//         let ffi_side =
//         <BIP39SeedSecretMagic as crate::UniffiCustomTypeConverter>::from_custom(
//             rust_side,
//         );

//         assert_eq!(ffi_side.to_hex(), builtin.to_hex());

//         let from_ffi_side =
//         <BIP39SeedSecretMagic as crate::UniffiCustomTypeConverter>::into_custom(
//             ffi_side,
//         )
//         .unwrap();

//         assert_eq!(
//             new_b_i_p39_seed_from_bytes(builtin.clone())
//                 .unwrap()
//                 .secret_magic
//                 .0,
//             from_ffi_side.0
//         );
//     }
// }
