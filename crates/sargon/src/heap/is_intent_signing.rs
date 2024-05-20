use crate::prelude::*;

pub trait IsIntentSigning: IsPrivateKey {
    fn sign_intent_hash(&self, intent_hash: &IntentHash) -> IntentSignature
    where
        (P, Self::Signature): Into<SignatureWithPublicKey>,
    {
        let public_key: P = self.public_key();
        let signature = self.sign(&intent_hash.hash);
        let tuple: SignatureWithPublicKey = (public_key, signature).into();
        tuple.into()
    }

    fn notarize_hash(
        &self,
        signed_intent_hash: &SignedIntentHash,
    ) -> NotarySignature
    where
        Self::Signature: Into<NotarySignature>,
    {
        self.sign(&signed_intent_hash.hash).into()
    }
}

pub trait HashSigning {
    pub fn sign_intent_hash(&self, intent_hash: &IntentHash)
        -> IntentSignature;

    pub fn notarize_hash(
        &self,
        signed_intent_hash: &SignedIntentHash,
    ) -> NotarySignature;
}

impl HashSigning for PrivateKey {
    fn sign_intent_hash(&self, intent_hash: &IntentHash) -> IntentSignature {
        match self {
            PrivateKey::Ed25519(key) => SignatureWithPublicKey::Ed25519 {
                public_key: key.public_key(),
                signature: key.sign(&intent_hash.hash),
            },
            PrivateKey::Secp256k1(key) => SignatureWithPublicKey::Secp256k1 {
                public_key: key.public_key(),
                signature: key.sign(&intent_hash.hash),
            },
        }
        .into()
    }

    fn notarize_hash(
        &self,
        signed_intent_hash: &SignedIntentHash,
    ) -> NotarySignature {
        self.sign(&signed_intent_hash.hash).into()
    }
}
