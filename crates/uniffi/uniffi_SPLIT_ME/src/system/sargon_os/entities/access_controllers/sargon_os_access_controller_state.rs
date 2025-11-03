use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn fetch_all_access_controllers_details(
        &self,
    ) -> Result<Vec<AccessControllerStateDetails>> {
        self.wrapped
            .fetch_all_access_controllers_details()
            .await
            .into_iter_result()
    }
}
