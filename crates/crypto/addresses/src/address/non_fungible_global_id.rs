use crate::prelude::*;
use core_utils::prelude::format_string;
use hierarchical_deterministic::HierarchicalDeterministicPublicKey;
use radix_common::{
    address::{AddressBech32Decoder, AddressBech32Encoder},
    prelude::NetworkDefinition,
};
use radix_engine_toolkit::prelude::NetworkDefinitionExt;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    Ord,
    PartialOrd,
    DeserializeFromStr,
    derive_more::Display,
)]
#[display("{}", self.to_canonical_string())]
pub struct NonFungibleGlobalId {
    // N.B. we WANT This to be a `NonFungibleResourceAddress` type, alas, it
    // cannot, since that validation does not happen part of Engine, so it is
    // possible (maybe even likely) that some Non Fungible tokens have addresses
    // which are "fungible" (i.e. entity type `GlobalFungibleResourceManager`
    // instead of `GlobalNonFungibleResourceManager`).
    //
    // For more info see slack:
    // https://rdxworks.slack.com/archives/C01HK4QFXNY/p1709633826502809?thread_ts=1709633374.199459&channel=C01HK4QFXNY&message_ts=1709633826.502809
    pub resource_address: ResourceAddress,
    pub non_fungible_local_id: NonFungibleLocalId,
}

impl NonFungibleGlobalId {
    /// Creates a`NonFungibleGlobalId` from an unchecked resource address, i.e.
    /// a `ResourceAddress` instead of `NonFungibleResourceAddress`, since
    /// unfortunately Radix Engine / Network does not validate that all `NonFungibleGlobalId`
    /// indeed consists of a non fungible `ResourceAddress` we must be lenient and
    /// accept that unchecked address type.
    pub fn new_unchecked(
        resource_address: impl Into<ResourceAddress>,
        local_id: NonFungibleLocalId,
    ) -> Self {
        let resource_address = resource_address.into();
        if resource_address.is_fungible() {
            debug!("Notice: Fungible resource address used with NonFungible Global ID.")
        }
        Self {
            resource_address,
            non_fungible_local_id: local_id,
        }
    }

    /// Crates a `NonFungibleGlobalId` from a checked resource address, i.e.
    /// `NonFungibleResourceAddress` instead of `ResourceAddress`, internally we
    /// prefer using this constructor.
    pub fn new(
        resource_address: NonFungibleResourceAddress,
        local_id: NonFungibleLocalId,
    ) -> Self {
        Self::new_unchecked(resource_address, local_id)
    }
}

impl From<&ResourceAddress> for ScryptoResourceAddress {
    fn from(value: &ResourceAddress) -> Self {
        TryInto::<ScryptoResourceAddress>::try_into(value.node_id())
        .expect("Should always be able to convert from Sargon to ScryptoResourceAddress")
    }
}

impl From<NonFungibleGlobalId> for ScryptoNonFungibleGlobalId {
    fn from(value: NonFungibleGlobalId) -> Self {
        Self::new(
            value.resource_address.into(),
            value.non_fungible_local_id.into(),
        )
    }
}

impl From<(ScryptoNonFungibleGlobalId, NetworkID)> for NonFungibleGlobalId {
    fn from(value: (ScryptoNonFungibleGlobalId, NetworkID)) -> Self {
        let (global_id, network_id) = value;
        Self::new_unchecked(
            (global_id.resource_address(), network_id),
            global_id.local_id().clone().into(),
        )
    }
}

impl FromStr for NonFungibleGlobalId {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let resource_address_string = s.split(':').next().ok_or(
            CommonError::InvalidNonFungibleGlobalID {
                bad_value: s.to_owned(),
            },
        )?;

        let network_definition =
            NetworkDefinition::from_address_string(resource_address_string)
                .ok_or(CommonError::InvalidNonFungibleGlobalID {
                    bad_value: s.to_owned(),
                })?;
        let bech32_decoder = AddressBech32Decoder::new(&network_definition);

        let non_fungible_global_id =
            ScryptoNonFungibleGlobalId::try_from_canonical_string(
                &bech32_decoder,
                s,
            )
            .map_err(|_| {
                CommonError::InvalidNonFungibleGlobalID {
                    bad_value: s.to_owned(),
                }
            })?;

        TryInto::<NetworkID>::try_into(network_definition.id).map(
            |network_id| {
                NonFungibleGlobalId::from((non_fungible_global_id, network_id))
            },
        )
    }
}

