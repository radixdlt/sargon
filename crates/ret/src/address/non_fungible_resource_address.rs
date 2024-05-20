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
            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                Ord,
                PartialOrd,
                derive_more::Display,
                derive_more::Debug,
                derive_more::FromStr,
                SerializeDisplay,
                DeserializeFromStr,
                uniffi::Record,
            )]
            #[debug("{:?}", self.secret_magic)]
            pub struct $specialized_address_type {
                secret_magic: $base_addr
            }

            /// Tries to bech32 decode the string into a specialized address.
            #[uniffi::export]
            pub fn [< new_ $specialized_address_type:snake >](bech32: String) -> Result<$specialized_address_type> {
                $base_addr::try_from_bech32(&bech32).and_then(TryInto::<$specialized_address_type>::try_into)
            }

            /// Returns the base address of this specialized address.
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _as_ $base_addr:snake>](address: &$specialized_address_type) -> $base_addr {
                address.secret_magic
            }

            /// Returns a new address, with the same node_id, but using `network_id` as
            /// network.
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _map_to_network >](address: &$specialized_address_type, network_id: NetworkID) -> $specialized_address_type {
                address.map_to_network(network_id)
            }

            /// Returns the bech32 encoding of this address
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _bech32_address >](address: &$specialized_address_type) -> String {
                address.to_string()
            }

            /// Returns the network id this address
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _network_id >](address: &$specialized_address_type) -> NetworkID {
                address.secret_magic.network_id()
            }

            impl $specialized_address_type {
                pub fn new(address: $base_addr) -> Result<Self> {
                    if <$base_addr>::$validate(&address) {
                        Ok(Self {
                            secret_magic: address
                        })
                    } else {
                        Err(CommonError::$validation_err)
                    }
                }

                /// Returns a new address, with the same node_id, but using `network_id` as
                /// network.
                pub fn map_to_network(&self, network_id: NetworkID) -> Self {
                    <Self as AddressViaRet>::new(self.node_id(), network_id).expect("Should always be able to map an address to a different network.")
                }

            }

            impl TryFrom<$base_addr> for $specialized_address_type {
                type Error = CommonError;

                fn try_from(value: $base_addr) -> Result<Self> {
                    $specialized_address_type::new(value)
                }
            }

            impl Deref for $specialized_address_type {
                type Target = $base_addr;

                fn deref(&self) -> &Self::Target {
                    &self.secret_magic
                }
            }

            impl From<$specialized_address_type> for $base_addr {
                fn from(value: $specialized_address_type) -> Self {
                    value.secret_magic
                }
            }

            #[cfg(test)]
            impl From<&str> for $specialized_address_type {
                /// TEST ONLY
                fn from(value: &str) -> Self {
                    value.parse().expect(&format!("Test failed since the passed in str is not a valid address: '{}'", value))
                }
            }

            impl AddressViaRet for $specialized_address_type {
                fn new(
                    node_id: impl Into<ScryptoNodeId>,
                    network_id: NetworkID,
                ) -> Result<Self, CommonError> {
                    <$base_addr as AddressViaRet>::new(node_id, network_id).and_then(Self::new)
                }
            }

            #[cfg(test)]
            mod [<uniffi_tests_of_ $specialized_address_type:snake>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $specialized_address_type;

                #[test]
                fn map_to_network() {
                    let sut = SUT::sample();
                    assert_eq!([< $specialized_address_type:snake _map_to_network >](&sut, sut.network_id()), sut); // unchanged
                }
            }

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

impl HasSampleValues for NonFungibleResourceAddress {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

impl NonFungibleResourceAddress {
    pub fn random(network_id: NetworkID) -> Self {
        let entity_byte = Self::sample().node_id().as_bytes()[0];
        let node_id = ScryptoNodeId::new(
            entity_byte,
            &generate_byte_array::<{ ScryptoNodeId::RID_LENGTH }>(),
        );
        let ret_address =
            RetResourceAddress::new(node_id, network_id.discriminant())
                .unwrap();
        Self::from_str(&ret_address.to_string()).unwrap()
    }

    pub fn sample_mainnet() -> Self {
        ResourceAddress::sample_mainnet_nft_gc_membership()
            .try_into()
            .expect("Valid sample")
    }

