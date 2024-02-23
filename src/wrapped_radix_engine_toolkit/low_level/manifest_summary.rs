use crate::prelude::*;

use radix_engine_toolkit_uniffi::{
    Instructions as RetInstructions, ManifestSummary as RetManifestSummary,
    TransactionManifest as RetTransactionManifest,
};

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ManifestSummary {
    pub addresses_of_accounts_deposited_into: Vec<AccountAddress>,
    pub addresses_of_accounts_withdrawn_from: Vec<AccountAddress>,
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,
    pub addresses_of_personas_requiring_auth: Vec<IdentityAddress>,
}

impl TryFrom<RetManifestSummary> for ManifestSummary {
    type Error = crate::CommonError;

    fn try_from(value: RetManifestSummary) -> Result<Self> {
        let addresses_of_accounts_deposited_into =
            try_map_addresses_from_ret(value.accounts_deposited_into)?;
        let addresses_of_accounts_withdrawn_from =
            try_map_addresses_from_ret(value.accounts_withdrawn_from)?;
        let addresses_of_accounts_requiring_auth =
            try_map_addresses_from_ret(value.accounts_requiring_auth)?;
        let addresses_of_personas_requiring_auth =
            try_map_addresses_from_ret(value.identities_requiring_auth)?;

        Ok(Self {
            addresses_of_accounts_deposited_into,
            addresses_of_accounts_withdrawn_from,
            addresses_of_accounts_requiring_auth,
            addresses_of_personas_requiring_auth,
        })
    }
}

pub fn try_map_addresses_from_ret<
    A: TryFrom<radix_engine_toolkit_uniffi::prelude::Address, Error = CommonError>,
>(
    addresses: Vec<Arc<radix_engine_toolkit_uniffi::prelude::Address>>,
) -> Result<Vec<A>> {
    addresses
        .into_iter()
        .map(|a: Arc<radix_engine_toolkit_uniffi::prelude::Address>| *a)
        .map(|a| {
            let res: Result<A, CommonError> = a.try_into();
            res
        })
        .collect()
}
