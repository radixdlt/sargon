use crate::prelude::*;
use sargon::AuthenticationSigningRequest as InternalAuthenticationSigningRequest;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct AuthenticationSigningRequest {
    pub input: AuthenticationSigningInput,
}
