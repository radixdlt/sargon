use crate::prelude::*;

#[async_trait::async_trait]
pub trait AuthenticationSigningInteractor {
    async fn sign(
        &self,
        request: AuthenticationSigningInteractorRequest,
    ) -> AuthenticationSigningResponse;
}

pub struct AuthenticationSigningInteractorRequest {
    input: AuthenticationSigningInput
}

impl AuthenticationSigningInteractorRequest {
    pub fn new(input: AuthenticationSigningInput) -> Self {
        Self { input }
    }
}

impl From<AuthenticationSigningInput> for AuthenticationSigningInteractorRequest {
    fn from(value: AuthenticationSigningInput) -> Self {
        Self::new(value)
    }
}

pub struct AuthenticationSigningResponse {
    signature: Result<Signature>
}

impl TryFrom<(AuthenticationSigningResponse, AuthenticationSigningInput)> for WalletToDappInteractionAuthProof {
    type Error = CommonError;

    fn try_from((response, input): (AuthenticationSigningResponse, AuthenticationSigningInput)) -> Result<Self, Self::Error> {
        let signature = response.signature?;

        Ok(WalletToDappInteractionAuthProof::new(
            input.owned_factor_instance.value.public_key.public_key,
            input.owned_factor_instance.value.public_key.public_key.curve(),
            signature
        ))
    }
}