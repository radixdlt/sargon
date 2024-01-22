use crate::prelude::*;

/// The currently used Gateway and a collection of other by user added
/// or predefined Gateways the user can switch to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct Gateways {
    /// The currently used Gateway, when a user query's asset balances of
    /// accounts or submits transactions, this Gateway will be used.
    pub current: Gateway,

    /// Other by user added or predefined Gateways the user can switch to.
    /// It might be Gateways with different URLs on the SAME network, or
    /// other networks, the identifier of a Gateway is the URL.
    pub other: IdentifiedVecVia<Gateway>,
}

#[uniffi::export]
pub fn new_gateways(current: Gateway) -> Gateways {
    Gateways::new(current)
}

#[uniffi::export]
pub fn new_gateways_placeholder() -> Gateways {
    Gateways::placeholder()
}
#[uniffi::export]
pub fn new_gateways_placeholder_other() -> Gateways {
    Gateways::placeholder_other()
}

impl Gateways {
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

impl Serialize for Gateways {
    #[cfg(not(tarpaulin_include))] // false negative
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

impl<'de> Deserialize<'de> for Gateways {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Gateways, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "current")]
            url: Url,
            saved: IdentifiedVecOf<Gateway>,
        }
        let wrapped = Wrapper::deserialize(deserializer)?;
        let current = wrapped
            .saved
            .iter()
            .find(|g| g.id() == wrapped.url)
            .cloned()
            .ok_or({
                CommonError::InvalidGatewaysJSONCurrentNotFoundAmongstSaved
            })
            .map_err(de::Error::custom)?;

        let mut other = wrapped.saved.clone();

        other.remove(&current);

        Gateways::new_with_other(current, other.items())
            .map_err(de::Error::custom)
    }
}

impl Gateways {
    pub fn new(current: Gateway) -> Self {
        Self {
            current,
            other: IdentifiedVecVia::new(),
        }
    }

    pub fn new_with_other<I>(current: Gateway, other: I) -> Result<Self>
    where
        I: IntoIterator<Item = Gateway>,
    {
        let other = IdentifiedVecVia::from_iter(other);
        if other.contains(&current) {
            return Err(
                CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent,
            );
        }
        Ok(Self { current, other })
    }
}

impl Gateways {
    /// Changes the current Gateway to `to`, if it is not already the current. If `to` is
    /// not a new Gateway, it will be removed from. Returns `Ok(false)` if `to` was already
    /// the `current`, returns `Ok(true)` if `to` was not already `current`.
    pub fn change_current(&mut self, to: Gateway) -> Result<bool> {
        if self.current == to {
            return Ok(false);
        }
        let old_current = &self.current;
        let was_inserted = self.append(old_current.clone());
        if !was_inserted {
            return Err(
                CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent,
            );
        }
        self.other.remove_by_id(&to.id());
        self.current = to;
        Ok(true)
    }

    /// Appends `gateway` to the `other` list, without changing the `current` Gateway.
    /// If `other` already contains `gateway` then `(false, other.len())` is returned.
    /// If `other` was new then `(true, index_of_new)` is returned.
    ///
    /// - Returns: `true` if it was added, `false` if it was already present (noop)
    pub fn append(&mut self, gateway: Gateway) -> bool {
        if self.other.contains(&gateway) {
            return false;
        }
        self.other.append(gateway);
        true
    }
}

impl Default for Gateways {
    fn default() -> Self {
        Self::new_with_other(Gateway::mainnet(), vec![Gateway::stokenet()])
            .expect("Stokenet and Mainnet should have different IDs.")
    }
}

impl HasPlaceholder for Gateways {
    fn placeholder() -> Self {
        let mut sut = Gateways::new(Gateway::rcnet());
        sut.append(Gateway::mainnet());
        sut.append(Gateway::stokenet());
        sut
    }

    fn placeholder_other() -> Self {
        Gateways::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(Gateways::placeholder(), Gateways::placeholder());
        assert_eq!(
            Gateways::placeholder_other(),
            Gateways::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(Gateways::placeholder(), Gateways::placeholder_other());
    }

    #[test]
    fn change_current_to_existing() {
        let mut sut = Gateways::default();
        assert_eq!(sut.current.network.id, NetworkID::Mainnet);
        assert_eq!(sut.change_current(Gateway::stokenet()), Ok(true));
        assert_eq!(sut.current.network.id, NetworkID::Stokenet);
    }

    #[test]
    fn new_throw_gateways_discrepancy_other_should_not_contain_current() {
        assert_eq!(
            Gateways::new_with_other(
                Gateway::mainnet(),
                vec![Gateway::mainnet()]
            ),
            Err(CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent)
        );
    }

    #[test]
    fn change_throw_gateways_discrepancy_other_should_not_contain_current() {
        let mut impossible = Gateways {
            current: Gateway::mainnet(),
            other: IdentifiedVecVia::from_iter([Gateway::mainnet()]),
        };
        assert_eq!(
            impossible.change_current(Gateway::stokenet()),
            Err(CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent)
        );
    }

    #[test]
    fn change_current_to_current() {
        let mut sut = Gateways::default();
        assert_eq!(sut.current.network.id, NetworkID::Mainnet);
        assert_eq!(sut.change_current(Gateway::mainnet()), Ok(false));
        assert_eq!(sut.current.network.id, NetworkID::Mainnet);
    }

    #[test]
    fn len() {
        assert_eq!(
            Gateways::new_with_other(
                Gateway::mainnet(),                         // 1
                [Gateway::stokenet(), Gateway::mardunet()]  // + 2
            )
            .unwrap()
            .len(),
            1 + 2
        );
    }

    #[test]
    fn change_current_to_new() {
        let mut sut = Gateways::default();
        assert_eq!(sut.current.network.id, NetworkID::Mainnet);
        assert_eq!(sut.change_current(Gateway::nebunet()), Ok(true));
        assert_eq!(sut.current.network.id, NetworkID::Nebunet);
        assert_eq!(
            sut.other.items(),
            [Gateway::stokenet(), Gateway::mainnet()]
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = Gateways::placeholder();

        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "current": "https://rcnet-v3.radixdlt.com/",
                "saved": [
                    {
                        "network":
                        {
                            "name": "zabanet",
                            "id": 14,
                            "displayDescription": "RCnet-V3 (Test Network)"
                        },
                        "url": "https://rcnet-v3.radixdlt.com/"
                    },
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
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{
        new_gateways, new_gateways_placeholder, new_gateways_placeholder_other,
        Gateway, HasPlaceholder,
    };

    use super::Gateways;

    #[test]
    fn equality_placeholders() {
        assert_eq!(Gateways::placeholder(), new_gateways_placeholder());
        assert_eq!(
            Gateways::placeholder_other(),
            new_gateways_placeholder_other()
        );
    }

    #[test]
    fn new_with_current() {
        assert_eq!(
            new_gateways(Gateway::mardunet()).all(),
            [Gateway::mardunet()]
        );
    }
}
