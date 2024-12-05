use crate::prelude::*;
use sargon::ArculusCSDKDriver as InternalArculusCSDKDriver;

#[uniffi::export(with_foreign)]
pub trait ArculusCSDKDriver: Send + Sync + std::fmt::Debug {
    fn wallet_init(&self) -> ArculusWalletPointer;
    fn wallet_free(&self, wallet: ArculusWalletPointer);

    fn select_wallet_request(&self, wallet: ArculusWalletPointer, aid: BagOfBytes) -> Result<BagOfBytes>;
    fn select_wallet_response(&self, wallet: ArculusWalletPointer, respose: BagOfBytes) -> Result<i32>;

    fn create_wallet_seed_request(&self, wallet: ArculusWalletPointer, word_count: i64) -> Result<BagOfBytes>;
    fn create_wallet_seed_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    fn seed_phrase_from_mnemonic_sentence(&self, wallet: ArculusWalletPointer, mnemonic_sentence: BagOfBytes, mnemonic_sentence_len: i64, passphrase: Option<BagOfBytes>, passphrase_len: i64) -> Result<BagOfBytes>;

    fn init_recover_wallet_request(&self, wallet: ArculusWalletPointer, word_count: i64) -> Result<BagOfBytes>;
    fn init_recover_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn finish_recover_wallet_request(&self, wallet: ArculusWalletPointer, seed: BagOfBytes, seed_length: i64) -> Result<BagOfBytes>;
    fn finish_recover_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn reset_wallet_request(&self, wallet: ArculusWalletPointer,) -> Result<BagOfBytes>;
    fn reset_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn get_gguid_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes>;
    fn get_gguid_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    fn get_firmware_version_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes>;
    fn get_firmware_version_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    fn store_data_pin_request(&self, wallet: ArculusWalletPointer, pin: String, pin_len: i64) -> Result<BagOfBytes>;
    fn store_data_pin_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn verify_pin_request(&self, wallet: ArculusWalletPointer, pin: String, pin_len: i64) -> Result<BagOfBytes>;
    fn verify_pin_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn init_encrypted_session_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes>;
    fn init_encrypted_session_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn get_public_key_by_path_request(&self, wallet: ArculusWalletPointer, path: BagOfBytes, curve: u16) -> Result<BagOfBytes>;
    fn get_public_key_by_path_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    fn sign_hash_path_request(&self, wallet: ArculusWalletPointer, path: BagOfBytes, curve: u16, algorithm: u8, hash: BagOfBytes) -> Result<Vec<BagOfBytes>>;
    fn sign_hash_path_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;
}

#[derive(Debug)]
pub struct ArculusCSDKDriverAdapter {
    pub wrapped: Arc<dyn ArculusCSDKDriver>,
}

use sargon::ArculusWalletPointer as InternalArculusWalletPointer;
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ArculusWalletPointer {
    pub pointer: u64
}

impl InternalArculusCSDKDriver for ArculusCSDKDriverAdapter {
    fn wallet_init(&self) -> InternalArculusWalletPointer {
        self.wrapped.wallet_init().into_internal()
    }

    fn wallet_free(&self, wallet: InternalArculusWalletPointer) {
        self.wrapped.wallet_free(wallet.into());
    }

    fn select_wallet_request(&self, wallet: InternalArculusWalletPointer, aid: sargon::BagOfBytes) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.select_wallet_request(wallet.into(), aid.into()).into_internal_result()
    }

    fn select_wallet_response(&self, wallet: InternalArculusWalletPointer, respose: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.select_wallet_response(wallet.into(), respose.into()).into_internal_result()
    }

    fn create_wallet_seed_request(&self, wallet: InternalArculusWalletPointer, word_count: i64) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.create_wallet_seed_request(wallet.into(), word_count).into_internal_result()
    }

    fn create_wallet_seed_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.create_wallet_seed_response(wallet.into(), response.into()).into_internal_result()
    }

    fn reset_wallet_request(&self, wallet: InternalArculusWalletPointer,) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.reset_wallet_request(wallet.into()).into_internal_result()
    }

    fn reset_wallet_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.reset_wallet_response(wallet.into(), response.into()).into_internal_result()
    }

    fn get_gguid_request(&self, wallet: InternalArculusWalletPointer) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.get_gguid_request(wallet.into()).into_internal_result()
    }

    fn get_gguid_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.get_gguid_response(wallet.into(), response.into()).into_internal_result()
    }

    fn get_firmware_version_request(&self, wallet: InternalArculusWalletPointer) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.get_firmware_version_request(wallet.into()).into_internal_result()
    }

    fn get_firmware_version_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.get_firmware_version_response(wallet.into(), response.into()).into_internal_result()
    }

    fn store_data_pin_request(&self, wallet: InternalArculusWalletPointer, pin: String, pin_len: i64) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.store_data_pin_request(wallet.into(), pin, pin_len).into_internal_result()
    }

    fn store_data_pin_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.store_data_pin_response(wallet.into(), response.into()).into_internal_result()
    }

    fn verify_pin_request(&self, wallet: InternalArculusWalletPointer, pin: String, pin_len: i64) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.verify_pin_request(wallet.into(), pin, pin_len).into_internal_result()
    }

    fn verify_pin_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.verify_pin_response(wallet.into(), response.into()).into_internal_result()
    }

    fn init_encrypted_session_request(&self, wallet: InternalArculusWalletPointer) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.init_encrypted_session_request(wallet.into()).into_internal_result()
    }

    fn init_encrypted_session_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.init_encrypted_session_response(wallet.into(), response.into()).into_internal_result()
    }

    

    fn get_public_key_by_path_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.get_public_key_by_path_response(wallet.into(), response.into()).into_internal_result()
    }

    fn sign_hash_path_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.sign_hash_path_response(wallet.into(), response.into()).into_internal_result()
    }
    
    fn init_recover_wallet_request(&self, wallet: InternalArculusWalletPointer, word_count: i64) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.init_recover_wallet_request(wallet.into(), word_count).into_internal_result()
    }
    
    fn init_recover_wallet_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.init_recover_wallet_response(wallet.into(), response.into()).into_internal_result()
    }
    
    fn finish_recover_wallet_request(&self, wallet: InternalArculusWalletPointer, seed: sargon::BagOfBytes, seed_length: i64) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.finish_recover_wallet_request(wallet.into(), seed.into(), seed_length).into_internal_result()
    }
    
    fn finish_recover_wallet_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.finish_recover_wallet_response(wallet.into(), response.into()).into_internal_result()
    }
    
    fn seed_phrase_from_mnemonic_sentence(&self, wallet: InternalArculusWalletPointer, mnemonic_sentence: sargon::BagOfBytes, mnemonic_sentence_len: i64, passphrase: Option<sargon::BagOfBytes>, passphrase_len: i64) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.seed_phrase_from_mnemonic_sentence(wallet.into(), mnemonic_sentence.into(), mnemonic_sentence_len, passphrase.map(BagOfBytes::from), passphrase_len).into_internal_result()
    }
    
    fn get_public_key_by_path_request(&self, wallet: InternalArculusWalletPointer, path: sargon::BagOfBytes, curve: u16) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.get_public_key_by_path_request(wallet.into(), path.into(), curve).into_internal_result()
    }
    
    fn sign_hash_path_request(&self, wallet: InternalArculusWalletPointer, path: sargon::BagOfBytes, curve: u16, algorithm: u8, hash: sargon::BagOfBytes) -> sargon::Result<Vec<sargon::BagOfBytes>> {
        self.wrapped.sign_hash_path_request(wallet.into(), path.into(), curve, algorithm, hash.into()).into_internal_result()
    }
}