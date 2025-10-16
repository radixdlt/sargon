use crate::prelude::*;
use sargon::{AuthIntent as InternalAuthIntent, OsSigning};

#[uniffi::export]
impl SargonOS {
    pub async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        execution_summary: ExecutionSummary,
        lock_fee_data: LockFeeData,
    ) -> Result<SignedIntent> {
        self.wrapped
            .sign_transaction(
                transaction_intent.into(),
                execution_summary.into(),
                lock_fee_data.into_internal(),
            )
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

    pub async fn sign_auth_accounts(
        &self,
        account_addresses: Vec<AccountAddress>,
        challenge_nonce: DappToWalletInteractionAuthChallengeNonce,
        metadata: DappToWalletInteractionMetadata,
    ) -> Result<SignedAuthIntent> {
        let auth_intent = InternalAuthIntent::new_from_request(
            challenge_nonce.into(),
            metadata.into(),
            account_addresses
                .into_iter()
                .map(|a| AddressOfAccountOrPersona::Account(a).into())
                .collect_vec(),
        )?;

        self.wrapped.sign_auth(auth_intent).await.into_result()
    }

    pub async fn sign_auth_persona(
        &self,
        identity_address: IdentityAddress,
        challenge_nonce: DappToWalletInteractionAuthChallengeNonce,
        metadata: DappToWalletInteractionMetadata,
    ) -> Result<SignedAuthIntent> {
        let auth_intent = InternalAuthIntent::new_from_request(
            challenge_nonce.into(),
            metadata.into(),
            vec![AddressOfAccountOrPersona::Identity(identity_address).into()],
        )?;

        self.wrapped.sign_auth(auth_intent).await.into_result()
    }
}
