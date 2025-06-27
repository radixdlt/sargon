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
        paste::paste! {
            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                enum_as_inner::EnumAsInner,
                derive_more::Display,
                derive_more::Debug,
                SerializeDisplay,
                DeserializeFromStr,
            )]
            pub enum $union_name {
                $(
                    $variant_name($variant_type),
                )+
            }

            #[cfg(test)]
            mod [< $union_name:snake _tests >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $union_name;

                #[test]
                fn equality() {
                    assert_eq!(SUT::sample(), SUT::sample());
                    assert_eq!(SUT::sample_other(), SUT::sample_other());
                }

                #[test]
                fn inequality() {
                    assert_ne!(SUT::sample(), SUT::sample_other());
                }

                #[test]
                fn network_id() {
                    assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
                    assert_eq!(SUT::sample_other().network_id(), NetworkID::Mainnet);
                }

                #[test]
                fn map_to_network() {
                    let mut to = NetworkID::Stokenet;
                    assert_eq!(
                        SUT::sample_mainnet().map_to_network(to).network_id(),
                        to
                    );
                    assert_eq!(
                        SUT::sample_stokenet_other().map_to_network(to).network_id(),
                        to
                    );
                    to = NetworkID::Mainnet;
                    assert_eq!(
                        SUT::sample_mainnet().map_to_network(to).network_id(),
                        to
                    );
                    assert_eq!(
                        SUT::sample_stokenet_other().map_to_network(to).network_id(),
                        to
                    );
                }

                #[test]
                fn string_roundtrip() {
                    let test = |a: SUT| {
                        let s = a.to_string();
                        assert_eq!(SUT::new_from_bech32(&s).unwrap(), a)
                    };
                    SUT::sample_values_all().into_iter().for_each(test);
                }

                #[test]
                fn format_address() {
                    let test = |a: SUT| {
                        assert_eq!(a.formatted(AddressFormat::Full), a.to_string());
                        assert_ne!(a.formatted(AddressFormat::Default), a.to_string());
                    };
                    SUT::sample_values_all().into_iter().for_each(test);
                }

                #[test]
                fn new_from_bech32_invalid_addr() {
                    assert!(SUT::new_from_bech32("super invalid address!!!")
                        .is_err());
                }


            }

            $(
                impl From<$variant_type> for $union_name {
                    fn from(value: $variant_type) -> Self {
                        Self::$variant_name(value)
                    }
                }
            )+

            $(
                impl TryFrom<$union_name> for $variant_type {
                    type Error = CommonError;
                    fn try_from(value: $union_name) -> Result<Self> {
                        match value {
                            $union_name::$variant_name(address) => Ok(address),
                            _ => Err(CommonError::FailedToMapAddressToSpecificType {
                                expected_specific_type: stringify!($variant_type).to_string(),
                                got_value: value.to_string(),
                            }),
                        }
                    }
                }
            )+

            impl IsAddress for $union_name {}
            impl IsNetworkAware for $union_name {

                /// Returns the [`NetworkID`]
                fn network_id(&self) -> NetworkID {
                    match self {
                        $(
                            Self::$variant_name(address) => address.network_id(),
                        )+
                    }
                }
            }

            impl HasNodeId for $union_name {
                fn node_id(&self) -> ScryptoNodeId {
                    match self {
                        $(
                            Self::$variant_name(address) => address.node_id(),
                        )+
                    }
                }
            }

            impl FromStr for $union_name {
                type Err = CommonError;

                fn from_str(s: &str) -> Result<Self> {
                    Self::new_from_bech32(s)
                }
            }

            impl IntoScryptoAddress for $union_name {
                fn scrypto(&self) -> ScryptoGlobalAddress {
                    match self {
                        $(
                            Self::$variant_name(address) => address.scrypto(),
                        )+
                    }
                }
            }

            impl $union_name {
                pub fn new_from_bech32(s: &str) -> Result<Self> {
                    let e = CommonError::FailedToDecodeAddressFromBech32 { bad_value: s.to_string() };
                    Err(e)
                    $(
                        .or($variant_type::from_str(s).map(Self::from))
                    )+
                }


                /// Returns a new address, with the same node_id, but using `network_id` as
                /// network.
                pub fn map_to_network(&self, network_id: NetworkID) -> Self {
                    match self {
                        $(
                            Self::$variant_name(address) => Self::from(address.map_to_network(network_id)),
                        )+
                    }
                }

                pub fn formatted(&self, format: AddressFormat) -> String {
                    match self {
                        $(
                            Self::$variant_name(address) => address.formatted(format),
                        )+
                    }
                }
            }

            impl HasSampleValues for $union_name {
                fn sample() -> Self {
                    Self::sample_mainnet()
                }

                fn sample_other() -> Self {
                    Self::sample_mainnet_other()
                }
            }

            #[allow(unused)]
            impl $union_name {
                $(
                    pub fn [< sample_ $variant_name:snake _mainnet >]() -> Self {
                        Self::from($variant_type::sample_mainnet())
                    }

                    pub fn [< sample_ $variant_name:snake _mainnet_other >]() -> Self {
                        Self::from($variant_type::sample_mainnet_other())
                    }

                    pub fn [< sample_ $variant_name:snake _stokenet >]() -> Self {
                        Self::from($variant_type::sample_stokenet())
                    }

                    pub fn [< sample_ $variant_name:snake _stokenet_other >]() -> Self {
                        Self::from($variant_type::sample_stokenet_other())
                    }
                )+

                pub fn sample_mainnet() -> Self {
                    Self::sample_values_mainnet().into_iter().next().unwrap()
                }

                pub fn sample_mainnet_other() -> Self {
                    Self::sample_values_mainnet().into_iter().rev().next().unwrap()
                }

                pub fn sample_stokenet() -> Self {
                    Self::sample_values_stokenet().into_iter().next().unwrap()
                }

                pub fn sample_stokenet_other() -> Self {
                    Self::sample_values_stokenet().into_iter().rev().next().unwrap()
                }

                pub fn sample_values_mainnet() -> Vec<Self> {
                    Self::sample_values_all().into_iter().filter(|x| x.network_id() == NetworkID::Mainnet).collect()
                }

                pub fn sample_values_stokenet() -> Vec<Self> {
                    Self::sample_values_all().into_iter().filter(|x| x.network_id() == NetworkID::Stokenet).collect()
                }

                pub fn sample_values_all() -> Vec<Self> {
                    vec![
                        $(
                            Self::[< sample_ $variant_name:snake _mainnet >](),
                            Self::[< sample_ $variant_name:snake _mainnet_other >](),
                            Self::[< sample_ $variant_name:snake _stokenet >](),
                            Self::[< sample_ $variant_name:snake _stokenet_other >](),
                        )+
                    ]
                }
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
        paste::paste! {
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
