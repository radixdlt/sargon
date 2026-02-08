use crate::prelude::*;
/// A tagged union of supported private keys on different curves, supported
/// curves are `secp256k1` and `Curve25519`
#[derive(Debug, PartialEq, Eq, EnumAsInner)]
pub enum PrivateKey {
    /// An Ed25519 private key used to create cryptographic signatures, using EdDSA scheme.
    Ed25519(Ed25519PrivateKey),

    /// A secp256k1 private key used to create cryptographic signatures,
    /// more specifically ECDSA signatures, that offer recovery of the public key
    Secp256k1(Secp256k1PrivateKey),
}

impl From<Ed25519PrivateKey> for PrivateKey {
    /// Enables:
    ///
    /// ```
    /// use ecc::prelude::*;
    ///
    /// let key: PublicKey = Ed25519PrivateKey::generate().public_key().into();
    /// ```
    fn from(value: Ed25519PrivateKey) -> Self {
        Self::Ed25519(value)
    }
}

impl From<Secp256k1PrivateKey> for PrivateKey {
    /// Enables:
    ///
    /// ```
    /// use ecc::prelude::*;
    ///
    /// let key: PublicKey = Secp256k1PrivateKey::generate().public_key().into();
    /// ```
    fn from(value: Secp256k1PrivateKey) -> Self {
        Self::Secp256k1(value)
    }
}

impl PrivateKey {
    /// Calculates the public key of the inner `PrivateKey` and wraps it
    /// in the `PublicKey` tagged union.
    pub fn public_key(&self) -> PublicKey {
        match self {
            PrivateKey::Ed25519(key) => PublicKey::Ed25519(key.public_key()),
            PrivateKey::Secp256k1(key) => {
                PublicKey::Secp256k1(key.public_key())
            }
        }
    }

    pub fn sign(&self, msg_hash: &Hash) -> Signature {
        match self {
            PrivateKey::Ed25519(key) => Signature::from(key.sign(msg_hash)),
            PrivateKey::Secp256k1(key) => Signature::from(key.sign(msg_hash)),
        }
    }

    /// Returns the hex representation of the inner private key's bytes as a `Vec`.
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            PrivateKey::Ed25519(key) => key.to_bytes(),
            PrivateKey::Secp256k1(key) => key.to_bytes(),
        }
    }

    /// Returns the hex representation of the inner private key as a `String`.
    pub fn to_hex(&self) -> String {
        match self {
            PrivateKey::Ed25519(key) => key.to_hex(),
            PrivateKey::Secp256k1(key) => key.to_hex(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn private_key_ed25519_into_as_roundtrip() {
        let bytes = generate_32_bytes();
        // test `into``
        let private_key: PrivateKey =
            Ed25519PrivateKey::from_vec(bytes.clone()).unwrap().into();
        // test `as`
        assert_eq!(
            private_key.as_ed25519().unwrap(),
            &Ed25519PrivateKey::from_vec(bytes).unwrap()
        );
    }

    #[test]
    fn private_key_ed25519_into_as_wrong_fails() {
        let bytes = generate_32_bytes();
        // test `into``
        let private_key: PrivateKey =
            Ed25519PrivateKey::from_vec(bytes.clone()).unwrap().into();
        // test `as`
        assert!(private_key.as_secp256k1().is_none());
    }

    #[test]
    fn private_key_secp256k1_into_as_roundtrip() {
        let bytes = generate_32_bytes();
        // test `into``
        let private_key: PrivateKey =
            Secp256k1PrivateKey::from_vec(bytes.clone()).unwrap().into();
        // test `as`
        assert_eq!(
            private_key.as_secp256k1().unwrap(),
            &Secp256k1PrivateKey::from_vec(bytes).unwrap()
        );
    }

    #[test]
    fn private_key_secp256k1_into_as_wrong_fails() {
        let bytes = generate_32_bytes();
        // test `into``
        let private_key: PrivateKey =
            Secp256k1PrivateKey::from_vec(bytes.clone()).unwrap().into();
        // test `as`
        assert!(private_key.as_ed25519().is_none());
    }

    #[test]
    fn generate_new() {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            let key: PrivateKey = Ed25519PrivateKey::generate().into();
            let bytes = key.to_bytes();
            assert_eq!(bytes.len(), 32);
            set.insert(bytes);
        }
        assert_eq!(set.len(), n);
    }

    #[test]
    fn secp256k1_to_bytes() {
        let bytes = generate_32_bytes();
        let key = Secp256k1PrivateKey::from_bytes(&bytes).unwrap();
        let private_key: PrivateKey = key.into();
        assert_eq!(private_key.to_bytes(), bytes);
    }
}
