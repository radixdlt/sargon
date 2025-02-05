use crate::prelude::*;
use hash::hash_of;

pub trait IsAddress:
    IsNetworkAware
    + HasNodeId
    + Serialize
    + for<'a> Deserialize<'a>
    + std::fmt::Display
    + FromStr
{
}

pub trait HasNodeId {
    fn node_id(&self) -> ScryptoNodeId;

    fn matches_public_key(&self, public_key: impl Into<PublicKey>) -> bool {
        let mut calculated_node_id: [u8; ScryptoNodeId::LENGTH] =
            hash_of(public_key.into().to_bytes()).0.lower_bytes();
        let node_id = self.node_id().0;
        calculated_node_id[0] = node_id[0]; // dummy
        calculated_node_id == node_id
    }
}
pub trait AddressFromNodeId: Sized {
    fn new_from_node_id(
        node_id: impl Into<ScryptoNodeId>,
        network_id: NetworkID,
    ) -> Result<Self>;
}

pub trait IntoScryptoAddress: IsNetworkAware {
    fn scrypto(&self) -> ScryptoGlobalAddress;
}

macro_rules! decl_address {
    (
        $(
            #[doc = $expr: expr]
        )*
        $address_type: ident => [$($entity_type: expr),* $(,)?]
    ) => {
        paste::paste! {
            use std::str::FromStr;
            use radix_common::prelude::{AddressBech32Decoder, AddressBech32Encoder, fmt, NetworkDefinition};
            use radix_engine_toolkit::prelude::NetworkDefinitionExt;


            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                derive_more::Debug,
                SerializeDisplay,
                DeserializeFromStr,
            )]
            #[debug("{}", self)]
            pub struct [< $address_type:camel Address >] {
                /// The NodeId of the address.
                node_id: ScryptoNodeId,
                /// The network that the address is to be used for. This is
                /// used in the Bech32m encoding and decoding of the address
                /// essentially providing us with the network context.
                network_id: NetworkID,
                /// The entity type of the address. This is checked in the
                /// constructor that it is one of the allowed entity types
                /// for this particular address type and then cached here
                /// to avoid any additional unwraps when being retrieved.
                entity_type: ScryptoEntityType
            }

            impl AddressFromNodeId for [< $address_type:camel Address >] {
                fn new_from_node_id(
                    node_id: impl Into<ScryptoNodeId>,
                    network_id: NetworkID
                ) -> Result<Self> {
                    let node_id = node_id.into();
                    if let Some(entity_type) = node_id.entity_type() {
                        if matches!(
                            entity_type,
                            $($entity_type)|*
                        ) {
                            Ok(Self {
                                node_id,
                                network_id,
                                entity_type
                            })
                        } else {
                            Err(CommonError::AddressInvalidEntityType { address_kind: stringify!([< $address_type:camel >]).to_string(), entity_type: entity_type as u8, node_id_as_hex: node_id.to_hex() })
                        }
                    } else {
                        Err(CommonError::AddressNodeIdNoEntityType { node_id_as_hex: node_id.to_hex() })
                    }
                }
            }

            impl Display for [< $address_type:camel Address >] {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    let encoder = AddressBech32Encoder::new(
                        &NetworkDefinition::from_network_id(self.network_id.discriminant()),
                    );
                    encoder
                        .encode_to_fmt(formatter, &self.node_id.0)
                        .map_err(|_| fmt::Error)
                }
            }

            impl IsNetworkAware for [< $address_type:camel Address >] {
                fn network_id(&self) -> NetworkID {
                    self.network_id
                }
            }

            impl HasNodeId for [< $address_type:camel Address >] {
                fn node_id(&self) -> ScryptoNodeId {
                    self.node_id
                }
            }

            impl Identifiable for [< $address_type:camel Address >] {
                type ID = Self;

                fn id(&self) -> Self::ID {
                    *self
                }
            }

            impl IsAddress for [< $address_type:camel Address >] {}

            impl FromStr for [< $address_type:camel Address >] {
                type Err = CommonError;
                fn from_str(s: &str) -> Result<Self> {
                    Self::try_from_bech32(s)
                }
            }

            impl From<(ScryptoGlobalAddress, NetworkID)> for [< $address_type:camel Address >] {
                fn from(value: (ScryptoGlobalAddress, NetworkID)) -> Self {
                    Self::new_from_global_address(value.0, value.1)
                    .expect("Should always be able to convert from ScryptoGlobalAddress to Sargon Address")
                }
            }

            impl TryFrom<(ScryptoManifestGlobalAddress, NetworkID)> for [< $address_type:camel Address >] {
                type Error = CommonError;
                fn try_from(value: (ScryptoManifestGlobalAddress, NetworkID)) -> Result<Self> {
                    match value.0 {
                        ScryptoManifestGlobalAddress::Static(node_id) => {
                            Self::new_from_node_id(node_id, value.1)
                        },
                        _ => Err(CommonError::NamedAddressesAreNotSupported),
                    }
                }
            }

            impl TryFrom<(ScryptoManifestAddress, NetworkID)> for [< $address_type:camel Address >] {
                type Error = CommonError;

                fn try_from(value: (ScryptoManifestAddress, NetworkID)) -> Result<Self> {
                    let (address, network_id) = value;

                    match address {
                        ScryptoManifestAddress::Static(node_id) => {
                            Self::new_from_node_id(node_id, network_id)
                        },
                        _ => Err(CommonError::NamedAddressesAreNotSupported),
                    }
                }
            }

            #[cfg(test)]
            impl From<&str> for [< $address_type:camel Address >] {
                /// TEST ONLY
                fn from(value: &str) -> Self {
                    value.parse::<Self>().expect(&format!("Test failed since the passed in str is not a valid address: '{}'", value))
                }
            }

            impl From<[< $address_type:camel Address >]> for ScryptoGlobalAddress {
                fn from(value: [< $address_type:camel Address >]) -> ScryptoGlobalAddress {
                    value.scrypto()
                }
            }

            impl From<[< $address_type:camel Address >]> for ScryptoManifestValue {
                fn from(value: [< $address_type:camel Address >]) -> ScryptoManifestValue {
                    ScryptoManifestValue::Custom {
                        value: ScryptoManifestCustomValue::Address(
                            ScryptoManifestAddress::Static(
                                value.node_id(),
                            ),
                        ),
                    }
                }
            }

            use std::cmp::Ordering;
            impl Ord for [< $address_type:camel Address >] {
                fn cmp(&self, other: &Self) -> Ordering {
                    self.address().cmp(&other.address())
                }
            }

            impl PartialOrd for [< $address_type:camel Address >] {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }

            use bytes::prelude::generate_byte_array;
            use core_utils::prelude::format_string;

            impl [< $address_type:camel Address >] {
                pub fn new_from_global_address(global_address: ScryptoGlobalAddress, network_id: NetworkID) -> Result<Self> {
                    Self::new_from_node_id(global_address.into_node_id(), network_id)
                }

                pub fn entity_type(&self) -> ScryptoEntityType {
                    self.entity_type
                }

                pub fn random(network_id: NetworkID) -> Self {
                    Self::with_node_id_bytes(&generate_byte_array::<{ ScryptoNodeId::RID_LENGTH }>(), network_id)
                }

                  pub fn with_node_id_of<A: IsNetworkAware + HasNodeId>(
                    address: &A,
                ) -> Self {
                    Self::with_node_id(address.node_id(), address.network_id())
                }

                pub fn with_node_id(
                    node_id: ScryptoNodeId,
                    network_id: NetworkID
                ) -> Self {
                    Self::with_node_id_bytes(node_id.0.as_slice()[1..].try_into().unwrap(), network_id)
                }

                pub fn with_node_id_bytes(
                    node_id_bytes: &[u8; { ScryptoNodeId::RID_LENGTH }],
                    network_id: NetworkID
                ) -> Self {
                    let entity_byte = Self::sample().node_id().as_bytes()[0];
                    let node_id = ScryptoNodeId::new(entity_byte, node_id_bytes);
                    Self::new_from_node_id(node_id, network_id).unwrap()
                }

                pub fn formatted(&self, format: AddressFormat) -> String {
                    match format {
                        AddressFormat::Default => format_string(self.address(), 4, 6),
                        AddressFormat::Full | AddressFormat::Raw => self.address(),
                    }
                }

                pub fn scrypto(&self) -> ScryptoGlobalAddress {
                    ScryptoGlobalAddress::try_from(self.node_id())
                    .expect("Should always be able to convert a Sargon Address into radix engine 'GlobalAddress'.")
                }

                /// Returns a new address, with the same node_id, but using `network_id` as
                /// network.
                pub fn map_to_network(&self, network_id: NetworkID) -> Self {
                    if network_id == self.network_id() {
                        return *self
                    }
                    Self::new_from_node_id(self.node_id(), network_id).expect("Should always be able to map an address to a different network.")
                }

                pub fn address(&self) -> String {
                    self.to_string()
                }

                /// Attempts to construct this type from a Bech32m string.
                /// The network id does not need to be passed as it will be
                /// determined based on the HRP of the address
                pub fn try_from_bech32(
                    address_string: &str,
                ) -> Result<Self> {
                    // Find the network definition based on the network of
                    // the passed address.
                    let network_definition = NetworkDefinition::from_address_string(address_string)
                        .ok_or(
                            CommonError::FailedToFindNetworkIdFromBech32mString {
                                bech32m_encoded_address: address_string.to_owned(),
                            },
                        )?;

                    // Construct the decoder and decode the address
                    let decoder = AddressBech32Decoder::new(&network_definition);
                    let (_, data) = decoder.validate_and_decode(address_string).unwrap();

                    // Construct a NodeId from the returned data.
                    let node_id = data.try_into().map(ScryptoNodeId)
                        .map_err(|vec| {
                        CommonError::InvalidNodeIdLength {
                            expected: ScryptoNodeId::LENGTH,
                            actual: vec.len(),
                        }
                    })?;

                    Self::new_from_node_id(node_id, network_definition.id.try_into()?)
                }
            }

            #[cfg(test)]
            mod [<tests_of_ $address_type:snake>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = [< $address_type:camel Address >];

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

                #[test]
                fn can_construct_address_for_valid_entity_type() {
                    for entity_type in [$($entity_type),*] {
                        let node_id = ScryptoNodeId::new(entity_type as u8, &[0xff; 29]);
                        let address = SUT::new_from_node_id(node_id, NetworkID::Mainnet);
                        assert!(address.is_ok());
                    }
                }
            }

            impl IntoScryptoAddress for [< $address_type:camel Address >] {
                fn scrypto(&self) -> ScryptoGlobalAddress {
                    ScryptoGlobalAddress::try_from(self.node_id())
                    .expect("Should always be able to convert a Sargon Address into radix engine 'GlobalAddress'.")
                }
            }

            impl TryInto<ScryptoDynamicGlobalAddress> for &[< $address_type:camel Address >] {
                type Error = crate::CommonError;

                fn try_into(
                    self,
                ) -> Result<ScryptoDynamicGlobalAddress, Self::Error> {
                    Ok(ScryptoDynamicGlobalAddress::Static(self.scrypto()))
                }
            }
        }
    };
}

