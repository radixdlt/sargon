use crate::prelude::*;

impl SargonOS {
    pub async fn arculus_card_read_firmware_version(&self) -> Result<String> {
        self.clients.arculus_wallet_client.read_card_firmware_version().await
    }
}