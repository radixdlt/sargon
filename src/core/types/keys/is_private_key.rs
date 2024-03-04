use crate::prelude::*;

pub trait IsPrivateKey<P: IsPublicKey<Self::Signature>>: Sized {
    type Signature;

    fn curve() -> SLIP10Curve;

    fn public_key(&self) -> P;

    fn sign(&self, msg_hash: &Hash) -> Self::Signature;

    fn sign_intent_hash(&self, intent_hash: &IntentHash) -> IntentSignature
    where
        (P, Self::Signature): Into<SignatureWithPublicKey>,
    {
        let public_key: P = self.public_key();
        let signature = self.sign(&intent_hash.hash);
        let signature_with_public_key: SignatureWithPublicKey =
            (public_key, signature).into();

        signature_with_public_key.into()
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
