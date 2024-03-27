use crate::prelude::*;

#[uniffi::export]
pub fn new_notarized_transaction_sample() -> NotarizedTransaction {
    NotarizedTransaction::sample()
}

#[uniffi::export]
pub fn new_notarized_transaction_sample_other() -> NotarizedTransaction {
    NotarizedTransaction::sample_other()
}

#[uniffi::export]
pub fn notarized_transaction_compile(
    notarized_transaction: &NotarizedTransaction,
) -> CompiledNotarizedIntent {
    notarized_transaction.compile()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NotarizedTransaction;

    #[test]
    fn inequality() {
        assert_ne!(
            new_notarized_transaction_sample(),
            new_notarized_transaction_sample_other(),
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            new_notarized_transaction_sample(),
            new_notarized_transaction_sample()
        );
        assert_eq!(
            new_notarized_transaction_sample_other(),
            new_notarized_transaction_sample_other()
        );
    }

    #[test]
    fn test_compile() {
        let sut = SUT::sample();
        assert_eq!(notarized_transaction_compile(&sut), sut.compile())
    }
}
