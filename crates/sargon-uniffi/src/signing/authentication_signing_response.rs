use crate::prelude::*;
use sargon::AuthenticationSigningResponse as InternalAuthenticationSigningResponse;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct AuthenticationSigningResponse {
    pub rola_challenge: RolaChallenge,
    pub signature_with_public_key: SignatureWithPublicKey,
}

#[uniffi::export]
pub fn new_authentication_signing_response(
    rola_challenge: RolaChallenge,
    signature_with_public_key: SignatureWithPublicKey,
) -> Result<AuthenticationSigningResponse> {
    InternalAuthenticationSigningResponse::new(
        rola_challenge.into(),
        signature_with_public_key.into(),
    )
    .into_result()
}

#[uniffi::export]
pub fn new_authentication_signing_response_sample(
) -> AuthenticationSigningResponse {
    InternalAuthenticationSigningResponse::sample().into()
}

#[uniffi::export]
pub fn new_authentication_signing_response_sample_other(
) -> AuthenticationSigningResponse {
    InternalAuthenticationSigningResponse::sample_other().into()
}
