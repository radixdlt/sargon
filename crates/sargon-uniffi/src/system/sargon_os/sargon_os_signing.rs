use crate::prelude::*;
use sargon::Subintent as InternalSubintent;
use sargon::TransactionIntent as InternalTransactionIntent;

#[uniffi::export]
impl SargonOS {
    pub async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        role_kind: RoleKind,
    ) -> Result<SignedIntent> {
        self.wrapped
            .sign_transaction(transaction_intent.into(), role_kind.into())
            .await
            .into_result()
    }

    pub async fn sign_subintent(
        &self,
        transaction_intent: Subintent,
        role_kind: RoleKind,
    ) -> Result<SignedSubintent> {
        self.wrapped
            .sign_subintent(transaction_intent.into(), role_kind.into())
            .await
            .into_result()
    }

    pub async fn sign_auth(
        &self,
        address_of_entity: AddressOfAccountOrPersona,
        challenge_nonce: DappToWalletInteractionAuthChallengeNonce,
        metadata: DappToWalletInteractionMetadata,
    ) -> Result<WalletToDappInteractionAuthProof> {
        self.wrapped
            .sign_auth(
                address_of_entity.into(),
                challenge_nonce.into(),
                metadata.into(),
            )
            .await
            .into_result()
    }
}
