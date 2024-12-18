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
    pub(crate) async fn do_card_io_1<Command, Response, F>(
        &self,
        command: Command,
        response_map: F,
    ) -> Result<Response>
    where
        Command: FnOnce() -> Result<BagOfBytes>,
        F: FnOnce(BagOfBytes) -> Result<Response>,
    {
        let command = command()?;
        let response = self.nfc_tag_driver.send_receive(command).await?;
        response_map(response)
    }

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
    ) -> Result<ArculusWalletCSDKResponseStatus> {
        self.do_card_io(
            self.csdk_driver.select_wallet_request(wallet, aid)?,
            |response| {
                self.csdk_driver
                    .select_wallet_response(wallet, response)?
                    .try_into()
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
    ) -> Result<ArculusWalletCSDKResponseStatus> {
        self.do_card_io(
            self.csdk_driver.reset_wallet_request(wallet)?,
            |response| {
                self.csdk_driver
                    .reset_wallet_response(wallet, response)?
                    .try_into()
            },
        )
        .await
    }

    /// Initializes the wallet recovery process
    pub(crate) async fn init_recover_wallet_io(
        &self,
        wallet: ArculusWalletPointer,
        seed_words_count: i64,
    ) -> Result<ArculusWalletCSDKResponseStatus> {
        self.do_card_io(
            self.csdk_driver
                .init_recover_wallet_request(wallet, seed_words_count)?,
            |response| {
                self.csdk_driver
                    .init_recover_wallet_response(wallet, response)?
                    .try_into()
            },
        )
        .await
    }

    /// Finishes the wallet recovery process
    pub(crate) async fn finish_recover_wallet_io(
        &self,
        wallet: ArculusWalletPointer,
        seed: BagOfBytes,
    ) -> Result<ArculusWalletCSDKResponseStatus> {
        self.do_card_io(
            self.csdk_driver
                .finish_recover_wallet_request(wallet, seed)?,
            |response| {
                self.csdk_driver
                    .finish_recover_wallet_response(wallet, response)?
                    .try_into()
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
        self.do_card_io_1(
            || self.csdk_driver.get_firmware_version_request(wallet),
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
    ) -> Result<ArculusWalletCSDKResponseStatus> {
        self.do_card_io(
            self.csdk_driver.store_data_pin_request(wallet, pin)?,
            |response| {
                self.csdk_driver
                    .store_data_pin_response(wallet, response)?
                    .try_into()
            },
        )
        .await
    }

    /// Verifies the pin on the card
    pub(crate) async fn verify_pin_io(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Result<ArculusWalletCSDKResponseStatus> {
        self.do_card_io(
            self.csdk_driver.verify_pin_request(wallet, pin)?,
            |response| {
                self.csdk_driver
                    .verify_pin_response(wallet, response)?
                    .try_into()
            },
        )
        .await
    }

    /// Initializes the encrypted session on the card
    pub(crate) async fn init_encrypted_session_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<ArculusWalletCSDKResponseStatus> {
        self.do_card_io(
            self.csdk_driver.init_encrypted_session_request(wallet)?,
            |response| {
                self.csdk_driver
                    .init_encrypted_session_response(wallet, response)?
                    .try_into()
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

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use mockall::Sequence;

    use super::*;

    #[actix_rt::test]
    async fn do_card_io_fails_to_build_command() {
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        nfc_tag_driver.expect_send_receive().never();

        let sut = ArculusWalletClient::new(
            Arc::new(MockArculusCSDKDriver::new()),
            Arc::new(nfc_tag_driver),
        );

        let result: Result<()> = sut
            .do_card_io_1(
                || Err(CommonError::Unknown),
                |_| Err(CommonError::Unknown),
            )
            .await;

        pretty_assertions::assert_eq!(result, Err(CommonError::Unknown))
    }

    #[actix_rt::test]
    async fn do_card_io_nfc_send_receive_fails() {
        let command = BagOfBytes::sample();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(command.clone()))
            .once()
            .return_const(Err(CommonError::BytesEmpty));

        let sut = ArculusWalletClient::new(
            Arc::new(MockArculusCSDKDriver::new()),
            Arc::new(nfc_tag_driver),
        );

        let result: Result<()> = sut
            .do_card_io_1(|| Ok(command), |_| Err(CommonError::Unknown))
            .await;

        pretty_assertions::assert_eq!(result, Err(CommonError::BytesEmpty))
    }

    #[actix_rt::test]
    async fn do_card_io_response_map_fails() {
        let command = BagOfBytes::sample();
        let response_bytes = BagOfBytes::sample_other();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(command.clone()))
            .once()
            .return_const(Ok(response_bytes.clone()));

        let sut = ArculusWalletClient::new(
            Arc::new(MockArculusCSDKDriver::new()),
            Arc::new(nfc_tag_driver),
        );

        let result: Result<()> = sut
            .do_card_io_1(
                || Ok(command),
                |bytes| {
                    pretty_assertions::assert_eq!(bytes, response_bytes);
                    Err(CommonError::Unknown)
                },
            )
            .await;

        pretty_assertions::assert_eq!(result, Err(CommonError::Unknown))
    }

    #[actix_rt::test]
    async fn do_card_io_response_map_succeeds() {
        let command = BagOfBytes::sample();
        let response_bytes = BagOfBytes::sample_other();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(command.clone()))
            .once()
            .return_const(Ok(response_bytes.clone()));

        let sut = ArculusWalletClient::new(
            Arc::new(MockArculusCSDKDriver::new()),
            Arc::new(nfc_tag_driver),
        );

        let result: Result<i32> = sut
            .do_card_io_1(
                || Ok(command),
                |bytes| {
                    pretty_assertions::assert_eq!(bytes, response_bytes);
                    Ok(42)
                },
            )
            .await;

        pretty_assertions::assert_eq!(result, Ok(42))
    }

    #[actix_rt::test]
    async fn get_firmware_version_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let firmware_version_request = BagOfBytes::sample();
        let firmware_version_response = BagOfBytes::sample_other();
        let parsed_firmware_version = BagOfBytes::sample_fade();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_get_firmware_version_request()
            .with(eq(wallet_pointer.clone()))
            .once()
            .return_const(Ok(firmware_version_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(firmware_version_request))
            .once()
            .return_const(Ok(firmware_version_response.clone()));

        csdk_driver
            .expect_get_firmware_version_response()
            .with(eq(wallet_pointer.clone()), eq(firmware_version_response))
            .once()
            .return_const(Ok(parsed_firmware_version.clone()));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.get_firmware_version_io(wallet_pointer).await;
        pretty_assertions::assert_eq!(result, Ok(parsed_firmware_version))
    }

    #[actix_rt::test]
    async fn get_gguid_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let get_gguid_request = BagOfBytes::sample();
        let get_gguid_response = BagOfBytes::sample_other();
        let get_gguid_parsed_response = BagOfBytes::sample_fade();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_get_gguid_request()
            .with(eq(wallet_pointer.clone()))
            .once()
            .return_const(Ok(get_gguid_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(get_gguid_request))
            .once()
            .return_const(Ok(get_gguid_response.clone()));

        csdk_driver
            .expect_get_gguid_response()
            .with(eq(wallet_pointer.clone()), eq(get_gguid_response))
            .once()
            .return_const(Ok(get_gguid_parsed_response.clone()));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.get_gguid_io(wallet_pointer).await;
        pretty_assertions::assert_eq!(result, Ok(get_gguid_parsed_response))
    }

    #[actix_rt::test]
    async fn reset_wallet_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let reset_wallet_request = BagOfBytes::sample();
        let reset_wallet_response = BagOfBytes::sample_other();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_reset_wallet_request()
            .with(eq(wallet_pointer.clone()))
            .once()
            .return_const(Ok(reset_wallet_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(reset_wallet_request))
            .once()
            .return_const(Ok(reset_wallet_response.clone()));

        csdk_driver
            .expect_reset_wallet_response()
            .with(eq(wallet_pointer.clone()), eq(reset_wallet_response))
            .once()
            .return_const(Ok(0));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.reset_wallet_io(wallet_pointer).await;
        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusWalletCSDKResponseStatus::Ok)
        )
    }

    #[actix_rt::test]
    async fn create_wallet_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let create_wallet_request = BagOfBytes::sample();
        let create_wallet_response = BagOfBytes::sample_other();
        let create_wallet_parsed_response = BagOfBytes::sample_fade();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_create_wallet_seed_request()
            .with(eq(wallet_pointer.clone()), eq(24))
            .once()
            .return_const(Ok(create_wallet_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(create_wallet_request))
            .once()
            .return_const(Ok(create_wallet_response.clone()));

        csdk_driver
            .expect_create_wallet_seed_response()
            .with(eq(wallet_pointer.clone()), eq(create_wallet_response))
            .once()
            .return_const(Ok(create_wallet_parsed_response.clone()));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.create_wallet_seed_io(wallet_pointer, 24).await;
        pretty_assertions::assert_eq!(result, Ok(create_wallet_parsed_response))
    }

    #[actix_rt::test]
    async fn init_recover_wallet_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let init_recover_wallet_request = BagOfBytes::sample();
        let init_recover_wallet_response = BagOfBytes::sample_other();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_init_recover_wallet_request()
            .with(eq(wallet_pointer.clone()), eq(24))
            .once()
            .return_const(Ok(init_recover_wallet_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(init_recover_wallet_request))
            .once()
            .return_const(Ok(init_recover_wallet_response.clone()));

        csdk_driver
            .expect_init_recover_wallet_response()
            .with(eq(wallet_pointer.clone()), eq(init_recover_wallet_response))
            .once()
            .return_const(Ok(0));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.init_recover_wallet_io(wallet_pointer, 24).await;
        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusWalletCSDKResponseStatus::Ok)
        )
    }

    #[actix_rt::test]
    async fn finish_recover_wallet_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let seed = BagOfBytes::sample_cafe();
        let finish_recover_wallet_request = BagOfBytes::sample();
        let finish_recover_wallet_response = BagOfBytes::sample_other();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_finish_recover_wallet_request()
            .with(eq(wallet_pointer.clone()), eq(seed.clone()))
            .once()
            .return_const(Ok(finish_recover_wallet_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(finish_recover_wallet_request))
            .once()
            .return_const(Ok(finish_recover_wallet_response.clone()));

        csdk_driver
            .expect_finish_recover_wallet_response()
            .with(
                eq(wallet_pointer.clone()),
                eq(finish_recover_wallet_response),
            )
            .once()
            .return_const(Ok(0));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.finish_recover_wallet_io(wallet_pointer, seed).await;
        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusWalletCSDKResponseStatus::Ok)
        )
    }

    #[actix_rt::test]
    async fn select_card_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let aid = BagOfBytes::sample_cafe();
        let select_card_request = BagOfBytes::sample();
        let select_card_response = BagOfBytes::sample_other();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_select_wallet_request()
            .with(eq(wallet_pointer.clone()), eq(aid.clone()))
            .once()
            .return_const(Ok(select_card_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(select_card_request))
            .once()
            .return_const(Ok(select_card_response.clone()));

        csdk_driver
            .expect_select_wallet_response()
            .with(eq(wallet_pointer.clone()), eq(select_card_response))
            .once()
            .return_const(Ok(0));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.select_card_io(wallet_pointer, aid).await;
        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusWalletCSDKResponseStatus::Ok)
        )
    }

    #[actix_rt::test]
    async fn store_pin_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let pin = "1234".to_string();
        let store_pin_request = BagOfBytes::sample();
        let store_pin_response = BagOfBytes::sample_other();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_store_data_pin_request()
            .with(eq(wallet_pointer.clone()), eq(pin.clone()))
            .once()
            .return_const(Ok(store_pin_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(store_pin_request))
            .once()
            .return_const(Ok(store_pin_response.clone()));

        csdk_driver
            .expect_store_data_pin_response()
            .with(eq(wallet_pointer.clone()), eq(store_pin_response))
            .once()
            .return_const(Ok(0));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.store_pin_io(wallet_pointer, pin).await;
        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusWalletCSDKResponseStatus::Ok)
        )
    }

    #[actix_rt::test]
    async fn verify_pin_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let pin = "1234".to_string();
        let verify_pin_request = BagOfBytes::sample();
        let verify_pin_response = BagOfBytes::sample_other();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_verify_pin_request()
            .with(eq(wallet_pointer.clone()), eq(pin.clone()))
            .once()
            .return_const(Ok(verify_pin_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(verify_pin_request))
            .once()
            .return_const(Ok(verify_pin_response.clone()));

        csdk_driver
            .expect_verify_pin_response()
            .with(eq(wallet_pointer.clone()), eq(verify_pin_response))
            .once()
            .return_const(Ok(0));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.verify_pin_io(wallet_pointer, pin).await;
        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusWalletCSDKResponseStatus::Ok)
        )
    }

    #[actix_rt::test]
    async fn init_encrypted_session_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let init_encrypted_session_request = BagOfBytes::sample();
        let init_encrypted_session_response = BagOfBytes::sample_other();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_init_encrypted_session_request()
            .with(eq(wallet_pointer.clone()))
            .once()
            .return_const(Ok(init_encrypted_session_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(init_encrypted_session_request))
            .once()
            .return_const(Ok(init_encrypted_session_response.clone()));

        csdk_driver
            .expect_init_encrypted_session_response()
            .with(
                eq(wallet_pointer.clone()),
                eq(init_encrypted_session_response),
            )
            .once()
            .return_const(Ok(0));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut.init_encrypted_session_io(wallet_pointer).await;
        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusWalletCSDKResponseStatus::Ok)
        )
    }

    #[actix_rt::test]
    async fn get_public_key_by_path_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let path = DerivationPath::sample();
        let path_bytes: BagOfBytes = path.to_string().into_bytes().into();
        let curve = CardCurve::Secp256k1;
        let get_public_key_by_path_request = BagOfBytes::sample();
        let get_public_key_by_path_response = BagOfBytes::sample_other();
        let get_public_key_by_path_parsed = BagOfBytes::sample_cafe();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_get_public_key_by_path_request()
            .with(
                eq(wallet_pointer.clone()),
                eq(path_bytes.clone()),
                eq(curve.val().clone()),
            )
            .once()
            .return_const(Ok(get_public_key_by_path_request.clone()));

        nfc_tag_driver
            .expect_send_receive()
            .with(eq(get_public_key_by_path_request))
            .once()
            .return_const(Ok(get_public_key_by_path_response.clone()));

        csdk_driver
            .expect_get_public_key_by_path_response()
            .with(
                eq(wallet_pointer.clone()),
                eq(get_public_key_by_path_response),
            )
            .once()
            .return_const(Ok(get_public_key_by_path_parsed.clone()));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut
            .get_public_key_by_path_io(wallet_pointer, path, curve)
            .await;
        pretty_assertions::assert_eq!(result, Ok(get_public_key_by_path_parsed))
    }

    #[actix_rt::test]
    async fn sign_hash_path_io() {
        let wallet_pointer = ArculusWalletPointer::sample();
        let path = DerivationPath::sample();
        let path_bytes: BagOfBytes = path.to_string().into_bytes().into();
        let hash = Hash::sample();
        let hash_bytes: BagOfBytes = hash.bytes().into();
        let curve = CardCurve::Secp256k1;
        let algorithm = CardAlgorithm::Ecdsa;
        let sign_hash_path_request = vec![BagOfBytes::sample()];
        let sign_hash_path_response = BagOfBytes::sample_other();
        let sign_hash_path_parsed = BagOfBytes::sample_cafe();

        let mut csdk_driver = MockArculusCSDKDriver::new();
        let mut nfc_tag_driver = MockNFCTagDriver::new();

        csdk_driver
            .expect_sign_hash_path_request()
            .with(
                eq(wallet_pointer.clone()),
                eq(path_bytes),
                eq(curve.val().clone()),
                eq(algorithm.val().clone()),
                eq(hash_bytes),
            )
            .once()
            .return_const(Ok(sign_hash_path_request.clone()));

        nfc_tag_driver
            .expect_send_receive_command_chain()
            .with(eq(sign_hash_path_request))
            .once()
            .return_const(Ok(sign_hash_path_response.clone()));

        csdk_driver
            .expect_sign_hash_path_response()
            .with(eq(wallet_pointer.clone()), eq(sign_hash_path_response))
            .once()
            .return_const(Ok(sign_hash_path_parsed.clone()));

        let sut = ArculusWalletClient::new(
            Arc::new(csdk_driver),
            Arc::new(nfc_tag_driver),
        );

        let result = sut
            .sign_hash_path_io(wallet_pointer, path, hash, curve, algorithm)
            .await;
        pretty_assertions::assert_eq!(result, Ok(sign_hash_path_parsed))
    }

    struct ArculusWalletTestStub {
        csdk_driver: MockArculusCSDKDriver,
        nfc_tag_driver: MockNFCTagDriver,
        wallet_pointer: ArculusWalletPointer,
        sequence: Sequence,
    }

    impl ArculusWalletTestStub {
        fn new() -> Self {
            Self {
                csdk_driver: MockArculusCSDKDriver::new(),
                nfc_tag_driver: MockNFCTagDriver::new(),
                wallet_pointer: ArculusWalletPointer::sample(),
                sequence: Sequence::new(),
            }
        }

        fn select_wallet(
            &mut self,
            wallet_aid: BagOfBytes,
        ) -> &mut Self {
            let request = BagOfBytes::sample();
            let nfc_card_response = BagOfBytes::sample_other();
            self.csdk_driver
                .expect_select_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(wallet_aid))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_card_response.clone());
            self.csdk_driver
                .expect_select_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }
    
        fn init_encrypted_session(
            &mut self,
        ) -> &mut Self {
            self.csdk_driver
                .expect_init_encrypted_session_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));
            self.nfc_send_receive();
            self.csdk_driver
                .expect_init_encrypted_session_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }
    
        fn start_nfc_session(&mut self) -> &mut Self {
            self.nfc_tag_driver
                .expect_start_session()
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(()));

            self
        }

        fn end_nfc_session(&mut self) -> &mut Self {
            self.nfc_tag_driver
                .expect_end_session()
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(());

            self
        }

        fn free_wallet(&mut self) -> &mut Self {
            self.csdk_driver
                .expect_wallet_free()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(());

            self
        }
    
        fn initialize_csdk_wallet(
            &mut self
        ) -> &mut Self {
            self.csdk_driver
                .expect_wallet_init()
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(self.wallet_pointer.clone());

            self
        }
    
        fn initialize_session(
            &mut self
        ) -> &mut Self {
            let aid: Vec<u8> =
            vec![0x41, 0x52, 0x43, 0x55, 0x4C, 0x55, 0x53, 0x01, 0x01, 0x57];
            self.start_nfc_session()
            .initialize_csdk_wallet()
            .select_wallet(aid.into())
            .init_encrypted_session()
        }

        fn end_session(&mut self) -> &mut Self {
            self.end_nfc_session()
            .free_wallet()
        }
    
        fn read_firmware_version(
            &mut self
        ) -> &mut Self {
            self.csdk_driver
                .expect_get_firmware_version_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));
            self.nfc_send_receive();
            self.csdk_driver
                .expect_get_firmware_version_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));

            self
        }
    
        fn reset_wallet(
            &mut self,
        ) -> &mut Self {
            self.csdk_driver
                .expect_reset_wallet_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));
            self.nfc_send_receive();
            self.csdk_driver
                .expect_reset_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }
    
        fn store_pin(
            &mut self,
            pin: String,
        ) -> &mut Self {
            self.csdk_driver
                .expect_store_data_pin_request()
                .with(eq(self.wallet_pointer.clone()), eq(pin.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));
            self.nfc_send_receive();
            self.csdk_driver
                .expect_store_data_pin_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }

        fn verify_pin(
            &mut self,
            pin: String,
        ) -> &mut Self {
            self.csdk_driver
                .expect_verify_pin_request()
                .with(eq(self.wallet_pointer.clone()), eq(pin.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));
            self.nfc_send_receive();
            self.csdk_driver
                .expect_verify_pin_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }
    
        fn create_wallet_seed(
            &mut self,
            word_count: i64,
            seed: BagOfBytes,
        ) -> &mut Self {
            let request = BagOfBytes::sample();
            self.csdk_driver
                .expect_create_wallet_seed_request()
                .with(eq(self.wallet_pointer.clone()), eq(word_count))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive();
            self.csdk_driver
                .expect_create_wallet_seed_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(seed));

            self
        }
    
        fn recover_wallet_seed(
            &mut self,
            seed: BagOfBytes
        ) -> &mut Self {
            self.csdk_driver
                .expect_finish_recover_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(seed.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));
            self.nfc_send_receive();
            self.csdk_driver
                .expect_finish_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }
    
        fn init_recover_wallet(
            &mut self,
            seed_words_count: i64,
        ) -> &mut Self {
            let request = BagOfBytes::sample();
            let nfc_card_reponse = BagOfBytes::sample_other();

            self.csdk_driver
                .expect_init_recover_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(seed_words_count))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_card_reponse.clone());
            self.csdk_driver
                .expect_init_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_reponse))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }

        fn finish_recover_wallet(
            &mut self,
            seed: BagOfBytes
        ) -> &mut Self {
            let request = BagOfBytes::sample();
            let nfc_card_reponse = BagOfBytes::sample_other();
            self.csdk_driver
                .expect_finish_recover_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(seed.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_card_reponse.clone());
            self.csdk_driver
                .expect_finish_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_reponse))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }

        fn seed_phrase_from_mnemonic_sentence(
            &mut self,
            mnemonic_sentence: BagOfBytes,
        ) -> &mut Self {
            self.csdk_driver
                .expect_seed_phrase_from_mnemonic_sentence()
                .with(eq(self.wallet_pointer.clone()), eq(mnemonic_sentence.clone()), eq(None))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));

            self
        }

        fn nfc_send_receive(&mut self, request: BagOfBytes, response: BagOfBytes) {
            self.nfc_tag_driver
            .expect_send_receive()
            .with(eq(request))
            .once()
            .in_sequence(&mut self.sequence)
            .return_const(Ok(response));
        }
    }
    
    #[actix_rt::test]
    async fn create_wallet_seed() {
        let pin = "1234".to_string();
        let wallet_pointer = ArculusWalletPointer::sample();
        let word_count = 24;
        let seed = BagOfBytes::sample();
        let create_wallet_seed_request = BagOfBytes::sample();
        let create_wallet_seed_response = BagOfBytes::sample_other();
        let create_wallet_seed_parsed = BagOfBytes::sample_fade();

        let mut stub = ArculusWalletTestStub::new();

        stub
        .initialize_session()
        .reset_wallet()
        .store_pin(pin.clone())
        .create_wallet_seed(word_count, seed.clone())
        .init_recover_wallet(word_count)
        .seed_phrase_from_mnemonic_sentence(seed.clone())
        .finish_recover_wallet(seed.clone())
        .verify_pin(pin.clone())
        .end_session();

        
        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.create_wallet_seed("1234".to_string(), 24).await;
        pretty_assertions::assert_eq!(result, Ok(Mnemonic::sample()))
    }
}
