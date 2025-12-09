use crate::prelude::*;

/// The URL endpoint for the NFT liquidity receipt service API
pub const FETCH_URL: &str =
    "https://nft-pricing-dev.rdx-works-main.extratools.works/liquidity-receipt";

impl NonFungiblePricesClient {
    pub(crate) async fn fetch_remote_liquidity_receipts(
        &self,
        state_version: u64,
        addresses: HashSet<NonFungibleGlobalId>,
    ) -> Result<Vec<NonFungibleLiquidityReceipt>> {
        let request_body = LiquidityReceiptRequestBody::from_global_ids(
            state_version,
            addresses,
        );
        let request =
            NetworkRequest::new_post(Url::from_str(FETCH_URL).unwrap())
                .with_serializing_body(request_body)?
                .with_gateway_api_headers();

        self.http_client
            .execute_request_with_decoding(request)
            .await
    }
}

/// ------------------------------ Request Body -------------------------- ///

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
struct LiquidityReceiptRequestBody {
    state_version: u64,
    items: Vec<LiquidityReceiptRequestItem>,
}

impl LiquidityReceiptRequestBody {
    pub fn new(
        state_version: u64,
        items: Vec<LiquidityReceiptRequestItem>,
    ) -> Self {
        Self {
            state_version,
            items,
        }
    }

