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
    /// Adds the relay service to app preferences.
    /// Returns true if service was inserted, false if it already existed.
    pub async fn add_relay_service(
        &self,
        service: RelayService,
    ) -> Result<bool> {
        self.update_profile_with(|p| {
            Ok(p.app_preferences.relay_services.append(service.clone()))
        })
        .await
    }

    /// Updates an existing relay service matched by URL.
    /// Returns true if service was updated, false if no matching service existed.
    pub async fn update_relay_service(
        &self,
        updated: RelayService,
    ) -> Result<bool> {
        self.update_profile_with(|p| {
            let services = &mut p.app_preferences.relay_services;
            if services.current.id() == updated.id() {
                if services.current == updated {
                    return Ok(false);
                }
                services.current = updated.clone();
                return Ok(true);
            }

            Ok(services.other.update_with(updated.id(), |existing| {
                *existing = updated.clone()
            }))
        })
        .await
    }

    /// Deletes a relay service from app preferences.
    /// Returns true if service was deleted, false if it was not found.
    ///
    /// Current service cannot be deleted through this API.
    pub async fn delete_relay_service(
        &self,
        service: RelayService,
    ) -> Result<bool> {
        self.update_profile_with(|p| {
            Ok(p.app_preferences.relay_services.remove(&service))
        })
        .await
    }

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

    #[actix_rt::test]
    async fn test_add_relay_service_adds_service() {
        let os = SUT::fast_boot().await;
        let custom = RelayService::parse(
            "Custom Relay",
            "https://relay.custom.example/api/v1",
        )
        .unwrap();

        let did_add = os
            .with_timeout(|x| x.add_relay_service(custom.clone()))
            .await
            .unwrap();

        assert!(did_add);
        assert!(os.relay_services().unwrap().has_url(custom.id()));
    }

    #[actix_rt::test]
    async fn test_update_relay_service_updates_existing_service() {
        let os = SUT::fast_boot().await;
        let mut current = os.relay_services().unwrap().current;
        current.name = "Updated Relay".to_owned();

        let did_update = os
            .with_timeout(|x| x.update_relay_service(current.clone()))
            .await
            .unwrap();

        assert!(did_update);
        assert_eq!(os.relay_services().unwrap().current, current);
    }

    #[actix_rt::test]
    async fn test_delete_relay_service_deletes_other_service() {
        let os = SUT::fast_boot().await;
        let custom = RelayService::parse(
            "Custom Relay",
            "https://relay.delete.example/api/v1",
        )
        .unwrap();

        os.with_timeout(|x| x.add_relay_service(custom.clone()))
            .await
            .unwrap();

        let did_delete = os
            .with_timeout(|x| x.delete_relay_service(custom.clone()))
            .await
            .unwrap();

        assert!(did_delete);
        assert!(!os.relay_services().unwrap().has_url(custom.id()));
    }
}
