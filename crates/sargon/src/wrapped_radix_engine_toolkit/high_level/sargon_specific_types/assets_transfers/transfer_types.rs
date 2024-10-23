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
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            pub struct [< $struct_prefix $struct_name Transfer >] {

                /// If `true` the `try_deposit_batch_or_abort` method will be used instead of `deposit`,
                /// typically wallets sets this to try if and only if the recipient is a self-owned account
                /// (`AccountOrAddressOf::ProfileAccount`) controlled by a DeviceFactorSource thy have
                /// access to and which third party deposit setting's `DepositRule` is `AcceptKnown` and
                /// which resource is known (`resource_address` is owned or has been owned before).
                pub use_try_deposit_or_abort: bool,

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
            pub recipient: AccountOrAddressOf,
        );

        paste! {
            impl [< PerAsset $struct_name Transfer>] {

                pub(crate) fn deposit_instruction(&self, builder: ScryptoTransactionManifestBuilder, bucket: &Bucket) -> ScryptoTransactionManifestBuilder {

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

macro_rules! decl_per_recipient_transfer_of {
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
    Fungible,
    /// Amount
    pub amount: Decimal192,
);

decl_per_asset_transfer_of!(
    /// A non fungible transfer to `recipient`, with specified Local IDs to send.
    NonFungible,
    /// Amount
    pub non_fungible_local_ids: Vec<NonFungibleLocalId>,
);

decl_per_recipient_transfer_of!(
    /// A fungible transfer of `resource_address` token, with a specified amount
    /// of tokens and divisibility.
    Fungible,
    /// Amount
    pub amount: Decimal192,
    pub divisibility: Option<u8>,
);

decl_per_recipient_transfer_of!(
    /// A non fungible transfer of `resource_address` token, with specified Local IDs to send.
    NonFungible,
    /// The local IDS of the NonFungible tokens being sent
    pub local_ids: Vec<NonFungibleLocalId>,
);