    fn from_global_ids(
        state_version: u64,
        addresses: HashSet<NonFungibleGlobalId>,
    ) -> Self {
        let mut per_resource_local_ids: HashMap<
            ResourceAddress,
            Vec<NonFungibleLocalId>,
        > = HashMap::new();

        for global_id in addresses {
            per_resource_local_ids
                .entry(global_id.resource_address)
                .or_default()
                .push(global_id.non_fungible_local_id.clone());
        }

        let items = per_resource_local_ids
            .into_iter()
            .map(|(resource_address, local_ids)| {
                LiquidityReceiptRequestItem::new(resource_address, local_ids)
            })
            .collect::<Vec<LiquidityReceiptRequestItem>>();

        Self::new(state_version, items)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
struct LiquidityReceiptRequestItem {
    resource_manager_address: ResourceAddress,
    local_ids: Vec<NonFungibleLocalId>,
}

impl LiquidityReceiptRequestItem {
    fn new(
        resource_manager_address: ResourceAddress,
        local_ids: Vec<NonFungibleLocalId>,
    ) -> Self {
        Self {
            resource_manager_address,
            local_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use drivers::{
        MockNetworkingDriver, NetworkMethod, NetworkResponse, NetworkingDriver,
    };
    use serde_json::Value;
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, Mutex};

    fn make_client(
        driver: Arc<dyn NetworkingDriver>,
    ) -> NonFungiblePricesClient {
        let http_client = Arc::new(HttpClient::new(driver));
        let file_system_client = Arc::new(FileSystemClient::in_memory());
        let fungibles_prices_client = FungiblesPricesClient::new(
            http_client.clone(),
            file_system_client.clone(),
        );
        NonFungiblePricesClient {
            http_client,
            file_system_client,
            fungibles_prices_client,
        }
    }

    #[actix_rt::test]
    async fn fetches_liquidity_receipts_and_groups_ids() {
        let resource_a = ResourceAddress::sample_mainnet_nft_gc_membership();
        let resource_b = ResourceAddress::sample_mainnet_nft_other();
        let local_a_1 = NonFungibleLocalId::integer(1);
        let local_a_2 = NonFungibleLocalId::integer(2);
        let local_b = NonFungibleLocalId::integer(9);

        let addresses = HashSet::from_iter([
            NonFungibleGlobalId::new_unchecked(
                resource_a.clone(),
                local_a_1.clone(),
            ),
            NonFungibleGlobalId::new_unchecked(
                resource_a.clone(),
                local_a_2.clone(),
            ),
            NonFungibleGlobalId::new_unchecked(
                resource_b.clone(),
                local_b.clone(),
            ),
        ]);

        let captured_requests =
            Arc::new(Mutex::new(Vec::<NetworkRequest>::new()));
        let captured_requests_clone = captured_requests.clone();

        let response_payload = serde_json::json!([
            {
                "resourceManagerAddress": resource_a.to_string(),
                "items": [
                    {
                        "localId": local_a_1.to_string(),
                        "resources": [{
                            "address": ResourceAddress::sample_mainnet_xrd().to_string(),
                            "amount": Decimal192::one().to_string()
                        }]
                    },
                    {
                        "localId": local_a_2.to_string(),
                        "resources": [{
                            "address": ResourceAddress::sample_mainnet_candy().to_string(),
                            "amount": Decimal192::two().to_string()
                        }]
                    }
                ]
            },
            {
                "resourceManagerAddress": resource_b.to_string(),
                "items": [
                    {
                        "localId": local_b.to_string(),
                        "resources": [{
                            "address": ResourceAddress::sample_mainnet_nft_gc_membership().to_string(),
                            "amount": Decimal192::three().to_string()
                        }]
                    }
                ]
            }
        ]);

        let driver = Arc::new(MockNetworkingDriver::with_lazy_responses(
            move |request, _| {
                captured_requests_clone
                    .lock()
                    .expect("request capture lock poisoned")
                    .push(request.clone());
                let body =
                    serde_json::to_vec(&response_payload).expect("serialize");
                NetworkResponse::new(200, body)
            },
        ));

        let client = make_client(driver);

        let receipts = client
            .fetch_remote_liquidity_receipts(0, addresses.clone())
            .await
            .unwrap();

        let receipts_map: HashMap<_, _> = receipts
            .into_iter()
            .map(|receipt| (receipt.resource_manager_address, receipt.items))
            .collect();
        assert_eq!(receipts_map.len(), 2);

        let items_a = receipts_map
            .get(&resource_a)
            .expect("receipt for resource_a");
        let mut ids_a = items_a
            .iter()
            .map(|item| item.local_id.clone())
            .collect::<Vec<_>>();
        ids_a.sort();
        let mut expected_ids_a = vec![local_a_1.clone(), local_a_2.clone()];
        expected_ids_a.sort();
        assert_eq!(ids_a, expected_ids_a);
        let items_a_by_id: HashMap<_, _> = items_a
            .iter()
            .map(|item| (item.local_id.clone(), item))
            .collect();
        let item_a1 = items_a_by_id.get(&local_a_1).expect("local_a_1");
        assert_eq!(
            item_a1.resources[0].address,
            ResourceAddress::sample_mainnet_xrd()
        );
        assert_eq!(item_a1.resources[0].amount, Decimal192::one());
        let item_a2 = items_a_by_id.get(&local_a_2).expect("local_a_2");
        assert_eq!(
            item_a2.resources[0].address,
            ResourceAddress::sample_mainnet_candy()
        );
        assert_eq!(item_a2.resources[0].amount, Decimal192::two());

        let items_b = receipts_map
            .get(&resource_b)
            .expect("receipt for resource_b");
        assert_eq!(items_b.len(), 1);
        assert_eq!(items_b[0].local_id, local_b);
        assert_eq!(items_b[0].resources[0].amount, Decimal192::three());

        let captured = captured_requests
            .lock()
            .expect("request capture lock poisoned");
        assert_eq!(captured.len(), 1);
        let request = captured.first().expect("request captured");
        assert_eq!(request.url.as_str(), FETCH_URL);
        assert_eq!(request.method, NetworkMethod::Post);

        let body: Vec<u8> = request.body.to_vec();
        let parsed: LiquidityReceiptRequestBody =
            serde_json::from_slice(&body).expect("request body");
        assert_eq!(parsed.state_version, 0);
        let mut per_resource: HashMap<
            ResourceAddress,
            Vec<NonFungibleLocalId>,
        > = HashMap::new();
        for item in parsed.items {
            let mut ids = item.local_ids.clone();
            ids.sort();
            per_resource.insert(item.resource_manager_address, ids);
        }
        assert_eq!(
            per_resource.get(&resource_a).cloned(),
            Some(expected_ids_a.clone())
        );
        assert_eq!(
            per_resource.get(&resource_b).cloned(),
            Some(vec![local_b.clone()])
        );
        assert_eq!(per_resource.len(), 2);
    }

    #[actix_rt::test]
    async fn propagates_network_errors() {
        let driver = Arc::new(MockNetworkingDriver::new_always_failing());
        let client = make_client(driver);
        let addresses = HashSet::from_iter([NonFungibleGlobalId::sample()]);

        let error = client
            .fetch_remote_liquidity_receipts(0, addresses)
            .await
            .expect_err("error expected");

        assert!(matches!(error, CommonError::NetworkResponseBadCode { .. }));
    }
}
