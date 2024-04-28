use crate::prelude::*;

declare_shared_with_dapp!(
    /// IDs that have been shared with an Dapp the user has interacted with
    /// that fulfill a Dapp request's specified [`RequestedQuantity`].
    SharedToDappWithPersonaAccountAddresses,
    AccountAddress,
    "Exactly: 2 - #2 ids shared",
    "Exactly: 2 - shared ids: [account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8, account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69]",
    r#"
    {
        "request": {
            "quantifier": "exactly",
            "quantity": 2
        },
        "ids": [
            "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8",
            "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
        ]
    }
    "#
);

impl HasSampleValues for SharedToDappWithPersonaAccountAddresses {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}
impl SharedToDappWithPersonaAccountAddresses {
    pub fn sample_mainnet() -> Self {
        Self::new(
            RequestedQuantity::exactly(2),
            IdentifiedVecVia::from_iter([
                Account::sample_mainnet().address,
                Account::sample_mainnet_other().address,
            ]),
        )
    }
    pub fn sample_mainnet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            IdentifiedVecVia::from_iter([
                Account::sample_mainnet_other().address
            ]),
        )
    }
    pub fn sample_stokenet() -> Self {
        Self::new(
            RequestedQuantity::exactly(2),
            IdentifiedVecVia::from_iter([
                AccountAddress::sample_stokenet(),
                AccountAddress::sample_stokenet_other(),
            ]),
        )
    }
    pub fn sample_stokenet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            IdentifiedVecVia::from_iter([
                AccountAddress::sample_stokenet_other(),
            ]),
        )
    }
}
