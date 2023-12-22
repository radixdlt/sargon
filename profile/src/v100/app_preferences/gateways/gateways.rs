use std::cell::RefCell;

use super::gateway::Gateway;

use crate::CommonError;
use identified_vec::{Identifiable, IdentifiedVecOf, IsIdentifiedVec, IsIdentifiedVecOf};
use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

/// The currently used Gateway and a collection of other by user added
/// or predefined Gateways the user can switch to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gateways {
    /// The currently used Gateway, when a user query's asset balances of
    /// accounts or submits transactions, this Gateway will be used.
    current: RefCell<Gateway>,

    /// Other by user added or predefined Gateways the user can switch to.
    /// It might be Gateways with different URLs on the SAME network, or
    /// other networks, the identifier of a Gateway is the URL.
    other: RefCell<IdentifiedVecOf<Gateway>>,
}

impl Gateways {
    pub fn all(&self) -> IdentifiedVecOf<Gateway> {
        let mut all = IdentifiedVecOf::new();
        all.append(self.current());
        all.append_other(self.other());
        all
    }
}

impl Serialize for Gateways {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Gateways", 2)?;
        state.serialize_field("current", self.current().id().as_str())?;
        state.serialize_field("saved", &self.all())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Gateways {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Gateways, D::Error> {
        use url::Url;

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
            .map(|g| g.clone())
            .ok_or_else(|| CommonError::InvalidGatewaysJSONCurrentNotFoundAmongstSaved)
            .map_err(de::Error::custom)?;

        let mut other = wrapped.saved.clone();

        other.remove(&current);

        Gateways::new_with_other(current, other).map_err(de::Error::custom)
    }
}

impl Gateways {
    pub fn new(current: Gateway) -> Self {
        Self::new_with_other(current, IdentifiedVecOf::new()).unwrap()
    }

    pub fn new_with_other(
        current: Gateway,
        other: IdentifiedVecOf<Gateway>,
    ) -> Result<Self, CommonError> {
        if other.contains_id(&current.id()) {
            return Err(CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent);
        }
        Ok(Self {
            current: RefCell::new(current),
            other: RefCell::new(other),
        })
    }

    pub fn current(&self) -> Gateway {
        self.current.borrow().clone()
    }

    pub fn other(&self) -> IdentifiedVecOf<Gateway> {
        self.other.borrow().clone()
    }
}

impl Gateways {
    /// Changes the current Gateway to `to`, if it is not already the current. If `to` is
    /// not a new Gateway, it will be removed from
    pub fn change_current(&self, to: Gateway) -> Result<usize, CommonError> {
        if self.current() == to {
            return Ok(0);
        }
        let old_current = self.current();
        let (was_inserted, index) = self.append(old_current);
        if !was_inserted {
            return Err(CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent);
        }
        self.other.borrow_mut().remove_by_id(&to.id());
        *self.current.borrow_mut() = to;
        Ok(index)
    }

    /// Appends `gateway` to the `other` list, without changing the `current` Gateway.
    /// If `other` already contains `gateway` then `(false, other.len())` is returned.
    /// If `other` was new then `(true, index_of_new)` is returned.
    ///
    /// - Returns: A pair `(inserted, index)`, where `inserted` is a Boolean value indicating whether
    ///   the operation added a new element, and `index` is the index of `item` in the resulting
    ///   `identified_vec`.
    pub fn append(&self, gateway: Gateway) -> (bool, usize) {
        self.other.borrow_mut().append(gateway)
    }
}

impl Default for Gateways {
    fn default() -> Self {
        Self::new_with_other(
            Gateway::mainnet(),
            IdentifiedVecOf::from_iter([Gateway::stokenet()]),
        )
        .expect("Stokenet and Mainnet should have different IDs.")
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for Gateways {
    fn placeholder() -> Self {
        let sut = Gateways::new(Gateway::rcnet());
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
    use std::cell::RefCell;

    use crate::{assert_eq_after_json_roundtrip, CommonError, HasPlaceholder};
    use identified_vec::{IdentifiedVecOf, IsIdentifiedVecOf, ItemsCloned};

    use crate::{v100::app_preferences::gateways::gateway::Gateway, NetworkID};

    use super::Gateways;

    #[test]
    fn equality() {
        assert_eq!(Gateways::placeholder(), Gateways::placeholder());
        assert_eq!(Gateways::placeholder_other(), Gateways::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Gateways::placeholder(), Gateways::placeholder_other());
    }

    #[test]
    fn change_current_to_existing() {
        let sut = Gateways::default();
        assert_eq!(sut.current().network().id(), &NetworkID::Mainnet);
        assert_eq!(sut.change_current(Gateway::stokenet()), Ok(1));
        assert_eq!(sut.current().network().id(), &NetworkID::Stokenet);
    }

    #[test]
    fn new_throw_gateways_discrepancy_other_should_not_contain_current() {
        assert_eq!(
            Gateways::new_with_other(
                Gateway::mainnet(),
                IdentifiedVecOf::from_iter([Gateway::mainnet()])
            ),
            Err(CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent)
        );
    }

    #[test]
    fn change_throw_gateways_discrepancy_other_should_not_contain_current() {
        let impossible = Gateways {
            current: RefCell::new(Gateway::mainnet()),
            other: RefCell::new(IdentifiedVecOf::from_iter([Gateway::mainnet()])),
        };
        assert_eq!(
            impossible.change_current(Gateway::stokenet()),
            Err(CommonError::GatewaysDiscrepancyOtherShouldNotContainCurrent)
        );
    }

    #[test]
    fn change_current_to_current() {
        let sut = Gateways::default();
        assert_eq!(sut.current().network().id(), &NetworkID::Mainnet);
        assert_eq!(sut.change_current(Gateway::mainnet()), Ok(0));
        assert_eq!(sut.current().network().id(), &NetworkID::Mainnet);
    }

    #[test]
    fn change_current_to_new() {
        let sut = Gateways::default();
        assert_eq!(sut.current().network().id(), &NetworkID::Mainnet);
        assert_eq!(sut.change_current(Gateway::nebunet()), Ok(1));
        assert_eq!(sut.current().network().id(), &NetworkID::Nebunet);
        assert_eq!(
            sut.other().items(),
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
