use crate::prelude::*;

pub struct TestAuthorizationInteractor {
    request_count: RwLock<u32>,
    user: AuthorisingUser,
}

#[derive(Clone)]
enum AuthorisingUser {
    Stubborn {
        response_per_purpose:
            IndexMap<AuthorizationPurpose, AuthorizationResponse>,
    },

    Docile {
        response_per_purpose_per_time:
            Vec<IndexMap<AuthorizationPurpose, AuthorizationResponse>>,
    },
}

impl AuthorisingUser {
    pub fn stubborn(response: AuthorizationResponse) -> Self {
        Self::Stubborn {
            response_per_purpose: IndexMap::from_iter([
                (AuthorizationPurpose::CreatingAccount, response.clone()),
                (AuthorizationPurpose::CreatingAccounts, response.clone()),
                (AuthorizationPurpose::CreatingPersona, response.clone()),
                (AuthorizationPurpose::CreatingPersonas, response.clone()),
            ]),
        }
    }
}

#[async_trait::async_trait]
impl AuthorizationInteractor for TestAuthorizationInteractor {
    async fn request_authorization(
        &self,
        purpose: AuthorizationPurpose,
    ) -> AuthorizationResponse {
        let response_per_purpose = match self.user.clone() {
            AuthorisingUser::Stubborn {
                response_per_purpose,
            } => response_per_purpose,
            AuthorisingUser::Docile {
                response_per_purpose_per_time,
            } => {
                let count = *self
                    .request_count
                    .read()
                    .expect("Request count should not have been poisoned");

                response_per_purpose_per_time
                    .get(count as usize)
                    .unwrap()
                    .clone()
            }
        };

        self.request_count
            .write()
            .expect("Request count should not have been poisoned")
            .add_assign(1u32);

        response_per_purpose
            .get(&purpose)
            .cloned()
            .unwrap_or(AuthorizationResponse::Authorized)
    }
}

impl TestAuthorizationInteractor {
    fn new(user: AuthorisingUser) -> Self {
        Self {
            request_count: RwLock::new(0u32),
            user,
        }
    }

    pub fn stubborn_authorizing() -> Self {
        Self::new(AuthorisingUser::stubborn(AuthorizationResponse::Authorized))
    }

    pub fn stubborn_rejecting_specific_purpose(
        purpose: AuthorizationPurpose,
    ) -> Self {
        let mut user =
            AuthorisingUser::stubborn(AuthorizationResponse::Authorized);

        match user {
            AuthorisingUser::Stubborn {
                ref mut response_per_purpose,
            } => {
                response_per_purpose
                    .insert(purpose, AuthorizationResponse::Rejected);
            }
            _ => unreachable!(),
        }

        Self::new(user)
    }

    pub fn docile_with(
        interactions: impl IntoIterator<
            Item = (AuthorizationPurpose, AuthorizationResponse),
        >,
    ) -> Self {
        let user = AuthorisingUser::Docile {
            response_per_purpose_per_time: Vec::from_iter(
                interactions.into_iter().map(|(purpose, response)| {
                    IndexMap::from_iter([(purpose, response)])
                }),
            ),
        };

        Self::new(user)
    }
}
