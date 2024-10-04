use crate::prelude::*;
use sargon::ProfileStateChangeDriver as InternalProfileStateChangeDriver;
use sargon::ProfileState as InternalProfileState;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait ProfileStateChangeDriver: Send + Sync + std::fmt::Debug {
    async fn handle_profile_state_change(
        &self,
        changed_profile_state: ProfileState,
    );
}

#[derive(Debug)]
pub struct ProfileStateChangeDriverAdapter {
    pub wrapped: Arc<dyn ProfileStateChangeDriver>,
}

#[async_trait::async_trait]
impl InternalProfileStateChangeDriver for ProfileStateChangeDriverAdapter {
    async fn handle_profile_state_change(
        &self,
        changed_profile_state: InternalProfileState,
    ) {
            self.wrapped
                .handle_profile_state_change(changed_profile_state.into())
                .await
    }
}