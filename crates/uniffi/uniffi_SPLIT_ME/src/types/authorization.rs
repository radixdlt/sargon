use crate::prelude::*;
use sargon::AuthorizationPurpose as InternalAuthorizationPurpose;
use sargon::AuthorizationResponse as InternalAuthorizationResponse;

/// The purpose of the authorization request
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum AuthorizationPurpose {
    /// When a new account is about to be created.
    CreatingAccount,

    /// When a batch of accounts is about to be created.
    CreatingAccounts,

    /// When a new persona is about to be created.
    CreatingPersona,

    /// When a batch of personas is about to be created.
    CreatingPersonas,
}

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum AuthorizationResponse {
    /// The user authorized the request
    Authorized,

    /// The user rejected the request
    Rejected,
}
