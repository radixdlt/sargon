use crate::prelude::*;

#[derive(Debug)]
pub struct AuthenticationSigner {
    pub input: AuthenticationSigningInput,
    interactor: Arc<dyn AuthenticationSigningInteractor>,
}

impl AuthenticationSigner {

    pub async fn sign(self) -> Result<WalletToDappInteractionAuthProof> {
        let response = self.interactor.sign(
            self.input.clone().into()
        ).await;

        WalletToDappInteractionAuthProof::try_from((response, self.input))
    }

}