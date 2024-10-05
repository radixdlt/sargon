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
            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                derive_more::Display,
                derive_more::Debug,
                uniffi::Enum,
            )]
            pub enum $union_name {
                $(
                    $variant_name($variant_type),
                )+
            }

            #[uniffi::export]
            pub fn [< new_ $union_name:snake _from_bech32 >](
                string: String,
            ) -> Result<$union_name> {
                $union_name::new_from_bech32(&string)
            }

            #[uniffi::export]
            pub fn [< $union_name:snake _to_string >](
                address: &$union_name,
            ) -> String {
                address.to_string()
            }

            #[uniffi::export]
            pub fn [< $union_name:snake _formatted>](address: &$union_name, format: AddressFormat) -> String {
                address.formatted(format)
            }

            #[uniffi::export]
            pub fn [< $union_name:snake _network_id >](
                address: &$union_name,
            ) -> NetworkID {
                address.network_id()
            }

            #[uniffi::export]
            pub fn [< $union_name:snake _map_to_network>](
                address: &$union_name,
                network_id: NetworkID,
            ) -> $union_name {
                address.map_to_network(network_id)
            }


            #[uniffi::export]
            pub fn [< $union_name:snake _sample_values_all>]() -> Vec<$union_name> {
                $union_name::sample_values_all()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $union_name:snake _sample_mainnet >]() -> $union_name {
                $union_name::sample_mainnet()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $union_name:snake _sample_mainnet_other >]() -> $union_name {
                $union_name::sample_mainnet_other()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $union_name:snake _sample_stokenet >]() -> $union_name {
                $union_name::sample_stokenet()
            }

            #[uniffi::export]
            pub(crate) fn [< new_ $union_name:snake _sample_stokenet_other >]() -> $union_name {
                $union_name::sample_stokenet_other()
            }

            $(
                #[uniffi::export]
                pub(crate) fn [< new_ $union_name:snake _sample_ $variant_name:snake _mainnet >]() -> $union_name {
                    $union_name::[<sample_ $variant_name:snake _mainnet >]()
                }

                #[uniffi::export]
                pub(crate) fn [< new_ $union_name:snake _sample_ $variant_name:snake _mainnet_other >]() -> $union_name {
                    $union_name::[<sample_ $variant_name:snake _mainnet_other >]()
                }

                #[uniffi::export]
                pub(crate) fn [< new_ $union_name:snake _sample_ $variant_name:snake _stokenet >]() -> $union_name {
                    $union_name::[<sample_ $variant_name:snake _stokenet >]()
                }

                #[uniffi::export]
                pub(crate) fn [< new_ $union_name:snake _sample_ $variant_name:snake _stokenet_other >]() -> $union_name {
                    $union_name::[<sample_ $variant_name:snake _stokenet_other >]()
                }
            )+


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
                fn into_scrypto_global_address() {
                    // this is quite a bad unit tests... but I could not come up with anything better.
                    assert_eq!(
                        SUT::sample().scrypto().to_vec().len(), 30
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

            #[cfg(test)]
            mod [< uniffi_ $union_name:snake _tests >] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $union_name;

                #[test]
                fn hash_of_samples() {
                    assert_eq!(
                        HashSet::<SUT>::from_iter([
                            $(
                                [< new_ $union_name:snake _sample_ $variant_name:snake _mainnet >](),
                                [< new_ $union_name:snake _sample_ $variant_name:snake _mainnet_other >](),
                                [< new_ $union_name:snake _sample_ $variant_name:snake _stokenet >](),
                                [< new_ $union_name:snake _sample_ $variant_name:snake _stokenet_other >](),
                            )+
                        ]),
                        HashSet::<SUT>::from_iter(
                            [< $union_name:snake _sample_values_all>]()
                        )
                    );

                    assert_eq!(
                        HashSet::<SUT>::from_iter([
                            [< new_ $union_name:snake _sample_mainnet >](),
                            [< new_ $union_name:snake _sample_mainnet_other >](),
                            [< new_ $union_name:snake _sample_stokenet >](),
                            [< new_ $union_name:snake _sample_stokenet_other >](),
                            // duplicates should be removed
                            [< new_ $union_name:snake _sample_mainnet >](),
                            [< new_ $union_name:snake _sample_mainnet_other >](),
                            [< new_ $union_name:snake _sample_stokenet >](),
                            [< new_ $union_name:snake _sample_stokenet_other >](),
                        ]).len(),
                        4
                    );

                    let mut samples = SUT::sample_values_mainnet();
                    samples.extend(SUT::sample_values_stokenet());
                    assert_eq!(
                        HashSet::<SUT>::from_iter(SUT::sample_values_all()),
                        HashSet::<SUT>::from_iter(samples)
                    );
                }

                #[test]
                fn bech32_roundtrip() {
                    $(
                        let sut = [< new_ $union_name:snake _sample_ $variant_name:snake _mainnet >]();
                        let s = [< $union_name:snake _to_string >](&sut);
                        assert_eq!(
                            [< new_ $union_name:snake _from_bech32 >](s).unwrap(),
                            sut
                        );
                    )+
                }

                #[test]
                fn network_id() {
                    $(
                        assert_eq!(
                            [< $union_name:snake _network_id >](&[< new_ $union_name:snake _sample_ $variant_name:snake _mainnet_other >]()),
                            NetworkID::Mainnet
                        );
                    )+

                    $(
                        assert_eq!(
                            [< $union_name:snake _network_id >](&[< new_ $union_name:snake _sample_ $variant_name:snake _stokenet_other >]()),
                            NetworkID::Stokenet
                        );
                    )+
                }

                #[test]
                fn map_to_network() {

                    SUT::sample_values_all().into_iter().for_each(|a| {
                        assert_eq!(
                            [< $union_name:snake _map_to_network>](&a, NetworkID::Stokenet).network_id(),
                            NetworkID::Stokenet
                        );
                    });

                    SUT::sample_values_all().into_iter().for_each(|a| {
                        assert_eq!(
                            [< $union_name:snake _map_to_network>](&a, NetworkID::Mainnet).network_id(),
                            NetworkID::Mainnet
                        );
                    });
                }

                #[test]
                fn format_address() {
                    let test = |a: SUT| {
                        assert_eq!([< $union_name:snake _formatted>](&a, AddressFormat::Full), a.to_string());
                        assert_ne!([< $union_name:snake _formatted>](&a, AddressFormat::Default), a.to_string());
                    };
                    SUT::sample_values_all().into_iter().for_each(test);
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
                    pub(crate) fn [< sample_ $variant_name:snake _mainnet >]() -> Self {
                        Self::from($variant_type::sample_mainnet())
                    }

                    pub(crate) fn [< sample_ $variant_name:snake _mainnet_other >]() -> Self {
                        Self::from($variant_type::sample_mainnet_other())
                    }

                    pub(crate) fn [< sample_ $variant_name:snake _stokenet >]() -> Self {
                        Self::from($variant_type::sample_stokenet())
                    }

                    pub(crate) fn [< sample_ $variant_name:snake _stokenet_other >]() -> Self {
                        Self::from($variant_type::sample_stokenet_other())
                    }
                )+

                pub(crate) fn sample_mainnet() -> Self {
                    Self::sample_values_mainnet().into_iter().next().unwrap()
                }

                pub(crate) fn sample_mainnet_other() -> Self {
                    Self::sample_values_mainnet().into_iter().rev().next().unwrap()
                }

                pub(crate) fn sample_stokenet() -> Self {
                    Self::sample_values_stokenet().into_iter().next().unwrap()
                }

                pub(crate) fn sample_stokenet_other() -> Self {
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
