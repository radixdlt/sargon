use crate::prelude::*;

impl SargonOS {
    /// The current Radix Connect relay service host client is using.
    pub fn current_relay_service(&self) -> Result<RelayService> {
        self.profile_state_holder.current_relay_service()
    }

    /// Returns the relay service values of the current Profile.
    pub fn relay_services(&self) -> Result<SavedRelayServices> {
        self.profile_state_holder.relay_services()
    }
}

impl SargonOS {
    /// Changes the current relay service to `to`, if it is not already current.
    /// Returns true if current relay service changed.
    pub async fn change_current_relay_service(
        &self,
        to: RelayService,
    ) -> Result<bool> {
        self.update_profile_with(|p| {
            Ok(p.app_preferences.relay_services.change_current(to.clone()))
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_change_relay_service_relay_services_returns_updated() {
        let os = SUT::fast_boot().await;
        let next = RelayService::sample_other();

        os.with_timeout(|x| x.change_current_relay_service(next.clone()))
            .await
            .unwrap();

        assert_eq!(os.relay_services().unwrap().current, next)
    }

    #[actix_rt::test]
    async fn test_change_relay_service_current_returns_updated() {
        let os = SUT::fast_boot().await;
        let next = RelayService::sample_other();

        os.with_timeout(|x| x.change_current_relay_service(next.clone()))
            .await
            .unwrap();

        assert_eq!(os.current_relay_service().unwrap(), next)
    }
}
