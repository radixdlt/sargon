use crate::prelude::*;

/// A summary of the manifest
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestSummary {
    /// The withdrawals done in the manifest.
    pub account_withdrawals: HashMap<AccountAddress, Vec<AccountWithdraw>>,

    /// The deposits done in the manifest.
    pub account_deposits: HashMap<AccountAddress, Vec<AccountDeposit>>,

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
        account_withdraws: impl Into<HashMap<AccountAddress, Vec<AccountWithdraw>>>,
        account_deposits: impl Into<HashMap<AccountAddress, Vec<AccountDeposit>>>,
        withdrawn_from: impl IntoIterator<Item = AccountAddress>,
        deposited_into: impl IntoIterator<Item = AccountAddress>,
        accounts_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        personas_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        Self {
            account_withdrawals: account_withdraws.into(),
            account_deposits: account_deposits.into(),
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

fn convert_from_scrypto<T, U>(
    scrypto: IndexMap<ScryptoComponentAddress, Vec<T>>,
    network_id: NetworkID,
    convert_item: fn(T, NetworkID) -> U,
) -> HashMap<AccountAddress, Vec<U>> {
    scrypto
        .into_iter()
        .map(|(addr, items)| {
            (
                AccountAddress::from((addr, network_id)),
                items
                    .into_iter()
                    .map(|item| convert_item(item, network_id))
                    .collect(),
            )
        })
        .collect()
}

fn account_withdraw_from_scrypto(
    item: ScryptoAccountWithdraw,
    network_id: NetworkID,
) -> AccountWithdraw {
    AccountWithdraw::from((item, network_id))
}

fn account_deposit_from_scrypto(
    item: ScryptoAccountDeposit,
    network_id: NetworkID,
) -> AccountDeposit {
    AccountDeposit::from((item, network_id))
}

impl From<(RetStaticAnalysis, NetworkID)> for ManifestSummary {
    fn from(value: (RetStaticAnalysis, NetworkID)) -> Self {
        let (ret, network_id) = value;

        let account_withdraws = convert_from_scrypto(
            ret.account_withdraws,
            network_id,
            account_withdraw_from_scrypto,
        );

        let account_deposits = convert_from_scrypto(
            ret.account_deposits,
            network_id,
            account_deposit_from_scrypto,
        );

        let addresses_of_accounts_withdrawn_from =
            to_vec_network_aware(ret.accounts_withdrawn_from, network_id);

        let addresses_of_accounts_deposited_into =
            to_vec_network_aware(ret.accounts_deposited_into, network_id);

        let addresses_of_accounts_requiring_auth =
            to_vec_network_aware(ret.accounts_requiring_auth, network_id);

        let addresses_of_personas_requiring_auth =
            to_vec_network_aware(ret.identities_requiring_auth, network_id);

        Self::new(
            account_withdraws,
            account_deposits,
            addresses_of_accounts_withdrawn_from,
            addresses_of_accounts_deposited_into,
            addresses_of_accounts_requiring_auth,
            addresses_of_personas_requiring_auth,
        )
    }
}

impl HasSampleValues for ManifestSummary {
    fn sample() -> Self {
        Self {
            addresses_of_accounts_withdrawn_from: vec![AccountAddress::sample()],
            addresses_of_accounts_deposited_into: vec![AccountAddress::sample()],
            addresses_of_accounts_requiring_auth: vec![AccountAddress::sample()],
            addresses_of_personas_requiring_auth: vec![
                IdentityAddress::sample(),
            ],
        }
    }

    fn sample_other() -> Self {
        Self {
            addresses_of_accounts_withdrawn_from: vec![
                AccountAddress::sample_other(),
            ],
            addresses_of_accounts_deposited_into: vec![
                AccountAddress::sample_other(),
            ],
            addresses_of_accounts_requiring_auth: vec![
                AccountAddress::sample_other(),
            ],
            addresses_of_personas_requiring_auth: vec![
                IdentityAddress::sample_other(),
            ],
        }
    }
}
