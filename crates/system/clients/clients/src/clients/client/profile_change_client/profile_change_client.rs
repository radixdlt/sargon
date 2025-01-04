use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ProfileStateChangeClient {
    driver: Arc<dyn ProfileStateChangeDriver>,
}

impl ProfileStateChangeClient {
    pub(crate) fn new(driver: Arc<dyn ProfileStateChangeDriver>) -> Self {
        Self { driver }
    }
}

impl ProfileStateChangeClient {
    pub async fn emit(&self, changed_profile_state: ProfileState) {
        self.driver
            .handle_profile_state_change(changed_profile_state)
            .await
    }
}
