use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ManifestSummary {
    pub addresses_of_accounts_deposited_into: Vec<AccountAddress>,
    pub addresses_of_accounts_withdrawn_from: Vec<AccountAddress>,
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,
    pub addresses_of_personas_requiring_auth: Vec<IdentityAddress>,
}

impl ManifestSummary {
    pub fn from_ret(
        _ret_summary: RetManifestSummary,
        _network_id: NetworkID,
    ) -> Self {
        // let addresses_of_accounts_deposited_into =
        //     try_map_addresses_from_ret(value.accounts_deposited_into)?;
        // let addresses_of_accounts_withdrawn_from =
        //     try_map_addresses_from_ret(value.accounts_withdrawn_from)?;
        // let addresses_of_accounts_requiring_auth =
        //     try_map_addresses_from_ret(value.accounts_requiring_auth)?;
        // let addresses_of_personas_requiring_auth =
        //     try_map_addresses_from_ret(value.identities_requiring_auth)?;

        // Ok(Self {
        //     addresses_of_accounts_deposited_into,
        //     addresses_of_accounts_withdrawn_from,
        //     addresses_of_accounts_requiring_auth,
        //     addresses_of_personas_requiring_auth,
        // })
        todo!()
    }
}