impl From<HierarchicalDeterministicPublicKey> for NonFungibleGlobalId {
    fn from(value: HierarchicalDeterministicPublicKey) -> Self {
        NonFungibleGlobalId::from((
            ScryptoNonFungibleGlobalId::from_public_key(
                ScryptoPublicKey::from(value.public_key),
            ),
            value.derivation_path.network_id(),
        ))
    }
}

#[cfg(test)]
impl From<&str> for NonFungibleGlobalId {
    /// TEST ONLY
    fn from(value: &str) -> Self {
        value.parse().unwrap_or_else(|_| panic!("Test failed since the passed in str is not a NonFungibleGlobalId: '{}'",
            value))
    }
}

impl NonFungibleGlobalId {
    /// Returns the canonical string representation of a NonFungibleGlobalID: "<resource>:<local>"
    ///
    /// For example:
    ///
    /// `resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>`
    pub fn to_canonical_string(&self) -> String {
        let network_definition =
            self.resource_address.network_id().network_definition();
        let bech32_encoder = AddressBech32Encoder::new(&network_definition);
        ScryptoNonFungibleGlobalId::from(self.clone())
            .to_canonical_string(&bech32_encoder)
    }

    pub fn formatted(&self, format: AddressFormat) -> String {
        match format {
            AddressFormat::Default => {
                let local_id_formatted = match self.non_fungible_local_id {
                    NonFungibleLocalId::Ruid { value: _ } => format_string(
                        self.non_fungible_local_id.to_string(),
                        5,
                        5,
                    ),
                    _ => self.non_fungible_local_id.to_string(),
                };

                format!(
                    "{}:{}",
                    self.resource_address.formatted(format),
                    local_id_formatted
                )
            }
            AddressFormat::Full => format!(
                "{}:{}",
                self.resource_address.formatted(format),
                self.non_fungible_local_id.formatted(format)
            ),
            AddressFormat::Raw => self.to_canonical_string(),
        }
    }
}

impl HasSampleValues for NonFungibleGlobalId {
    fn sample() -> Self {
        Self::new(
            NonFungibleResourceAddress::sample(),
            NonFungibleLocalId::string("Member_237").unwrap(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            NonFungibleResourceAddress::sample_other(),
            NonFungibleLocalId::sample_other(),
        )
    }
}

impl NonFungibleGlobalId {
    #[allow(unused)]
    pub(crate) fn sample_ruid() -> Self {
        Self::new(
            NonFungibleResourceAddress::sample(),
            NonFungibleLocalId::ruid(
                hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
            ).unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleGlobalId;

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
    fn ord_same_resource_address() {
        let r = ResourceAddress::sample();
        assert!(
            SUT::new_unchecked(r, NonFungibleLocalId::integer(1))
                < SUT::new_unchecked(r, NonFungibleLocalId::integer(2))
        );
    }

    #[test]
    fn ord_diff_resource_address_addr_takes_precedence_over_local_id_integer() {
        assert!(
            // lazy test, using FUNGIBLE Resource address instead of Non-Fungible ResourceAddress
            // as we should be doing... alas we accept fungible ones since they can appear "in the wild".
            SUT::new_unchecked(
                ResourceAddress::sample_mainnet_candy(),
                NonFungibleLocalId::integer(99999)
            ) < SUT::new_unchecked(
                ResourceAddress::sample_mainnet_xrd(),
                NonFungibleLocalId::integer(111)
            )
        );
    }

    #[test]
    fn test_deserialize() {
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: SUT = str.parse().unwrap();
        match id.clone().non_fungible_local_id {
            NonFungibleLocalId::Integer { value } => assert_eq!(value, 2244),
            _ => panic!("wrong"),
        }

        assert_eq!(id.to_canonical_string(), str);
    }

    #[test]
    fn test_address() {
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: SUT = str.parse().unwrap();
        assert_eq!(
            id.resource_address.address(),
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd"
        );
    }

    #[test]
    fn test_network_id() {
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: SUT = str.parse().unwrap();
        assert_eq!(id.to_string(), str);
    }

    #[test]
    fn test_as_str() {
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: SUT = str.parse().unwrap();
        assert_eq!(id.to_string(), str);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::sample()), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>");
    }

    #[test]
    fn formatted_raw() {
        assert_eq!(SUT::sample_ruid().formatted(AddressFormat::Raw), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}");
    }

    #[test]
    fn formatted_default() {
        assert_eq!(
            SUT::sample_ruid().formatted(AddressFormat::Default),
            "reso...c9wlxa:{dead...3210}"
        );
    }

    #[test]
    fn formatted_full() {
        assert_eq!(
            SUT::sample_ruid().formatted(AddressFormat::Full),
            "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210"
        );
    }

    #[test]
    fn formatted_default_vs_middle() {
        let resource_address = NonFungibleResourceAddress::sample();

        // local_id: integer
        let mut local_id = NonFungibleLocalId::integer(12345678);
        let mut item = SUT::new(resource_address, local_id);
        assert_eq!(
            item.formatted(AddressFormat::Default),
            "reso...c9wlxa:#12345678#"
        );

        // local_id: string
        local_id = NonFungibleLocalId::string("foobar").unwrap();
        item = SUT::new(resource_address, local_id);
        assert_eq!(
            item.formatted(AddressFormat::Default),
            "reso...c9wlxa:<foobar>"
        );

        // local_id: bytes
        local_id = NonFungibleLocalId::bytes([0xde, 0xad]).unwrap();
        item = SUT::new(resource_address, local_id);
        assert_eq!(
            item.formatted(AddressFormat::Default),
            "reso...c9wlxa:[dead]"
        );

        // local_id: ruid
        local_id = NonFungibleLocalId::ruid(hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()).unwrap();
        item = SUT::new(resource_address, local_id);
        assert_eq!(
            item.formatted(AddressFormat::Default),
            "reso...c9wlxa:{dead...3210}"
        );
    }

    #[test]
    fn json_roundtrip() {
        let id: SUT =
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &id,
            json!("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#"),
        );
        assert_json_roundtrip(&id);
        assert_json_value_ne_after_roundtrip(
            &id,
            json!("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#9999#"),
        );
    }

