use crate::prelude::*;

impl SargonOS {
    /// Returns the P2P transport profiles values of the current Profile.
    pub fn p2p_transport_profiles(&self) -> Result<SavedP2PTransportProfiles> {
        self.profile_state_holder.access_profile_with(|p| {
            p.app_preferences.p2p_transport_profiles.clone()
        })
    }
}

impl SargonOS {
    /// Adds the P2P transport profile to app preferences.
    /// Returns true if profile was inserted, false if it already existed.
    pub async fn add_p2p_transport_profile(
        &self,
        profile: P2PTransportProfile,
    ) -> Result<bool> {
        self.update_profile_with(|p| {
            Ok(p.app_preferences
                .p2p_transport_profiles
                .append(profile.clone()))
        })
        .await
    }

    /// Updates an existing P2P transport profile matched by signaling server.
    /// Returns true if profile was updated, false if no matching profile existed.
    pub async fn update_p2p_transport_profile(
        &self,
        updated: P2PTransportProfile,
    ) -> Result<bool> {
        self.update_profile_with(|p| {
            let profiles = &mut p.app_preferences.p2p_transport_profiles;
            if profiles.current.id() == updated.id() {
                if profiles.current == updated {
                    return Ok(false);
                }
                profiles.current = updated.clone();
                return Ok(true);
            }

            Ok(profiles.other.update_with(updated.id(), |existing| {
                *existing = updated.clone()
            }))
        })
        .await
    }

    /// Deletes a P2P transport profile from app preferences.
    /// Returns true if profile was deleted, false if it was not found.
    ///
    /// Current profile cannot be deleted through this API.
    pub async fn delete_p2p_transport_profile(
        &self,
        profile: P2PTransportProfile,
    ) -> Result<bool> {
        self.update_profile_with(|p| {
            Ok(p.app_preferences.p2p_transport_profiles.remove(&profile))
        })
        .await
    }

    /// Changes current active P2P transport profile.
    /// Returns true if current profile changed.
    pub async fn change_current_p2p_transport_profile(
        &self,
        to: P2PTransportProfile,
    ) -> Result<bool> {
        self.update_profile_with(|p| {
            Ok(p.app_preferences
                .p2p_transport_profiles
                .change_current(to.clone()))
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    fn custom_profile(url: &str) -> P2PTransportProfile {
        P2PTransportProfile::new(
            "Custom",
            url,
            P2PStunServer::sample(),
            P2PTurnServer::sample(),
        )
    }

    #[actix_rt::test]
    async fn add_p2p_transport_profile_adds_profile() {
        let os = SUT::fast_boot().await;
        let custom = custom_profile("wss://p2p.custom.example/");

        let did_add = os
            .with_timeout(|x| x.add_p2p_transport_profile(custom.clone()))
            .await
            .unwrap();

        assert!(did_add);
        assert!(os
            .p2p_transport_profiles()
            .unwrap()
            .has_signaling_server(&custom.signaling_server));
    }

    #[actix_rt::test]
    async fn update_p2p_transport_profile_updates_existing_profile() {
        let os = SUT::fast_boot().await;
        let mut current = os.p2p_transport_profiles().unwrap().current;
        current.name = "Updated Name".to_owned();

        let did_update = os
            .with_timeout(|x| x.update_p2p_transport_profile(current.clone()))
            .await
            .unwrap();

        assert!(did_update);
        assert_eq!(os.p2p_transport_profiles().unwrap().current, current);
    }

    #[actix_rt::test]
    async fn delete_p2p_transport_profile_deletes_other_profile() {
        let os = SUT::fast_boot().await;
        let custom = custom_profile("wss://p2p.delete.example/");

        os.with_timeout(|x| x.add_p2p_transport_profile(custom.clone()))
            .await
            .unwrap();

        let did_delete = os
            .with_timeout(|x| x.delete_p2p_transport_profile(custom.clone()))
            .await
            .unwrap();

        assert!(did_delete);
        assert!(!os
            .p2p_transport_profiles()
            .unwrap()
            .has_signaling_server(&custom.signaling_server));
    }

    #[actix_rt::test]
    async fn change_current_p2p_transport_profile_changes_current() {
        let os = SUT::fast_boot().await;
        let next = os
            .p2p_transport_profiles()
            .unwrap()
            .other
            .into_iter()
            .next()
            .unwrap();

        let did_change = os
            .with_timeout(|x| {
                x.change_current_p2p_transport_profile(next.clone())
            })
            .await
            .unwrap();

        assert!(did_change);
        assert_eq!(os.p2p_transport_profiles().unwrap().current, next);
    }
}
