use crate::prelude::*;

#[macro_export]
macro_rules! address_union {
    (
        $(
            #[doc = $expr: expr]
        )*
        $union_name: ident,
        $(
            $variant_name: ident,
            $variant_type: ty
        )+
    ) => {
        paste! {
            use sargon::IsNetworkAware;
            use sargon::$union_name as [< Internal $union_name >];

            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,

                PartialEq,
                Eq,
                Hash,
                InternalConversion,
                uniffi::Enum,
            )]
            pub enum $union_name {
                $(
                    $variant_name($variant_type),
                )+
            }

            impl From<[< Internal $union_name >]> for $union_name {
                fn from(value: [< Internal $union_name >]) -> Self {
                    match value {
                        $(
                            [< Internal $union_name >]::$variant_name(address) => Self::$variant_name(address.into()),
                        )+
                    }
                }
            }

            impl Into<[< Internal $union_name >]> for $union_name {
                fn into(self) -> [< Internal $union_name >] {
                    match self {
                        $(
                            Self::$variant_name(address) => [< Internal $union_name >]::$variant_name(address.into()),
                        )+
                    }
                }
            }

            #[uniffi::export]
            pub fn [< new_ $union_name:snake _from_bech32 >](
                string: String,
            ) -> Result<$union_name> {
                [< Internal $union_name >]::new_from_bech32(&string).map_result()
            }

            #[uniffi::export]
            pub fn [< $union_name:snake _to_string >](
                address: &$union_name,
            ) -> String {
                address.into_internal().to_string()
            }

            #[uniffi::export]
            pub fn [< $union_name:snake _formatted>](address: &$union_name, format: AddressFormat) -> String {
                address.into_internal().formatted(format.into())
            }

            #[uniffi::export]
            pub fn [< $union_name:snake _network_id >](
                address: &$union_name,
            ) -> NetworkID {
                address.into_internal().network_id().into()
            }

            #[uniffi::export]
            pub fn [< $union_name:snake _map_to_network>](
                address: &$union_name,
                network_id: NetworkID,
            ) -> $union_name {
                address.into_internal().map_to_network(network_id.into()).into()
            }


            #[uniffi::export]
            pub fn [< $union_name:snake _sample_values_all>]() -> Vec<$union_name> {
                [< Internal $union_name >]::sample_values_all().into_iter().map(Into::into).collect()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $union_name:snake _sample_mainnet >]() -> $union_name {
                [< Internal $union_name >]::sample_mainnet().into()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $union_name:snake _sample_mainnet_other >]() -> $union_name {
                [< Internal $union_name >]::sample_mainnet_other().into()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $union_name:snake _sample_stokenet >]() -> $union_name {
                [< Internal $union_name >]::sample_stokenet().into()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $union_name:snake _sample_stokenet_other >]() -> $union_name {
                [< Internal $union_name >]::sample_stokenet_other().into()
            }
        }
    };
    (
        $(
            #[doc = $expr: expr]
        )*
        enum $union_name: ident:
        $(
            $address_type: ident
        ),+
    ) => {
        paste! {
            address_union!(
                $(
                    #[doc = $expr]
                )*
                $union_name,
                $(
                    [< $address_type: camel >],
                    [< $address_type: camel Address >]
                )+
            );
        }
    };
}
