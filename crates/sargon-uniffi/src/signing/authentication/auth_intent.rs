use crate::prelude::*;
use sargon::AuthIntent as InternalAuthIntent;
use std::hash::Hasher;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct AuthIntent {
    /// The challenge nonce that with some `metadata` values are generating the `RolaChallenge`
    /// needed to be signed
    pub challenge_nonce: Exactly32Bytes,

    /// The `NetworkID` on which the request was made
    pub network_id: NetworkID,

    /// The origin `Url` of the dApp from which the request was made
    pub origin: Url,

    /// The dApp's definition address
    pub dapp_definition_address: DappDefinitionAddress,

    /// The entities needed to be signed.
    pub entities_to_sign: Vec<AddressOfAccountOrPersona>,
}

/// Since `AuthIntent` is also acting as a payload in `SignaturesCollector` when signing auth,
/// needs to conform to Hash too. For other `Signable`s like `TransactionIntent` or `Subintent`
/// there are specific compiled versions of them like `CompiledTransactionIntent` and
/// `CompiledSubintent` respectively, which conform to Hash.
impl std::hash::Hash for AuthIntent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.into_internal().hash(state);
    }
}

impl AuthIntent {
    pub fn into_internal(&self) -> InternalAuthIntent {
        self.clone().into()
    }
}

impl From<InternalAuthIntent> for AuthIntent {
    fn from(value: InternalAuthIntent) -> Self {
        Self {
            challenge_nonce: value.challenge_nonce.into(),
            network_id: value.network_id.into(),
            origin: value.origin,
            dapp_definition_address: value.dapp_definition_address.into(),
            entities_to_sign: value
                .entities_to_sign
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<AuthIntent> for InternalAuthIntent {
    fn from(value: AuthIntent) -> Self {
        Self::new(
            value.challenge_nonce.into(),
            value.network_id.into(),
            value.origin,
            value.dapp_definition_address.into(),
            value.entities_to_sign.into_iter().map(Into::into).collect(),
        )
    }
}

#[uniffi::export]
pub fn new_auth_intent_from_request(
    challenge_nonce: DappToWalletInteractionAuthChallengeNonce,
    metadata: DappToWalletInteractionMetadata,
    entities_to_sign: Vec<AddressOfAccountOrPersona>,
) -> Result<AuthIntent> {
    InternalAuthIntent::new_from_request(
        challenge_nonce.into(),
        metadata.into(),
        entities_to_sign.into_iter().map(|a| a.into_internal()),
    )
    .into_result()
}

#[uniffi::export]
pub fn auth_intent_get_hash(auth_intent: AuthIntent) -> AuthIntentHash {
    auth_intent.into_internal().auth_intent_hash().into()
}

#[uniffi::export]
pub fn new_auth_intent_sample() -> AuthIntent {
    InternalAuthIntent::sample().into()
}

#[uniffi::export]
pub fn new_auth_intent_sample_other() -> AuthIntent {
    InternalAuthIntent::sample_other().into()
}
