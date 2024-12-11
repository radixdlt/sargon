#![allow(unused)]

use crate::prelude::*;

pub(crate) struct TestAuthenticationInteractor {
    should_fail: bool,
}

impl TestAuthenticationInteractor {
    pub(crate) fn new_failing() -> Self {
        Self { should_fail: true }
    }

    pub(crate) fn new_succeeding() -> Self {
        Self { should_fail: false }
    }
}

#[async_trait::async_trait]
impl AuthenticationSigningInteractor for TestAuthenticationInteractor {
    async fn sign(
        &self,
        request: AuthenticationSigningRequest,
    ) -> Result<AuthenticationSigningResponse> {
        let id = request.input.owned_factor_instance.factor_source_id();

        let mnemonic_with_passphrase = id.sample_associated_mnemonic();

        let challenge = request.input.rola_challenge()?;
        let signature = mnemonic_with_passphrase.sign(
            &challenge.hash(),
            &request.input.owned_factor_instance.value.derivation_path(),
        );

        if self.should_fail {
            Err(CommonError::SigningRejected)
        } else {
            AuthenticationSigningResponse::new(challenge, signature)
        }
    }
}
