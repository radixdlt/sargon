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
pub fn new_transaction_manifest_from_unvalidated_transaction_manifest(
    unvalidated_transaction_manifest: UnvalidatedTransactionManifest,
    network_id: NetworkID,
) -> Result<TransactionManifest> {
    TransactionManifest::try_from((
        unvalidated_transaction_manifest,
        network_id,
    ))
}

#[uniffi::export]
pub fn transaction_manifest_string(manifest: &TransactionManifest) -> String {
    manifest.manifest_string()
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
) -> Option<ManifestSummary> {
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
    engine_toolkit_receipt: String,
) -> Result<ExecutionSummary> {
    manifest.execution_summary(engine_toolkit_receipt)
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
        let s = new_transaction_manifest_sample().manifest_string();

        assert_eq!(
            new_transaction_manifest_from_instructions_string_and_blobs(
                s.clone(),
                NetworkID::Mainnet,
                Blobs::default(),
            )
            .unwrap()
            .manifest_string(),
            s
        );
    }

    #[test]
    fn test_new_transaction_manifest_from_unvalidated_transaction_manifest() {
        let unvalidated_transaction_manifest =
            UnvalidatedTransactionManifest::sample();
        let network_id = NetworkID::Mainnet;
        assert_eq!(
            new_transaction_manifest_from_unvalidated_transaction_manifest(
                unvalidated_transaction_manifest.clone(),
                network_id
            ),
            SUT::try_from((unvalidated_transaction_manifest, network_id))
        )
    }

    #[test]
    fn test_manifest_string() {
        assert_eq!(
            transaction_manifest_string(&SUT::sample()),
            SUT::sample().manifest_string()
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
        let receipt = include_str!(concat!(
            env!("FIXTURES_TX"),
            "unstake_partially_from_one_validator.dat"
        ));

        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "unstake_partially_from_one_validator.rtm"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest_execution_summary(
            &transaction_manifest,
            receipt.to_owned(),
        )
        .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".into();
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
                .unwrap()
                .addresses_of_accounts_withdrawn_from,
            vec![AccountAddress::sample_mainnet()]
        );
    }
}
