use crate::prelude::*;

#[derive(Debug)]
pub struct ProfileChangeClient {
    driver: Arc<dyn ProfileChangeDriver>,
}

impl ProfileChangeClient {
    pub(crate) fn new(driver: Arc<dyn ProfileChangeDriver>) -> Self {
        Self { driver }
    }
}

impl ProfileChangeClient {
    pub async fn emit(&self, changed_profile: Profile) {
        self.driver.handle_profile_change(changed_profile).await
    }
}
