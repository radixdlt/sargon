use crate::prelude::*;

#[async_trait::async_trait]
pub trait AuthenticationSigningInteractor: Send + Sync {
    async fn sign(
        &self,
        request: AuthenticationSigningRequest,
    ) -> Result<AuthenticationSigningResponse>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthenticationSigningRequest {
    pub input: AuthenticationSigningInput,
}

impl AuthenticationSigningRequest {
    pub fn new(input: AuthenticationSigningInput) -> Self {
        Self { input }
    }
}

impl From<AuthenticationSigningInput> for AuthenticationSigningRequest {
    fn from(value: AuthenticationSigningInput) -> Self {
        Self::new(value)
    }
}

impl HasSampleValues for AuthenticationSigningRequest {
    fn sample() -> Self {
        Self::new(AuthenticationSigningInput::sample())
    }

    fn sample_other() -> Self {
        Self::new(AuthenticationSigningInput::sample_other())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthenticationSigningResponse {
    pub signature_with_public_key: SignatureWithPublicKey,
}

impl AuthenticationSigningResponse {
    pub fn new(signature_with_public_key: SignatureWithPublicKey) -> Self {
        Self {
            signature_with_public_key,
        }
    }
}

impl HasSampleValues for AuthenticationSigningResponse {
    fn sample() -> Self {
        Self::new(SignatureWithPublicKey::sample())
    }

    fn sample_other() -> Self {
        Self::new(SignatureWithPublicKey::sample_other())
    }
}

impl From<AuthenticationSigningResponse> for WalletToDappInteractionAuthProof {
    fn from(value: AuthenticationSigningResponse) -> Self {
        let signature_with_public_key = value.signature_with_public_key;

        let public_key = signature_with_public_key.public_key();
        Self::new(
            public_key,
            public_key.curve(),
            signature_with_public_key.signature(),
        )
    }
}
