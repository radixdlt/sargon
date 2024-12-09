use crate::prelude::*;
use sargon::AuthenticationSigningInput as InternalAuthenticationSigningInput;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct AuthenticationSigningInput {
    /// The account or identity address of the entity which signs the rola challenge,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,

    /// The challenge nonce that with some `metadata` values are generating the `RolaChallenge`
    /// needed to be signed
    pub challenge_nonce: DappToWalletInteractionAuthChallengeNonce,

    /// The metadata that together with the `challenge_nonce` are generating the `RolaChallenge`
    /// needed to be signed
    pub metadata: DappToWalletInteractionMetadata,
}

#[uniffi::export]
pub fn authentication_signing_input_get_rola_challenge(
    input: &AuthenticationSigningInput,
) -> Result<RolaChallenge> {
    input.into_internal().rola_challenge().into_result()
}

#[uniffi::export]
pub fn new_authentication_signing_input_sample() -> AuthenticationSigningInput {
    InternalAuthenticationSigningInput::sample().into()
}

#[uniffi::export]
pub fn new_authentication_signing_input_sample_other(
) -> AuthenticationSigningInput {
    InternalAuthenticationSigningInput::sample_other().into()
}
