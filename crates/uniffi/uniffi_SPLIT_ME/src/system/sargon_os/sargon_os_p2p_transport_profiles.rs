use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub fn p2p_transport_profiles(&self) -> Result<SavedP2PTransportProfiles> {
        self.wrapped.p2p_transport_profiles().into_result()
    }
}

#[uniffi::export]
impl SargonOS {
    pub async fn add_p2p_transport_profile(
        &self,
        profile: P2PTransportProfile,
    ) -> Result<bool> {
        self.wrapped
            .add_p2p_transport_profile(profile.into())
            .await
            .into_result()
    }

    pub async fn update_p2p_transport_profile(
        &self,
        updated: P2PTransportProfile,
    ) -> Result<bool> {
        self.wrapped
            .update_p2p_transport_profile(updated.into())
            .await
            .into_result()
    }

    pub async fn delete_p2p_transport_profile(
        &self,
        profile: P2PTransportProfile,
    ) -> Result<bool> {
        self.wrapped
            .delete_p2p_transport_profile(profile.into())
            .await
            .into_result()
    }

    pub async fn change_current_p2p_transport_profile(
        &self,
        to: P2PTransportProfile,
    ) -> Result<bool> {
        self.wrapped
            .change_current_p2p_transport_profile(to.into())
            .await
            .into_result()
    }
}
