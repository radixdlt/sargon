use crate::prelude::*;
use sargon::AuthenticationSigningResponse as InternalAuthenticationSigningResponse;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct AuthenticationSigningResponse {
    pub signature_with_public_key: SignatureWithPublicKey,
}
