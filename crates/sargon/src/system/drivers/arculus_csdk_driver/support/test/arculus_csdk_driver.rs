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

    fn create_wallet_seed_request(&self, wallet: ArculusWalletPointer, word_count: i64) -> Result<BagOfBytes> {
        todo!()
    }

    fn create_wallet_seed_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn seed_phrase_from_mnemonic_sentence(&self, wallet: ArculusWalletPointer, mnemonic_sentence: BagOfBytes, mnemonic_sentence_len: i64, passphrase: Option<BagOfBytes>, passphrase_len: i64) -> Result<BagOfBytes> {
        todo!()
    }

    fn init_recover_wallet_request(&self, wallet: ArculusWalletPointer, seed_length: i64) -> Result<BagOfBytes> {
        todo!()
    }

    fn init_recover_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn finish_recover_wallet_request(&self, wallet: ArculusWalletPointer, seed: BagOfBytes, seed_length: i64) -> Result<BagOfBytes> {
        todo!()
    }

    fn finish_recover_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn reset_wallet_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes> {
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

    fn store_data_pin_request(&self, wallet: ArculusWalletPointer, pin: String, pin_len: i64) -> Result<BagOfBytes> {
        todo!()
    }

    fn store_data_pin_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn verify_pin_request(&self, wallet: ArculusWalletPointer, pin: String, pin_len: i64) -> Result<BagOfBytes> {
        todo!()
    }

    fn verify_pin_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn init_encrypted_session_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes> {
        todo!()
    }

    fn init_encrypted_session_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32> {
        todo!()
    }

    fn get_public_key_by_path_request(&self, wallet: ArculusWalletPointer, path: BagOfBytes, curve: u16) -> Result<BagOfBytes> {
        todo!()
    }

    fn get_public_key_by_path_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn sign_hash_path_request(&self, wallet: ArculusWalletPointer, path: BagOfBytes, curve: u16, algorithm: u8, hash: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }

    fn sign_hash_path_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes> {
        todo!()
    }
}