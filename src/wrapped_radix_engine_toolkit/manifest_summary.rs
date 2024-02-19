use crate::prelude::*;

dummy_sargon!(ManifestSummary);

impl ManifestSummary {
    pub fn addresses_of_accounts_deposited_into(&self) -> Vec<AccountAddress> {
        todo!()
    }

    pub fn addresses_of_accounts_withdrawn_from(&self) -> Vec<AccountAddress> {
        todo!()
    }

    pub fn addresses_of_accounts_requiring_auth(&self) -> Vec<AccountAddress> {
        todo!()
    }

    pub fn addresses_of_personas_requiring_auth(&self) -> Vec<IdentityAddress> {
        todo!()
    }
}
