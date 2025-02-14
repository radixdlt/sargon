use metadata::prelude::MetadataKey;

use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct StateEntityDetailsRequest {
    /// The addresses of the entities for which details are requested. Limited 20 items max.
    pub addresses: Vec<Address>,

    /// This allows for a request to be made against a historic state. If a constraint is specified,
    /// the Gateway will resolve the request against the ledger state at that time.
    /// If not specified, requests will be made with respect to the top of the committed ledger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_ledger_state: Option<LedgerStateSelector>,

    /// The opt-ins for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_ins: Option<StateEntityDetailsOptIns>,
}

impl StateEntityDetailsRequest {
    pub fn new(
        addresses: Vec<Address>,
        at_ledger_state: impl Into<Option<LedgerStateSelector>>,
        opt_ins: impl Into<Option<StateEntityDetailsOptIns>>,
    ) -> StateEntityDetailsRequest {
        StateEntityDetailsRequest {
            addresses,
            at_ledger_state: at_ledger_state.into(),
            opt_ins: opt_ins.into(),
        }
    }

    pub fn addresses_only<A: Into<Address>>(
        addresses: Vec<A>,
    ) -> StateEntityDetailsRequest {
        Self::new(
            addresses.into_iter().map(Into::into).collect_vec(),
            None,
            None,
        )
    }

    pub fn address_ledger_state(
        address: Address,
        at_ledger_state: LedgerStateSelector,
    ) -> StateEntityDetailsRequest {
        Self::new(vec![address], at_ledger_state, None)
    }

    pub fn address_metadata(
        address: Address,
        explicit_metadata: Vec<MetadataKey>,
    ) -> StateEntityDetailsRequest {
        Self::new(
            vec![address],
            None,
            StateEntityDetailsOptIns::new(Some(explicit_metadata)),
        )
    }
}

#[cfg(test)]
mod tests {
    use prelude::fixture_gw_model;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StateEntityDetailsRequest;

    #[test]
    fn json_request_entity_details_single_account_no_assets() {
        let _ = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/request_entity_details__single_account_no_assets"
        ))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_request_entity_details_single_resource() {
        let _ = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/request_entity_details__single_resource"
        ))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }

    #[test]
    fn json_request_entity_details_two_accounts() {
        let _ = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/request_entity_details__two_accounts"
        ))
        .unwrap();
        // assert_json_value_eq_after_roundtrip(&sut, json) // FIXME: Once fully implemented
    }
}
