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
    ) -> SignOutcomeTransactionIntent {
        self.wrapped
            .sign_transaction(transaction_intent.into(), role_kind.into())
            .await
            .into()
    }

    pub async fn sign_subintent(
        &self,
        transaction_intent: Subintent,
        role_kind: RoleKind,
    ) -> SignOutcomeSubintent {
        self.wrapped
            .sign_subintent(transaction_intent.into(), role_kind.into())
            .await
            .into()
    }
}

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SigningAbandonedReason {
    /// The user rejected the signing process
    Rejected,

    /// The signing process started with no profile present
    ProfileMissing,

    /// Preprocessing of signatures collector state failed.
    PreprocessingError(String),

    /// Could not validate signatures
    InvalidSignatures,
}

/// Outcome of signing a transaction intent
#[derive(Clone, PartialEq, Eq, uniffi::Enum)]
pub enum SignOutcomeTransactionIntent {
    /// The user has provided all needed signatures, the transaction intent is considered signed
    Signed(SignedIntent),
    /// The signing process was abandoned
    Abandoned(SigningAbandonedReason),
}

impl From<InternalSignedOutcome<InternalTransactionIntent>>
    for SignOutcomeTransactionIntent
{
    fn from(value: InternalSignedOutcome<InternalTransactionIntent>) -> Self {
        match value {
            InternalSignedOutcome::Signed(signed) => {
                SignOutcomeTransactionIntent::Signed(signed.into())
            }
            InternalSignedOutcome::Abandoned(reason) => {
                SignOutcomeTransactionIntent::Abandoned(reason.into())
            }
        }
    }
}

impl From<SignOutcomeTransactionIntent>
    for InternalSignedOutcome<InternalTransactionIntent>
{
    fn from(value: SignOutcomeTransactionIntent) -> Self {
        match value {
            SignOutcomeTransactionIntent::Signed(signed) => {
                Self::Signed(signed.into())
            }
            SignOutcomeTransactionIntent::Abandoned(reason) => {
                Self::Abandoned(reason.into())
            }
        }
    }
}

/// Outcome of signing a subintent
#[derive(Clone, PartialEq, Eq, uniffi::Enum)]
pub enum SignOutcomeSubintent {
    /// The user has provided all needed signatures, the subintent is considered signed
    Signed(SignedSubintent),
    /// The signing process was abandoned
    Abandoned(SigningAbandonedReason),
}

impl From<InternalSignedOutcome<InternalSubintent>> for SignOutcomeSubintent {
    fn from(value: InternalSignedOutcome<InternalSubintent>) -> Self {
        match value {
            InternalSignedOutcome::Signed(signed) => {
                SignOutcomeSubintent::Signed(signed.into())
            }
            InternalSignedOutcome::Abandoned(reason) => {
                SignOutcomeSubintent::Abandoned(reason.into())
            }
        }
    }
}

impl From<SignOutcomeSubintent> for InternalSignedOutcome<InternalSubintent> {
    fn from(value: SignOutcomeSubintent) -> Self {
        match value {
            SignOutcomeSubintent::Signed(signed) => Self::Signed(signed.into()),
            SignOutcomeSubintent::Abandoned(reason) => {
                Self::Abandoned(reason.into())
            }
        }
    }
}
