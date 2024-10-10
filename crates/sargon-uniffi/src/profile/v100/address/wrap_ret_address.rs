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
            use sargon::[< $address_type:camel Address >] as InternalAddress;

            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                PartialEq,
                Eq,
                Hash,
                InternalConversion,
                 uniffi::Record,
            )]
            pub struct [< $address_type:camel Address >] {
                pub(crate) secret_magic: String,
            }

            delegate_display_debug_into!([< $address_type:camel Address >], InternalAddress);

            impl From<InternalAddress> for [< $address_type:camel Address >] {
                fn from(value: InternalAddress) -> Self {
                    Self { secret_magic: value.to_string() }
                }
            }

            impl Into<InternalAddress> for [< $address_type:camel Address >] {
                fn into(self) -> InternalAddress {
                    self.secret_magic.parse().unwrap()
                }
            }

            #[uniffi::export]
            pub fn [<new_ $address_type:snake _address>](bech32: String) -> Result<[< $address_type:camel Address >]> {
                InternalAddress::try_from_bech32(&bech32).map_result()
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
                InternalAddress::random(network_id.into()).into()
            }

            #[cfg(test)]
            mod [<uniffi_tests_of_ $address_type:snake>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = [< $address_type:camel Address >];

                #[test]
                fn map_to_network() {
                    let sut = SUT::sample();
                    assert_eq!([<$address_type:snake _address_map_to_network>](&sut, sut.network_id()), sut); // unchanged
                }

                #[test]
                fn random_address() {
                    let n = 100;
                    for network_id in NetworkID::all() {
                        let addresses = (0..n)
                            .map(|_| [<new_ $address_type:snake _address_random>](network_id))
                            .collect::<HashSet<SUT>>();
                        assert_eq!(addresses.len(), n);
                    }
                }
            }
        }
    };
}

pub(crate) use decl_ret_wrapped_address;