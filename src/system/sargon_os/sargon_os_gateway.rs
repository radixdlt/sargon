use crate::prelude::*;

// ==================
// Get Current Cateway/Network
// ==================
#[uniffi::export]
impl SargonOS {
    pub fn current_network_id(&self) -> NetworkID {
        self.profile_holder.current_network_id()
    }

    pub fn current_gateway(&self) -> Gateway {
        self.profile_holder.current_gateway().clone()
    }

    pub fn current_network(&self) -> ProfileNetwork {
        self.profile_holder.current_network().clone()
    }
}

// ==================
// Change Current Cateway
// ==================
#[uniffi::export]
impl SargonOS {
    /// Changes the current Gateway to `to`, if it is not already the current. Returns `Ok(false)` if `to` was already
    /// the `current`, returns `Ok(true)` if `to` was not already `current`.
    pub async fn change_current_gateway(&self, to: Gateway) -> Result<bool> {
        self.update_profile_with(|mut p| {
            p.app_preferences.gateways.change_current(to.clone())
        })
        .await
    }
}
