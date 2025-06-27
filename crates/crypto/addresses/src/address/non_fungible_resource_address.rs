use crate::prelude::*;

macro_rules! decl_specialized_address {
    (
        $(
            #[doc = $expr: expr]
        )*
        $specialized_address_type: ident,
        $base_addr: ident,
        $validate: ident,
        $validation_err: ident
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
                Ord,
                PartialOrd,
                derive_more::Display,
                derive_more::Debug,
                derive_more::FromStr,
                SerializeDisplay,
                DeserializeFromStr,
            )]
            #[debug("{:?}", self.0)]
            pub struct $specialized_address_type(pub $base_addr);

            impl $specialized_address_type {
                pub fn new(address: $base_addr) -> Result<Self> {
                    if <$base_addr>::$validate(&address) {
                        Ok(Self(address))
                    } else {
                        Err(CommonError::$validation_err)
                    }
                }

                /// Returns a new address, with the same node_id, but using `network_id` as
                /// network.
                pub fn map_to_network(&self, network_id: NetworkID) -> Self {
                    self.0.map_to_network(network_id).try_into().expect("Should always be able to map an address to a different network.")
                }

                pub fn new_from_bech32(bech32: String) -> Result<Self> {
                    $base_addr::try_from_bech32(&bech32).and_then(TryInto::try_into)
                }

                pub fn bech32_address(&self) -> String {
                    self.0.to_string()
                }
            }

            impl AddressFromNodeId for $specialized_address_type {
                fn new_from_node_id(
                    node_id: impl Into<ScryptoNodeId>,
                    network_id: NetworkID
                ) -> Result<Self> {
                    $base_addr::new_from_node_id(node_id, network_id).and_then(Self::new)
                }
            }

            impl IsNetworkAware for $specialized_address_type {
                fn network_id(&self) -> NetworkID {
                    self.0.network_id()
                }
            }

            impl HasNodeId for $specialized_address_type {
                fn node_id(&self) -> ScryptoNodeId {
                    self.0.node_id()
                }
            }

            impl TryFrom<$base_addr> for $specialized_address_type {
                type Error = CommonError;

                fn try_from(value: $base_addr) -> Result<Self> {
                    $specialized_address_type::new(value)
                }
            }

            impl std::ops::Deref for $specialized_address_type {
                type Target = $base_addr;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl From<$specialized_address_type> for $base_addr {
                fn from(value: $specialized_address_type) -> Self {
                    value.0
                }
            }

            #[cfg(test)]
            impl From<&str> for $specialized_address_type {
                /// TEST ONLY
                fn from(value: &str) -> Self {
                    value.parse().expect(&format!("Test failed since the passed in str is not a valid address: '{}'", value))
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
        Self::new_from_node_id(node_id, network_id).unwrap()
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
            SUT::sample_mainnet().0,
            ResourceAddress::sample_mainnet_nft_gc_membership()
        );
    }

    #[test]
    fn debug() {
        let s = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa";
        let a = s.parse::<SUT>().unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

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
