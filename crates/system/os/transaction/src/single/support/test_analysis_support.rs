use crate::prelude::*;

pub(crate) async fn prepare_os(
    mock_networking_driver: MockNetworkingDriver,
) -> Arc<SargonOS> {
    let req = SargonOS::boot_test_with_networking_driver(Arc::new(
        mock_networking_driver,
    ));
    let os = actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
        .await
        .unwrap()
        .unwrap();

    os.update_profile_with(|profile| {
        profile.networks.insert(ProfileNetwork::sample_mainnet());
        profile.factor_sources.insert(FactorSource::sample());
        Ok(())
    })
    .await
    .unwrap();
    os
}

pub(crate) fn prepare_preview_response(
    ledger_state: LedgerState,
    preview_response: TransactionPreviewResponse,
) -> Vec<BagOfBytes> {
    vec![
        to_bag_of_bytes(TransactionConstructionResponse { ledger_state }),
        to_bag_of_bytes(preview_response),
    ]
}

pub(crate) fn prepare_preview_response_v2(
    ledger_state: LedgerState,
    preview_response: TransactionPreviewResponseV2,
) -> Vec<BagOfBytes> {
    vec![
        to_bag_of_bytes(TransactionConstructionResponse { ledger_state }),
        to_bag_of_bytes(preview_response),
    ]
}

fn to_bag_of_bytes<T>(value: T) -> BagOfBytes
where
    T: Serialize,
{
    BagOfBytes::from(serde_json::to_vec(&value).unwrap())
}
