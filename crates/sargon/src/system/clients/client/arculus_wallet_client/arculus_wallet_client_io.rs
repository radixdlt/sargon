use crate::prelude::*;

/// The curves supported by the arculus card
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum CardCurve {
    Secp256k1 = 0x0100,
    Ed25519 = 0x0201,
    Ed25519Blake2bNano = 0x0202,
    Ed25519Curve = 0x0203,
    Nist256p1 = 0x0301,
    Ed25519ExtendedCardano = 0x0401,
    Sr25519 = 0x0501,
}

impl CardCurve {
    pub fn val(&self) -> u16 {
        *self as u16
    }
}

/// The hash algorithms supported by the arculus card
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CardAlgorithm {
    Ecdsa = 1,
    Eddsa = 2,
    Schnorr = 3,
    Ristretto = 4,
    Cardano = 5,
}

impl CardAlgorithm {
    // Returns the raw value of the enum
    pub fn val(&self) -> u8 {
        *self as u8
    }
}

impl ArculusWalletClient {
    pub(crate) async fn do_card_io<Response, F>(
        &self,
        command: BagOfBytes,
        response_map: F,
    ) -> Result<Response>
    where
        F: FnOnce(BagOfBytes) -> Result<Response>,
    {
        let response = self.nfc_tag_driver.send_receive(command).await?;

        response_map(response)
    }

    pub(crate) async fn do_chainned_card_io<Response, F>(
        &self,
        commands: Vec<BagOfBytes>,
        response_map: F,
    ) -> Result<Response>
    where
        F: FnOnce(BagOfBytes) -> Result<Response>,
    {
        let response = self
            .nfc_tag_driver
            .send_receive_command_chain(commands)
            .await?;

        response_map(response)
    }
}

/// All of the IO operations that can be performed on the arculus wallet.
/// Each operations implies the following steps:
/// 1. Create the request through native Arculus CSDK.
/// 2. Send the request to the card through the NFC driver.
/// 3. Parse the response from the card received through the NFC driver by the native Arculus CSDK.
impl ArculusWalletClient {
    /// Selects the given wallet on the card
    pub(crate) async fn select_card_io(
        &self,
        wallet: ArculusWalletPointer,
        aid: BagOfBytes,
    ) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.select_wallet_request(wallet, aid)?,
            |response| {
                self.csdk_driver.select_wallet_response(wallet, response)
            },
        )
        .await
    }

    /// Creates the new wallet seed for the seed phrase word count
    pub(crate) async fn create_wallet_seed_io(
        &self,
        wallet: ArculusWalletPointer,
        word_count: i64,
    ) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver
                .create_wallet_seed_request(wallet, word_count)?,
            |response| {
                self.csdk_driver
                    .create_wallet_seed_response(wallet, response)
            },
        )
        .await
    }

    /// Resets the wallet on the card
    pub(crate) async fn reset_wallet_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.reset_wallet_request(wallet)?,
            |response| self.csdk_driver.reset_wallet_response(wallet, response),
        )
        .await
    }

    /// Initializes the wallet recovery process
    pub(crate) async fn init_recover_wallet_io(
        &self,
        wallet: ArculusWalletPointer,
        seed_words_count: i64,
    ) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver
                .init_recover_wallet_request(wallet, seed_words_count)?,
            |response| {
                self.csdk_driver
                    .init_recover_wallet_response(wallet, response)
            },
        )
        .await
    }

    /// Finishes the wallet recovery process
    pub(crate) async fn finish_recover_wallet_io(
        &self,
        wallet: ArculusWalletPointer,
        seed: BagOfBytes,
    ) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver
                .finish_recover_wallet_request(wallet, seed)?,
            |response| {
                self.csdk_driver
                    .finish_recover_wallet_response(wallet, response)
            },
        )
        .await
    }

    /// Gets the card gguid
    pub(crate) async fn get_gguid_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.get_gguid_request(wallet)?,
            |response| self.csdk_driver.get_gguid_response(wallet, response),
        )
        .await
    }

    /// Gets the card firmware version
    pub(crate) async fn get_firmware_version_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.get_firmware_version_request(wallet)?,
            |response| {
                self.csdk_driver
                    .get_firmware_version_response(wallet, response)
            },
        )
        .await
    }

    /// Stores the pin on the card
    pub(crate) async fn store_pin_io(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.store_data_pin_request(wallet, pin)?,
            |response| {
                self.csdk_driver.store_data_pin_response(wallet, response)
            },
        )
        .await
    }

    /// Verifies the pin on the card
    pub(crate) async fn verify_pin_io(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.verify_pin_request(wallet, pin)?,
            |response| self.csdk_driver.verify_pin_response(wallet, response),
        )
        .await
    }

    /// Initializes the encrypted session on the card
    pub(crate) async fn init_encrypted_session_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<i32> {
        self.do_card_io(
            self.csdk_driver.init_encrypted_session_request(wallet)?,
            |response| {
                self.csdk_driver
                    .init_encrypted_session_response(wallet, response)
            },
        )
        .await
    }

    /// Gets the public key by path on the card
    pub(crate) async fn get_public_key_by_path_io(
        &self,
        wallet: ArculusWalletPointer,
        path: DerivationPath,
        curve: CardCurve,
    ) -> Result<BagOfBytes> {
        self.do_card_io(
            self.csdk_driver.get_public_key_by_path_request(
                wallet,
                path.to_string().into_bytes().into(),
                curve.val(),
            )?,
            |response| {
                self.csdk_driver
                    .get_public_key_by_path_response(wallet, response)
            },
        )
        .await
    }

    /// Signs the hash by path on the card
    pub(crate) async fn sign_hash_path_io(
        &self,
        wallet: ArculusWalletPointer,
        path: DerivationPath,
        hash: Hash,
        curve: CardCurve,
        algorithm: CardAlgorithm,
    ) -> Result<BagOfBytes> {
        self.do_chainned_card_io(
            self.csdk_driver.sign_hash_path_request(
                wallet,
                path.to_string().into_bytes().into(),
                curve.val(),
                algorithm.val(),
                hash.bytes().into(),
            )?,
            |response| {
                self.csdk_driver.sign_hash_path_response(wallet, response)
            },
        )
        .await
    }
}
