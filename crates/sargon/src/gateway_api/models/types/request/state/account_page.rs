use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountPageRequest {
    /// Bech32m-encoded human readable version of the address.
    pub(crate) account_address: AccountAddress,

    /// This cursor allows forward pagination, by providing the cursor from the previous request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cursor: Option<String>,

    /// The page size requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit_per_page: Option<u64>,
}

impl AccountPageRequest {
    pub fn new(
        account_address: AccountAddress,
        cursor: Option<String>,
        limit_per_page: impl Into<Option<u64>>,
    ) -> AccountPageRequest {
        AccountPageRequest {
            account_address,
            cursor,
            limit_per_page: limit_per_page.into(),
        }
    }
}