pub(crate) use decl_address;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_entity_type() {
        let node_id = ScryptoNodeId::new(0x01, &[0xff; 29]);
        assert_eq!(
            AccountAddress::new_from_node_id(node_id, NetworkID::Mainnet),
            Err(CommonError::AddressNodeIdNoEntityType {
                node_id_as_hex: node_id.to_hex()
            })
        );
    }

    #[test]
    fn invalid_entity_type() {
        let input = "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw";
        assert_eq!(
            AccountAddress::from_str(input),
            Err(CommonError::AddressInvalidEntityType { address_kind: "Account".to_string(), entity_type: ScryptoEntityType::GlobalPreallocatedEd25519Identity as u8, node_id_as_hex: "52dda1acb8afe9a18b413eb598d17b10694d4c9b36c323dfa84e5c03aad0".to_string() })
        );
    }

    #[test]
    fn invalid_string() {
        assert_eq!(
            AccountAddress::try_from_bech32("x"),
            Err(CommonError::FailedToFindNetworkIdFromBech32mString {
                bech32m_encoded_address: "x".to_string()
            })
        )
    }

    #[test]
    fn invalid_checksum() {
        let s = "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdxx";
        assert_eq!(
            AccountAddress::try_from_bech32(s),
            Err(CommonError::FailedToFindNetworkIdFromBech32mString { bech32m_encoded_address: "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdxx".to_string() })
        )
    }

    #[test]
    fn from_str() {
        assert!(
            "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j"
                .parse::<IdentityAddress>()
                .is_ok()
        );
        assert!("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr".parse::<AccountAddress>().is_ok());
        assert!("internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d".parse::<VaultAddress>().is_ok());
        assert!(
            "locker_rdx1dqeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jxgeryv3jjs0l6p"
                .parse::<LockerAddress>()
                .is_ok()
        );
        assert!("validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0".parse::<ValidatorAddress>().is_ok());
        assert!("accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt".parse::<AccessControllerAddress>().is_ok());
        assert!("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet".parse::<ComponentAddress>().is_ok());
        assert!("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd".parse::<ResourceAddress>().is_ok());
        assert!(
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
                .parse::<PoolAddress>()
                .is_ok()
        );
        assert!("package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet".parse::<PackageAddress>().is_ok());
    }
}
