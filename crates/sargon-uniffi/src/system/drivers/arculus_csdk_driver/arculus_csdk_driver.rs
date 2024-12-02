use crate::prelude::*;
use sargon::ArculusCSDKDriver as InternalArculusCSDKDriver;

#[uniffi::export(with_foreign)]
pub trait ArculusCSDKDriver: Send + Sync + std::fmt::Debug {
    fn wallet_init(&self) -> ArculusWalletPointer;
    fn wallet_free(&self, wallet: ArculusWalletPointer);

    fn select_wallet_request(&self, wallet: ArculusWalletPointer, aid: BagOfBytes) -> Result<BagOfBytes>;
    fn select_wallet_response(&self, wallet: ArculusWalletPointer, respose: BagOfBytes) -> Result<i32>;

    fn create_wallet_seed_request(&self, wallet: ArculusWalletPointer, word_count: u8) -> Result<BagOfBytes>;
    fn create_wallet_seed_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    fn reset_wallet_request(&self, wallet: ArculusWalletPointer,) -> Result<BagOfBytes>;
    fn reset_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn get_gguid_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes>;
    fn get_gguid_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    fn get_firmware_version_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes>;
    fn get_firmware_version_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    fn store_data_pin_request(&self, wallet: ArculusWalletPointer, pin: String, pin_len: u8) -> Result<BagOfBytes>;
    fn store_data_pin_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn verify_pin_request(&self, wallet: ArculusWalletPointer, pin: String, pin_len: u8) -> Result<BagOfBytes>;
    fn verify_pin_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn init_encrypted_session_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes>;
    fn init_encrypted_session_response(&self, wallet: ArculusWalletPointer) -> Result<i32>;

    fn get_public_key_by_path_request(&self, wallet: ArculusWalletPointer, path: DerivationPath) -> Result<BagOfBytes>;
    fn get_public_key_by_path_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    fn sign_hash_path_request(&self, wallet: ArculusWalletPointer, path: DerivationPath, hash: Hash) -> Result<BagOfBytes>;
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

    fn create_wallet_seed_request(&self, wallet: InternalArculusWalletPointer, word_count: u8) -> sargon::Result<sargon::BagOfBytes> {
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

    fn store_data_pin_request(&self, wallet: InternalArculusWalletPointer, pin: String, pin_len: u8) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.store_data_pin_request(wallet.into(), pin, pin_len).into_internal_result()
    }

    fn store_data_pin_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.store_data_pin_response(wallet.into(), response.into()).into_internal_result()
    }

    fn verify_pin_request(&self, wallet: InternalArculusWalletPointer, pin: String, pin_len: u8) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.verify_pin_request(wallet.into(), pin, pin_len).into_internal_result()
    }

    fn verify_pin_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<i32> {
        self.wrapped.verify_pin_response(wallet.into(), response.into()).into_internal_result()
    }

    fn init_encrypted_session_request(&self, wallet: InternalArculusWalletPointer) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.init_encrypted_session_request(wallet.into()).into_internal_result()
    }

    fn init_encrypted_session_response(&self, wallet: InternalArculusWalletPointer) -> sargon::Result<i32> {
        self.wrapped.init_encrypted_session_response(wallet.into()).into_internal_result()
    }

    fn get_public_key_by_path_request(&self, wallet: InternalArculusWalletPointer, path: sargon::DerivationPath) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.get_public_key_by_path_request(wallet.into(), path.into()).into_internal_result()
    }

    fn get_public_key_by_path_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.get_public_key_by_path_response(wallet.into(), response.into()).into_internal_result()
    }

    fn sign_hash_path_request(&self, wallet: InternalArculusWalletPointer, path: sargon::DerivationPath, hash: sargon::Hash) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.sign_hash_path_request(wallet.into(), path.into(), hash.into()).into_internal_result()
    }

    fn sign_hash_path_response(&self, wallet: InternalArculusWalletPointer, response: sargon::BagOfBytes) -> sargon::Result<sargon::BagOfBytes> {
        self.wrapped.sign_hash_path_response(wallet.into(), response.into()).into_internal_result()
    }
}