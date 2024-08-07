use crate::prelude::*;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait ProfileChangeDriver: Send + Sync + std::fmt::Debug {
    async fn handle_profile_change(&self, changed_profile: Profile);
}
