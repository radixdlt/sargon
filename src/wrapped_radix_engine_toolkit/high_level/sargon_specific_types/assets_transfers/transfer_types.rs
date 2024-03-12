use crate::prelude::*;

macro_rules! decl_transfer_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_prefix: ident,
        $struct_name: ident,
        $($fields:tt)*
    ) => {
        paste! {
            $(
                #[doc = $expr]
            )*
            #[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
            pub struct [< $struct_prefix $struct_name >] {

                /// If `true` the `try_deposit_batch_or_abort` method will be used instead of `deposit`,
                /// typically wallets sets this to try if and only if the recipient is a self-owned account
                /// (`AssetsTransfersRecipient::MyOwnAccount`) controlled by a DeviceFactorSource thy have
                /// access to and which third party deposit setting's `DepositRule` is `AcceptKnown` and
                /// which resource is known (`resource_address` is owned or has been owned before).
                pub(crate) use_try_deposit_or_abort: bool,

                $($fields)*

            }
        }
    };
}

macro_rules! decl_per_asset_transfer_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $($fields:tt)*
    ) => {

        decl_transfer_of!(
            $(
                #[doc = $expr]
            )*
            PerAsset,
            $struct_name,
            $($fields)*
            /// The account or account address to send the tokens to.
            pub recipient: AssetsTransfersRecipient,
        );

        paste! {
            impl [< PerAsset $struct_name >] {

                pub(crate) fn deposit_instruction(&self, builder: ScryptoManifestBuilder, bucket: &Bucket) -> ScryptoManifestBuilder {

                    if self.use_try_deposit_or_abort {
                        return builder.try_deposit_or_abort(
                            self.recipient.account_address(),
                            None,
                            bucket,
                        )
                    } else {
                        return builder
                            .deposit(self.recipient.account_address(), bucket);
                    }
                }
            }
        }
    };
}

macro_rules! decl_per_reci_transfer_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $($fields:tt)*
    ) => {

        decl_transfer_of!(
            $(
                #[doc = $expr]
            )*
            PerRecipient,
            $struct_name,
            $($fields)*
            /// The address of the resource being sent
            pub resource_address: ResourceAddress,
        );
    };
}

decl_per_asset_transfer_of!(
    /// A fungible transfer to `recipient`, with a specified amount of tokens to send.
    FungibleTransfer,
    /// Amount
    pub(crate) amount: Decimal192,
);

decl_per_asset_transfer_of!(
    /// A non fungible transfer to `recipient`, with specified Local IDs to send.
    NonFungibleTransfer,
    /// Amount
    pub(crate) non_fungible_local_ids: Vec<NonFungibleLocalId>,
);

decl_per_reci_transfer_of!(
    /// A fungible transfer of `resource_address` token, with a specified amount
    /// of tokens and divisibility.
    FungibleTransfer,
    /// Amount
    pub(crate) amount: Decimal192,
    pub divisibility: Option<i32>,
);

decl_per_reci_transfer_of!(
    /// A non fungible transfer of `resource_address` token, with specified Local IDs to send.
    NonFungiblesTransfer,
    /// The local IDS of the NonFungible tokens being sent
    pub(crate) local_ids: Vec<NonFungibleLocalId>,
);

impl PerAssetFungibleTransfer {
    pub fn new(
        recipient: impl Into<AssetsTransfersRecipient>,
        use_try_deposit_or_abort: bool,
        amount: impl Into<Decimal192>,
    ) -> Self {
        Self {
            recipient: recipient.into(),
            use_try_deposit_or_abort,
            amount: amount.into(),
        }
    }

    pub fn amount(
        &self,
        divisibility: impl Into<Option<i32>>,
    ) -> ScryptoDecimal192 {
        self.amount.round(divisibility).into()
    }
}

