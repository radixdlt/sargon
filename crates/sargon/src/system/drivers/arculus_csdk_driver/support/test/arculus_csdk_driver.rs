use crate::prelude::*;

#[derive(Debug)]
pub struct RustArculusCSDKDriver;

impl RustArculusCSDKDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(RustArculusCSDKDriver)
    }
}

impl ArculusCSDKDriver for RustArculusCSDKDriver {
    fn wallet_init(&self) -> ArculusWalletPointer {
        todo!()
    }

    fn wallet_free(&self, wallet: ArculusWalletPointer) {
        todo!()
    }

    fn select_wallet_request(&self, wallet: ArculusWalletPointer, aid: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn select_wallet_response(&self, wallet: ArculusWalletPointer, respose: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn create_wallet_seed_request(&self, wallet: ArculusWalletPointer, word_count: u8) -> Result<BagOfBytes> {
        todo!()
    }

    fn create_wallet_seed_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn reset_wallet_request(&self, wallet: ArculusWalletPointer,) -> Result<BagOfBytes> {
        todo!()
    }

    fn reset_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn get_gguid_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes> {
        todo!()
    }

    fn get_gguid_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn get_firmware_version_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes> {
        todo!()
    }

    fn get_firmware_version_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn store_data_pin_request(&self, wallet: ArculusWalletPointer, pin: String, pin_len: u8) -> Result<BagOfBytes> {
        todo!()
    }

    fn store_data_pin_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn verify_pin_request(&self, wallet: ArculusWalletPointer, pin: String, pin_len: u8) -> Result<BagOfBytes> {
        todo!()
    }

    fn verify_pin_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn init_encrypted_session_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes> {
        todo!()
    }

    fn init_encrypted_session_response(&self, wallet: ArculusWalletPointer) -> Result<i32> {
        todo!()
    }

    fn get_public_key_by_path_request(&self, wallet: ArculusWalletPointer, path: DerivationPath) -> Result<BagOfBytes> {
        todo!()
    }

    fn get_public_key_by_path_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn sign_hash_path_request(&self, wallet: ArculusWalletPointer, path: DerivationPath, hash: Hash) -> Result<BagOfBytes> {
        todo!()
    }

    fn sign_hash_path_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }
}