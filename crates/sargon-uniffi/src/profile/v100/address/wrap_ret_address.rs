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
            $(
                #[doc = $expr]
            )*
            use sargon::[< $address_type:camel Address >] as [< Internal $address_type:camel Address >];
            #[derive(
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                derive_more::Display,
                derive_more::Debug,
                uniffi::Record,
            )]
            #[display("{secret_magic}")]
            #[debug("{secret_magic}")]
            pub struct [< $address_type:camel Address >] {
                pub(crate) secret_magic: [< Internal $address_type:camel Address >], // Do NOT add comments above
            }

            #[uniffi::export]
            pub fn [<new_ $address_type:snake _address>](bech32: String) -> Result<[< $address_type:camel Address >]> {
                map_result_from_internal([< Internal $address_type:camel Address >]::try_from_bech32(&bech32))
            }

            impl From<[< Internal $address_type:camel Address >]> for [< $address_type:camel Address >] {
                fn from(value: [< Internal $address_type:camel Address >]) -> Self {
                    Self { secret_magic: value }
                }
            }

            impl Into<[< Internal $address_type:camel Address >]> for [< $address_type:camel Address > {
                fn into(self) -> [< Internal $address_type:camel Address >] {
                    self.secret_magic
                }
            }

            /// Returns a new address, with the same node_id, but using `network_id` as
            /// network.
            #[uniffi::export]
            pub fn [<$address_type:snake _address_map_to_network>](address: &[< $address_type:camel Address >], network_id: NetworkID) -> [< $address_type:camel Address >] {
                address.secret_magic.map_to_network(network_id).into()
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_network_id>](address: &[< $address_type:camel Address >]) -> NetworkID {
                address.secret_magic.network_id().into()
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_bech32_address>](address: &[< $address_type:camel Address >]) -> String {
                address.secret_magic.address()
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_formatted>](address: &[< $address_type:camel Address >], format: AddressFormat) -> String {
                address.secret_magic.formatted(format)
            }

            /// Returns a random address in `network_id` as Network
            #[uniffi::export]
            pub fn [<new_ $address_type:snake _address_random>](network_id: NetworkID) -> [<$address_type:camel Address >] {
                [Internal <$address_type:camel Address >]::random(network_id.into()).into()
            }

            uniffi::custom_type!([< Internal $address_type:camel Address >], String);

             /// UniFFI conversion for RET types which are DisplayFromStr using String as builtin.
            impl crate::UniffiCustomTypeConverter for [< Internal $address_type:camel Address >] {
                type Builtin = String;

                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    val.parse::<Self>()
                    .map_err(|_| {
                        CommonError::FailedToDecodeAddressFromBech32 { bad_value: val }.into()
                    })
                }

                fn from_custom(obj: Self) -> Self::Builtin {
                    obj.to_string()
                }
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