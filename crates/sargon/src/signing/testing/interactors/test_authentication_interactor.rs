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
        request: AuthenticationSigningInteractorRequest,
    ) -> Result<AuthenticationSigningResponse> {
        let id = request.input.owned_factor_instance.factor_source_id();

        let mnemonic_with_passphrase = id.sample_associated_mnemonic();

        let signature = mnemonic_with_passphrase.sign(
            &request.input.challenge.hash(),
            &request.input.owned_factor_instance.value.derivation_path(),
        );

        if self.should_fail {
            Err(CommonError::Unknown)
        } else {
            Ok(AuthenticationSigningResponse {
                signature_with_public_key: signature,
            })
        }
    }
}
