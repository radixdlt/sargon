use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WhenAllTransactionsAreValid(pub SignaturesCollectingContinuation);

impl WhenAllTransactionsAreValid {
    pub fn finish_early() -> Self {
        Self(SignaturesCollectingContinuation::FinishEarly)
    }
    pub fn r#continue() -> Self {
        Self(SignaturesCollectingContinuation::Continue)
    }
}

impl Default for WhenAllTransactionsAreValid {
    fn default() -> Self {
        Self::finish_early()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WhenSomeTransactionIsInvalid(pub SignaturesCollectingContinuation);

impl WhenSomeTransactionIsInvalid {
    pub fn finish_early() -> Self {
        Self(SignaturesCollectingContinuation::FinishEarly)
    }
    pub fn r#continue() -> Self {
        Self(SignaturesCollectingContinuation::Continue)
    }
}

impl Default for WhenSomeTransactionIsInvalid {
    fn default() -> Self {
        Self::r#continue()
    }
}

/// Strategy to use for finishing early, i.e. stop collecting more signatures
#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
pub struct SigningFinishEarlyStrategy {
    pub(crate) when_all_transactions_are_valid: WhenAllTransactionsAreValid,
    pub(crate) when_some_transaction_is_invalid: WhenSomeTransactionIsInvalid,
}
impl SigningFinishEarlyStrategy {
    pub fn new(
        when_all_transactions_are_valid: WhenAllTransactionsAreValid,
        when_some_transaction_is_invalid: WhenSomeTransactionIsInvalid,
    ) -> Self {
        Self {
            when_all_transactions_are_valid,
            when_some_transaction_is_invalid,
        }
    }

    #[allow(unused)]
    pub(crate) fn r#continue() -> Self {
        Self::new(
            WhenAllTransactionsAreValid::r#continue(),
            WhenSomeTransactionIsInvalid::r#continue(),
        )
    }

    #[allow(unused)]
    pub(crate) fn finish_early() -> Self {
        Self::new(
            WhenAllTransactionsAreValid::finish_early(),
            WhenSomeTransactionIsInvalid::finish_early(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = SigningFinishEarlyStrategy;

    #[test]
    fn test_continue() {
        let sut = Sut::r#continue();
        assert_eq!(
            sut.when_all_transactions_are_valid.0,
            SignaturesCollectingContinuation::Continue
        );
        assert_eq!(
            sut.when_some_transaction_is_invalid.0,
            SignaturesCollectingContinuation::Continue
        );
    }

    #[test]
    fn test_finish_early() {
        let sut = Sut::finish_early();
        assert_eq!(
            sut.when_all_transactions_are_valid.0,
            SignaturesCollectingContinuation::FinishEarly
        );
        assert_eq!(
            sut.when_some_transaction_is_invalid.0,
            SignaturesCollectingContinuation::FinishEarly
        );
    }

    #[test]
    fn test_default_is_finish_when_valid_continue_if_invalid() {
        let sut = Sut::default();
        assert_eq!(
            sut.when_all_transactions_are_valid.0,
            SignaturesCollectingContinuation::FinishEarly
        );
        assert_eq!(
            sut.when_some_transaction_is_invalid.0,
            SignaturesCollectingContinuation::Continue
        );
    }
}