    #[test]
    fn json_roundtrip_str() {
        let sut: SUT =
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &sut,
            json!("resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha : foobar")
        );
        assert_json_value_fails::<SUT>(
            json!("account_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn hash() {
        let a: SUT =
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#1#"
                .parse()
                .unwrap();
        let b: SUT =
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2#"
                .parse()
                .unwrap();
        let mut set = HashSet::<SUT>::new();
        set.insert(a.clone());
        assert_eq!(set.len(), 1);
        set.insert(a);
        assert_eq!(set.len(), 1);
        set.insert(b);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn global_id_with_fungible_resource_addresses_are_accepted() {
        assert!("resource_tdx_2_1t4kep9ldg9t0cszj78z6fcr2zvfxfq7muetq7pyvhdtctwxum90scq:#1#".parse::<SUT>().unwrap().resource_address.is_fungible());
    }

    #[test]
    fn into_scrypto() {
        let resource_address =
            ResourceAddress::sample_mainnet_nft_gc_membership();
        let local_id = NonFungibleLocalId::string("Member_237").unwrap();
        let sut = SUT::new_unchecked(resource_address, local_id.clone());
        assert_eq!(
            ScryptoNonFungibleGlobalId::from(sut),
            ScryptoNonFungibleGlobalId::new(
                resource_address.into(),
                local_id.into()
            )
        );
    }

    #[test]
    fn test_to_canonical_string() {
        let sut = SUT::sample();

        pretty_assertions::assert_eq!(
            sut.to_canonical_string(),
            "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>".to_string()
        );
    }

    #[test]
    fn test_resource_address_to_scrypto() {
        let resource_address = ResourceAddress::sample();
        let scrypto: ScryptoResourceAddress = (&resource_address).into();
        assert_eq!(
            scrypto,
            ScryptoResourceAddress::try_from(resource_address.node_id())
                .unwrap()
        );
    }

    #[test]
    fn from_str_invalid_non_fungible_global_id() {
        let invalid = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:invalid";
        let result = NonFungibleGlobalId::from_str(invalid);
        assert_eq!(
            result,
            Err(CommonError::InvalidNonFungibleGlobalID {
                bad_value: invalid.to_owned()
            })
        );
    }

    #[test]
    fn from_hierarchical_deterministic_public_key() {
        let public_key = HierarchicalDeterministicPublicKey::sample();
        let result = SUT::from(public_key.clone());
        let expected: SUT = "resource_rdx1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxxed25sg:[dabcaee921f921c12d12f3b24acfb32f3edf4dea474f24623d1b872b07]".parse().unwrap();
        assert_eq!(result, expected);
    }
}
