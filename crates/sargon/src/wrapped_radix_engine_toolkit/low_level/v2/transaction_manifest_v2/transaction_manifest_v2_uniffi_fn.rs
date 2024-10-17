use crate::prelude::*;

#[uniffi::export]
pub fn transaction_manifest_string_v2(
    manifest: &TransactionManifestV2,
) -> String {
    manifest.manifest_string()
}

#[uniffi::export]
pub fn transaction_manifest_summary_v2(
    manifest: &TransactionManifestV2,
) -> Option<ManifestSummary> {
    manifest.summary()
}

#[uniffi::export]
pub fn transaction_manifest_involved_resource_addresses_v2(
    manifest: &TransactionManifestV2,
) -> Vec<ResourceAddress> {
    manifest.involved_resource_addresses()
}

#[uniffi::export]
pub fn transaction_manifest_involved_pool_addresses_v2(
    manifest: &TransactionManifestV2,
) -> Vec<PoolAddress> {
    manifest.involved_pool_addresses()
}

#[uniffi::export]
pub fn transaction_manifest_network_id_v2(
    manifest: &TransactionManifestV2,
) -> NetworkID {
    manifest.network_id()
}

#[uniffi::export]
pub fn transaction_manifest_blobs_v2(
    manifest: &TransactionManifestV2,
) -> Blobs {
    manifest.blobs().clone()
}

#[uniffi::export]
pub fn new_transaction_manifest_v2_sample() -> TransactionManifestV2 {
    TransactionManifestV2::sample()
}

#[uniffi::export]
pub fn new_transaction_manifest_v2_sample_other() -> TransactionManifestV2 {
    TransactionManifestV2::sample_other()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifestV2;

    #[test]
    fn samples() {
        assert_eq!(new_transaction_manifest_v2_sample(), SUT::sample());
        assert_eq!(
            new_transaction_manifest_v2_sample_other(),
            SUT::sample_other()
        );
    }

    #[test]
    fn test_manifest_string() {
        assert_eq!(
            transaction_manifest_string_v2(&SUT::sample()),
            SUT::sample().manifest_string()
        );
    }

    #[test]
    fn test_transaction_manifest_summary_v2() {
        assert_eq!(
            transaction_manifest_summary_v2(&SUT::sample())
                .unwrap()
                .addresses_of_accounts_withdrawn_from,
            vec![AccountAddress::sample_mainnet()]
        );
    }

    #[test]
    fn test_transaction_manifest_involved_resource_addresses_v2() {
        assert_eq!(
            transaction_manifest_involved_resource_addresses_v2(&SUT::sample()),
            vec![ResourceAddress::sample_mainnet_xrd()]
        );
    }

    #[test]
    fn test_transaction_manifest_involved_pool_addresses_v2() {
        assert_eq!(
            transaction_manifest_involved_pool_addresses_v2(&SUT::sample()),
            Vec::new()
        );
    }

    #[test]
    fn test_transaction_manifest_network_id_v2() {
        assert_eq!(
            transaction_manifest_network_id_v2(&SUT::sample()),
            SUT::sample().network_id()
        );
    }

    #[test]
    fn test_transaction_manifest_blobs_v2() {
        assert_eq!(
            transaction_manifest_blobs_v2(&SUT::sample()),
            SUT::sample().blobs().clone()
        );
    }
}
