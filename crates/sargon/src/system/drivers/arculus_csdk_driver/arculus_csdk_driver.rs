use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
pub struct ArculusWalletPointer {
    pub pointer: u64
}

impl HasSampleValues for ArculusWalletPointer {
    fn sample() -> Self {
        todo!()
    }

    fn sample_other() -> Self {
        todo!()
    }
}

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