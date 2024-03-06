use crate::prelude::*;

use radix_engine_common::address::AddressBech32Decoder;
use radix_engine_common::types::ResourceAddress as ScryptoResourceAddress;
// use radix_engine_interface::blueprints::resource::NonFungibleGlobalId as ScryptoNonFungibleGlobalId;

use radix_engine::types::NonFungibleGlobalId as ScryptoNonFungibleGlobalId;

use radix_engine_toolkit_json::models::scrypto::non_fungible_global_id::{
    SerializableNonFungibleGlobalId as RETNonFungibleGlobalId,
    SerializableNonFungibleGlobalIdInternal as RETNonFungibleGlobalIdInternal,
};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    uniffi::Record,
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
    pub(crate) resource_address: ResourceAddress,
    pub(crate) non_fungible_local_id: NonFungibleLocalId,
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
            info!("Notice: Fungible resource address used with NonFungible Global ID.")
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

impl TryFrom<RETNonFungibleGlobalIdInternal> for NonFungibleGlobalId {
    type Error = crate::CommonError;

    fn try_from(
        value: RETNonFungibleGlobalIdInternal,
    ) -> sbor::prelude::Result<Self, Self::Error> {
        let (scrypto_resource_address, scrypto_local_id) =
            value.non_fungible_global_id.into_parts();

        TryInto::<NetworkID>::try_into(value.network_id)
            .and_then(|network_id| {
                ResourceAddress::new(
                    scrypto_resource_address.into_node_id(),
                    network_id,
                )
            })
            .map(|r| Self::new_unchecked(r, scrypto_local_id.into()))
    }
}

impl From<NonFungibleGlobalId> for RETNonFungibleGlobalId {
    fn from(value: NonFungibleGlobalId) -> Self {
        let scrypto_global_id = ScryptoNonFungibleGlobalId::new(
            Into::<ScryptoResourceAddress>::into(&value.resource_address),
            value.non_fungible_local_id.clone().into(),
        );
        RETNonFungibleGlobalId::new(
            scrypto_global_id,
            value.network_id().discriminant(),
        )
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

impl NonFungibleGlobalId {
    fn network_id(&self) -> NetworkID {
        self.resource_address.network_id()
    }

    fn engine(&self) -> RETNonFungibleGlobalId {
        self.clone().into()
    }
}

impl FromStr for NonFungibleGlobalId {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RETNonFungibleGlobalIdInternal::from_str(s)
            .map_err(|_| CommonError::InvalidNonFungibleGlobalID {
                bad_value: s.to_owned(),
            })
            .and_then(TryInto::<Self>::try_into)
    }
}

impl NonFungibleGlobalId {
    /// Returns the canonical string representation of a NonFungibleGlobalID: "<resource>:<local>"
    ///
    /// For example:
    ///
    /// `resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>`
    pub fn to_canonical_string(&self) -> String {
        format!("{}", self.engine().0)
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: SUT = str.parse().unwrap();
        assert_eq!(format!("{}", id), str);
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
}
