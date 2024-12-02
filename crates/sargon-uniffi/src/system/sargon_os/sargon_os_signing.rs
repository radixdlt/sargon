use sargon::{SignedOutcome, SignedTransaction};
use crate::prelude::*;
use sargon::TransactionIntent as InternalTransactionIntent;
use sargon::Subintent as InternalSubintent;
use sargon::SignedOutcome as InternalSignedOutcome;

#[uniffi::export]
impl SargonOS {

    pub async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        role_kind: RoleKind
    ) -> SignOutcomeTransactionIntent {
        self
            .wrapped
            .sign_transaction(transaction_intent.into(), role_kind.into())
            .await
            .into()
    }

    pub async fn sign_subintent(
        &self,
        transaction_intent: Subintent,
        role_kind: RoleKind
    ) -> SignOutcomeSubintent {
        self
            .wrapped
            .sign_subintent(transaction_intent.into(), role_kind.into())
            .await
            .into()
    }

}

/// Outcome of signing a transaction intent
#[derive(Clone, PartialEq, Eq, uniffi::Enum)]
pub enum SignOutcomeTransactionIntent {
    /// The user has provided all needed signatures, the transaction intent is considered signed
    Signed(SignedIntent),
    /// The user has not provided all needed signatures, thus rejecting the signing process
    Rejected
}

impl From<InternalSignedOutcome<InternalTransactionIntent>> for SignOutcomeTransactionIntent {
    fn from(value: InternalSignedOutcome<InternalTransactionIntent>) -> Self {
        match value {
            SignedOutcome::Signed(signed) => {
                SignOutcomeTransactionIntent::Signed(signed.into())
            }
            SignedOutcome::Rejected => {
                SignOutcomeTransactionIntent::Rejected
            }
        }
    }
}

impl From<SignOutcomeTransactionIntent> for InternalSignedOutcome<InternalTransactionIntent> {
    fn from(value: SignOutcomeTransactionIntent) -> Self {
        match value {
            SignOutcomeTransactionIntent::Signed(signed) => {
                Self::Signed(signed.into())
            }
            SignOutcomeTransactionIntent::Rejected => {
                Self::Rejected
            }
        }
    }
}

/// Outcome of signing a subintent
#[derive(Clone, PartialEq, Eq, uniffi::Enum)]
pub enum SignOutcomeSubintent {
    /// The user has provided all needed signatures, the subintent is considered signed
    Signed(SignedSubintent),
    /// The user has not provided all needed signatures, thus rejecting the signing process
    Rejected
}

impl From<InternalSignedOutcome<InternalSubintent>> for SignOutcomeSubintent {
    fn from(value: InternalSignedOutcome<InternalSubintent>) -> Self {
        match value {
            SignedOutcome::Signed(signed) => {
                SignOutcomeSubintent::Signed(signed.into())
            }
            SignedOutcome::Rejected => {
                SignOutcomeSubintent::Rejected
            }
        }
    }
}

impl From<SignOutcomeSubintent> for InternalSignedOutcome<InternalSubintent> {
    fn from(value: SignOutcomeSubintent) -> Self {
        match value {
            SignOutcomeSubintent::Signed(signed) => {
                Self::Signed(signed.into())
            }
            SignOutcomeSubintent::Rejected => {
                Self::Rejected
            }
        }
    }
}