use crate::prelude::*;

declare_shared_with_dapp!(
    /// IDs that have been shared with an Dapp the user has interacted with
    /// that fulfill a Dapp request's specified [`RequestedQuantity`].
    SharedToDappWithPersonaAccountAddresses,
    AccountAddress,
    "Exactly: 2 - #2 ids shared",
    "Exactly: 2 - shared ids: [account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr, account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264]",
    r#"
    {
        "request": {
            "quantifier": "exactly",
            "quantity": 2
        },
        "ids": [
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
            "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
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
                AccountAddress::sample_mainnet(),
                AccountAddress::sample_mainnet_other(),
            ]),
        )
    }
    pub fn sample_mainnet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            IdentifiedVecVia::from_iter([
                AccountAddress::sample_mainnet_other(),
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
