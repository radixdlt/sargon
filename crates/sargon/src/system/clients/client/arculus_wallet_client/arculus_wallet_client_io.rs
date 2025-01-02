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
            let request = BagOfBytes::from_hex("00a404000a415243554c5553010157").unwrap();
            let nfc_card_response = BagOfBytes::from_hex("6f819d840a415243554c5553010157a5049f6501ff5fc104020207069f33081f003f003f0007039f2304070807089fca0200015f402042bab07235ab1c19638e05403a2e28c33268eb560d078cefc5919a5c0195f02b9f4b47304502210097587f1dd5e23609dd86c2c398de70cd781dd874c6586f8b1f8eb47539554d9c022002d292d2d6a7272b7845f93524e15b053d86bc2fccb11d9d6a5ea5f5f8c759f59000").unwrap();
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
            let request = BagOfBytes::from_hex("805000004104b210690d389ffab381d432df17b034bd6faa456776c9a5c1707e5159d57374037ab8665c40c76917614e0593def07ce25d2756d6fd3e53593f0deb47b6480d8b").unwrap();
            let nfc_card_response = BagOfBytes::from_hex("041b1e7d6fecb10ddadb4610085fdb6d73fc3445ce3321fa6286863c0a36face961910b3b13cdacd93060905571abe0601e57fc64830806838a19a8233ff040a259000").unwrap();
            self.csdk_driver
                .expect_init_encrypted_session_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_card_response.clone());
            self.csdk_driver
                .expect_init_encrypted_session_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_response))
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
            let request = BagOfBytes::from_hex("80ef000010be5d7d3929bb96637a4aeca143e9d00e").unwrap();
            let nfc_response = BagOfBytes::from_hex("24ad24564f18ba954ad9a643f1e948e29000").unwrap();
            self.csdk_driver
                .expect_reset_wallet_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_reset_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }
    
        fn store_pin(
            &mut self,
            pin: String,
        ) -> &mut Self {
            let request = BagOfBytes::from_hex("80ef0000207f2e8ac59205e712da95a800381873bfa970adc416e4d3e07928b0d00a25cb1e").unwrap();
            let nfc_response = BagOfBytes::from_hex("24ad24564f18ba954ad9a643f1e948e29000").unwrap();
            self.csdk_driver
                .expect_store_data_pin_request()
                .with(eq(self.wallet_pointer.clone()), eq(pin.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_store_data_pin_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }

        fn verify_pin(
            &mut self,
            pin: String,
        ) -> &mut Self {
            let request = BagOfBytes::from_hex("80ef0000201a3ee2b393a32d8f53deff8bb32617eedb42a23e2221e31236401799676a8971").unwrap();
            let nfc_response = BagOfBytes::from_hex("24ad24564f18ba954ad9a643f1e948e29000").unwrap();

            self.csdk_driver
                .expect_verify_pin_request()
                .with(eq(self.wallet_pointer.clone()), eq(pin.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_verify_pin_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
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
            let request = BagOfBytes::from_hex("80ef00001057cf5fa28514eabaf0eab603ea53f93a").unwrap();
            let nfc_response = BagOfBytes::from_hex("50d7935ddd31d62e051ee15b238335d3f5bdaecc6470de19eb0ae682480bfb75c987c708cefb8e596ffbe2399eea095b6b29a12f3b662024c89c880f049f0f5e9000").unwrap();
            self.csdk_driver
                .expect_create_wallet_seed_request()
                .with(eq(self.wallet_pointer.clone()), eq(word_count))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_create_wallet_seed_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
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
            let request = BagOfBytes::from_hex("80ef0000106fb9c950cd0c9fe9af4de773c1be2e45").unwrap();
            let nfc_card_reponse = BagOfBytes::from_hex("24ad24564f18ba954ad9a643f1e948e29000").unwrap();

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
            let request = BagOfBytes::from_hex("80ef0000d05428019539de0097469d6cd787a22d2f38cc1b8d93339ba2b6fdfc3bea15697d829d2e6433b4f3c4c24e759c8fbf2a0091cf16caf95d4656123d0255318af24211990bc7a3eaab90f85c366e15f3e597fe9758041b921e64960eef4aa6b0a9deb85d661621f3c3c9e58d1d5d53fef52799c092add2cec2640a06f60977059ff7e16a3d1f1f75f6414c3eb37f158fb98c1977bce36afbc216f8dc696c5a6f34cded8a2c19028d3034690be56fc5857b19eaf8021a55302a8399b445b30a7e35e7a3ec6ad2d4ac4dcebdfd561c9d7e9290").unwrap();
            let nfc_card_reponse = BagOfBytes::from_hex("24ad24564f18ba954ad9a643f1e948e29000").unwrap();
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
            seed_response: BagOfBytes,
        ) -> &mut Self {
            self.csdk_driver
                .expect_seed_phrase_from_mnemonic_sentence()
                .with(eq(self.wallet_pointer.clone()), eq(mnemonic_sentence.clone()), eq(None))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(seed_response));

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
        let pin = "123456".to_string();
        let wallet_pointer = ArculusWalletPointer::sample();
        let word_count = 24;
        let created_mnemonic_sentence = BagOfBytes::from_hex("6578616d706c652070656c6963616e207370656e6420646172696e6720696e6d61746520676c616420746f6e6520636f6c756d6e2064657061727420726f736520736f727420706561722062726965662063656c657279206e6f7465206f6c796d706963206d697373207370617469616c206f626a65637420756e69666f726d207265736f7572636520646973706c617920666973682073686f7000").unwrap();
        let seed_response = BagOfBytes::from_hex("8b10dc86df2ff3d44f8c350fc0a7bf5c7eeb27f74e529d90eea280fb1ea9b62b679328235b6eca5de27c89e2bc655d38385d7c3c0a3543845938ce9f1ee4c69951a1e3652c8b2aac1b013c3971c92acff03f32592d62999f4a96f06a2b1b8e542d9a91103a36f55752e7c2230f04399c1f69907bf814c6938f8a69c6410bc8789a60825588362f333be01b5b60dca6bf7eb998da864a3e86fe4979f5d051baa597d45fef318ae1ad8b68a81645d60eeb12a2b63fc90b8f7506d0e14957e0249a").unwrap();
        let mut stub = ArculusWalletTestStub::new();

        stub
        .initialize_session()
        .reset_wallet()
        .store_pin(pin.clone())
        .create_wallet_seed(word_count, created_mnemonic_sentence.clone())
        .init_recover_wallet(word_count)
        .seed_phrase_from_mnemonic_sentence(created_mnemonic_sentence.clone(), seed_response.clone())
        .finish_recover_wallet(seed_response.clone())
        .verify_pin(pin.clone())
        .end_session();

        
        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.create_wallet_seed(pin, 24).await;
        pretty_assertions::assert_eq!(result, Ok(Mnemonic::sample()))
    }
}
