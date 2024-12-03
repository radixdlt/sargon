use radix_engine_interface::freeze_roles;

pub use crate::prelude::*;

#[derive(Debug)]
pub struct ArculusCSDKClient {
    /// An object implementing the `NetworkingDriver` traits, which iOS/Android
    /// clients pass into the constructor of this GatewayClient, so that it can
    /// execute network requests.
    pub driver: Arc<dyn ArculusCSDKDriver>,
}

pub struct ArculusWalletClient {
    pointer: ArculusWalletPointer,
    csdk_driver: Arc<dyn ArculusCSDKDriver>,
    nfc_tag_driver: Arc<dyn NFCTagDriver>
}

impl ArculusWalletClient {
    pub fn new(csdk_driver: Arc<dyn ArculusCSDKDriver>, nfc_tag_driver: Arc<dyn NFCTagDriver>) -> Self {
        Self {
            pointer: csdk_driver.wallet_init(),
            csdk_driver,
            nfc_tag_driver,
        }
    }
}

impl ArculusWalletClient {
    fn select_wallet_request(&self, aid: BagOfBytes) -> Result<BagOfBytes> {
        self.csdk_driver.select_wallet_request(self.pointer.clone(), aid)
    }

    fn select_wallet_response(&self, respose: BagOfBytes) -> Result<i32> {
        self.csdk_driver.select_wallet_response(self.pointer.clone(), respose)
    }

    fn create_wallet_seed_request(&self, word_count: u8) -> Result<BagOfBytes> {
        todo!()
    }

    fn create_wallet_seed_response(&self, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn reset_wallet_request(&self,) -> Result<BagOfBytes> {
        todo!()
    }

    fn reset_wallet_response(&self, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn get_gguid_request(&self) -> Result<BagOfBytes> {
        self.csdk_driver.get_gguid_request(self.pointer.clone())
    }

    fn get_gguid_response(&self, response: BagOfBytes) -> Result<BagOfBytes> {
        self.csdk_driver.get_gguid_response(self.pointer.clone(), response)
    }

    fn get_firmware_version_request(&self) -> Result<BagOfBytes> {
        self.csdk_driver.get_firmware_version_request(self.pointer.clone())
    }

    fn get_firmware_version_response(&self, response: BagOfBytes) -> Result<BagOfBytes> {
        self.csdk_driver.get_firmware_version_response(self.pointer.clone(), response)
    }

    fn store_data_pin_request(&self, pin: String, pin_len: u8) -> Result<BagOfBytes> {
        todo!()
    }

    fn store_data_pin_response(&self, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn verify_pin_request(&self, pin: String, pin_len: u8) -> Result<BagOfBytes> {
        todo!()
    }

    fn verify_pin_response(&self, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn init_encrypted_session_request(&self) -> Result<BagOfBytes> {
        self.csdk_driver.init_encrypted_session_request(self.pointer.clone())
    }

    fn init_encrypted_session_response(&self, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn get_public_key_by_path_request(&self, path: DerivationPath) -> Result<BagOfBytes> {
        todo!()
    }

    fn get_public_key_by_path_response(&self, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn sign_hash_path_request(&self, path: DerivationPath, hash: Hash) -> Result<BagOfBytes> {
        todo!()
    }

    fn sign_hash_path_response(&self, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }
}

impl ArculusWalletClient {
    pub async fn read_card_firmware_version(&self) -> Result<String> {
        self.nfc_tag_driver.start_session().await?;

        let raw_response = self.do_card_io(self.select_wallet_request(BagOfBytes::sample_aced())?).await?;
        let response = self.select_wallet_response(raw_response);
        // parse status
        let raw_encrypted_session_response = self.do_card_io(self.init_encrypted_session_request()?).await?;
        let status_res = self.init_encrypted_session_response(raw_encrypted_session_response)?;

        let firmware_raw_response = self.do_card_io(self.get_firmware_version_request()?).await?;
        let firmware_response = self.get_firmware_version_response(firmware_raw_response)?;

        self.nfc_tag_driver.end_session().await;
        
        Ok(firmware_response.to_hex())
    }
}

// impl ArculusWalletClient {
//     fn select_wallet_io(&self) -> Result<BagOfBytes> {
//         let raw_response = self.do_card_io(self.select_wallet_request(BagOfBytes::sample_aced())?)?;
//         let response = self.select_wallet_response(raw_response)
//         // parse status
//         let raw_encrypted_session_response = self.do_card_io(self.init_encrypted_session_request()?)?;
//         let status_res = self.init_encrypted_session_response(raw_encrypted_session_response)?;

//         let firmware_raw_response = self.do_card_io(self.get_firmware_version_request()?)?;
//         let firmware_response = self.get_firmware_version_response(firmware_raw_response)?;
//         let str = firmware_raw_response.to_hex();

        
//     }
// }

impl ArculusWalletClient {
    async fn do_card_io(&self, command: BagOfBytes) -> Result<BagOfBytes> {
        self.nfc_tag_driver.send_receive(command).await
    }
}