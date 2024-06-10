use crate::prelude::*;

#[uniffi::export]
pub fn new_unvalidated_transaction_manifest_from_transaction_manifest(
    transaction_manifest: TransactionManifest,
) -> UnvalidatedTransactionManifest {
    UnvalidatedTransactionManifest::from(transaction_manifest)
}

#[uniffi::export]
pub fn new_transaction_manifest(
    unvalidated_transaction_manifest: &UnvalidatedTransactionManifest,
    network_id: NetworkID,
) -> Result<TransactionManifest> {
    unvalidated_transaction_manifest.transaction_manifest(network_id)
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
                transaction_manifest,
            );
        assert_eq!(sut, SUT::sample());
    }

    #[test]
    fn test_new_transaction_manifest() {
        let sut = SUT::sample();
        let network_id = NetworkID::Mainnet;
        assert_eq!(
            new_transaction_manifest(&sut, network_id),
            sut.transaction_manifest(network_id)
        )
    }
}
