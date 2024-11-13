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
    pub(crate) addresses: Vec<Address>,

    /// This allows for a request to be made against a historic state. If a constraint is specified,
    /// the Gateway will resolve the request against the ledger state at that time.
    /// If not specified, requests will be made with respect to the top of the committed ledger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) at_ledger_state: Option<LedgerStateSelector>,

    /// The opt-ins for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) opt_ins: Option<StateEntityDetailsOptIns>,
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
