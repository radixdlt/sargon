use crate::prelude::*;

// ==================
// Get Current Gateway/Network
// ==================
#[uniffi::export]
impl SargonOS {
    /// Returns the id of the Network of the current Gateway set in AppPreferences
    /// of the active Profile. This is the canonical value of "current network",
    /// which affects which accounts host clients display to end user and to
    /// which network transactions are submitted, amongst other behaviors.
    pub fn current_network_id(&self) -> Result<NetworkID> {
        self.wrapped.current_network_id().map_result()
    }

    /// The current gateway host client is using, which affects `current_network_id`.
    /// All Network Requests reading from Radix ledger and submission of new
    /// transactions will go the the Radix Network of the current Gateway.
    pub fn current_gateway(&self) -> Result<Gateway> {
        self.wrapped.current_gateway().map_result()
    }

    /// Returns the `gateways` values of the current Profile.
    pub fn gateways(&self) -> Result<SavedGateways> {
        self.wrapped.gateways().map_result()
    }

    /// Returns the `ProfileNetwork` corresponding to the network ID set by the
    /// current gateway.
    pub fn current_network(&self) -> Result<ProfileNetwork> {
        self.wrapped.current_network().map_result()
    }
}

// ==================
// Change Current Gateway
// ==================
#[uniffi::export]
impl SargonOS {
    /// Changes the current Gateway to `to`, if it is not already the current.
    /// Returns the outcome of the change, if we did in fact switch (current != to),
    /// and if we switched then if `to` is new.
    ///
    /// If we did in fact change current, an `EventNotification` is emitted.
    ///
    /// # Emits Event
    /// Emits `Event::GatewayChangedCurrent` if we changed the gateway.
    pub async fn change_current_gateway(
        &self,
        to: Gateway,
    ) -> Result<ChangeGatewayOutcome> {
        self.wrapped
            .change_current_gateway(to.into())
            .await
            .map_result()
    }
}
