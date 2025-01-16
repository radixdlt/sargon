use crate::prelude::*;
use sargon::ArculusCSDKDriver as InternalArculusCSDKDriver;

#[uniffi::export(with_foreign)]
pub trait ArculusCSDKDriver: Send + Sync + std::fmt::Debug {
    fn wallet_init(&self) -> Option<ArculusWalletPointer>;
    fn wallet_free(&self, wallet: ArculusWalletPointer);

    fn select_wallet_request(
        &self,
        wallet: ArculusWalletPointer,
        aid: BagOfBytes,
    ) -> Option<BagOfBytes>;
    fn select_wallet_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Option<BagOfBytes>;

    fn create_wallet_seed_request(
        &self,
        wallet: ArculusWalletPointer,
        word_count: i64,
    ) -> Option<BagOfBytes>;
    fn create_wallet_seed_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Option<BagOfBytes>;

    fn seed_phrase_from_mnemonic_sentence(
        &self,
        wallet: ArculusWalletPointer,
        mnemonic_sentence: BagOfBytes,
        passphrase: Option<BagOfBytes>,
    ) -> Option<BagOfBytes>;

    fn init_recover_wallet_request(
        &self,
        wallet: ArculusWalletPointer,
        word_count: i64,
    ) -> Option<BagOfBytes>;
    fn init_recover_wallet_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> i32;

    fn finish_recover_wallet_request(
        &self,
        wallet: ArculusWalletPointer,
        seed: BagOfBytes,
    ) -> Option<BagOfBytes>;
    fn finish_recover_wallet_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> i32;

    fn reset_wallet_request(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Option<BagOfBytes>;
    fn reset_wallet_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> i32;

    fn get_gguid_request(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Option<BagOfBytes>;
    fn get_gguid_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Option<BagOfBytes>;

    fn get_firmware_version_request(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Option<BagOfBytes>;
    fn get_firmware_version_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Option<BagOfBytes>;

    fn store_data_pin_request(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Option<BagOfBytes>;
    fn store_data_pin_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> i32;

    fn verify_pin_request(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Option<BagOfBytes>;
    fn verify_pin_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> i32;

    fn init_encrypted_session_request(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Option<BagOfBytes>;
    fn init_encrypted_session_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> i32;

    fn get_public_key_by_path_request(
        &self,
        wallet: ArculusWalletPointer,
        path: BagOfBytes,
        curve: u16,
    ) -> Option<BagOfBytes>;
    fn get_public_key_by_path_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Option<BagOfBytes>;

    fn sign_hash_path_request(
        &self,
        wallet: ArculusWalletPointer,
        path: BagOfBytes,
        curve: u16,
        algorithm: u8,
        hash: BagOfBytes,
    ) -> Option<Vec<BagOfBytes>>;
    fn sign_hash_path_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Option<BagOfBytes>;
}

#[derive(Debug)]
pub struct ArculusCSDKDriverAdapter {
    pub wrapped: Arc<dyn ArculusCSDKDriver>,
}

use sargon::ArculusWalletPointer as InternalArculusWalletPointer;
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ArculusWalletPointer {
    pub pointer: u64,
}

impl InternalArculusCSDKDriver for ArculusCSDKDriverAdapter {
    fn wallet_init(&self) -> Option<InternalArculusWalletPointer> {
        self.wrapped.wallet_init().into_internal()
    }

    fn wallet_free(&self, wallet: InternalArculusWalletPointer) {
        self.wrapped.wallet_free(wallet.into());
    }

    fn select_wallet_request(
        &self,
        wallet: InternalArculusWalletPointer,
        aid: sargon::BagOfBytes,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .select_wallet_request(wallet.into(), aid.into())
            .into_internal()
    }

    fn select_wallet_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .select_wallet_response(wallet.into(), response.into())
            .map(IntoInternal::into_internal)
    }

    fn create_wallet_seed_request(
        &self,
        wallet: InternalArculusWalletPointer,
        word_count: i64,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .create_wallet_seed_request(wallet.into(), word_count)
            .into_internal()
    }

    fn create_wallet_seed_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .create_wallet_seed_response(wallet.into(), response.into())
            .into_internal()
    }

    fn reset_wallet_request(
        &self,
        wallet: InternalArculusWalletPointer,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .reset_wallet_request(wallet.into())
            .into_internal()
    }

    fn reset_wallet_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> i32 {
        self.wrapped
            .reset_wallet_response(wallet.into(), response.into())
    }

    fn get_gguid_request(
        &self,
        wallet: InternalArculusWalletPointer,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .get_gguid_request(wallet.into())
            .into_internal()
    }

    fn get_gguid_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .get_gguid_response(wallet.into(), response.into())
            .into_internal()
    }

    fn get_firmware_version_request(
        &self,
        wallet: InternalArculusWalletPointer,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .get_firmware_version_request(wallet.into())
            .into_internal()
    }

    fn get_firmware_version_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .get_firmware_version_response(wallet.into(), response.into())
            .into_internal()
    }

    fn store_data_pin_request(
        &self,
        wallet: InternalArculusWalletPointer,
        pin: String,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .store_data_pin_request(wallet.into(), pin)
            .into_internal()
    }

    fn store_data_pin_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> i32 {
        self.wrapped
            .store_data_pin_response(wallet.into(), response.into())
    }

    fn verify_pin_request(
        &self,
        wallet: InternalArculusWalletPointer,
        pin: String,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .verify_pin_request(wallet.into(), pin)
            .into_internal()
    }

    fn verify_pin_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> i32 {
        self.wrapped
            .verify_pin_response(wallet.into(), response.into())
    }

    fn init_encrypted_session_request(
        &self,
        wallet: InternalArculusWalletPointer,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .init_encrypted_session_request(wallet.into())
            .into_internal()
    }

    fn init_encrypted_session_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> i32 {
        self.wrapped
            .init_encrypted_session_response(wallet.into(), response.into())
    }

    fn get_public_key_by_path_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .get_public_key_by_path_response(wallet.into(), response.into())
            .into_internal()
    }

    fn sign_hash_path_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .sign_hash_path_response(wallet.into(), response.into())
            .into_internal()
    }

