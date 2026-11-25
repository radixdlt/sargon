use crate::prelude::*;
use sargon::{AuthIntent as InternalAuthIntent, OsSigning};

#[uniffi::export]
impl SargonOS {
    pub async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        execution_summary: ExecutionSummary,
    ) -> Result<SignedIntent> {
        self.wrapped
            .sign_transaction(
                transaction_intent.into(),
                execution_summary.into(),
            )
            .await
            .into_result()
    }

    /// Signs a subintent. `execution_summary` is the one its review showed,
    /// if it was previewed: for the access controllers it calls, it decides
    /// which roles the wallet signs for.
    #[uniffi::method(default(execution_summary = None))]
    pub async fn sign_subintent(
        &self,
        transaction_intent: Subintent,
        role_kind: RoleKind,
        execution_summary: Option<ExecutionSummary>,
    ) -> Result<SignedSubintent> {
        self.wrapped
            .sign_subintent(
                transaction_intent.into(),
                role_kind.into(),
                execution_summary.map(Into::into),
            )
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
