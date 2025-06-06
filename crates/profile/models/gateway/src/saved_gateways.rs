use identified_vec_of::decl_identified_vec_of;

use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered collection of unique [`Gateway`]s.
    /// It might be Gateways with different URLs on the SAME network, or
    /// other networks, the identifier of a Gateway is the URL.
    Gateway
);

/// The currently used Gateway and a collection of other by user added
/// or predefined Gateways the user can switch to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display)]
#[display("current: {}, other: {}", current, other)]
pub struct SavedGateways {
    /// The currently used Gateway, when a user query's asset balances of
    /// accounts or submits transactions, this Gateway will be used.
    pub current: Gateway,

    /// Other by user added or predefined Gateways the user can switch to.
    /// It might be Gateways with different URLs on the SAME network, or
    /// other networks, the identifier of a Gateway is the URL.
    pub other: Gateways,
}

impl SavedGateways {
    pub fn len(&self) -> usize {
        self.other.len() + 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn all(&self) -> Vec<Gateway> {
        let mut all = Vec::new();
        all.push(self.current.clone());
        all.append(&mut self.other.items());
        all
    }
}

impl Serialize for SavedGateways {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Gateways", 2)?;
        state.serialize_field("current", self.current.url.as_str())?;
        state.serialize_field("saved", &self.all())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for SavedGateways {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<SavedGateways, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "current")]
            url: Url,
            saved: Vec<Gateway>,
        }
        let wrapped = Wrapper::deserialize(deserializer)?;
        let current = wrapped
            .saved
            .iter()
            .find(|g| g.id() == wrapped.url)
            .ok_or({
                CommonError::InvalidGatewaysJSONCurrentNotFoundAmongstSaved
            })
            .map_err(de::Error::custom)?;

        let saved = wrapped.saved.clone();
        let mut other = IdentifiedVecOf::<Gateway>::new();
        for item in saved {
            if let Err(e) = other.try_insert_unique(item.clone()) {
                error!(
                    "Failed to insert unique Gateway {}, error: {:?}",
                    item,
                    e.clone()
                )
            };
        }

        other.remove_id(&current.id());

        SavedGateways::new_with_other(current.clone(), other.items())
            .map_err(de::Error::custom)
    }
}

impl SavedGateways {
    pub fn new(current: Gateway) -> Self {
        Self {
            current,
            other: Gateways::default(),
        }
    }

    pub fn new_with_other<I>(current: Gateway, other: I) -> Result<Self>
    where
        I: IntoIterator<Item = Gateway>,
    {
        let other = Gateways::from_iter(other);
        if other.contains_by_id(&current) {
            return Err(
                CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent,
            );
        }
        Ok(Self { current, other })
    }
}

impl Default for SavedGateways {
    fn default() -> Self {
        Self::new_with_other(Gateway::mainnet(), [Gateway::stokenet()])
            .expect("Stokenet and Mainnet should have different NetworkIDs.")
    }
}

impl HasSampleValues for SavedGateways {
    fn sample() -> Self {
        SavedGateways::default()
    }

    fn sample_other() -> Self {
        Self::new_with_other(Gateway::stokenet(), [Gateway::mainnet()])
            .expect("Stokenet and Mainnet should have different NetworkIDs.")
    }
}

impl HasSampleValues for Gateways {
    fn sample() -> Self {
        Self::from_iter([Gateway::stokenet()])
    }

    fn sample_other() -> Self {
        Self::from_iter([Gateway::stokenet(), Gateway::hammunet()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SavedGateways;

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
    fn new_throw_gateways_discrepancy_other_should_not_contain_current() {
        assert_eq!(
            SUT::new_with_other(Gateway::mainnet(), vec![Gateway::mainnet()]),
            Err(CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent)
        );
    }

    #[test]
    fn len() {
        let sut = SUT::new_with_other(
            Gateway::mainnet(),                         // 1
            [Gateway::stokenet(), Gateway::mardunet()], // + 2
        )
        .unwrap();
        assert_eq!(sut.clone().len(), 1 + 2);
        assert!(!sut.is_empty());
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();

        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "current": "https://mainnet.radixdlt.com/",
                "saved": [
                    {
                        "network":
                        {
                            "name": "mainnet",
                            "id": 1,
                            "displayDescription": "Mainnet"
                        },
                        "url": "https://mainnet.radixdlt.com/"
                    },
                    {
                        "network":
                        {
                            "name": "stokenet",
                            "id": 2,
                            "displayDescription": "Stokenet"
                        },
                        "url": "https://babylon-stokenet-gateway.radixdlt.com/"
                    }
                ]
            }
            "#,
        )
    }

    #[test]
    fn deserialize_from_json_with_different_description_treats_both_gateways_as_wellknown(
    ) {
        let json = r#"
        {
            "current": "https://mainnet.radixdlt.com/",
            "saved": [
                {
                    "network": {
                        "name": "mainnet",
                        "id": 1,
                        "displayDescription": "Mainnet Gateway"
                    },
                    "url": "https://mainnet.radixdlt.com/"
                },
                {
                    "network": {
                        "name": "stokenet",
                        "id": 2,
                        "displayDescription": "Stokenet (testnet) Gateway"
                    },
                    "url": "https://babylon-stokenet-gateway.radixdlt.com/"
                }
            ]
        }
        "#;

        let sut = serde_json::from_str::<SUT>(json).unwrap();
        let is_wellknown_vec = sut
            .all()
            .iter()
            .map(|gateway| gateway.is_wellknown())
            .collect_vec();

        assert_eq!(vec![true, true], is_wellknown_vec)
    }

    #[test]
    fn test_gateways_identification() {
        let mainnet = Gateway::new(
            String::from("https://mainnet.radixdlt.com/"),
            NetworkID::Mainnet,
        )
        .unwrap();

        let mainnet_no_slash = Gateway::new(
            String::from("https://mainnet.radixdlt.com"),
            NetworkID::Mainnet,
        )
        .unwrap();

        let other_mainnet = Gateway::new(
            String::from("https://other-mainnet.radixdlt.com"),
            NetworkID::Mainnet,
        )
        .unwrap();

        let other_mainnet_http = Gateway::new(
            String::from("http://other-mainnet.radixdlt.com"),
            NetworkID::Mainnet,
        )
        .unwrap();

        let gateways_vec =
            vec![mainnet, mainnet_no_slash, other_mainnet, other_mainnet_http];

        let identified_gateways =
            Gateways::from_iter(gateways_vec.iter().cloned());

        // Expecting only 3 unique Gateways, since the two mainnet ones differ in only a slash which is
        // considered as the same URL
        assert_eq!(3, identified_gateways.len())
    }
}
