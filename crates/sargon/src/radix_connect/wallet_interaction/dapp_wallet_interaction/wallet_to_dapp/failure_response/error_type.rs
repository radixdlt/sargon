use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DappWalletInteractionErrorType {
    RejectedByUser,
    WrongNetwork,
    FailedToPrepareTransaction,
    FailedToCompileTransaction,
    FailedToSignTransaction,
    FailedToSubmitTransaction,
    FailedToPollSubmittedTransaction,
    FailedToFindAccountWithEnoughFundsToLockFee,
    SubmittedTransactionWasDuplicate,
    SubmittedTransactionHasFailedTransactionStatus,
    SubmittedTransactionHasRejectedTransactionStatus,
    WrongAccountType,
    UnknownWebsite,
    InvalidOriginURL,
    RadixJsonNotFound,
    RadixJsonUnknownFileFormat,
    UnknownDappDefinitionAddress,
    InvalidPersona,
    InvalidRequest,
    IncompatibleVersion,
    FailedToSignAuthChallenge,
}

impl HasSampleValues for DappWalletInteractionErrorType {
    fn sample() -> Self {
        DappWalletInteractionErrorType::FailedToPrepareTransaction
    }

    fn sample_other() -> Self {
        DappWalletInteractionErrorType::FailedToCompileTransaction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappWalletInteractionErrorType;

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
