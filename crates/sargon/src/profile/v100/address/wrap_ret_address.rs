use crate::prelude::*;

pub trait AddressViaRet: Sized {
    fn new(
        node_id: impl Into<ScryptoNodeId>,
        network_id: NetworkID,
    ) -> Result<Self>;
}

pub trait IsAddress:
    IsNetworkAware
    + Serialize
    + for<'a> Deserialize<'a>
    + std::fmt::Display
    + FromStr
{
}

/// Helps with unit testing, so that we do not need to explicitly specify each
/// (Sargon) Address types corresponding RET address type, but can use, e.g.
/// `AccountAddress::RetAddress`
/// instead of `radix_engine_toolkit::models::canonical_address_types::CanonicalAccountAddress`
#[allow(dead_code)]
pub(crate) trait FromRetAddress {
    type RetAddress;
}

pub(crate) fn format_string(
    s: impl AsRef<str>,
    start: usize,
    end: usize,
) -> String {
    let s = s.as_ref();
    let prefix = &s[0..start];
    let suffix = suffix_str(end, s);
    format!("{}...{}", prefix, suffix)
}

pub trait IntoScryptoAddress: IsNetworkAware {
    fn scrypto(&self) -> ScryptoGlobalAddress;
}

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
            #[derive(
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                derive_more::Display,
                derive_more::Debug,
                SerializeDisplay,
                DeserializeFromStr,
            )]
            #[display("{}", self.0)]
            #[debug("{}", self.0)]
            pub struct [< $address_type:camel Address >](pub(crate) [< Ret $address_type:camel Address >]);

            impl From<[< Ret $address_type:camel Address >]> for [< $address_type:camel Address >] {
                fn from(value: [< Ret $address_type:camel Address >]) -> Self {
                    Self(value)
                }
            }

            impl FromStr for [< $address_type:camel Address >] {
                type Err = CommonError;
                fn from_str(s: &str) -> Result<Self> {
                    Self::try_from_bech32(s)
                }
            }

            #[cfg(test)]
            impl From<&str> for [< $address_type:camel Address >] {
                /// TEST ONLY
                fn from(value: &str) -> Self {
                    value.parse().expect(&format!("Test failed since the passed in str is not a valid address: '{}'", value))
                }
            }

            impl IsAddress for [< $address_type:camel Address >] {

            }

            impl FromRetAddress for [< $address_type:camel Address >] {
                type RetAddress = [< Ret $address_type:camel Address >];
            }

            impl From<[< $address_type:camel Address >]> for ScryptoGlobalAddress {
                fn from(value: [< $address_type:camel Address >]) -> ScryptoGlobalAddress {
                    value.scrypto()
                }
            }

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

            impl [< $address_type:camel Address >] {

                pub fn random(network_id: NetworkID) -> Self {
                    Self::with_node_id_bytes(&generate_byte_array::<{ ScryptoNodeId::RID_LENGTH }>(), network_id)
                }

                pub fn with_node_id_bytes(
                    node_id_bytes: &[u8; { ScryptoNodeId::RID_LENGTH }],
                    network_id: NetworkID
                ) -> Self {
                    let entity_byte = Self::sample().node_id().as_bytes()[0];
                    let node_id = ScryptoNodeId::new(entity_byte, node_id_bytes);
                    let ret_address = [<Ret $address_type:camel Address>]::new(node_id, network_id.discriminant()).unwrap();
                    Self::from(ret_address)
                }

                pub fn formatted(&self, format: AddressFormat) -> String {
                    match format {
                        AddressFormat::Default => format_string(self.address(), 4, 6),
                        AddressFormat::Full | AddressFormat::Raw => self.address(),
                    }
                }

                pub(crate) fn scrypto(&self) -> ScryptoGlobalAddress {
                    ScryptoGlobalAddress::try_from(self.node_id())
                    .expect("Should always be able to convert a Sargon Address into radix engine 'GlobalAddress'.")
                }

                pub(crate) fn node_id(&self) -> ScryptoNodeId {
                    self.0.node_id()
                }

                /// Returns a new address, with the same node_id, but using `network_id` as
                /// network.
                pub fn map_to_network(&self, network_id: NetworkID) -> Self {
                    if network_id == self.network_id() {
                        return *self
                    }
                    <Self as AddressViaRet>::new(self.node_id(), network_id).expect("Should always be able to map an address to a different network.")
                }

                pub fn address(&self) -> String {
                    self.to_string()
                }

                pub fn entity_type(&self) -> ScryptoEntityType {
                    self.0.entity_type()
                }

                pub fn try_from_bech32(bech32: impl AsRef<str>) -> Result<Self> {
                    bech32.as_ref().parse::<[< Ret $address_type:camel Address >]>()
                    .map_err(|e| {
                        error!("Failed Bech32 decode String, RET error: {:?}", e);
                        CommonError::FailedToDecodeAddressFromBech32 { bad_value: bech32.as_ref().to_owned() }
                    })
                    .map(Self::from)
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
            }

            impl IsNetworkAware for [< $address_type:camel Address >] {
                fn network_id(&self) -> NetworkID {
                    self.0.network_id().try_into().expect("Should have known all network ids")
                }
            }

            impl IntoScryptoAddress for [< $address_type:camel Address >] {
                fn scrypto(&self) -> ScryptoGlobalAddress {
                    ScryptoGlobalAddress::try_from(self.node_id())
                    .expect("Should always be able to convert a Sargon Address into radix engine 'GlobalAddress'.")
                }
            }

            impl AddressViaRet for [< $address_type:camel Address >] {
                fn new(
                    node_id: impl Into<ScryptoNodeId>,
                    network_id: NetworkID,
                ) -> Result<Self, CommonError> {
                    let node_id: ScryptoNodeId = node_id.into();
                    [< Ret $address_type:camel Address >]::new(node_id.clone(), network_id.discriminant())
                    .map_err(|e| {
                        error!("Failed create address, from node and network_id, RET error: {:?}", e);
                        CommonError::FailedToCreateAddressViaRetAddressFromNodeIdAndNetworkID { node_id_as_hex: node_id.to_hex(), network_id }
                    })
                    .map(|i| [< $address_type:camel Address >]::from(i))
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

pub(crate) use decl_ret_wrapped_address;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_address_from_invalid_node_id() {
        let unknown_node_id = ScryptoNodeId::new(222, &[0xff; 29]);
        assert_eq!(
            <AccountAddress as AddressViaRet>::new(unknown_node_id, NetworkID::Mainnet),
            Err(CommonError::FailedToCreateAddressViaRetAddressFromNodeIdAndNetworkID {
                node_id_as_hex: "deffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(), 
                network_id: NetworkID::Mainnet,
            })
        );
    }
}
