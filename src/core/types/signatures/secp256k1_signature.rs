use crate::prelude::*;

use radix_engine_common::crypto::Secp256k1Signature as ScryptoSecp256k1Signature;

/// Represents an Secp256k1 signature.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    derive_more::FromStr,
    uniffi::Record,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Secp256k1Signature {
    // recovery id + signature
    pub bytes: Hex65Bytes,
}

impl From<ScryptoSecp256k1Signature> for Secp256k1Signature {
    fn from(value: ScryptoSecp256k1Signature) -> Self {
        Self {
            bytes: Hex65Bytes::from_bytes(&value.0),
        }
    }
}

impl From<Secp256k1Signature> for ScryptoSecp256k1Signature {
    fn from(value: Secp256k1Signature) -> Self {
        ScryptoSecp256k1Signature(value.bytes.bytes())
    }
}

impl Secp256k1Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }
}

impl HasPlaceholder for Secp256k1Signature {
    /// Returns a valid Secp256k1Signature, see doc test below,
    /// with the value:
    ///
    /// `"018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef"`
    ///
    /// ```
    /// extern crate sargon;
    /// use sargon::prelude::*;
    ///
    /// let mnemonic: Mnemonic = "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van".parse().unwrap();
    ///
    /// let path: BIP44LikePath = "m/44H/1022H/0H/0/5H".parse().unwrap();
    ///
    /// let mwp = MnemonicWithPassphrase::with_passphrase(mnemonic, BIP39Passphrase::default());
    ///
    /// let hd_private_key = mwp.derive_private_key(path);
    ///
    /// assert_eq!(&hd_private_key.private_key.to_hex(), "111323d507d9d690836798e3ef2e5292cfd31092b75b9b59fa584ff593a3d7e4");
    ///
    /// assert_eq!(&hd_private_key.public_key().to_hex(), "03e78cdb2e0b7ea6e55e121a58560ccf841a913d3a4a9b8349e0ef00c2102f48d8");
    ///
    /// let message = "There is a computer disease that anybody who works with computers knows about. It's a very serious disease and it interferes completely with the work. The trouble with computers is that you 'play' with them!";
    ///
    /// let hash = hash(message.as_bytes());
    ///
    /// let signature: Secp256k1Signature = "018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef".parse().unwrap();
    ///
    /// assert_eq!(
    ///     &hd_private_key
    ///     .private_key
    ///     .public_key()
    ///     .into_secp256k1()
    ///     .unwrap()
    ///     .is_valid(&signature, &hash),
    ///     &true
    /// );
    /// ```
    ///
    fn placeholder() -> Self {
        "018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef".parse().expect("Should construct valid placeholders.")
    }

    /// Returns a valid Secp256k1Signature, see doc test below,
    /// with the value:
    ///
    /// `"01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd"`
    ///
    /// ```
    /// extern crate sargon;
    /// use sargon::prelude::*;
    ///
    /// let mnemonic: Mnemonic = "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van".parse().unwrap();
    ///
    /// let path: BIP44LikePath = "m/44H/1022H/0H/0/1H".parse().unwrap();
    ///
    /// let mwp = MnemonicWithPassphrase::with_passphrase(mnemonic, BIP39Passphrase::default());
    ///
    /// let hd_private_key = mwp.derive_private_key(path);
    ///
    /// assert_eq!(&hd_private_key.private_key.to_hex(), "84d8a5991e8f2885fe49d77da0ee6ee9f3f03ef419ac9c19a48cd32e10244ecd");
    ///
    /// assert_eq!(&hd_private_key.public_key().to_hex(), "02f0d85a3b9082683f689e6115f37e1e24b7448fff14b14877e3a4e750e86fba8b");
    ///
    /// let message = "All those moments will be lost in time, like tears in rain. Time to die...";
    ///
    /// let hash = hash(message.as_bytes());
    ///
    /// let signature: Secp256k1Signature = "01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd".parse().unwrap();
    ///
    /// assert_eq!(
    ///     &hd_private_key
    ///     .private_key
    ///     .public_key()
    ///     .into_secp256k1()
    ///     .unwrap()
    ///     .is_valid(&signature, &hash),
    ///     &true
    /// );
    /// ```
    ///
    fn placeholder_other() -> Self {
        "01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd".parse().expect("Should construct valid placeholders.")
    }
}

#[cfg(test)]
mod tests {
    use crate::HasPlaceholder;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Secp256k1Signature;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn scrypto_roundtrip() {
        let sut = SUT::placeholder();
        assert_eq!(
            Into::<SUT>::into(Into::<ScryptoSecp256k1Signature>::into(
                sut.clone()
            )),
            sut
        );
    }

    #[test]
    fn scrypto_roundtrip_start_scrypto() {
        let sig: ScryptoSecp256k1Signature = "01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd".parse().unwrap();
        assert_eq!(
            Into::<ScryptoSecp256k1Signature>::into(Into::<SUT>::into(
                sig.clone()
            )),
            sig
        );
    }

    #[test]
    fn to_hex() {
        assert_eq!(SUT::placeholder().to_hex(), "018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef");
    }
}
