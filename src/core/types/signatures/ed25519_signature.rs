use crate::prelude::*;

json_string_convertible!(Ed25519Signature);

/// Represents an ED25519 signature.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
    derive_more::FromStr,
    uniffi::Record,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Ed25519Signature {
    pub bytes: Exactly64Bytes,
}

impl From<Exactly64Bytes> for Ed25519Signature {
    fn from(value: Exactly64Bytes) -> Self {
        Self { bytes: value }
    }
}

impl TryFrom<BagOfBytes> for Ed25519Signature {
    type Error = CommonError;
    fn try_from(value: BagOfBytes) -> Result<Self> {
        Exactly64Bytes::try_from(value).map(Self::from)
    }
}

impl From<ScryptoEd25519Signature> for Ed25519Signature {
    fn from(value: ScryptoEd25519Signature) -> Self {
        Self::from(Exactly64Bytes::from(&value.0))
    }
}

impl From<Ed25519Signature> for ScryptoEd25519Signature {
    fn from(value: Ed25519Signature) -> Self {
        ScryptoEd25519Signature(*value.bytes.bytes())
    }
}

impl Ed25519Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }
}

impl HasSampleValues for Ed25519Signature {
    /// Returns a valid Ed25519Signature, see doc test below,
    /// with the value:
    ///
    /// `"2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b"`
    ///
    /// ```
    /// extern crate sargon;
    /// use sargon::prelude::*;
    ///
    /// let mnemonic: Mnemonic = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate".parse().unwrap();
    ///
    /// let path: AccountPath = "m/44H/1022H/1H/525H/1460H/0H".parse().unwrap();
    ///
    /// let mwp = MnemonicWithPassphrase::with_passphrase(mnemonic, BIP39Passphrase::new("radix"));
    ///
    /// let seed = mwp.to_seed();
    /// let hd_private_key = seed.derive_private_key(&path);
    ///
    /// assert_eq!(&hd_private_key.private_key.to_hex(), "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003");
    ///
    /// assert_eq!(&hd_private_key.public_key().to_hex(), "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b");
    ///
    /// let message = "There is a computer disease that anybody who works with computers knows about. It's a very serious disease and it interferes completely with the work. The trouble with computers is that you 'play' with them!";
    ///
    /// let hash = hash_of(message.as_bytes());
    ///
    /// let signature: Ed25519Signature = "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b".parse().unwrap();
    ///
    /// assert_eq!(
    ///     &hd_private_key
    ///     .private_key
    ///     .public_key()
    ///     .into_ed25519()
    ///     .unwrap()
    ///     .is_valid_signature_for_hash(&signature, &hash),
    ///     &true
    /// );
    /// ```
    ///
    fn sample() -> Self {
        "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b".parse().expect("Should produce a valid sample Ed25519Signature")
    }

    /// Returns a valid Ed25519Signature, see doc test below,
    /// with the value:
    ///
    /// `"06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09"`
    ///
    /// ```
    /// extern crate sargon;
    /// use sargon::prelude::*;
    ///
    /// let mnemonic: Mnemonic = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate".parse().unwrap();
    ///
    /// let path: AccountPath = "m/44H/1022H/1H/525H/1460H/1H".parse().unwrap();
    ///
    /// let mwp = MnemonicWithPassphrase::with_passphrase(mnemonic, BIP39Passphrase::new("radix"));
    /// let seed = mwp.to_seed();
    /// let hd_private_key = seed.derive_private_key(&path);
    ///
    /// assert_eq!(&hd_private_key.private_key.to_hex(), "6b736e59d41c5ba47dc427ebee9990426441e01db4abee5c44192492c269d8e0");
    ///
    /// assert_eq!(&hd_private_key.public_key().to_hex(), "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf");
    ///
    /// let message = "All those moments will be lost in time, like tears in rain. Time to die...";
    ///
    /// let hash = hash_of(message.as_bytes());
    ///
    /// let signature: Ed25519Signature = "06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09".parse().unwrap();
    ///
    /// assert_eq!(
    ///     &hd_private_key
    ///     .private_key
    ///     .public_key()
    ///     .into_ed25519()
    ///     .unwrap()
    ///     .is_valid_signature_for_hash(&signature, &hash),
    ///     &true
    /// );
    /// ```
    ///
    fn sample_other() -> Self {
        "06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09".parse().expect("Should produce a valid sample Ed25519Signature")
    }
}

#[cfg(test)]
mod tests {
    use crate::HasSampleValues;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Ed25519Signature;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn scrypto_roundtrip() {
        let sut = SUT::sample();
        assert_eq!(SUT::from(ScryptoEd25519Signature::from(sut)), sut);
    }

    #[test]
    fn scrypto_roundtrip_start_scrypto() {
        let sig: ScryptoEd25519Signature = "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b".parse().unwrap();
        assert_eq!(ScryptoEd25519Signature::from(SUT::from(sig)), sig);
    }

    #[test]
    fn to_hex() {
        assert_eq!(SUT::sample_other().to_hex(), "06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09");
    }
}
