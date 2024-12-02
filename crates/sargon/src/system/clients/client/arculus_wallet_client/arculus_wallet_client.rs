pub use crate::prelude::*;

#[derive(Debug)]
pub struct ArculusCSDKClient {
    /// An object implementing the `NetworkingDriver` traits, which iOS/Android
    /// clients pass into the constructor of this GatewayClient, so that it can
    /// execute network requests.
    pub driver: Arc<dyn ArculusCSDKDriver>,
}

struct ArculusWalletClient {
    pointer: ArculusWalletPointer,
    csdk_client: Arc<ArculusCSDKClient> 
}

impl ArculusWalletClient {
    pub fn new(csdk_client: Arc<ArculusCSDKClient>) -> Self {
        Self {
            pointer: csdk_client.driver.wallet_init(),
            csdk_client: csdk_client
        }
    }
}

impl ArculusWalletClient {
    fn select_wallet_request(&self, aid: BagOfBytes) -> Result<BagOfBytes> {
        self.csdk_client.driver.select_wallet_request(self.pointer.clone(), aid)
    }

    fn select_wallet_response(&self, respose: BagOfBytes) -> Result<i32> {
        self.csdk_client.driver.select_wallet_response(self.pointer.clone(), respose)
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
        self.csdk_client.driver.get_gguid_request(self.pointer.clone())
    }

    fn get_gguid_response(&self, response: BagOfBytes) -> Result<BagOfBytes> {
        self.csdk_client.driver.get_gguid_response(self.pointer.clone(), response)
    }

    fn get_firmware_version_request(&self) -> Result<BagOfBytes> {
        todo!()
    }

    fn get_firmware_version_response(&self, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
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
        self.csdk_client.driver.init_encrypted_session_request(self.pointer.clone())
    }

    fn init_encrypted_session_response(&self) -> Result<i32> {
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