    pub fn sample_mainnet_other() -> Self {
        ResourceAddress::sample_mainnet_nft_other()
            .try_into()
            .expect("Valid sample")
    }

    pub fn sample_stokenet() -> Self {
        ResourceAddress::sample_stokenet_nft_gc_membership()
            .try_into()
            .expect("Valid sample")
    }

    pub fn sample_stokenet_other() -> Self {
        ResourceAddress::sample_stokenet_nft_other()
            .try_into()
            .expect("Valid sample")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleResourceAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample_mainnet(), SUT::sample_mainnet());
        assert_eq!(SUT::sample_mainnet_other(), SUT::sample_mainnet_other());
        assert_eq!(SUT::sample_stokenet(), SUT::sample_stokenet());
        assert_eq!(SUT::sample_stokenet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::sample_mainnet(), SUT::sample_stokenet());
        assert_ne!(SUT::sample_mainnet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn map_to_network() {
        let to = NetworkID::Stokenet;
        assert_eq!(
            SUT::sample_mainnet().map_to_network(to),
            SUT::try_from(
                ResourceAddress::sample_mainnet_nft_gc_membership()
                    .map_to_network(to)
            )
            .unwrap()
        );
    }

    #[test]
    fn ord() {
        assert!(SUT::sample_mainnet_other() < SUT::sample_mainnet());
        assert!(SUT::sample_stokenet_other() > SUT::sample_stokenet());
    }

    #[test]
    fn display() {
        let s = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa";
        let a = s.parse::<SUT>().unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn as_base() {
        assert_eq!(
            non_fungible_resource_address_as_resource_address(
                &SUT::sample_mainnet()
            ),
            ResourceAddress::sample_mainnet_nft_gc_membership()
        );
    }

    #[test]
    fn debug() {
        let s = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa";
        let a = s.parse::<SUT>().unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

    // #[test]
    // fn manual_perform_uniffi_conversion() {
    //     type RetAddr = <ResourceAddress as FromRetAddress>::RetAddress;
    //     let sut = SUT::sample();
    //     let bech32 = sut.to_string();
    //     let ret = RetAddr::try_from_bech32(&bech32).unwrap();

    //     let ffi_side =
    //         <RetAddr as crate::UniffiCustomTypeConverter>::from_custom(ret);
    //     assert_eq!(ffi_side, bech32);
    //     let from_ffi_side =
    //         <RetAddr as crate::UniffiCustomTypeConverter>::into_custom(
    //             ffi_side,
    //         )
    //         .unwrap();
    //     assert_eq!(ret, from_ffi_side);
    // }

    #[test]
    fn json_roundtrip() {
        let a = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &a,
            json!("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd"),
        );
    }

    #[test]
    fn deref() {
        assert_eq!(*SUT::sample(), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa".parse::<ResourceAddress>().unwrap());
    }

    #[test]
    fn into_resource_address() {
        assert_eq!(ResourceAddress::from(SUT::sample()), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa".parse::<ResourceAddress>().unwrap());
    }

    #[test]
    fn new_ok() {
        assert!(
            SUT::new(ResourceAddress::sample_mainnet_nft_gc_membership())
                .is_ok()
        );
    }

    #[test]
    fn try_from_err() {
        assert_eq!(SUT::try_from(ResourceAddress::sample_mainnet_xrd()), Err(CommonError::FungibleResourceAddressNotAcceptedInNonFungibleContext));
    }

    #[test]
    fn random_address_bech32_roundtrip() {
        for network_id in NetworkID::all() {
            let sut = SUT::random(network_id);
            // Bech32 roundtrip ensures the correct [Scrypto]`EntityType`
            // is used across, and thus correct Bech32 HRP.
            assert_eq!(SUT::from_str(&sut.to_string()).unwrap(), sut);
        }
    }

    #[test]
    fn random_address_is_random() {
        let n = 100;

        for network_id in NetworkID::all() {
            let addresses = (0..n)
                .map(|_| SUT::random(network_id))
                .collect::<HashSet<SUT>>();
            assert_eq!(addresses.len(), n);
        }
    }
}
