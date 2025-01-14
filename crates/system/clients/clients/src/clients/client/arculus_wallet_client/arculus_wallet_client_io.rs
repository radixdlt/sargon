use crate::prelude::*;

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
/// 3. Parse the NFC response received from the card using the native Arculus CSDK.
impl ArculusWalletClient {
    /// Selects the given wallet on the card
    pub(crate) async fn select_card_io(
        &self,
        wallet: ArculusWalletPointer,
        aid: BagOfBytes,
    ) -> Result<BagOfBytes> {
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
        path: HDPath,
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
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use mockall::Sequence;

    use super::*;

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

        fn select_wallet(&mut self, wallet_aid: BagOfBytes) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_card_response = BagOfBytes::random();
            self.csdk_driver
                .expect_select_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(wallet_aid.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_card_response.clone());
            self.csdk_driver
                .expect_select_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(wallet_aid));

            self
        }

        fn init_encrypted_session(&mut self) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_card_response = BagOfBytes::random();
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

        fn initialize_csdk_wallet(&mut self) -> &mut Self {
            self.csdk_driver
                .expect_wallet_init()
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(self.wallet_pointer.clone());

            self
        }

        fn initialize_session(&mut self) -> &mut Self {
            let aid: Vec<u8> = vec![
                0x41, 0x52, 0x43, 0x55, 0x4C, 0x55, 0x53, 0x01, 0x01, 0x57,
            ];
            self.start_nfc_session()
                .initialize_csdk_wallet()
                .select_wallet(aid.into())
                .init_encrypted_session()
        }

        fn end_session(&mut self) -> &mut Self {
            self.end_nfc_session().free_wallet()
        }

        fn expect_read_card_info(
            &mut self,
            stubbed_card_info: ArculusCardInfo,
            stubbed_factor_source_id_pub_key: PublicKey,
        ) -> &mut Self {
            self.expect_read_firmware_version(
                stubbed_card_info.firmware_version,
            )
            .expect_read_card_gguid(stubbed_card_info.gguid)
            .expect_read_factor_source_id(stubbed_factor_source_id_pub_key)
        }

        fn expect_read_factor_source_id(
            &mut self,
            stubbed_factor_source_id_pub_key: PublicKey,
        ) -> &mut Self {
            self.expect_derive_public_key(
                GetIDPath.to_hd_path(),
                stubbed_factor_source_id_pub_key,
            )
        }

        fn expect_read_firmware_version(
            &mut self,
            stubbed_response: String,
        ) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();

            let stubbed_response_bytes: Vec<u8> = stubbed_response
                .split('.') // Split by '.'
                .map(|s| s.parse::<u8>().unwrap()) // Convert each part to u8
                .collect();

            self.csdk_driver
                .expect_get_firmware_version_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_get_firmware_version_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(stubbed_response_bytes.into()));

            self
        }

        fn expect_read_card_gguid(
            &mut self,
            stubbed_response: Uuid,
        ) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();

            self.csdk_driver
                .expect_get_gguid_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(request.clone()));

            self.nfc_send_receive(request, nfc_response.clone());

            self.csdk_driver
                .expect_get_gguid_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(stubbed_response
                    .into_bytes()
                    .to_vec()
                    .into()));

            self
        }

        fn reset_wallet(&mut self) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();
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

        fn store_pin(&mut self, pin: String) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();
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

        fn verify_pin(&mut self, pin: String) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();

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
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();
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

        fn recover_wallet_seed(&mut self, seed: BagOfBytes) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();

            self.csdk_driver
                .expect_finish_recover_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(seed.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(BagOfBytes::sample()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_finish_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(0));

            self
        }

        fn init_recover_wallet(&mut self, seed_words_count: i64) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_card_reponse = BagOfBytes::random();

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

        fn finish_recover_wallet(&mut self, seed: BagOfBytes) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_card_reponse = BagOfBytes::random();
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
                .with(
                    eq(self.wallet_pointer.clone()),
                    eq(mnemonic_sentence.clone()),
                    eq(None),
                )
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(seed_response));

            self
        }

        fn expect_derive_public_key(
            &mut self,
            expected_derivation_path: HDPath,
            expected_pub_key: PublicKey,
        ) -> &mut Self {
            let stub_card_request = BagOfBytes::random();
            let stub_nfc_response = BagOfBytes::random();

            self.csdk_driver
                .expect_get_public_key_by_path_request()
                .with(
                    eq(self.wallet_pointer.clone()),
                    eq(BagOfBytes::from(
                        expected_derivation_path.to_string().into_bytes(),
                    )),
                    eq(CardCurve::Ed25519Curve.val()),
                )
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(stub_card_request.clone()));

            self.nfc_send_receive(stub_card_request, stub_nfc_response.clone());

            self.csdk_driver
                .expect_get_public_key_by_path_response()
                .with(eq(self.wallet_pointer.clone()), eq(stub_nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(expected_pub_key.to_bytes().into()));
            self
        }

        fn sign_hash(
            &mut self,
            expected_path: HDPath,
            expected_hash: Hash,
            stubbed_signature_response: Ed25519Signature,
        ) -> &mut Self {
            let stub_card_request = vec![BagOfBytes::random()];
            let stub_nfc_response = BagOfBytes::random();

            self.csdk_driver
                .expect_sign_hash_path_request()
                .with(
                    eq(self.wallet_pointer.clone()),
                    eq(BagOfBytes::from(
                        expected_path.to_string().into_bytes(),
                    )),
                    eq(CardCurve::Ed25519Curve.val()),
                    eq(CardAlgorithm::Eddsa.val()),
                    eq(BagOfBytes::from(expected_hash.bytes())),
                )
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(stub_card_request.clone()));

            self.nfc_send_receive_chain(
                stub_card_request,
                stub_nfc_response.clone(),
            );

            self.csdk_driver
                .expect_sign_hash_path_response()
                .with(eq(self.wallet_pointer.clone()), eq(stub_nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(stubbed_signature_response.to_bytes().into()));

            self
        }

        fn expect_to_sign_hash(
            &mut self,
            hash: Hash,
            derivation_path: DerivationPath,
            expected_signature_with_pub_key: SignatureWithPublicKey,
        ) -> &mut Self {
            self.sign_hash(
                derivation_path.to_hd_path(),
                hash,
                expected_signature_with_pub_key
                    .signature()
                    .into_ed25519()
                    .unwrap(),
            )
            .expect_derive_public_key(
                derivation_path.to_hd_path(),
                expected_signature_with_pub_key.public_key(),
            )
        }

        fn nfc_send_receive(
            &mut self,
            request: BagOfBytes,
            response: BagOfBytes,
        ) {
            self.nfc_tag_driver
                .expect_send_receive()
                .with(eq(request))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(response));
        }

        fn nfc_send_receive_chain(
            &mut self,
            request: Vec<BagOfBytes>,
            response: BagOfBytes,
        ) {
            self.nfc_tag_driver
                .expect_send_receive_command_chain()
                .with(eq(request))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(response));
        }
    }

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
    async fn create_wallet_seed() {
        let pin = "123456".to_string();
        let word_count = 24;
        let mnemonic_sentence_create_by_card = BagOfBytes::from_hex("6578616d706c652070656c6963616e207370656e6420646172696e6720696e6d61746520676c616420746f6e6520636f6c756d6e2064657061727420726f736520736f727420706561722062726965662063656c657279206e6f7465206f6c796d706963206d697373207370617469616c206f626a65637420756e69666f726d207265736f7572636520646973706c617920666973682073686f7000").unwrap();
        let seed_from_mnemonic_sentence_created_by_csdk = BagOfBytes::from_hex("8b10dc86df2ff3d44f8c350fc0a7bf5c7eeb27f74e529d90eea280fb1ea9b62b679328235b6eca5de27c89e2bc655d38385d7c3c0a3543845938ce9f1ee4c69951a1e3652c8b2aac1b013c3971c92acff03f32592d62999f4a96f06a2b1b8e542d9a91103a36f55752e7c2230f04399c1f69907bf814c6938f8a69c6410bc8789a60825588362f333be01b5b60dca6bf7eb998da864a3e86fe4979f5d051baa597d45fef318ae1ad8b68a81645d60eeb12a2b63fc90b8f7506d0e14957e0249a").unwrap();
        let mut stub = ArculusWalletTestStub::new();

        stub.initialize_session()
            .reset_wallet()
            .store_pin(pin.clone())
            .create_wallet_seed(
                word_count,
                mnemonic_sentence_create_by_card.clone(),
            )
            .init_recover_wallet(word_count)
            .seed_phrase_from_mnemonic_sentence(
                mnemonic_sentence_create_by_card.clone(),
                seed_from_mnemonic_sentence_created_by_csdk.clone(),
            )
            .finish_recover_wallet(
                seed_from_mnemonic_sentence_created_by_csdk.clone(),
            )
            .verify_pin(pin.clone())
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.create_wallet_seed(pin, 24).await;
        let expected_seed_phrase = "example pelican spend daring inmate glad tone column depart rose sort pear brief celery note olympic miss spatial object uniform resource display fish shop";

        let expected_mnemonic =
            Mnemonic::from(expected_seed_phrase, BIP39Language::English)
                .unwrap();
        pretty_assertions::assert_eq!(result, Ok(expected_mnemonic))
    }

    #[actix_rt::test]
    async fn derive_public_keys() {
        let mut stub = ArculusWalletTestStub::new();
        let path1 = DerivationPath::sample();
        let path2 = DerivationPath::sample_other();
        let pub_key1 = PublicKey::sample_ed25519_alice();
        let pub_key2 = PublicKey::sample_ed25519_bob();

        let paths = IndexSet::from([path1.clone(), path2.clone()]);

        let factor_source_id_pub_key = PublicKey::sample_ed25519();
        let factor_source_id = FactorSourceIDFromHash::from_public_key_bytes(
            FactorSourceKind::ArculusCard,
            factor_source_id_pub_key.to_bytes(),
        );
        let factor_source = ArculusCardFactorSource::new(
            factor_source_id,
            ArculusCardHint::sample(),
        );

        stub.initialize_session()
            .expect_read_factor_source_id(factor_source_id_pub_key)
            .expect_derive_public_key(path1.clone().to_hd_path(), pub_key1)
            .expect_derive_public_key(path2.clone().to_hd_path(), pub_key2)
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.derive_public_keys(factor_source, paths).await;

        let expect_key1 =
            HierarchicalDeterministicPublicKey::new(pub_key1, path1);
        let expect_key2 =
            HierarchicalDeterministicPublicKey::new(pub_key2, path2);
        pretty_assertions::assert_eq!(
            result,
            Ok(IndexSet::from([expect_key1, expect_key2]))
        )
    }

    #[actix_rt::test]
    async fn sign_hashes() {
        let input = IndexMap::from([
            (
                Hash::sample(),
                IndexSet::from([
                    DerivationPath::sample(),
                    DerivationPath::sample_other(),
                ]),
            ),
            (
                Hash::sample_other(),
                IndexSet::from([
                    DerivationPath::sample(),
                    DerivationPath::sample_other(),
                ]),
            ),
        ]);
        let pin = "123456".to_string();
        let expected_signature_with_pub_key_1 =
            SignatureWithPublicKey::Ed25519 {
                public_key: Ed25519PublicKey::sample_aced(),
                signature: Ed25519Signature::sample(),
            };
        let expected_signature_with_pub_key_2 =
            SignatureWithPublicKey::Ed25519 {
                public_key: Ed25519PublicKey::sample_alice(),
                signature: Ed25519Signature::sample_other(),
            };
        let expected_signature_with_pub_key_3 =
            SignatureWithPublicKey::Ed25519 {
                public_key: Ed25519PublicKey::sample_bob(),
                signature: Ed25519Signature::sample_another(),
            };
        let expected_signature_with_pub_key_4 =
            SignatureWithPublicKey::Ed25519 {
                public_key: Ed25519PublicKey::sample_fade(),
                signature: Ed25519Signature::sample_extra(),
            };

        let factor_source_id_pub_key = PublicKey::sample_ed25519();
        let factor_source_id = FactorSourceIDFromHash::new_for_arculus(
            factor_source_id_pub_key.to_bytes(),
        );
        let factor_source = ArculusCardFactorSource::new(
            factor_source_id,
            ArculusCardHint::sample(),
        );

        let mut stub = ArculusWalletTestStub::new();

        stub.initialize_session()
            .expect_read_factor_source_id(factor_source_id_pub_key)
            .verify_pin(pin.clone())
            .expect_to_sign_hash(
                Hash::sample(),
                DerivationPath::sample(),
                expected_signature_with_pub_key_1.clone(),
            )
            .expect_to_sign_hash(
                Hash::sample(),
                DerivationPath::sample_other(),
                expected_signature_with_pub_key_2.clone(),
            )
            .expect_to_sign_hash(
                Hash::sample_other(),
                DerivationPath::sample(),
                expected_signature_with_pub_key_3.clone(),
            )
            .expect_to_sign_hash(
                Hash::sample_other(),
                DerivationPath::sample_other(),
                expected_signature_with_pub_key_4.clone(),
            )
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.sign_hashes(factor_source, pin, input).await;

        let expected_output = IndexMap::from([
            (
                Hash::sample(),
                IndexSet::from([
                    expected_signature_with_pub_key_1.clone(),
                    expected_signature_with_pub_key_2.clone(),
                ]),
            ),
            (
                Hash::sample_other(),
                IndexSet::from([
                    expected_signature_with_pub_key_3.clone(),
                    expected_signature_with_pub_key_4.clone(),
                ]),
            ),
        ]);

        pretty_assertions::assert_eq!(result, Ok(expected_output));
    }

    #[actix_rt::test]
    async fn get_card_info() {
        let mut stub = ArculusWalletTestStub::new();

        let factor_source_id_pub_key = PublicKey::sample_ed25519();
        let factor_source_id = FactorSourceIDFromHash::from_public_key_bytes(
            FactorSourceKind::ArculusCard,
            factor_source_id_pub_key.to_bytes(),
        );
        let card_info = ArculusCardInfo::new(
            "2.2.7.6".to_string(),
            Uuid::sample(),
            Some(factor_source_id),
        );

        stub.initialize_session()
            .expect_read_card_info(card_info.clone(), factor_source_id_pub_key)
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.get_card_info().await;

        pretty_assertions::assert_eq!(result, Ok(card_info))
    }
}
