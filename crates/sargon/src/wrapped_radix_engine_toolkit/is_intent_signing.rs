use crate::prelude::*;

pub trait BaseIsTransactionIntentHashSigning {
    fn sign_transaction_intent_hash(
        &self,
        transaction_intent_hash: &TransactionIntentHash,
    ) -> IntentSignature;
}

pub trait BaseHashNotarizing {
    fn notarize_hash(
        &self,
        signed_transaction_intent_hash: &SignedTransactionIntentHash,
    ) -> NotarySignature;
}

pub trait IsTransactionIntentHashSigning<P, S>
where
    (P, S): Into<SignatureWithPublicKey>,
{
    fn sign_transaction_intent_hash(
        &self,
        transaction_intent_hash: &TransactionIntentHash,
    ) -> IntentSignature;
}

impl BaseHashNotarizing for PrivateKey {
    fn notarize_hash(
        &self,
        signed_transaction_intent_hash: &SignedTransactionIntentHash,
    ) -> NotarySignature {
        self.sign(&signed_transaction_intent_hash.hash).into()
    }
}

impl BaseIsTransactionIntentHashSigning for PrivateKey {
    fn sign_transaction_intent_hash(
        &self,
        transaction_intent_hash: &TransactionIntentHash,
    ) -> IntentSignature {
        match self {
            PrivateKey::Ed25519(key) => SignatureWithPublicKey::Ed25519 {
                public_key: key.public_key(),
                signature: key.sign(&transaction_intent_hash.hash),
            },
            PrivateKey::Secp256k1(key) => SignatureWithPublicKey::Secp256k1 {
                public_key: key.public_key(),
                signature: key.sign(&transaction_intent_hash.hash),
            },
        }
        .into()
    }
}

impl<P, T> IsTransactionIntentHashSigning<P, T::Signature> for T
where
    T: IsPrivateKey<P>,
    P: IsPublicKey<T::Signature>,
    (P, T::Signature): Into<SignatureWithPublicKey>,
{
    fn sign_transaction_intent_hash(
        &self,
        transaction_intent_hash: &TransactionIntentHash,
    ) -> IntentSignature {
        let public_key: P = self.public_key();
        let signature = self.sign(&transaction_intent_hash.hash);
        let tuple: SignatureWithPublicKey = (public_key, signature).into();
        tuple.into()
    }
}

pub trait IsSubIntentHashSigning<P: IsPublicKey<Self::Signature>>:
    IsPrivateKey<P>
{
    fn sign_subintent_hash(
        &self,
        subintent_hash: &SubintentHash,
    ) -> IntentSignature
    where
        (P, Self::Signature): Into<SignatureWithPublicKey>;
}
impl<P, T> IsSubIntentHashSigning<P> for T
where
    T: IsPrivateKey<P>,
    P: IsPublicKey<T::Signature>,
{
    fn sign_subintent_hash(
        &self,
        subintent_hash: &SubintentHash,
    ) -> IntentSignature
    where
        (P, Self::Signature): Into<SignatureWithPublicKey>,
    {
        let public_key: P = self.public_key();
        let signature = self.sign(&subintent_hash.hash);
        let tuple: SignatureWithPublicKey = (public_key, signature).into();
        tuple.into()
    }
}

pub trait IsNotaryHashSigning<P: IsPublicKey<Self::Signature>>:
    IsPrivateKey<P>
{
    fn notarize_hash(
        &self,
        signed_transaction_intent_hash: &SignedTransactionIntentHash,
    ) -> NotarySignature
    where
        Self::Signature: Into<NotarySignature>;
}
impl<P, T> IsNotaryHashSigning<P> for T
where
    T: IsPrivateKey<P>,
    P: IsPublicKey<T::Signature>,
{
    fn notarize_hash(
        &self,
        signed_transaction_intent_hash: &SignedTransactionIntentHash,
    ) -> NotarySignature
    where
        Self::Signature: Into<NotarySignature>,
    {
        self.sign(&signed_transaction_intent_hash.hash).into()
    }
}
