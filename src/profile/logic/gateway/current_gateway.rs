use crate::prelude::*;

impl Profile {
    pub fn current_gateway(&self) -> Gateway {
        self.app_preferences.gateways.current.clone()
    }

    pub fn current_network_id(&self) -> NetworkID {
        self.current_gateway().network.id
    }

    pub fn current_network(&self) -> &ProfileNetwork {
        self.networks
            .get_id(self.current_network_id())
            .expect("Should have current network")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum ChangeGatewayOutcome {
    /// If the we did in fact change the gateway, and if the gateway was unknown
    /// or known before it was added, i.e. `is_new` will be true iff the gateway
    /// was unknown before changing to it.
    DidChange { is_new: bool },

    /// We tried to change to the current gateway.
    NoChange,
}

impl SavedGateways {
    /// Changes the current Gateway to `to`, if it is not already the current. If `to` is
    /// not a new Gateway, it will be removed from. Returns `Ok(false)` if `to` was already
    /// the `current`, returns `Ok(true)` if `to` was not already `current`.
    pub fn change_current(&mut self, to: Gateway) -> ChangeGatewayOutcome {
        if self.current == to {
            return ChangeGatewayOutcome::NoChange;
        }
        let old_current = &self.current;
        let was_inserted = self.append(old_current.clone());
        if !was_inserted {
            let msg = "Discrepancy! 'other' already contained 'current'";
            error!("{}", msg);
            panic!("{}", msg);
        }
        let is_new = self.other.remove_id(&to.id()).is_none();
        self.current = to;
        ChangeGatewayOutcome::DidChange { is_new }
    }

    /// Appends `gateway` to the `other` list, without changing the `current` Gateway.
    /// If `other` already contains `gateway` then `(false, other.len())` is returned.
    /// If `other` was new then `(true, index_of_new)` is returned.
    ///
    /// - Returns: `true` if it was added, `false` if it was already present (noop)
    pub fn append(&mut self, gateway: Gateway) -> bool {
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
}
