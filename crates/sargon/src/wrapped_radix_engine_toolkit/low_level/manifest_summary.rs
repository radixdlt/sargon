use crate::prelude::*;

/// A summary of the manifest
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ManifestSummary {
    /// Addresses of accounts withdrawn from in the manifest.
    pub addresses_of_accounts_withdrawn_from: Vec<AccountAddress>,

    /// Addresses of accounts deposited into in the manifest.
    pub addresses_of_accounts_deposited_into: Vec<AccountAddress>,

    /// Addresses of accounts encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the accounts of all those addresses, which might be multiple
    /// signatures per Account, if MFA has been setup.
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,

    /// Addresses of identities (Personas) encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the identities of all those addresses, which might be multiple
    /// signatures per Persona, if MFA has been setup.
    pub addresses_of_personas_requiring_auth: Vec<IdentityAddress>,
}

impl ManifestSummary {
    pub fn new(
        withdrawn_from: impl IntoIterator<Item = AccountAddress>,
        deposited_into: impl IntoIterator<Item = AccountAddress>,
        accounts_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        personas_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        Self {
            addresses_of_accounts_withdrawn_from: withdrawn_from
                .into_iter()
                .collect(),
            addresses_of_accounts_deposited_into: deposited_into
                .into_iter()
                .collect(),
            addresses_of_accounts_requiring_auth: accounts_requiring_auth
                .into_iter()
                .collect(),
            addresses_of_personas_requiring_auth: personas_requiring_auth
                .into_iter()
                .collect(),
        }
    }
}

impl From<(RetStaticAnalysis, NetworkID)> for ManifestSummary {
    fn from(value: (RetStaticAnalysis, NetworkID)) -> Self {
        let (ret, network_id) = value;

        let addresses_of_accounts_withdrawn_from =
            to_vec_network_aware(ret.accounts_withdrawn_from, network_id);

        let addresses_of_accounts_deposited_into =
            to_vec_network_aware(ret.accounts_deposited_into, network_id);

        let addresses_of_accounts_requiring_auth =
            to_vec_network_aware(ret.accounts_requiring_auth, network_id);

        let addresses_of_personas_requiring_auth =
            to_vec_network_aware(ret.identities_requiring_auth, network_id);

        Self::new(
            addresses_of_accounts_withdrawn_from,
            addresses_of_accounts_deposited_into,
            addresses_of_accounts_requiring_auth,
            addresses_of_personas_requiring_auth,
        )
    }
}
