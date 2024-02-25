use crate::prelude::*;

use radix_engine_common::address::AddressBech32Decoder;
use radix_engine_interface::blueprints::resource::NonFungibleGlobalId as ScryptoNonFungibleGlobalId;

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
    pub resource_address: ResourceAddress,
    pub non_fungible_local_id: NonFungibleLocalId,
}

impl From<ResourceAddress> for radix_engine_common::types::ResourceAddress {
    fn from(value: ResourceAddress) -> Self {
        radix_engine_common::types::ResourceAddress::try_from_bech32(
            &AddressBech32Decoder::new(
                &value.network_id().network_definition(),
            ),
            value.address().clone().as_str(),
        )
        .expect("Should always be able to convert from Sargon to RET for ResourceAddress")
    }
}

impl NonFungibleGlobalId {
    fn from_internal_engine(internal: RETNonFungibleGlobalIdInternal) -> Self {
        let (engine_resource_address, engine_local_id) =
            internal.non_fungible_global_id.into_parts();

        let network_id: NetworkID = internal
            .network_id
            .try_into()
            .expect("should be able to know network");

        let non_fungible_local_id: NonFungibleLocalId = engine_local_id.into();

        let resource_address = ResourceAddress::new(
            engine_resource_address.into_node_id(),
            network_id,
        )
        .expect("Should always be able to convert between Sargon and RET");

        Self {
            resource_address,
            non_fungible_local_id,
        }
    }

    fn network_id(&self) -> NetworkID {
        self.resource_address.network_id()
    }

    fn engine(&self) -> RETNonFungibleGlobalId {
        let scrypto_global_id = ScryptoNonFungibleGlobalId::new(
            self.resource_address.clone().into(),
            self.non_fungible_local_id.clone().into(),
        );
        RETNonFungibleGlobalId::new(
            scrypto_global_id,
            self.network_id().discriminant(),
        )
    }
}

impl FromStr for NonFungibleGlobalId {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RETNonFungibleGlobalIdInternal::from_str(s)
            .map(Self::from_internal_engine)
            .map_err(|_| CommonError::InvalidNonFungibleGlobalID {
                bad_value: s.to_owned(),
            })
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

impl HasPlaceholder for NonFungibleGlobalId {
    fn placeholder() -> Self {
        "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>".parse().expect("Valid GC NFT Global ID")
    }

    fn placeholder_other() -> Self {
        "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#1337#".parse().expect("Valid Scorpion NFT Global ID")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleGlobalId;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
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
}
