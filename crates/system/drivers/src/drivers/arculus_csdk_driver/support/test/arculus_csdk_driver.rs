use crate::prelude::*;

#[derive(Debug)]
pub struct RustArculusCSDKDriver;

impl RustArculusCSDKDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(RustArculusCSDKDriver)
    }
}

impl ArculusCSDKDriver for RustArculusCSDKDriver {
    fn wallet_init(&self) -> Option<ArculusWalletPointer> {
        todo!()
    }

    fn wallet_free(&self, _wallet: ArculusWalletPointer) {
        todo!()
    }

    fn select_wallet_request(
        &self,
        _wallet: ArculusWalletPointer,
        _aid: BagOfBytes,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn select_wallet_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn create_wallet_seed_request(
        &self,
        _wallet: ArculusWalletPointer,
        _seed_phrase_word_count: i64,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn create_wallet_seed_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn seed_phrase_from_mnemonic_sentence(
        &self,
        _wallet: ArculusWalletPointer,
        _mnemonic_sentence: BagOfBytes,
        _passphrase: Option<BagOfBytes>,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn init_recover_wallet_request(
        &self,
        _wallet: ArculusWalletPointer,
        _seed_phrase_word_count: i64,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn init_recover_wallet_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> i32 {
        todo!()
    }

    fn finish_recover_wallet_request(
        &self,
        _wallet: ArculusWalletPointer,
        _seed: BagOfBytes,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn finish_recover_wallet_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> i32 {
        todo!()
    }

    fn reset_wallet_request(
        &self,
        _wallet: ArculusWalletPointer,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn reset_wallet_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> i32 {
        todo!()
    }

    fn get_gguid_request(
        &self,
        _wallet: ArculusWalletPointer,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn get_gguid_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn get_firmware_version_request(
        &self,
        _wallet: ArculusWalletPointer,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn get_firmware_version_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn store_data_pin_request(
        &self,
        _wallet: ArculusWalletPointer,
        _pin: String,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn store_data_pin_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> i32 {
        todo!()
    }

    fn verify_pin_request(
        &self,
        _wallet: ArculusWalletPointer,
        _pin: String,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn verify_pin_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> ArculusVerifyPINResponse {
        todo!()
    }

    fn init_encrypted_session_request(
        &self,
        _wallet: ArculusWalletPointer,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn init_encrypted_session_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> i32 {
        todo!()
    }

    fn get_public_key_by_path_request(
        &self,
        _wallet: ArculusWalletPointer,
        _path: BagOfBytes,
        _curve: u16,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn get_public_key_by_path_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> Option<BagOfBytes> {
        todo!()
    }

    fn sign_hash_path_request(
        &self,
        _wallet: ArculusWalletPointer,
        _path: BagOfBytes,
        _curve: u16,
        _algorithm: u8,
        _hash: BagOfBytes,
    ) -> Option<Vec<BagOfBytes>> {
        todo!()
    }

    fn sign_hash_path_response(
        &self,
        _wallet: ArculusWalletPointer,
        _response: BagOfBytes,
    ) -> Option<BagOfBytes> {
        todo!()
    }
}
