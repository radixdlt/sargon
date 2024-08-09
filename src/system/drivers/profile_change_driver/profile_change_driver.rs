use crate::prelude::*;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait ProfileStateChangeDriver: Send + Sync + std::fmt::Debug {
    async fn handle_profile_state_change(
        &self,
        changed_profile_state: ProfileState,
    );
}