    fn init_recover_wallet_request(
        &self,
        wallet: InternalArculusWalletPointer,
        word_count: i64,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .init_recover_wallet_request(wallet.into(), word_count)
            .into_internal()
    }

    fn init_recover_wallet_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> i32 {
        self.wrapped
            .init_recover_wallet_response(wallet.into(), response.into())
    }

    fn finish_recover_wallet_request(
        &self,
        wallet: InternalArculusWalletPointer,
        seed: sargon::BagOfBytes,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .finish_recover_wallet_request(wallet.into(), seed.into())
            .into_internal()
    }

    fn finish_recover_wallet_response(
        &self,
        wallet: InternalArculusWalletPointer,
        response: sargon::BagOfBytes,
    ) -> i32 {
        self.wrapped
            .finish_recover_wallet_response(wallet.into(), response.into())
    }

    fn seed_phrase_from_mnemonic_sentence(
        &self,
        wallet: InternalArculusWalletPointer,
        mnemonic_sentence: sargon::BagOfBytes,
        passphrase: Option<sargon::BagOfBytes>,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .seed_phrase_from_mnemonic_sentence(
                wallet.into(),
                mnemonic_sentence.into(),
                passphrase.map(BagOfBytes::from),
            )
            .into_internal()
    }

    fn get_public_key_by_path_request(
        &self,
        wallet: InternalArculusWalletPointer,
        path: sargon::BagOfBytes,
        curve: u16,
    ) -> Option<sargon::BagOfBytes> {
        self.wrapped
            .get_public_key_by_path_request(wallet.into(), path.into(), curve)
            .into_internal()
    }

    fn sign_hash_path_request(
        &self,
        wallet: InternalArculusWalletPointer,
        path: sargon::BagOfBytes,
        curve: u16,
        algorithm: u8,
        hash: sargon::BagOfBytes,
    ) -> Option<Vec<sargon::BagOfBytes>> {
        self.wrapped
            .sign_hash_path_request(
                wallet.into(),
                path.into(),
                curve,
                algorithm,
                hash.into(),
            )
            .map(|vec| vec.into_internal())
    }
}
