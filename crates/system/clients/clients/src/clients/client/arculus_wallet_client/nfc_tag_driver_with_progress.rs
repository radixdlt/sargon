use std::cmp::min;

pub use crate::prelude::*;

#[derive(Debug)]
pub struct NFCTagDriverWithProgressReporting {
    nfc_tag_driver: Arc<dyn NFCTagDriver>,
    number_of_executed_commands: RwLock<u8>,
    number_of_total_commands: RwLock<u8>,
}

impl NFCTagDriverWithProgressReporting {
    pub fn new(nfc_tag_driver: Arc<dyn NFCTagDriver>) -> Self {
        Self {
            nfc_tag_driver,
            number_of_executed_commands: RwLock::new(0),
            number_of_total_commands: RwLock::new(0),
        }
    }

    pub fn set_number_of_total_commands(&self, number_of_total_commands: u8) {
        *self.number_of_total_commands.write().unwrap() =
            number_of_total_commands;
    }

    async fn update_progress(&self) {
        let progress: u8 = {
            let mut number_of_executed_commands =
                self.number_of_executed_commands.write().unwrap();
            *number_of_executed_commands += 1;
            let number_of_total_commands =
                *self.number_of_total_commands.read().unwrap();
            if number_of_total_commands == 0 {
                100
            } else {
                let progress = (*number_of_executed_commands as f32
                    / number_of_total_commands as f32)
                    * 100.0;
                min(progress as u8, 100)
            }
        };

        self.nfc_tag_driver.set_progress(progress).await;
    }
}

#[async_trait::async_trait]
impl NFCTagDriver for NFCTagDriverWithProgressReporting {
    async fn start_session(&self, purpose: NFCTagDriverPurpose) -> Result<()> {
        *self.number_of_executed_commands.write().unwrap() = 0;
        self.nfc_tag_driver.start_session(purpose).await
    }

    async fn end_session(&self, error: Option<CommonError>) {
        if error.is_none() {
            self.nfc_tag_driver.set_progress(100).await;
        }

        self.nfc_tag_driver.end_session(error).await
    }

    async fn send_receive(&self, request: BagOfBytes) -> Result<BagOfBytes> {
        let response = self.nfc_tag_driver.send_receive(request).await?;
        self.update_progress().await;
        Ok(response)
    }

    async fn set_progress(&self, progress: u8) {
        self.nfc_tag_driver.set_progress(progress).await
    }

    async fn send_receive_command_chain(
        &self,
        request: Vec<BagOfBytes>,
    ) -> Result<BagOfBytes> {
        let response = self
            .nfc_tag_driver
            .send_receive_command_chain(request)
            .await?;
        self.update_progress().await;
        Ok(response)
    }
}
