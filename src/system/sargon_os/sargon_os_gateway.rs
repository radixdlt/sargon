use crate::prelude::*;

// ==================
// Get Current Gateway/Network
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
// Change Current Gateway
// ==================
#[uniffi::export]
impl SargonOS {
    /// Changes the current Gateway to `to`, if it is not already the current.
    /// Returns the outcome of the change, if we did in fact switch (current != to),
    /// and if we switched then if `to` as new.
    ///
    /// If we did in fact change current, an `EventNotification` is emitted.
    pub async fn change_current_gateway(
        &self,
        to: Gateway,
    ) -> Result<ChangeGatewayOutcome> {
        let outcome = self
            .update_profile_with(|mut p| {
                Ok(p.app_preferences.gateways.change_current(to.clone()))
            })
            .await?;

        match outcome {
            ChangeGatewayOutcome::DidChange { is_new } => {
                self.event_bus
                    .emit(EventNotification::new(
                        Event::GatewayChangedCurrent { to, is_new },
                    ))
                    .await;
            }
            ChangeGatewayOutcome::NoChange => {}
        };
        Ok(outcome)
    }
}
