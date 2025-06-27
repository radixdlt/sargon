use crate::prelude::*;

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_specialized_address {
    (
        $(
            #[doc = $expr: expr]
        )*
        $specialized_address_type: ident,
        $base_addr: ty,
        $validate: ident,
        $validation_err: ident
    ) => {

        paste! {
            use sargon::$specialized_address_type as [< Internal $specialized_address_type >];
            use sargon::IsNetworkAware;

            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,

                PartialEq,
                Eq,
                Hash,
                 uniffi::Record,
            )]
            pub struct $specialized_address_type {
                value: $base_addr
            }

            impl $specialized_address_type {
                pub fn into_internal(&self) -> [< Internal $specialized_address_type >] {
                    self.clone().into()
                }
            }

            impl From<[< Internal $specialized_address_type >]> for $specialized_address_type {
                fn from(value: [< Internal $specialized_address_type >]) -> Self {
                    Self { value: value.0.into() }
                }
            }

            #[allow(clippy::from_over_into)]
            impl Into<[< Internal $specialized_address_type >]> for $specialized_address_type {
                #[allow(clippy::from_over_into)]
                fn into(self) -> [< Internal $specialized_address_type >] {
                    [< Internal $specialized_address_type >](self.value.into())
                }
            }

            /// Tries to bech32 decode the string into a specialized address.
            #[uniffi::export]
            pub fn [< new_ $specialized_address_type:snake >](bech32: String) -> Result<$specialized_address_type> {
                [< Internal $specialized_address_type >]::new_from_bech32(bech32).into_result()
            }

            /// Returns the base address of this specialized address.
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _as_ $base_addr:snake>](address: &$specialized_address_type) -> $base_addr {
                address.value.clone()
            }

            /// Returns a new address, with the same node_id, but using `network_id` as
            /// network.
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _map_to_network >](address: &$specialized_address_type, network_id: NetworkID) -> $specialized_address_type {
                address.into_internal().map_to_network(network_id.into()).into()
            }

            /// Returns the bech32 encoding of this address
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _bech32_address >](address: &$specialized_address_type) -> String {
                address.into_internal().bech32_address()
            }

            /// Returns the network id this address
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _network_id >](address: &$specialized_address_type) -> NetworkID {
                address.into_internal().network_id().into()
            }

            decl_conversion_tests_for!($specialized_address_type);
        }
    };
}

decl_specialized_address!(
    /// NonFungibleResourceAddress is a specialized ResourceAddress for resources
    /// which are non fungible, it ALWAYS has an `'n'` after bech32 separator `'1'`, e.g.:
    /// `"resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"`.
    ///
    /// As opposed to a fungible resource address, e.g. that of XRD which has `'t'`
    /// after bech32 separator `'1'`, see:
    /// `"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"`
    ///
    /// This means that given an instance of `NonFungibleResourceAddress`, it is
    /// guaranteed that its entity type is [`::GlobalNonFungibleResourceManager`],
    /// and not `::GlobalFungibleResourceManager`.
    ///
    /// This type can safely be used with [`StakeClaim`]s, unfortunately since Radix Engine
    /// and/or network does not validate the resource address of a `NonFungibleGlobalId`,
    /// we cannot use this for that type.
    NonFungibleResourceAddress,
    ResourceAddress,
    is_non_fungible,
    FungibleResourceAddressNotAcceptedInNonFungibleContext
);

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_mainnet(
) -> NonFungibleResourceAddress {
    InternalNonFungibleResourceAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_mainnet_other(
) -> NonFungibleResourceAddress {
    InternalNonFungibleResourceAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_stokenet(
) -> NonFungibleResourceAddress {
    InternalNonFungibleResourceAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_stokenet_other(
) -> NonFungibleResourceAddress {
    InternalNonFungibleResourceAddress::sample_stokenet_other().into()
}

/// Returns a random address in `network_id` as Network
#[uniffi::export]
pub fn new_non_fungible_resource_address_random(
    network_id: NetworkID,
) -> NonFungibleResourceAddress {
    InternalNonFungibleResourceAddress::random(network_id.into()).into()
}
