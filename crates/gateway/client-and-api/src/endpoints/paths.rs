use crate::prelude::*;

impl GatewayClient {
    pub const PATH_STATUS_GATEWAY_STATUS: &'static str =
        "status/gateway-status";
    pub const PATH_TRANSACTION_CONSTRUCTION: &'static str =
        "transaction/construction";
    pub const PATH_TRANSACTION_PREVIEW: &'static str = "transaction/preview";
    pub const PATH_TRANSACTION_PREVIEW_V2: &'static str =
        "transaction/preview-v2";
    pub const PATH_TRANSACTION_SUBMIT: &'static str = "transaction/submit";
    pub const PATH_TRANSACTION_STATUS: &'static str = "transaction/status";
    pub const PATH_TRANSACTION_SUBINTENT_STATUS: &'static str =
        "transaction/subintent-status";
    pub const PATH_STATE_ENTITY_DETAILS: &str = "state/entity/details";
    pub const PATH_STATE_ENTITY_PAGE_FUNGIBLES: &str =
        "state/entity/page/fungibles/";
    pub const PATH_STATE_ENTITY_PAGE_NON_FUNGIBLES: &str =
        "state/entity/page/non-fungibles/";
    pub const PATH_STATE_ENTITY_PAGE_NON_FUNGIBLE_VAULTS: &str =
        "state/entity/page/non_fungible-vaults/";
    pub const PATH_STATE_ENTITY_PAGE_NON_FUNGIBLE_VAULT_IDS: &str =
        "state/entity/page/non_fungible-vault/ids";
    pub const PATH_ACCOUNT_PAGE_RESOURCE_PREFERENCES: &str =
        "state/account/page/resource-preferences";
    pub const PATH_ACCOUNT_PAGE_AUTHORIZED_DEPOSITORS: &str =
        "state/account/page/authorized-depositors";
    pub const PATH_STATE_NON_FUNGIBLE_LOCATION: &str =
        "/state/non-fungible/location";
    pub const PATH_STATE_NON_FUNGIBLE_DATA: &str = "/state/non-fungible/data";
}
