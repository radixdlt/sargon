use crate::prelude::*;

const ROLA_PREFIX: u8 = 0x52;

#[derive(
    Debug, Clone, PartialEq, Eq, derive_more::Display, std::hash::Hash,
)]
#[display("{}", self.payload.to_hex())]
pub struct AuthIntentHash {
    pub payload: BagOfBytes,
}

impl AuthIntentHash {
    pub fn hash(&self) -> Hash {
        hash_of(self.payload.clone())
    }
}

impl From<AuthIntentHash> for Hash {
    fn from(val: AuthIntentHash) -> Self {
        val.hash()
    }
}

impl HasSampleValues for AuthIntentHash {
    fn sample() -> Self {
        From::<AuthIntent>::from(AuthIntent::sample())
    }

    fn sample_other() -> Self {
        From::<AuthIntent>::from(AuthIntent::sample_other())
    }
}

impl From<AuthIntent> for AuthIntentHash {
    /// Constructs a payload to sign in conjunction with the `challenge_nonce` received and
    /// the `metadata` of the dApp that sent the request.
    ///
    /// The logic of constructing the payload is as follows:
    /// * Prefixes with constant `ROLA_PREFIX` (0x52)
    /// * Extends with the 32 raw bytes of the challenge
    /// * Pushes 1 byte which is the length of the bech32-encoded dapp-definition address
    /// * Extends with the bytes of the bech32-encoded dapp-definition address
    /// * Extends with the bytes of the origin UTF-8 encoded.
    fn from(value: AuthIntent) -> Self {
        let mut payload = Vec::<u8>::new();
        payload.push(ROLA_PREFIX);
        payload.extend(value.challenge_nonce.bytes());
        payload.push(value.dapp_definition_address.address().len() as u8);
        payload.extend(value.dapp_definition_address.address().bytes());
        payload.extend(value.origin.0.as_bytes());

        Self {
            payload: BagOfBytes::from(payload),
        }
    }
}
