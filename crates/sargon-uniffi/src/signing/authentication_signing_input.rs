use crate::prelude::*;
use sargon::AuthenticationSigningInput as InternalAuthenticationSigningInput;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct AuthenticationSigningInput {
    /// The account or identity address of the entity which signs the rola challenge,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,

    /// The challenge that will be signed by `owned_factor_instance`
    pub challenge: RolaChallenge,
}
