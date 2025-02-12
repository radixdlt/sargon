use crate::prelude::*;

pub trait ProfileCurrentGateway {
    fn current_gateway(&self) -> Gateway;

    /// The NetworkID currently being used, dependent on `current` gateway in
    /// AppPreferences
    fn current_network_id(&self) -> NetworkID {
        self.current_gateway().network.id
    }
}

impl ProfileCurrentGateway for Profile {
    /// Returns the `current` gateway in AppPreferences, used by host clients to
    /// know the NetworkID currently being used.
    fn current_gateway(&self) -> Gateway {
        self.app_preferences.gateways.current.clone()
    }
}

pub trait ProfileCurrentNetwork {
    fn current_network(&self) -> Result<&ProfileNetwork>;

    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    fn accounts_on_current_network(&self) -> Result<Accounts> {
        self.current_network().map(|n| n.accounts.visible())
    }

    /// Returns the non-hidden accounts on the current network as `AccountForDisplay`
    fn accounts_for_display_on_current_network(
        &self,
    ) -> Result<AccountsForDisplay> {
        self.accounts_on_current_network().map(|accounts| {
            accounts
                .iter()
                .map(AccountForDisplay::from)
                .collect::<AccountsForDisplay>()
        })
    }

    /// Returns the hidden accounts on the current network, empty if no hidden accounts
    /// on the network
    fn hidden_accounts_on_current_network(&self) -> Result<Accounts> {
        self.current_network().map(|n| n.accounts.hidden())
    }
}

impl ProfileCurrentNetwork for Profile {
    /// The ProfileNetwork of the currently used Network dependent on the `current`
    /// Gateway set in AppPreferences. This affects which Accounts users see in
    /// "Home screen" in wallet apps.
    fn current_network(&self) -> Result<&ProfileNetwork> {
        let current_network_id = self.current_network_id();
        self.networks.get_id(current_network_id).ok_or(
            CommonError::NoNetworkInProfile {
                network_id: current_network_id.to_string(),
            },
        )
    }
}

/// When user changes `current` Gateway in AppPreferences host clients should
/// make it so that they can only change to non current gateway, this small type
/// represents the outcome of switching, e.g. if they just switched to a "new"
/// network, i.e. if the gateway was in `other` list in saved gateways, or if
/// we just added it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChangeGatewayOutcome {
    /// If we did in fact change the gateway, and if the gateway was unknown
    /// or known before it was added, i.e. `is_new` will be true iff the gateway
    /// was unknown before changing to it.
    DidChange {
        /// If the Gateway we just switched to already was in the `other` list of
        /// saved gateways in AppPreferences, or if it was entirely new.
        is_new: bool,
    },

    /// We tried to change to the current gateway.
    NoChange,
}

impl HasSampleValues for ChangeGatewayOutcome {
    fn sample() -> Self {
        ChangeGatewayOutcome::DidChange { is_new: true }
    }

    fn sample_other() -> Self {
        ChangeGatewayOutcome::NoChange
    }
}

pub trait SavedGatewaysChangeCurrent {
    fn change_current(&mut self, to: Gateway) -> ChangeGatewayOutcome;
}

impl SavedGatewaysChangeCurrent for SavedGateways {
    /// Changes the current Gateway to `to`, if it is not already the current. If `to` is
    /// not a new Gateway, it will be removed from. Returns `Ok(false)` if `to` was already
    /// the `current`, returns `Ok(true)` if `to` was not already `current`.
    fn change_current(&mut self, to: Gateway) -> ChangeGatewayOutcome {
        if self.current == to {
            return ChangeGatewayOutcome::NoChange;
        }
        let old_current = &self.current;
        let was_inserted = self.append_to_other(old_current.clone(), true);
        if !was_inserted {
            let msg = "Discrepancy! 'other' already contained 'current'";
            error!("{}", msg);
            panic!("{}", msg);
        }
        let is_new = self.other.remove_id(&to.id()).is_none();
        self.current = to;
        ChangeGatewayOutcome::DidChange { is_new }
    }
}

pub trait SavedGatewaysAppend {
    fn append_to_other(&mut self, gateway: Gateway, is_switching: bool)
        -> bool;

    /// Appends `gateway` to the `other` list if `gateway` not equals `current`,
    /// without changing the `current` Gateway.
    /// If `other` already contains `gateway` then `(false, other.len())` is returned.
    /// If `other` was new then `(true, index_of_new)` is returned.
    ///
    /// - Returns: `true` if it was added, `false` if it was already present (noop)
    fn append(&mut self, gateway: Gateway) -> bool {
        self.append_to_other(gateway, false)
    }
}

impl SavedGatewaysAppend for SavedGateways {
    fn append_to_other(
        &mut self,
        gateway: Gateway,
        is_switching: bool,
    ) -> bool {
        if !is_switching && self.current == gateway {
            return false;
        }
        self.other.append(gateway).0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SavedGateways;

    #[test]
    fn change_current_to_new() {
        let mut sut = SUT::default();
        assert_eq!(sut.current.network.id, NetworkID::Mainnet);
        assert_eq!(
            sut.change_current(Gateway::nebunet()),
            ChangeGatewayOutcome::DidChange { is_new: true }
        );
        assert_eq!(sut.current.network.id, NetworkID::Nebunet);
        assert_eq!(
            sut.other.items(),
            [Gateway::stokenet(), Gateway::mainnet()]
        );
    }

    #[test]
    fn change_current_to_existing() {
        let mut sut = SUT::default();
        assert_eq!(sut.current.network.id, NetworkID::Mainnet);
        assert_eq!(
            sut.change_current(Gateway::stokenet()),
            ChangeGatewayOutcome::DidChange { is_new: false }
        );
        assert_eq!(sut.current.network.id, NetworkID::Stokenet);
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy! 'other' already contained 'current'"
    )]
    fn change_throw_gateways_discrepancy_other_should_not_contain_current() {
        let mut impossible = SUT {
            current: Gateway::mainnet(),
            other: Gateways::from_iter([Gateway::mainnet()]),
        };
        let _ = impossible.change_current(Gateway::stokenet());
    }

    #[test]
    fn change_current_to_current() {
        let mut sut = SUT::default();
        assert_eq!(sut.current.network.id, NetworkID::Mainnet);
        assert_eq!(
            sut.change_current(Gateway::mainnet()),
            ChangeGatewayOutcome::NoChange
        );
        assert_eq!(sut.current.network.id, NetworkID::Mainnet);
    }

    #[test]
    fn append() {
        let mut sut = SUT::sample();
        assert!(!sut.append(Gateway::mainnet()));
        assert!(!sut.append(Gateway::stokenet()));
        assert_eq!(sut, SUT::sample());
        assert!(sut.append(Gateway::kisharnet()));
        assert_eq!(
            sut,
            SUT::new_with_other(
                Gateway::mainnet(),
                [Gateway::stokenet(), Gateway::kisharnet()]
            )
            .unwrap()
        );
    }

    #[test]
    fn deserialize_from_json_ignore_repetitions() {
        let json = r#"
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
                    },
                    {
                        "network":
                        {
                            "name": "different",
                            "id": 11,
                            "displayDescription": "All differs but Url is the same than stokenet"
                        },
                        "url": "https://babylon-stokenet-gateway.radixdlt.com/"
                    }
                ]
            }
        "#;

        let sut = serde_json::from_str::<SUT>(json).unwrap();

        let mut expected = SUT::new(Gateway::rcnet());
        expected.append(Gateway::mainnet());
        expected.append(Gateway::stokenet());

        assert_eq!(sut, expected);
    }
}
