use crate::prelude::*;

#[cfg(any(test, feature = "mock"))]
use mockall::automock;

/// Driver to interact with natice Arculus CSDK library
#[cfg_attr(any(test, feature = "mock"), automock)]
pub trait ArculusCSDKDriver: Send + Sync + std::fmt::Debug {
    /// Initialize Wallet "session" in the Arculus CSDK
    ///
    /// Allocates and populates the Wallet object in the Arculus CSDK and then returns an opaque pointer to the instance.
    /// All of the subsequent calls to Arculus CSDK are made by usingt the pointer.
    /// Should be freed by calling `wallet_free` once all operations are performed.
    ///
    /// Returns the Wallet pointer to be used.
    fn wallet_init(&self) -> ArculusWalletPointer;

    /// Deallocate the Wallet "session" in the Arculus CSDK
    ///
    /// Frees the Wallet object created in Arculus CSDK. Any usage of the pointer after it is freed will result in undefined behaviour.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    fn wallet_free(&self, wallet: ArculusWalletPointer);

    /// Create Select Wallet Request payload.
    ///
    /// Will create the appropriate request to be sent to the Arculus card to select the given card `aid`.
    /// Any chain of operations starts by first selecting the Wallet on the card.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `aid` - The unique wallet identifier to be selected on the card.
    ///
    /// Returns the request that needs to be sent to the card.
    fn select_wallet_request(
        &self,
        wallet: ArculusWalletPointer,
        aid: BagOfBytes,
    ) -> Result<BagOfBytes>;

    /// Select wallet response.
    ///
    /// Will parse the response received from the card to a response status.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the response status for wallet selection.
    fn select_wallet_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<BagOfBytes>;

    /// Create seed phrase request.
    ///
    /// Will create the appropriate request to be sent to the Arculus Card to create a seed phrase for the given word count.
    /// Just sending this request will not create and store the seed phrase, `finish_recover_wallet_request` should be called
    /// with the seed bytes created from the seed phrase received as response to this request
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `seed_phrase_word_count` - The number of mnemonic words to create the wallet. 12 to 24, if set to 0 , default value is 12.
    ///
    /// Returns the request to be sent to the card.
    fn create_wallet_seed_request(
        &self,
        wallet: ArculusWalletPointer,
        seed_phrase_word_count: i64,
    ) -> Result<BagOfBytes>;

    /// Parse the Create Wallet Seed response received from the card.
    ///
    /// This is the response received from the card after sending **create_wallet_seed_request**.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the bytes representation of the mnemonic words non NULL terminated string.
    fn create_wallet_seed_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<BagOfBytes>;

    /// Generate a seed from a mnemonic sentence.
    ///
    /// Use this function a create a Seed from the given mnemonic and passphrase(optional).
    /// It is only meant to be used to be sent as payload to the Arculus card.
    /// In current API, the payload will be used to finish the wallet recovery.
    ///
    /// * `wallet` - Pointer to wallet instance,
    /// * `mnemonic_sentence` - Byte array containing concatenated words, space-separated
    /// * `passphrase` - (Optional, use `None`) Byte array containing the passphrase
    ///
    /// Returns the byte array containing the generated seed.
    fn seed_phrase_from_mnemonic_sentence(
        &self,
        wallet: ArculusWalletPointer,
        mnemonic_sentence: BagOfBytes,
        passphrase: Option<BagOfBytes>,
    ) -> Result<BagOfBytes>;

    /// Init wallet recovery request.
    ///
    /// This request will be sent to the card to start the process of recovery,
    /// which will be followed up by storing a given seed to the card - either generated by the card itself or entered by the user.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `seed_phrase_word_count` - The number of mnemonic words for the seed phrase that will be stored on the card.
    ///
    /// Returns the request to be sent to the card.
    fn init_recover_wallet_request(
        &self,
        wallet: ArculusWalletPointer,
        seed_phrase_word_count: i64,
    ) -> Result<BagOfBytes>;

    /// Init wallet recovery response.
    ///
    /// Will parse the response received from the card to a response status.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the response status for init wallet recovery.
    fn init_recover_wallet_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<i32>;

    /// Finish wallet recovery request.
    ///
    /// Creates the finish wallet recovery request to be sent to the card, which implies storingt the given seed on the card.
    ///
    ///
    /// * `seed` - The seed bytes to be stored on the card. The bytes are created by calling `seed_phrase_from_mnemonic_sentence`.
    ///
    /// Returns the request to be sent to the card.
    fn finish_recover_wallet_request(
        &self,
        wallet: ArculusWalletPointer,
        seed: BagOfBytes,
    ) -> Result<BagOfBytes>;

