use crate::prelude::*;

pub trait TransactionSubmitRequestFromNotarizedTransaction {
    fn new(notarized_transaction: NotarizedTransaction) -> Self;
}

impl TransactionSubmitRequestFromNotarizedTransaction
    for TransactionSubmitRequest
{
    fn new(notarized_transaction: NotarizedTransaction) -> Self {
        let compiled = notarized_transaction.compile();
        Self {
            notarized_transaction_hex: compiled.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionSubmitRequest;

    #[test]
    fn test_new() {
        let tx = NotarizedTransaction::sample();
        let sut = SUT::new(tx.clone());
        assert_eq!(
            sut.notarized_transaction_hex,
            tx.compile().bytes().to_hex()
        );
    }
}
