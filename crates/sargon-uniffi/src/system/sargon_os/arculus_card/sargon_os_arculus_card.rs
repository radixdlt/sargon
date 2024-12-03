use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn arculus_card_read_firmware_version(&self) -> Result<String> {
        self.wrapped.arculus_card_read_firmware_version().await.into_result()
    }
}