    /// Finish wallet recovery response.
    ///
    /// Will parse the response received from the card to a response status.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the response status for finish wallet recovery.
    fn finish_recover_wallet_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<i32>;

    /// Reset wallet request
    ///
    /// Creates the reset wallet request to be sent to the card. The Wallet needs to be reset before storing a new seed on it.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    ///
    /// Returns the request to be sent to the card.
    fn reset_wallet_request(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<BagOfBytes>;

    /// Reset wallet response.
    ///
    /// Parse the response received from the card to a response status.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the response status for reset wallet request.
    fn reset_wallet_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<i32>;

    /// Get GGUID request.
    ///
    /// Creates the request for reading the unique global identifier of the card.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    ///
    /// Returns the request to be sent to the card.
    fn get_gguid_request(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<BagOfBytes>;

    /// Get GGUID response.
    ///
    /// Parse the response received from the card to the card GGUID bytes.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the GGUID bytes.
    fn get_gguid_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<BagOfBytes>;

    /// Get firmware version request.
    ///
    /// Creates the request for reading the firmware version of the card.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    ///
    /// Returns the request to be sent to the card.
    fn get_firmware_version_request(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<BagOfBytes>;

    /// Get firmware version response.
    ///
    /// Parse the response received from the card to the firmware version bytes.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the utf8 representation bytes of firmware version.
    fn get_firmware_version_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<BagOfBytes>;

    /// Store Data PIN request.
    ///
    /// Creates the request for storing the PIN on the card.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `pin` - The PIN to be stored on the card.
    ///
    /// Returns the request to be sent to the card.
    fn store_data_pin_request(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Result<BagOfBytes>;

    /// Store Data PIN response.
    ///
    /// Parse the response received from the card to the response status.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the response status for storing the PIN.
    fn store_data_pin_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<i32>;

    /// Verify PIN request.
    ///
    /// Creates the request for verifying the PIN on the card.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `pin` - The PIN to be verified on the card.
    ///
    /// Returns the request to be sent to the card.
    fn verify_pin_request(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Result<BagOfBytes>;

    /// Verify PIN response.
    ///
    /// Parse the response received from the card to the response status.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the response status for verifying the PIN.
    fn verify_pin_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<i32>;

    /// Init encrypted session request.
    ///
    /// Creates the request for initializing the encrypted session with the card.
    /// After selecting the wallet, the ecrpyted session needs to be initialized before any other operation.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    ///
    /// Returns the request to be sent to the card.
    fn init_encrypted_session_request(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<BagOfBytes>;

    /// Init encrypted session response.
    ///
    /// Parse the response received from the card to the response status.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the response status for initializing the encrypted session.
    fn init_encrypted_session_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<i32>;

    /// Get public key by path request.
    ///
    /// Creates the request for getting the public key by the given path and curve.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `path` - The path to derive the public key.
    /// * `curve` - The curve to derive the public key.
    ///
    /// Returns the request to be sent to the card.
    fn get_public_key_by_path_request(
        &self,
        wallet: ArculusWalletPointer,
        path: BagOfBytes,
        curve: u16,
    ) -> Result<BagOfBytes>;

    /// Get public key by path response.
    ///
    /// Parse the response received from the card to the public key bytes.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the public key bytes.
    fn get_public_key_by_path_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<BagOfBytes>;

    /// Sign hash path request.
    ///
    /// Creates the request for signing the hash by the given path, curve and algorithm.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `path` - The path to derive the public key.
    /// * `curve` - The curve to derive the public key.
    /// * `algorithm` - The algorithm to sign the hash.
    /// * `hash` - The hash to be signed.
    ///
    /// Returns the request to be sent to the card.
    fn sign_hash_path_request(
        &self,
        wallet: ArculusWalletPointer,
        path: BagOfBytes,
        curve: u16,
        algorithm: u8,
        hash: BagOfBytes,
    ) -> Result<Vec<BagOfBytes>>;

    /// Sign hash path response.
    ///
    /// Parse the response received from the card to the signature bytes.
    ///
    /// * `wallet` - Pointer to the wallet instance.
    /// * `response` - The response bytes received from the card.
    ///
    /// Returns the signature bytes.
    fn sign_hash_path_response(
        &self,
        wallet: ArculusWalletPointer,
        response: BagOfBytes,
    ) -> Result<BagOfBytes>;
}

/// Pointer to the wallet instance in the Arculus CSDK.
#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::Debug)]
pub struct ArculusWalletPointer {
    /// The pointer bit pattern to the wallet instance.
    /// Using the bit pattern, the opaque pointer will be crafted in the native code.
    pub pointer: u64,
}

impl ArculusWalletPointer {
    pub fn new(pointer: u64) -> Self {
        Self { pointer }
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
