use crate::prelude::*;
use crate::ScryptoSborValue;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateNonFungibleDataResponse {
    pub ledger_state: LedgerState,
    pub resource_address: ResourceAddress,
    pub non_fungible_ids: Vec<StateNonFungibleDataResponseItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateNonFungibleDataResponseItem {
    pub non_fungible_id: NonFungibleLocalId,
    pub is_burned: bool,
    pub last_updated_at_state_version: i64,
    pub data: Option<ScryptoSborValue>,
}

#[cfg(test)]
mod tests {
    use prelude::fixture_gw_model;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StateNonFungibleDataResponse;

    #[test]
    fn json_decode_nft_data_domain() {
        let response = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/request_non_fungible_data_domain"
        ))
        .unwrap();
        
        let data = response.0.non_fungible_ids.first().unwrap().data.clone();
        assert!(data.is_some(), "Expected data to be present");
        print!("Decoded NFT data: {:?}", data);
    }
}