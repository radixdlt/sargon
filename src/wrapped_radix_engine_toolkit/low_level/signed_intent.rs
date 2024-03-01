use crate::prelude::*;

use radix_engine_toolkit::functions::signed_intent::hash as RET_signed_intent_hash;
use transaction::model::{
    IntentSignatureV1 as ScryptoIntentSignature,
    IntentSignaturesV1 as ScryptoIntentSignatures,
    SignatureWithPublicKeyV1 as ScryptoSignatureWithPublicKey,
    SignedIntentHash as ScryptoSignedIntentHash,
    SignedIntentV1 as ScryptoSignedIntent,
};

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct SignedIntent {
    pub intent: TransactionIntent,
    pub intent_signatures: IntentSignatures,
}

impl SignedIntent {
    pub fn new(
        intent: TransactionIntent,
        intent_signatures: IntentSignatures,
    ) -> Self {
        Self {
            intent,
            intent_signatures,
        }
    }
}

impl SignedIntent {
    pub fn network_id(&self) -> NetworkID {
        self.intent.network_id()
    }

    pub fn hash(&self) -> Result<SignedIntentHash> {
        let scrypto_signed_intent: ScryptoSignedIntent = self.clone().into();
        RET_signed_intent_hash(&scrypto_signed_intent)
            .map_err(|e| {
                error!("Failed to hash signed intent, error: {:?}", e);
                CommonError::Unknown
            })
            .map(|h| ScryptoSignedIntentHash(h.hash))
            .map(|h| SignedIntentHash::from_scrypto(h, self.network_id()))
    }
}

impl TryFrom<ScryptoSignedIntent> for SignedIntent {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoSignedIntent) -> Result<Self, Self::Error> {
        let intent: TransactionIntent = value.intent.try_into()?;
        let intent_signatures: IntentSignatures =
            value.intent_signatures.into();
        Ok(Self {
            intent,
            intent_signatures,
        })
    }
}

impl From<SignedIntent> for ScryptoSignedIntent {
    fn from(value: SignedIntent) -> Self {
        Self {
            intent: value.intent.into(),
            intent_signatures: value.intent_signatures.into(),
        }
    }
}
