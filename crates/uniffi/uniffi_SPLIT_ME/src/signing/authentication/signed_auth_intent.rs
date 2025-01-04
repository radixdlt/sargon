use crate::prelude::*;
use sargon::AddressOfAccountOrPersona as InternalAddressOfAccountOrPersona;
use sargon::IntentSignature as InternalIntentSignature;
use sargon::SignedAuthIntent as InternalSignedAuthIntent;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignedAuthIntent {
    pub intent: AuthIntent,
    pub intent_signatures_per_owner: Vec<IntentSignatureOfOwner>,
}

#[uniffi::export]
pub fn new_signed_auth_intent_sample() -> SignedAuthIntent {
    InternalSignedAuthIntent::sample().into()
}

#[uniffi::export]
pub fn new_signed_auth_intent_sample_other() -> SignedAuthIntent {
    InternalSignedAuthIntent::sample_other().into()
}

impl SignedAuthIntent {
    pub fn into_internal(&self) -> InternalSignedAuthIntent {
        self.clone().into()
    }
}

impl From<InternalSignedAuthIntent> for SignedAuthIntent {
    fn from(value: InternalSignedAuthIntent) -> Self {
        SignedAuthIntent {
            intent: value.intent.into(),
            intent_signatures_per_owner: value
                .intent_signatures_per_owner
                .iter()
                .map(|(owner, signature)| {
                    IntentSignatureOfOwner::new(
                        (*owner).into(),
                        (*signature).into(),
                    )
                })
                .collect(),
        }
    }
}

impl From<SignedAuthIntent> for InternalSignedAuthIntent {
    fn from(value: SignedAuthIntent) -> Self {
        Self {
            intent: value.intent.into(),
            intent_signatures_per_owner: value
                .intent_signatures_per_owner
                .into_iter()
                .map(|item| {
                    (
                        item.owner.into_internal(),
                        item.intent_signature.into_internal(),
                    )
                })
                .collect::<IndexMap<
                    InternalAddressOfAccountOrPersona,
                    InternalIntentSignature,
                >>(),
        }
    }
}
