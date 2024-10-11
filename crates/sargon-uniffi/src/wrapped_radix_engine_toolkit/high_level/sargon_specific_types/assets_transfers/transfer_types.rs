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
            #[derive(Clone,  PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
            pub struct [< $struct_prefix $struct_name Transfer >] {

                /// If `true` the `try_deposit_batch_or_abort` method will be used instead of `deposit`,
                /// typically wallets sets this to try if and only if the recipient is a self-owned account
                /// (`AccountOrAddressOf::ProfileAccount`) controlled by a DeviceFactorSource thy have
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
            pub recipient: AccountOrAddressOf,
        );
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
    pub(crate) amount: Decimal192,
);

decl_per_asset_transfer_of!(
    /// A non fungible transfer to `recipient`, with specified Local IDs to send.
    NonFungible,
    /// Amount
    pub(crate) non_fungible_local_ids: Vec<NonFungibleLocalId>,
);

decl_per_recipient_transfer_of!(
    /// A fungible transfer of `resource_address` token, with a specified amount
    /// of tokens and divisibility.
    Fungible,
    /// Amount
    pub(crate) amount: Decimal192,
    pub divisibility: Option<u8>,
);

decl_per_recipient_transfer_of!(
    /// A non fungible transfer of `resource_address` token, with specified Local IDs to send.
    NonFungible,
    /// The local IDS of the NonFungible tokens being sent
    pub(crate) local_ids: Vec<NonFungibleLocalId>,
);

use sargon::PerAssetFungibleTransfer as InternalPerAssetFungibleTransfer;
use sargon::PerAssetNonFungibleTransfer as InternalPerAssetNonFungibleTransfer;
use sargon::PerRecipientFungibleTransfer as InternalPerRecipientFungibleTransfer;
use sargon::PerRecipientNonFungibleTransfer as InternalPerRecipientNonFungibleTransfer;

impl From<InternalPerAssetFungibleTransfer> for PerAssetFungibleTransfer {
    fn from(value: InternalPerAssetFungibleTransfer) -> Self {
        Self {
            use_try_deposit_or_abort: value.use_try_deposit_or_abort,
            recipient: value.recipient.into(),
            amount: value.amount.into(),
        }
    }
}

impl Into<InternalPerAssetFungibleTransfer> for PerAssetFungibleTransfer {
    fn into(self) -> InternalPerAssetFungibleTransfer {
        InternalPerAssetFungibleTransfer {
            use_try_deposit_or_abort: self.use_try_deposit_or_abort,
            recipient: self.recipient.into(),
            amount: self.amount.into(),
        }
    }
}

impl From<InternalPerAssetNonFungibleTransfer> for PerAssetNonFungibleTransfer {
    fn from(value: InternalPerAssetNonFungibleTransfer) -> Self {
        Self {
            use_try_deposit_or_abort: value.use_try_deposit_or_abort,
            recipient: value.recipient.into(),
            non_fungible_local_ids: value.non_fungible_local_ids.into_vec(),
        }
    }
}

impl Into<InternalPerAssetNonFungibleTransfer> for PerAssetNonFungibleTransfer {
    fn into(self) -> InternalPerAssetNonFungibleTransfer {
        InternalPerAssetNonFungibleTransfer {
            use_try_deposit_or_abort: self.use_try_deposit_or_abort,
            recipient: self.recipient.into(),
            non_fungible_local_ids: self
                .non_fungible_local_ids
                .into_internal_vec(),
        }
    }
}

impl From<InternalPerRecipientFungibleTransfer>
    for PerRecipientFungibleTransfer
{
    fn from(value: InternalPerRecipientFungibleTransfer) -> Self {
        Self {
            use_try_deposit_or_abort: value.use_try_deposit_or_abort,
            resource_address: value.resource_address.into(),
            amount: value.amount.into(),
            divisibility: value.divisibility,
        }
    }
}

impl Into<InternalPerRecipientFungibleTransfer>
    for PerRecipientFungibleTransfer
{
    fn into(self) -> InternalPerRecipientFungibleTransfer {
        InternalPerRecipientFungibleTransfer {
            use_try_deposit_or_abort: self.use_try_deposit_or_abort,
            resource_address: self.resource_address.into(),
            amount: self.amount.into(),
            divisibility: self.divisibility,
        }
    }
}

impl From<InternalPerRecipientNonFungibleTransfer>
    for PerRecipientNonFungibleTransfer
{
    fn from(value: InternalPerRecipientNonFungibleTransfer) -> Self {
        Self {
            use_try_deposit_or_abort: value.use_try_deposit_or_abort,
            resource_address: value.resource_address.into(),
            local_ids: value.local_ids.into_vec(),
        }
    }
}

impl Into<InternalPerRecipientNonFungibleTransfer>
    for PerRecipientNonFungibleTransfer
{
    fn into(self) -> InternalPerRecipientNonFungibleTransfer {
        InternalPerRecipientNonFungibleTransfer {
            use_try_deposit_or_abort: self.use_try_deposit_or_abort,
            resource_address: self.resource_address.into(),
            local_ids: self.local_ids.into_internal_vec(),
        }
    }
}
