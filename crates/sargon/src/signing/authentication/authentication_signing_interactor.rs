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
    pub rola_challenge: RolaChallenge,
    pub signature_with_public_key: SignatureWithPublicKey,
}

impl AuthenticationSigningResponse {
    pub fn new(
        rola_challenge: RolaChallenge,
        signature_with_public_key: SignatureWithPublicKey,
    ) -> Result<Self> {
        if !signature_with_public_key.is_valid_for_hash(&rola_challenge.hash())
        {
            Err(CommonError::InvalidSignatureForRolaChallenge)
        } else {
            Ok(Self {
                rola_challenge,
                signature_with_public_key,
            })
        }
    }
}

impl HasSampleValues for AuthenticationSigningResponse {
    fn sample() -> Self {
        let rola_challenge = RolaChallenge::sample();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();

        let signature = mnemonic_with_passphrase
            .sign(&rola_challenge.hash(), &DerivationPath::sample());

        Self::new(rola_challenge, signature).unwrap()
    }

    fn sample_other() -> Self {
        let rola_challenge = RolaChallenge::sample_other();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();

        let signature = mnemonic_with_passphrase
            .sign(&rola_challenge.hash(), &DerivationPath::sample());

        Self::new(rola_challenge, signature).unwrap()
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

#[cfg(test)]
mod test_auth_sign_request {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthenticationSigningRequest;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}

#[cfg(test)]
mod test_auth_sign_response {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthenticationSigningResponse;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
