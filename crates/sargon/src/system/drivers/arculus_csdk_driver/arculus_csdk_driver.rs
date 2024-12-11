use crate::prelude::*;

/// Driver to interact with natice Arculus CSDK library
pub trait ArculusCSDKDriver: Send + Sync + std::fmt::Debug {
    /// Allocates and populates the Wallet on the card. The reference needs to be used for making any requests.
    /// Should be freed by calling `wallet_free` once all operations are performed.
    fn wallet_init(&self) -> ArculusWalletPointer;

    /// Frees memory allocated for wallet. Any usage of the pointer after it is freed will result in undefined behaviour.
    fn wallet_free(&self, wallet: ArculusWalletPointer);

    /// Select the Wallet to be used for the given aid.
    /// Returns the request that needs to be sent to the card.
    fn select_wallet_request(&self, wallet: ArculusWalletPointer, aid: BagOfBytes) -> Result<BagOfBytes>;

    /// 
    fn select_wallet_response(&self, wallet: ArculusWalletPointer, respose: BagOfBytes) -> Result<i32>;

    /// Used to initialize the Hardware wallet.
    /// The word count should be in 12-24 range. Returns the request to be sent to the card.
    fn create_wallet_seed_request(&self, wallet: ArculusWalletPointer, word_count: i64) -> Result<BagOfBytes>;
    
    /// Used to initialize the Hardware Wallet.
    /// Does parse the response received from the card and returns mnemonic words.
    fn create_wallet_seed_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<BagOfBytes>;

    /// Generate Seed From a mnemonic sentence.
    /// Returns the seed bytes to be sent to the card. 
    fn seed_phrase_from_mnemonic_sentence(&self, wallet: ArculusWalletPointer, mnemonic_sentence: BagOfBytes, mnemonic_sentence_len: i64, passphrase: Option<BagOfBytes>, passphrase_len: i64) -> Result<BagOfBytes>;

    /// Request to initialize the wallet recovery.
    /// Returns the response to be sent to the card.
    fn init_recover_wallet_request(&self, wallet: ArculusWalletPointer, seed_length: i64) -> Result<BagOfBytes>;
    
    /// Parse the card response to initialize the wallet recovery
    fn init_recover_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn finish_recover_wallet_request(&self, wallet: ArculusWalletPointer, seed: BagOfBytes, seed_length: i64) -> Result<BagOfBytes>;
    fn finish_recover_wallet_response(&self, wallet: ArculusWalletPointer, response: BagOfBytes) -> Result<i32>;

    fn reset_wallet_request(&self, wallet: ArculusWalletPointer) -> Result<BagOfBytes>;
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::Debug)]
pub struct ArculusWalletPointer {
    pub pointer: u64
}

impl ArculusWalletPointer {
    pub fn new(pointer: u64) -> Self {
        Self {
            pointer
        }
    }
}

impl HasSampleValues for ArculusWalletPointer {
    fn sample() -> Self {
        ArculusWalletPointer::new(0)
    }

    fn sample_other() -> Self {
        ArculusWalletPointer::new(1)
    }
}