impl From<(&AssetsTransfersRecipient, PerRecipientFungibleTransfer)>
    for PerAssetFungibleTransfer
{
    fn from(
        value: (&AssetsTransfersRecipient, PerRecipientFungibleTransfer),
    ) -> Self {
        let (recipient, transfer) = value;
        Self::new(
            recipient.clone(),
            transfer.use_try_deposit_or_abort,
            transfer.amount,
        )
    }
}

impl HasSampleValues for PerAssetFungibleTransfer {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
    }
}

impl PerAssetFungibleTransfer {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            AssetsTransfersRecipient::MyOwnAccount {
                value: Account::sample_mainnet_carol(),
            },
            true,
            Decimal192::from_str("237.13372718281828").unwrap(),
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(AssetsTransfersRecipient::ForeignAccount {
            value: AccountAddress::from_str("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69").unwrap() 
        },
        true,
        Decimal192::from_str("987654.1234").unwrap())
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            AssetsTransfersRecipient::MyOwnAccount {
                value: Account::sample_stokenet_diana(),
            },
            true,
            Decimal192::from_str("42.311415").unwrap(),
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(AssetsTransfersRecipient::ForeignAccount {
            value: AccountAddress::from_str("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk").unwrap() 
        },
        true,
        Decimal192::from_str("1337.2371828128").unwrap())
    }
}

impl PerAssetNonFungibleTransfer {
    pub fn new(
        recipient: impl Into<AssetsTransfersRecipient>,
        use_try_deposit_or_abort: bool,
        non_fungible_local_ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self {
            recipient: recipient.into(),
            use_try_deposit_or_abort,
            non_fungible_local_ids: non_fungible_local_ids
                .into_iter()
                .collect_vec(),
        }
    }

    pub fn local_ids(&self) -> Vec<ScryptoNonFungibleLocalId> {
        self.non_fungible_local_ids
            .clone()
            .into_iter()
            .map(ScryptoNonFungibleLocalId::from)
            .collect_vec()
    }
}

impl From<(&AssetsTransfersRecipient, PerRecipientNonFungiblesTransfer)>
    for PerAssetNonFungibleTransfer
{
    fn from(
        value: (&AssetsTransfersRecipient, PerRecipientNonFungiblesTransfer),
    ) -> Self {
        let (recipient, non_fungibles) = value;
        Self::new(
            recipient.clone(),
            non_fungibles.use_try_deposit_or_abort,
            non_fungibles.local_ids,
        )
    }
}

impl PerAssetNonFungibleTransfer {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            AssetsTransfersRecipient::MyOwnAccount {
                value: Account::sample_mainnet_carol(),
            },
            true,
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            AssetsTransfersRecipient::ForeignAccount {
                value: AccountAddress::from_str("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69").unwrap() 
            },
        true,
        [NonFungibleLocalId::sample_other()]
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            AssetsTransfersRecipient::MyOwnAccount {
                value: Account::sample_stokenet_carol(),
            },
            true,
            [
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            AssetsTransfersRecipient::ForeignAccount {
                value: AccountAddress::from_str("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk").unwrap() 
            },
        true,
        [NonFungibleLocalId::sample_other()]
        )
    }
}

impl HasSampleValues for PerAssetNonFungibleTransfer {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
    }
}

impl PerRecipientNonFungiblesTransfer {
    pub fn new(
        resource_address: impl Into<ResourceAddress>,
        use_try_deposit_or_abort: bool,
        local_ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) -> Self {
        Self {
            resource_address: resource_address.into(),
            use_try_deposit_or_abort,
            local_ids: local_ids.into_iter().collect_vec(),
        }
    }
}

impl PerRecipientFungibleTransfer {
    pub fn new(
        resource_address: ResourceAddress,
        amount: impl Into<Decimal192>,
        use_try_deposit_or_abort: bool,
        divisibility: impl Into<Option<i32>>,
    ) -> Self {
        Self {
            resource_address,
            amount: amount.into(),
            use_try_deposit_or_abort,
            divisibility: divisibility.into(),
        }
    }
}
