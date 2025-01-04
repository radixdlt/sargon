use crate::prelude::*;

declare_shared_with_dapp!(
    /// IDs that have been shared with an Dapp the user has interacted with
    /// that fulfill a Dapp request's specified [`RequestedQuantity`].
    SharedToDappWithPersonaAccountAddresses,
    AccountAddress,
    "Exactly: 2 - #2 ids shared",
    "Exactly: 2 - shared ids: [account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87, account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7]",
    r#"
    {
        "request": {
            "quantifier": "exactly",
            "quantity": 2
        },
        "ids": [
            "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
            "account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
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
            [
                "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87".parse().unwrap(),
                "account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7".parse().unwrap(),
            ],
        )
    }
    pub fn sample_mainnet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            ["account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7".parse().unwrap()],
        )
    }
    pub fn sample_stokenet() -> Self {
        Self::new(
            RequestedQuantity::exactly(2),
            [
                AccountAddress::sample_stokenet(),
                AccountAddress::sample_stokenet_other(),
            ],
        )
    }
    pub fn sample_stokenet_other() -> Self {
        Self::new(
            RequestedQuantity::at_least(1),
            [AccountAddress::sample_stokenet_other()],
        )
    }
}
