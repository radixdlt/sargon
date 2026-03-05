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
    pub async fn add_relay_service(
        &self,
        service: RelayService,
    ) -> Result<bool> {
        self.wrapped
            .add_relay_service(service.into())
            .await
            .into_result()
    }

    pub async fn update_relay_service(
        &self,
        updated: RelayService,
    ) -> Result<bool> {
        self.wrapped
            .update_relay_service(updated.into())
            .await
            .into_result()
    }

    pub async fn delete_relay_service(
        &self,
        service: RelayService,
    ) -> Result<bool> {
        self.wrapped
            .delete_relay_service(service.into())
            .await
            .into_result()
    }

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
