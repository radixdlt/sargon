use crate::prelude::*;

/// An interactor responsible for communicating with the user on host, to authorize.
/// Can be used in tandem with flows defined in sargon os, which require user authorization
/// before continuing.
#[async_trait::async_trait]
pub trait AuthorizationInteractor: Send + Sync {
    /// The user can only authorize or reject.
    async fn request_authorization(
        &self,
        purpose: AuthorizationPurpose,
    ) -> AuthorizationResponse;
}

/// The purpose of the authorization request
#[derive(Clone, std::hash::Hash, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthorizationResponse {
    /// The user authorized the request
    Authorized,

    /// The user rejected the request
    Rejected,
}
