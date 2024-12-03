use crate::prelude::*;

impl ArculusWalletClient {
    pub(crate) async fn do_card_io<Response, F>(
        &self, 
        command: BagOfBytes, 
        response_map: F
    ) -> Result<Response> 
    where F: FnOnce(BagOfBytes) -> Result<Response>,
    {
        let response = self.nfc_tag_driver.send_receive(command).await?;

        response_map(response)
    }
}

impl ArculusWalletClient {
    pub(crate) async fn select_card_io(&self, wallet: ArculusWalletPointer, aid: BagOfBytes) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.select_wallet_request(wallet, aid)?, 
            |response| self.csdk_driver.select_wallet_response(wallet, response)
        ).await
    }

    pub(crate) async fn create_wallet_seed_io(&self, wallet: ArculusWalletPointer, word_count: u8) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.create_wallet_seed_request(wallet, word_count)?,
            |response| self.csdk_driver.create_wallet_seed_response(wallet, response)
        ).await
    }

    pub(crate) async fn reset_wallet_io(&self, wallet: ArculusWalletPointer) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.reset_wallet_request(wallet)?,
            |response| self.csdk_driver.reset_wallet_response(wallet, response)
        ).await
    }

    pub(crate) async fn init_recover_wallet_io(&self, wallet: ArculusWalletPointer, seed_words_count: u8) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.init_recover_wallet_request(wallet, seed_words_count)?,
            |response| self.csdk_driver.init_recover_wallet_response(wallet, response)
        ).await
    }

    pub(crate) async fn finish_recover_wallet_io(&self, wallet: ArculusWalletPointer, seed: BagOfBytes, seed_length: u8) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.finish_recover_wallet_request(wallet, seed, seed_length)?,
            |response| self.csdk_driver.finish_recover_wallet_response(wallet, response)
        ).await
    }

    pub(crate) async fn get_gguid_io(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.get_gguid_request(wallet)?,
            |response| self.csdk_driver.get_gguid_response(wallet, response)
        ).await
    }

    pub(crate) async fn get_firmware_version_io(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.get_firmware_version_request(wallet)?,
            |response| self.csdk_driver.get_firmware_version_response(wallet, response)
        ).await
    }

    pub(crate) async fn store_pin_io(&self, wallet: ArculusWalletPointer, pin: String, pin_len: u8) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.store_data_pin_request(wallet, pin, pin_len)?,
            |response| self.csdk_driver.store_data_pin_response(wallet, response)
        ).await
    }

    pub(crate) async fn verify_pin_io(&self, wallet: ArculusWalletPointer, pin: String, pin_len: u8) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.verify_pin_request(wallet, pin, pin_len)?,
            |response| self.csdk_driver.verify_pin_response(wallet, response)
        ).await
    }

    pub(crate) async fn init_encrypted_session_io(&self, wallet: ArculusWalletPointer) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.init_encrypted_session_request(wallet)?,
            |response| self.csdk_driver.init_encrypted_session_response(wallet, response)
        ).await
    }

    pub(crate) async fn get_public_key_by_path_io(&self, wallet: ArculusWalletPointer, path: DerivationPath) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.get_public_key_by_path_request(wallet, path)?,
            |response| self.csdk_driver.get_public_key_by_path_response(wallet, response)
        ).await
    }

    pub(crate) async fn sign_hash_path_io(&self, wallet: ArculusWalletPointer, path: DerivationPath, hash: Hash) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.sign_hash_path_request(wallet, path, hash)?,
            |response| self.csdk_driver.sign_hash_path_response(wallet, response)
        ).await
    }
}