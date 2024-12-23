// use crate::prelude::*;

// pub trait IsTransactionIntentHashSigning {
//     fn sign_transaction_intent_hash(
//         &self,
//         transaction_intent_hash: &TransactionIntentHash,
//     ) -> IntentSignature
//     where
//         (P, Self::Signature): Into<SignatureWithPublicKey>;
// }

// impl<T: IsPrivateKey> IsTransactionIntentHashSigning for T {
//     fn sign_transaction_intent_hash(
//         &self,
//         transaction_intent_hash: &TransactionIntentHash,
//     ) -> IntentSignature
//     where
//         (P, Self::Signature): Into<SignatureWithPublicKey>,
//     {
//         let public_key: P = self.public_key();
//         let signature = self.sign(&transaction_intent_hash.hash);
//         let tuple: SignatureWithPublicKey = (public_key, signature).into();
//         tuple.into()
//     }
// }
// pub trait IsSubIntentHashSigning {
//     fn sign_subintent_hash(
//         &self,
//         subintent_hash: &SubintentHash,
//     ) -> IntentSignature
//     where
//         (P, Self::Signature): Into<SignatureWithPublicKey>;
// }
// impl<T: IsPrivateKey> IsSubIntentHashSigning for T {
//     fn sign_subintent_hash(
//         &self,
//         subintent_hash: &SubintentHash,
//     ) -> IntentSignature
//     where
//         (P, Self::Signature): Into<SignatureWithPublicKey>,
//     {
//         let public_key: P = self.public_key();
//         let signature = self.sign(&subintent_hash.hash);
//         let tuple: SignatureWithPublicKey = (public_key, signature).into();
//         tuple.into()
//     }
// }

// pub trait IsNotaryHashSigning {
//     fn notarize_hash(
//         &self,
//         signed_transaction_intent_hash: &SignedTransactionIntentHash,
//     ) -> NotarySignature
//     where
//         Self::Signature: Into<NotarySignature>;
// }
// impl<T: IsPrivateKey> IsNotaryHashSigning for T {
//     fn notarize_hash(
//         &self,
//         signed_transaction_intent_hash: &SignedTransactionIntentHash,
//     ) -> NotarySignature
//     where
//         Self::Signature: Into<NotarySignature>,
//     {
//         self.sign(&signed_transaction_intent_hash.hash).into()
//     }
// }
