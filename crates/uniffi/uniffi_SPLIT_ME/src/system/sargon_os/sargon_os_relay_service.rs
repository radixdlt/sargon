use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub fn current_relay_service(&self) -> Result<RelayService> {
        self.wrapped.current_relay_service().into_result()
    }

    pub fn relay_services(&self) -> Result<SavedRelayServices> {
        self.wrapped.relay_services().into_result()
    }
}

#[uniffi::export]
impl SargonOS {
    pub async fn change_current_relay_service(
        &self,
        to: RelayService,
    ) -> Result<bool> {
        self.wrapped
            .change_current_relay_service(to.into())
            .await
            .into_result()
    }
}
