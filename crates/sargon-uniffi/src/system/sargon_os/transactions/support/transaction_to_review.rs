use crate::prelude::*;
use sargon::TransactionToReview as InternalTransactionToReview;

/// This is the result of the transaction preview analysis.
/// It contains all the information needed to compute and display the transaction details to the user.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct TransactionToReview {
    pub transaction_manifest: TransactionManifest,
    pub execution_summary: ExecutionSummary,
}

decl_conversion_tests_for!(TransactionToReview);
