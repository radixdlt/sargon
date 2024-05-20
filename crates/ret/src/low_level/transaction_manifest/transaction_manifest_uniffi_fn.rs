use crate::prelude::*;

#[uniffi::export]
pub fn new_transaction_manifest_from_instructions_string_and_blobs(
    instructions_string: String,
    network_id: NetworkID,
    blobs: Blobs,
) -> Result<TransactionManifest> {
    TransactionManifest::new(instructions_string, network_id, blobs)
}

#[uniffi::export]
pub fn transaction_manifest_instructions_string(
    manifest: &TransactionManifest,
) -> String {
    manifest.instructions_string()
}

#[uniffi::export]
pub fn transaction_manifest_summary(
    manifest: &TransactionManifest,
) -> ManifestSummary {
    manifest.summary()
}

#[uniffi::export]
pub fn transaction_manifest_involved_resource_addresses(
    manifest: &TransactionManifest,
) -> Vec<ResourceAddress> {
    manifest.involved_resource_addresses()
}

#[uniffi::export]
pub fn transaction_manifest_involved_pool_addresses(
    manifest: &TransactionManifest,
) -> Vec<PoolAddress> {
    manifest.involved_pool_addresses()
}

#[uniffi::export]
pub fn transaction_manifest_execution_summary(
    manifest: &TransactionManifest,
    encoded_receipt: BagOfBytes,
) -> Result<ExecutionSummary> {
    manifest.execution_summary(encoded_receipt)
}

#[uniffi::export]
pub fn transaction_manifest_network_id(
    manifest: &TransactionManifest,
) -> NetworkID {
    manifest.network_id()
}

#[uniffi::export]
pub fn transaction_manifest_blobs(manifest: &TransactionManifest) -> Blobs {
    manifest.blobs().clone()
}

#[uniffi::export]
pub fn new_transaction_manifest_sample() -> TransactionManifest {
    TransactionManifest::sample()
}

#[uniffi::export]
pub fn new_transaction_manifest_sample_other() -> TransactionManifest {
    TransactionManifest::sample_other()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn samples() {
        assert_eq!(new_transaction_manifest_sample(), SUT::sample());
        assert_eq!(
            new_transaction_manifest_sample_other(),
            SUT::sample_other()
        );
    }

    #[test]
    fn test_new_transaction_manifest_from_instructions_string_and_blobs() {
        let s = new_transaction_manifest_sample().instructions_string();

        assert_eq!(
            new_transaction_manifest_from_instructions_string_and_blobs(
                s.clone(),
                NetworkID::Mainnet,
                Blobs::default()
            )
            .unwrap()
            .instructions_string(),
            s
        );
    }

    #[test]
    fn test_instructions_string() {
        assert_eq!(
            transaction_manifest_instructions_string(&SUT::sample()),
            SUT::sample().instructions_string()
        );
    }

    #[test]
    fn test_network_id() {
        assert_eq!(
            transaction_manifest_network_id(&SUT::sample()),
            SUT::sample().network_id()
        );
    }

    #[test]
    fn test_blobs() {
        assert_eq!(
            transaction_manifest_blobs(&SUT::sample()),
            SUT::sample().blobs().clone()
        );
    }

    #[test]
    fn test_execution_summary() {
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "transfer_1to2_multiple_nf_and_f_tokens.dat"
        ));

        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "transfer_1to2_multiple_nf_and_f_tokens.rtm"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest_execution_summary(
            &transaction_manifest,
            BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
        )
        .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".into();
        assert_eq!(sut.addresses_of_accounts_requiring_auth, vec![acc_gk])
    }

    #[test]
    fn test_involved_pool_addresses() {
        assert_eq!(
            transaction_manifest_involved_pool_addresses(&SUT::sample()),
            Vec::new()
        );
    }

    #[test]
    fn test_involved_resource_addresses() {
        assert_eq!(
            transaction_manifest_involved_resource_addresses(&SUT::sample()),
            vec![ResourceAddress::sample_mainnet_xrd()]
        );
    }

    #[test]
    fn test_manifest_summary() {
        assert_eq!(
            transaction_manifest_summary(&SUT::sample())
                .addresses_of_accounts_withdrawn_from,
            vec![AccountAddress::sample_mainnet()]
        );
    }
}
