use crate::prelude::*;

pub struct TestAuthorizationInteractor {
    response_per_purpose: IndexMap<AuthorizationPurpose, AuthorizationResponse>,
}

#[async_trait::async_trait]
impl AuthorizationInteractor for TestAuthorizationInteractor {
    async fn request_authorization(
        &self,
        purpose: AuthorizationPurpose,
    ) -> AuthorizationResponse {
        self.response_per_purpose
            .get(&purpose)
            .cloned()
            .unwrap_or(AuthorizationResponse::Authorized)
    }
}

impl TestAuthorizationInteractor {
    pub fn new_authorizing() -> Self {
        Self {
            response_per_purpose: IndexMap::from_iter([
                (
                    AuthorizationPurpose::CreatingAccount,
                    AuthorizationResponse::Authorized,
                ),
                (
                    AuthorizationPurpose::CreatingAccounts,
                    AuthorizationResponse::Authorized,
                ),
                (
                    AuthorizationPurpose::CreatingPersona,
                    AuthorizationResponse::Authorized,
                ),
                (
                    AuthorizationPurpose::CreatingPersonas,
                    AuthorizationResponse::Authorized,
                ),
            ]),
        }
    }

    pub fn new_rejecting() -> Self {
        Self {
            response_per_purpose: IndexMap::from_iter([
                (
                    AuthorizationPurpose::CreatingAccount,
                    AuthorizationResponse::Rejected,
                ),
                (
                    AuthorizationPurpose::CreatingAccounts,
                    AuthorizationResponse::Rejected,
                ),
                (
                    AuthorizationPurpose::CreatingPersona,
                    AuthorizationResponse::Rejected,
                ),
                (
                    AuthorizationPurpose::CreatingPersonas,
                    AuthorizationResponse::Rejected,
                ),
            ]),
        }
    }

    pub fn new_rejecting_only(purpose: AuthorizationPurpose) -> Self {
        let mut interactor = TestAuthorizationInteractor::new_authorizing();
        interactor
            .response_per_purpose
            .insert(purpose, AuthorizationResponse::Rejected);
        interactor
    }
}
