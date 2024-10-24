use crate::prelude::*;

#[async_trait::async_trait]
pub trait AuthenticationSigningInteractor {
    async fn sign(
        &self,
        request: AuthenticationSigningInteractorRequest,
    ) -> Result<AuthenticationSigningResponse>;
}

pub struct AuthenticationSigningInteractorRequest {
    pub input: AuthenticationSigningInput,
}

impl AuthenticationSigningInteractorRequest {
    pub fn new(input: AuthenticationSigningInput) -> Self {
        Self { input }
    }
}

impl From<AuthenticationSigningInput>
    for AuthenticationSigningInteractorRequest
{
    fn from(value: AuthenticationSigningInput) -> Self {
        Self::new(value)
    }
}

pub struct AuthenticationSigningResponse {
    pub signature_with_public_key: SignatureWithPublicKey,
}

impl From<AuthenticationSigningResponse> for WalletToDappInteractionAuthProof {
    fn from(value: AuthenticationSigningResponse) -> Self {
        let signature_with_public_key = value.signature_with_public_key;

        let public_key = signature_with_public_key.public_key();
        WalletToDappInteractionAuthProof::new(
            public_key,
            public_key.curve(),
            signature_with_public_key.signature(),
        )
    }
}
