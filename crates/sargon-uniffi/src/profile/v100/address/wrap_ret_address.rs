use crate::prelude::*;

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_ret_wrapped_address {
    (
        $(
            #[doc = $expr: expr]
        )*
        $address_type: ident
    ) => {
        paste! {
            use sargon::IsNetworkAware;
            use sargon::[< $address_type:camel Address >] as [< Internal $address_type:camel Address >];

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
            pub struct [< $address_type:camel Address >] {
                pub(crate) secret_magic: String,
            }

            delegate_display_debug_into!([< $address_type:camel Address >], [< Internal $address_type:camel Address >]);

            impl [< $address_type:camel Address >] {
                pub fn into_internal(&self) -> [< Internal $address_type:camel Address >] {
                    self.clone().into()
                }
            }

            impl From<[< Internal $address_type:camel Address >]> for [< $address_type:camel Address >] {
                fn from(internal: [< Internal $address_type:camel Address >]) -> Self {
                    Self {
                        secret_magic: internal.to_string(),
                    }
                }
            }

            #[allow(clippy::from_over_into)]
            impl Into<[< Internal $address_type:camel Address >]> for [< $address_type:camel Address >] {
                fn into(self) -> [< Internal $address_type:camel Address >] {
                    self.secret_magic.parse().unwrap()
                }
            }

            #[uniffi::export]
            pub fn [<new_ $address_type:snake _address>](bech32: String) -> Result<[< $address_type:camel Address >]> {
                [< Internal $address_type:camel Address >]::try_from_bech32(&bech32).into_result()
            }

            /// Returns a new address, with the same node_id, but using `network_id` as
            /// network.
            #[uniffi::export]
            pub fn [<$address_type:snake _address_map_to_network>](address: &[< $address_type:camel Address >], network_id: NetworkID) -> [< $address_type:camel Address >] {
                address.into_internal().map_to_network(network_id.into()).into()
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_network_id>](address: &[< $address_type:camel Address >]) -> NetworkID {
                address.into_internal().network_id().into()
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_bech32_address>](address: &[< $address_type:camel Address >]) -> String {
                address.into_internal().address()
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_formatted>](address: &[< $address_type:camel Address >], format: AddressFormat) -> String {
                address.into_internal().formatted(format.into())
            }

            /// Returns a random address in `network_id` as Network
            #[uniffi::export]
            pub fn [<new_ $address_type:snake _address_random>](network_id: NetworkID) -> [<$address_type:camel Address >] {
                [< Internal $address_type:camel Address >]::random(network_id.into()).into()
            }

            decl_conversion_tests_for!([< $address_type:camel Address >]);
        }
    };
}

pub(crate) use decl_ret_wrapped_address;
