use identified_vec::Identifiable;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Deref,
    sync::Arc,
};
use url::Url;

use crate::{CommonError, NetworkID};

use super::radix_network::RadixNetwork;

/// A gateway to some Radix Network, which is a high level REST API which clients (wallets) can
/// consume in order to query asset balances and submit transactions.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, uniffi::Record)]
pub struct Gateway {
    /// The Radix network the API is a Gateway to.
    pub network: RadixNetwork,

    /// The URL to the gateways API endpoint
    pub url: Url,
}

impl Identifiable for Gateway {
    type ID = Url;

    fn id(&self) -> Self::ID {
        self.url.clone()
    }
}

impl Debug for Gateway {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.network.display_description,
            self.url.to_string(),
        )
    }
}

impl Display for Gateway {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url.to_string(),)
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::mainnet()
    }
}

// #[uniffi::export]
impl Gateway {
    // #[uniffi::constructor]
    pub fn new(url: String, id: NetworkID) -> Result<Arc<Self>, crate::CommonError> {
        let url = Url::try_from(url.as_str()).map_err(|_| CommonError::InvalidURL(url))?;
        let network = RadixNetwork::lookup_by_id(id)?;
        Ok(Self { url, network }.into())
    }
}

impl Gateway {
    fn declare(url: &str, id: NetworkID) -> Self {
        Self::new(url.to_string(), id)
            .expect("Valid")
            .deref()
            .clone()
    }
}

#[uniffi::export]
pub fn gateway_mainnet() -> Gateway {
    Gateway::mainnet().into()
}

#[uniffi::export]
pub fn gateway_stokenet() -> Gateway {
    Gateway::stokenet().into()
}

impl Gateway {
    pub fn mainnet() -> Self {
        Self::declare("https://mainnet.radixdlt.com/", NetworkID::Mainnet)
    }

    pub fn stokenet() -> Self {
        Self::declare(
            "https://babylon-stokenet-gateway.radixdlt.com/",
            NetworkID::Stokenet,
        )
    }

    pub fn rcnet() -> Self {
        Self::declare("https://rcnet-v3.radixdlt.com/", NetworkID::Zabanet)
    }

    pub fn nebunet() -> Self {
        Self::declare("https://betanet.radixdlt.com/", NetworkID::Nebunet)
    }

    pub fn kisharnet() -> Self {
        Self::declare("https://rcnet.radixdlt.com/", NetworkID::Kisharnet)
    }

    pub fn ansharnet() -> Self {
        Self::declare(
            "https://ansharnet-gateway.radixdlt.com/",
            NetworkID::Ansharnet,
        )
    }

    pub fn hammunet() -> Self {
        Self::declare(
            "https://hammunet-gateway.radixdlt.com/",
            NetworkID::Hammunet,
        )
    }

    pub fn enkinet() -> Self {
        Self::declare("https://enkinet-gateway.radixdlt.com/", NetworkID::Enkinet)
    }

    pub fn mardunet() -> Self {
        Self::declare(
            "https://mardunet-gateway.radixdlt.com/",
            NetworkID::Mardunet,
        )
    }
}

impl Gateway {
    fn wellknown() -> Vec<Self> {
        vec![Self::mainnet(), Self::stokenet()]
    }

    pub fn is_wellknown(&self) -> bool {
        Self::wellknown().contains(&self)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_eq_after_json_roundtrip;
    use identified_vec::Identifiable;
    use radix_engine_common::prelude::HashSet;

    use super::Gateway;

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = Gateway::mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "network":
                {
                    "name": "mainnet",
                    "id": 1,
                    "displayDescription": "Mainnet"
                },
                "url": "https://mainnet.radixdlt.com/"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = Gateway::stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "network":
                {
                    "name": "stokenet",
                    "id": 2,
                    "displayDescription": "Stokenet"
                },
                "url": "https://babylon-stokenet-gateway.radixdlt.com/"
            }
            "#,
        )
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", Gateway::mainnet()),
            "https://mainnet.radixdlt.com/"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Gateway::mainnet()),
            "Mainnet: https://mainnet.radixdlt.com/"
        );
    }

    #[test]
    fn identifiable() {
        assert_eq!(Gateway::mainnet().id(), Gateway::mainnet().url);
    }

    #[test]
    fn mainnet_is_default() {
        assert_eq!(Gateway::default(), Gateway::mainnet());
    }

    #[test]
    fn mainnet_is_wellknown() {
        assert_eq!(Gateway::mainnet().is_wellknown(), true);
    }

    #[test]
    fn stokenet_is_wellknown() {
        assert_eq!(Gateway::stokenet().is_wellknown(), true);
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::from_iter([
                Gateway::mainnet(),
                Gateway::stokenet(),
                Gateway::rcnet(),
                Gateway::nebunet(),
                Gateway::kisharnet(),
                Gateway::ansharnet(),
                Gateway::hammunet(),
                Gateway::enkinet(),
                Gateway::mardunet(),
                // Twice
                Gateway::mainnet(),
                Gateway::stokenet(),
                Gateway::rcnet(),
                Gateway::nebunet(),
                Gateway::kisharnet(),
                Gateway::ansharnet(),
                Gateway::hammunet(),
                Gateway::enkinet(),
                Gateway::mardunet(),
            ])
            .len(),
            9
        );
    }
}

#[cfg(test)]
mod tests_uniffi_api {

    use crate::{gateway_mainnet, gateway_stokenet};

    use super::Gateway;

    #[test]
    fn test_gateway_mainnet() {
        assert_eq!(gateway_mainnet(), Gateway::mainnet());
    }

    #[test]
    fn test_gateway_stokenet() {
        assert_eq!(gateway_stokenet(), Gateway::stokenet());
    }
}
