use crate::prelude::*;
use crypto::keys::x25519::PublicKey as X25519PublicKey;
use crypto::keys::x25519::SecretKey as X25519PrivateKey;


/// ❗️ NOT PRODUCTION READY YET ❗️
/// A key derivation function which produces Encryption Keys from a set of
/// key exchange keys, by performing Diffie-Hellman key exchange on each
/// Key Exchange Key in a Set, by "folding" from left to right.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold;

impl HasSampleValues
    for SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold
{
    fn sample() -> Self {
        Self
    }

    fn sample_other() -> Self {
        Self
    }
}

impl Default
    for SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold
{
    fn default() -> Self {
        Self
    }
}

impl SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold {
    fn key_exchange_between_more_than_two_keys(
        &self,
        between: Vec<&X25519PrivateKey>,
    ) -> X25519PublicKey {
        let mut private_keys = between.clone();
        assert!(private_keys.len() > 2);
        let tail = private_keys.split_off(1);
        let head = private_keys.into_iter().last().unwrap();

        tail.into_iter().fold(head.public_key(), |acc_pub, x_priv| {
            let shared_secret = x_priv.diffie_hellman(&acc_pub);
            X25519PublicKey::from_bytes(shared_secret.to_bytes())
        })
    }

    fn key_exchange_between_all_combinations(
        &self,
        of: Vec<X25519PrivateKey>,
        minus: usize,
    ) -> Vec<X25519PublicKey> {
        let private_keys = of;

        let private_key_combinations =
            private_keys.iter().combinations(private_keys.len() - minus);

        private_key_combinations
            .map(|xs| self.key_exchange_between_more_than_two_keys(xs))
            .collect_vec()
    }

    fn encryption_keys_from_key_exchange_between_all_combinations(
        &self,
        of: Vec<X25519PrivateKey>,
        minus: usize,
    ) -> Vec<EncryptionKey> {
        let private_keys = of;
        let keys =
            self.key_exchange_between_all_combinations(private_keys, minus);
        keys.into_iter().map(EncryptionKey::from).collect_vec()
    }
}

impl SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold {
    pub fn derive_encryption_keys_from(
        &self,
        key_exchange_keys: Vec<X25519PrivateKey>,
    ) -> Vec<EncryptionKey> {
        let minus = 2;
        assert!((key_exchange_keys.len() - minus) > 1);

        self.encryption_keys_from_key_exchange_between_all_combinations(
            key_exchange_keys,
            minus,
        )
    }
}
