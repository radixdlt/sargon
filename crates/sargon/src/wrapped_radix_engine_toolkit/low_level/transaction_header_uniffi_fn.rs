use crate::prelude::*;

#[uniffi::export]
pub fn new_transaction_header_sample() -> TransactionHeader {
    TransactionHeader::sample()
}

#[uniffi::export]
pub fn new_transaction_header_sample_other() -> TransactionHeader {
    TransactionHeader::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionHeader;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_transaction_header_sample(),
                new_transaction_header_sample_other(),
                // duplicates should get removed
                new_transaction_header_sample(),
                new_transaction_header_sample_other(),
            ])
            .len(),
            2
        );
    }
}
