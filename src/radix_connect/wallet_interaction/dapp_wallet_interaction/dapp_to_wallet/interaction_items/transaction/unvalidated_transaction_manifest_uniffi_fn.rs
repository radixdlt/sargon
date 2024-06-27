use crate::prelude::*;

#[uniffi::export]
pub fn new_unvalidated_transaction_manifest_from_transaction_manifest(
    transaction_manifest: TransactionManifest,
) -> UnvalidatedTransactionManifest {
    UnvalidatedTransactionManifest::from(transaction_manifest)
}

#[uniffi::export]
pub fn new_unvalidated_transaction_manifest_sample(
) -> UnvalidatedTransactionManifest {
    UnvalidatedTransactionManifest::sample()
}

#[uniffi::export]
pub fn new_unvalidated_transaction_manifest_sample_other(
) -> UnvalidatedTransactionManifest {
    UnvalidatedTransactionManifest::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UnvalidatedTransactionManifest;

    #[test]
    fn sample_values() {
        assert_ne!(
            new_unvalidated_transaction_manifest_sample(),
            new_unvalidated_transaction_manifest_sample_other(),
        );
    }

    #[test]
    fn test_new_unvalidated_transaction_manifest_from_transaction_manifest() {
        let transaction_manifest = TransactionManifest::sample();
        let sut =
            new_unvalidated_transaction_manifest_from_transaction_manifest(
                transaction_manifest.clone(),
            );
        pretty_assertions::assert_eq!(
            sut.transaction_manifest_string,
            transaction_manifest.instructions_string()
        );
        pretty_assertions::assert_eq!(
            sut.blobs,
            transaction_manifest.blobs().clone()
        );
    }
}
