use crate::prelude::*;
use sargon::SignedOutcome as InternalSignedOutcome;
use sargon::SigningAbandonedReason as InternalSigningAbandonedReason;
use sargon::Subintent as InternalSubintent;
use sargon::TransactionIntent as InternalTransactionIntent;

#[uniffi::export]
impl SargonOS {
    pub async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        role_kind: RoleKind,
    ) -> SignedOutcomeOfTransactionIntent {
        self.wrapped
            .sign_transaction(transaction_intent.into(), role_kind.into())
            .await
            .into()
    }

    pub async fn sign_subintent(
        &self,
        transaction_intent: Subintent,
        role_kind: RoleKind,
    ) -> SignedOutcomeOfSubintent {
        self.wrapped
            .sign_subintent(transaction_intent.into(), role_kind.into())
            .await
            .into()
    }
}
