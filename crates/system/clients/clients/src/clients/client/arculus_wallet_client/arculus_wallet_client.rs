pub use crate::prelude::*;
use std::future::Future;

#[derive(Debug)]
pub struct ArculusWalletClient {
    pub(crate) csdk_driver: Arc<dyn ArculusCSDKDriver>,
    pub(crate) nfc_tag_driver: NFCTagDriverWithProgressReporting,
}

impl ArculusWalletClient {
    pub fn new(
        csdk_driver: Arc<dyn ArculusCSDKDriver>,
        nfc_tag_driver: Arc<dyn NFCTagDriver>,
    ) -> Self {
        Self {
            csdk_driver,
            nfc_tag_driver: NFCTagDriverWithProgressReporting::new(
                nfc_tag_driver,
            ),
        }
    }
}

/// Client public API
impl ArculusWalletClient {
    pub async fn validate_min_firmware_version(
        &self,
    ) -> Result<ArculusMinFirmwareVersionRequirement> {
        let purpose = NFCTagDriverPurpose::Arculus(
            NFCTagArculusInteractonPurpose::IdentifyingCard,
        );

        self.nfc_tag_driver.set_number_of_total_commands(
            Self::number_of_commands_for_purpose(&purpose),
        );

        self.execute_card_operation(purpose, |wallet| {
            self._validate_min_firmware_version(wallet)
        })
        .await
    }

    pub async fn get_configured_factor_source_id(
        &self,
    ) -> Result<Option<FactorSourceIDFromHash>> {
        let purpose = NFCTagDriverPurpose::Arculus(
            NFCTagArculusInteractonPurpose::IdentifyingCard,
        );

        self.nfc_tag_driver.set_number_of_total_commands(
            Self::number_of_commands_for_purpose(&purpose),
        );

        self.execute_card_operation(purpose, |wallet| async move {
            let factor_source_id =
                self._get_factor_source_id(wallet).await.ok();
            Ok(factor_source_id)
        })
        .await
    }

    pub async fn restore_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<()> {
        let purpose = NFCTagDriverPurpose::Arculus(
            NFCTagArculusInteractonPurpose::RestoringCardPin(
                factor_source.clone(),
            ),
        );

        self.nfc_tag_driver.set_number_of_total_commands(
            Self::number_of_commands_for_purpose(&purpose),
        );

        self.execute_card_operation(purpose, |wallet| {
            self._restore_wallet_pin(wallet, factor_source, mnemonic, pin)
        })
        .await?;

        Ok(())
    }

    pub async fn configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        let purpose = NFCTagDriverPurpose::Arculus(
            NFCTagArculusInteractonPurpose::ConfiguringCardMnemonic,
        );

        self.nfc_tag_driver.set_number_of_total_commands(
            Self::number_of_commands_for_purpose(&purpose),
        );

        self.execute_card_operation(
            NFCTagDriverPurpose::Arculus(
                NFCTagArculusInteractonPurpose::ConfiguringCardMnemonic,
            ),
            |wallet| self._restore_wallet_seed(wallet, mnemonic, pin),
        )
        .await
    }

    pub async fn reset_wallet(&self) -> Result<()> {
        self.execute_card_operation(
            NFCTagDriverPurpose::Arculus(
                NFCTagArculusInteractonPurpose::IdentifyingCard,
            ),
            |wallet| self.reset_wallet_io(wallet),
        )
        .await
    }

    pub async fn sign<S: Signable>(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        purpose: NFCTagArculusInteractonPurpose,
        pin: String,
        per_transaction: IndexSet<TransactionSignRequestInput<S>>,
    ) -> Result<IndexSet<HDSignature<S::ID>>> {
        let purpose = NFCTagDriverPurpose::Arculus(purpose);
        let total_number_of_signatures = per_transaction
            .iter()
            .flat_map(|transaction| transaction.signature_inputs())
            .collect::<Vec<_>>()
            .len() as u8;

        self.nfc_tag_driver.set_number_of_total_commands(
            Self::number_of_commands_for_purpose(&purpose)
                + total_number_of_signatures,
        );

        self.execute_card_operation(purpose, |wallet| {
            self._sign(wallet, factor_source_id, pin, per_transaction)
        })
        .await
    }

    pub async fn derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        let purpose = NFCTagDriverPurpose::Arculus(
            NFCTagArculusInteractonPurpose::DerivingPublicKeys(
                factor_source.clone(),
            ),
        );
        self.nfc_tag_driver.set_number_of_total_commands(
            Self::number_of_commands_for_purpose(&purpose) + paths.len() as u8,
        );

        self.execute_card_operation(purpose, |wallet| {
            self._derive_public_keys(wallet, factor_source.id, paths)
        })
        .await
    }

    pub async fn verify_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
    ) -> Result<()> {
        let purpose = NFCTagDriverPurpose::Arculus(
            NFCTagArculusInteractonPurpose::VerifyingPin(factor_source.clone()),
        );
        self.nfc_tag_driver.set_number_of_total_commands(
            Self::number_of_commands_for_purpose(&purpose),
        );

        self.execute_card_operation(purpose, |wallet| {
            self._verify_card_pin(wallet, factor_source.id, pin)
        })
        .await
    }

    pub async fn set_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        old_pin: String,
        new_pin: String,
    ) -> Result<()> {
        let purpose = NFCTagDriverPurpose::Arculus(
            NFCTagArculusInteractonPurpose::ConfiguringCardPin(
                factor_source.clone(),
            ),
        );
        self.nfc_tag_driver.set_number_of_total_commands(
            Self::number_of_commands_for_purpose(&purpose),
        );

        self.execute_card_operation(purpose, |wallet| {
            self._set_card_pin(wallet, factor_source.id, old_pin, new_pin)
        })
        .await
    }
}

impl ArculusWalletClient {
    const AID: [u8; 10] =
        [0x41, 0x52, 0x43, 0x55, 0x4C, 0x55, 0x53, 0x01, 0x01, 0x57];

    async fn execute_card_operation<Response, Op, Fut>(
        &self,
        purpose: NFCTagDriverPurpose,
        op: Op,
    ) -> Result<Response>
    where
        Op: FnOnce(ArculusWalletPointer) -> Fut,
        Fut: Future<Output = Result<Response>>,
    {
        self.nfc_tag_driver.start_session(purpose).await?;
        let wallet = self.start_arculus_wallet_session().await;

        let result = {
            let wallet = wallet?;
            let result = op(wallet).await;
            self.csdk_driver.wallet_free(wallet);
            result
        };

        self.nfc_tag_driver
            .end_session(result.as_ref().err().cloned())
            .await;

        result
    }
}

/// Wallet seed setup
impl ArculusWalletClient {
    async fn _validate_min_firmware_version(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<ArculusMinFirmwareVersionRequirement> {
        let firmware_version = self._get_firmware_version(wallet).await?;
        Ok(ArculusMinFirmwareVersionRequirement::new(firmware_version))
    }

    async fn _restore_wallet_pin(
        &self,
        wallet: ArculusWalletPointer,
        factor_source: ArculusCardFactorSource,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        self.validate_factor_source(wallet, factor_source.id)
            .await?;
        self._restore_wallet_seed(wallet, mnemonic, pin).await
    }

    async fn _restore_wallet_seed(
        &self,
        wallet: ArculusWalletPointer,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        // First reset the Wallet to prepare it for a new seed
        self.reset_wallet_io(wallet).await?;

        // Select the card again after reset
        self.select_wallet(wallet).await?;
        self.store_pin_io(wallet, pin.clone()).await?;

        // Start the recover flow
        self.init_recover_wallet_io(
            wallet,
            mnemonic.word_count.discriminant() as i64,
        )
        .await?;

        // Generate seed bytes that will be stored on the card
        let seed = self.csdk_driver.seed_phrase_from_mnemonic_sentence(
            wallet,
            mnemonic.phrase().as_bytes().into(),
            None,
        ).ok_or(CommonError::ArculusCSDKFailedToCreateSeedPhraseFromMnemonicSentence)?;

        // Store the seed on the Arculus Card
        self.finish_recover_wallet_io(wallet, seed).await?;

        // After seed is configured, do a consistency check for the PIN.
        self.verify_pin_io(wallet, pin).await?;

        // Get the factor source id for the just configured seed
        self._get_factor_source_id(wallet).await
    }
}

/// Pin management
impl ArculusWalletClient {
    async fn _set_card_pin(
        &self,
        wallet: ArculusWalletPointer,
        factor_source_id: FactorSourceIDFromHash,
        old_pin: String,
        new_pin: String,
    ) -> Result<()> {
        self.validate_factor_source(wallet, factor_source_id)
            .await?;
        self._verify_pin(wallet, old_pin).await?;
        self.store_pin_io(wallet, new_pin).await
    }

    async fn _verify_card_pin(
        &self,
        wallet: ArculusWalletPointer,
        factor_source_id: FactorSourceIDFromHash,
        pin: String,
    ) -> Result<()> {
        self.validate_factor_source(wallet, factor_source_id)
            .await?;
        self._verify_pin(wallet, pin).await
    }

    async fn _verify_pin(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Result<()> {
        let response = self.verify_pin_io(wallet, pin).await?;
        let status =
            ArculusWalletCSDKResponseStatus::try_from(response.status)?;

        match status {
            ArculusWalletCSDKResponseStatus::Ok => Ok(()),
            ArculusWalletCSDKResponseStatus::WrongPin => {
                Err(CommonError::ArculusCardWrongPIN {
                    number_of_remaining_tries: response
                        .number_of_tries_remaining,
                })
            }
            _ => status.as_result(),
        }
    }
}

/// Deriving public keys
impl ArculusWalletClient {
    async fn _derive_public_keys(
        &self,
        wallet: ArculusWalletPointer,
        factor_source_id: FactorSourceIDFromHash,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        self.validate_factor_source(wallet, factor_source_id)
            .await?;

        let mut keys = IndexSet::new();
        for path in paths {
            let public_key =
                self.derive_public_key(wallet, path.clone()).await?;
            let key = HierarchicalDeterministicPublicKey::new(
                public_key.into(),
                path,
            );
            keys.insert(HierarchicalDeterministicFactorInstance::new(
                factor_source_id,
                key,
            ));
        }

        Ok(keys)
    }

    async fn derive_public_key(
        &self,
        wallet: ArculusWalletPointer,
        path: DerivationPath,
    ) -> Result<Ed25519PublicKey> {
        let public_key_bytes = self
            .get_public_key_by_path_io(
                wallet,
                path.clone().to_hd_path(),
                CardCurve::Ed25519Curve,
            )
            .await?;
        Ed25519PublicKey::try_from(public_key_bytes.bytes())
    }
}

/// Signing
impl ArculusWalletClient {
    pub async fn _sign<S: Signable>(
        &self,
        wallet: ArculusWalletPointer,
        factor_source_id: FactorSourceIDFromHash,
        pin: String,
        per_transaction: IndexSet<TransactionSignRequestInput<S>>,
    ) -> Result<IndexSet<HDSignature<S::ID>>> {
        self.validate_factor_source(wallet, factor_source_id)
            .await?;
        self._verify_pin(wallet, pin.clone()).await?;

        let mut signatures = IndexSet::new();

        let signature_inputs: Vec<HDSignatureInput<S::ID>> = per_transaction
            .iter()
            .flat_map(|transaction| transaction.signature_inputs())
            .collect();

        for signature_input in signature_inputs {
            let signature = self
                .sign_hash_path(
                    wallet,
                    signature_input.payload_id.clone().into(),
                    signature_input
                        .owned_factor_instance
                        .factor_instance()
                        .derivation_path(),
                )
                .await?;
            let signature = HDSignature::new(signature_input, signature)?;
            signatures.insert(signature);
        }

        Ok(signatures)
    }

    async fn sign_hash_path(
        &self,
        wallet: ArculusWalletPointer,
        hash: Hash,
        path: DerivationPath,
    ) -> Result<SignatureWithPublicKey> {
        let signature = self
            .sign_hash_path_io(
                wallet,
                path.to_hd_path().clone(),
                hash,
                CardCurve::Ed25519Curve,
                CardAlgorithm::Eddsa,
            )
            .await?;
        let public_key = self.derive_public_key(wallet, path).await?;

        Ok(SignatureWithPublicKey::Ed25519 {
            public_key,
            signature,
        })
    }
}

impl ArculusWalletClient {
    async fn start_arculus_wallet_session(
        &self,
    ) -> Result<ArculusWalletPointer> {
        let wallet = self
            .csdk_driver
            .wallet_init()
            .ok_or(CommonError::ArculusCSDKFailedToInitWallet)?;
        self.select_wallet(wallet).await?;
        self.init_encrypted_session_io(wallet).await?;
        Ok(wallet)
    }

    async fn select_wallet(&self, wallet: ArculusWalletPointer) -> Result<()> {
        let card_aid_to_select =
            BagOfBytes::from(ArculusWalletClient::AID.to_vec());
        let card_aid = self
            .select_card_io(wallet, card_aid_to_select.clone())
            .await?;

        if card_aid != card_aid_to_select {
            self.select_card_io(wallet, card_aid).await?;
        }

        Ok(())
    }

    async fn validate_factor_source(
        &self,
        wallet: ArculusWalletPointer,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Result<()> {
        let card_fs_id = self._get_factor_source_id(wallet).await?;
        if card_fs_id != factor_source_id {
            return Err(CommonError::WrongArculusCard);
        }
        Ok(())
    }

    async fn _get_firmware_version(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<String> {
        let version = self.get_firmware_version_io(wallet).await?;
        Ok(version
            .bytes()
            .iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join("."))
    }

    async fn _get_factor_source_id(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<FactorSourceIDFromHash> {
        let public_key_bytes = self
            .get_public_key_by_path_io(
                wallet,
                GetIDPath.to_hd_path(),
                CardCurve::Ed25519Curve,
            )
            .await?;

        Ok(FactorSourceIDFromHash::new_for_arculus(
            public_key_bytes.to_vec(),
        ))
    }
}

impl ArculusWalletClient {
    fn session_start_number_of_commands() -> u8 {
        2 // select wallet + init encrypted session
    }

    fn number_of_commands_for_purpose(purpose: &NFCTagDriverPurpose) -> u8 {
        match purpose {
            NFCTagDriverPurpose::Arculus(purpose) => match purpose {
                NFCTagArculusInteractonPurpose::IdentifyingCard => Self::session_start_number_of_commands() + 1, // read firmware version
                NFCTagArculusInteractonPurpose::RestoringCardPin(_) => Self::number_of_commands_for_purpose(
                    &NFCTagDriverPurpose::Arculus(
                        NFCTagArculusInteractonPurpose::ConfiguringCardMnemonic,
                    ),
                ) + 1, // derive factor source id
                NFCTagArculusInteractonPurpose::ConfiguringCardMnemonic => Self::session_start_number_of_commands() + 7,
                NFCTagArculusInteractonPurpose::SignTransaction(_) => Self::session_start_number_of_commands() + 2, // derive factor source id + verify pin
                NFCTagArculusInteractonPurpose::SignPreAuth(_) => Self::session_start_number_of_commands() + 2, // derive factor source id + verify pin
                NFCTagArculusInteractonPurpose::ProveOwnership(_) => Self::session_start_number_of_commands() + 2, // derive factor source id + verify pin
                NFCTagArculusInteractonPurpose::DerivingPublicKeys(_) => Self::session_start_number_of_commands() + 1, // derive factor source id
                NFCTagArculusInteractonPurpose::VerifyingPin(_) => Self::session_start_number_of_commands() + 2, // derive factor source id + read pin
                NFCTagArculusInteractonPurpose::ConfiguringCardPin(_) => Self::session_start_number_of_commands() + 3, // derive factor source id + read pin + set pin
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::{always, eq};
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

        fn select_wallet(&mut self) -> &mut Self {
            let card_aid_to_select =
                BagOfBytes::from(ArculusWalletClient::AID.to_vec());

            let request = BagOfBytes::random();
            let nfc_card_response = BagOfBytes::random();
            self.csdk_driver
                .expect_select_wallet_request()
                .with(
                    eq(self.wallet_pointer.clone()),
                    eq(card_aid_to_select.clone()),
                )
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_card_response.clone());
            self.csdk_driver
                .expect_select_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(card_aid_to_select));

            self
        }

        fn set_progress(&mut self) -> &mut Self {
            self.nfc_tag_driver
                .expect_set_progress()
                .return_const(()) // always succeed
                .times(..);

            self
        }

        fn init_encrypted_session(&mut self) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();
            self.csdk_driver
                .expect_init_encrypted_session_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_init_encrypted_session_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(ArculusWalletCSDKResponseStatus::Ok as i32);

            self
        }

        fn start_nfc_session(&mut self) -> &mut Self {
            self.nfc_tag_driver
                .expect_start_session()
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Ok(()));

            self.set_progress();

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
            self.start_nfc_session()
                .initialize_csdk_wallet()
                .select_wallet()
                .init_encrypted_session()
        }

        fn end_session(&mut self) -> &mut Self {
            self.free_wallet().end_nfc_session()
        }

        fn expect_read_factor_source_id(
            &mut self,
            stubbed_factor_source_id_pub_key: Option<PublicKey>,
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
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_get_firmware_version_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(stubbed_response_bytes.into()));

            self
        }

        #[allow(dead_code)]
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
                .return_const(Some(request.clone()));

            self.nfc_send_receive(request, nfc_response.clone());

            self.csdk_driver
                .expect_get_gguid_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(
                    stubbed_response.into_bytes().to_vec().into(),
                ));

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
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_reset_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(ArculusWalletCSDKResponseStatus::Ok as i32);

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
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_store_data_pin_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(ArculusWalletCSDKResponseStatus::Ok as i32);

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
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_verify_pin_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(ArculusVerifyPINResponse::new(
                    ArculusWalletCSDKResponseStatus::Ok as i32,
                    3,
                ));

            self
        }

        #[allow(dead_code)]
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
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_create_wallet_seed_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(seed));

            self
        }

        #[allow(dead_code)]
        fn recover_wallet_seed(&mut self, seed: BagOfBytes) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_response = BagOfBytes::random();

            self.csdk_driver
                .expect_finish_recover_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(seed.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_response.clone());
            self.csdk_driver
                .expect_finish_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(BagOfBytes::sample()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(ArculusWalletCSDKResponseStatus::Ok as i32);

            self
        }

        fn init_recover_wallet(&mut self, seed_words_count: i64) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_card_response = BagOfBytes::random();

            self.csdk_driver
                .expect_init_recover_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(seed_words_count))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_card_response.clone());
            self.csdk_driver
                .expect_init_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(ArculusWalletCSDKResponseStatus::Ok as i32);

            self
        }

        fn finish_recover_wallet(&mut self, seed: BagOfBytes) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_card_response = BagOfBytes::random();
            self.csdk_driver
                .expect_finish_recover_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(seed.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_card_response.clone());
            self.csdk_driver
                .expect_finish_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(ArculusWalletCSDKResponseStatus::Ok as i32);

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
                .return_const(Some(seed_response));

            self
        }

        fn expect_derive_public_key(
            &mut self,
            expected_derivation_path: HDPath,
            expected_pub_key: Option<PublicKey>,
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
                .return_const(Some(stub_card_request.clone()));

            self.nfc_send_receive(stub_card_request, stub_nfc_response.clone());

            self.csdk_driver
                .expect_get_public_key_by_path_response()
                .with(eq(self.wallet_pointer.clone()), eq(stub_nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(
                    expected_pub_key.map(|key| key.to_bytes().into()),
                );
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
                .return_const(Some(stub_card_request.clone()));

            self.nfc_send_receive_chain(
                stub_card_request,
                stub_nfc_response.clone(),
            );

            self.csdk_driver
                .expect_sign_hash_path_response()
                .with(eq(self.wallet_pointer.clone()), eq(stub_nfc_response))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(
                    stubbed_signature_response.to_bytes().into(),
                ));

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
                Some(expected_signature_with_pub_key.public_key()),
            );

            self
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
    async fn validate_min_firmware_version() {
        let mut stub = ArculusWalletTestStub::new();
        let firmware_version = "2.2.7.6".to_string();

        stub.initialize_session()
            .expect_read_firmware_version(firmware_version.clone())
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.validate_min_firmware_version().await;

        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusMinFirmwareVersionRequirement::new(firmware_version))
        );
    }

    #[actix_rt::test]
    async fn configure_card_with_mnemonic() {
        let mut stub = ArculusWalletTestStub::new();
        let mnemonic = Mnemonic::sample();
        let pin = "123456".to_string();
        let seed_from_mnemonic_sentence_created_by_csdk = BagOfBytes::from_hex("8b10dc86df2ff3d44f8c350fc0a7bf5c7eeb27f74e529d90eea280fb1ea9b62b679328235b6eca5de27c89e2bc655d38385d7c3c0a3543845938ce9f1ee4c69951a1e3652c8b2aac1b013c3971c92acff03f32592d62999f4a96f06a2b1b8e542d9a91103a36f55752e7c2230f04399c1f69907bf814c6938f8a69c6410bc8789a60825588362f333be01b5b60dca6bf7eb998da864a3e86fe4979f5d051baa597d45fef318ae1ad8b68a81645d60eeb12a2b63fc90b8f7506d0e14957e0249a").unwrap();

        let factor_source_id_pub_key = PublicKey::sample_ed25519();
        let factor_source_id = FactorSourceIDFromHash::new_for_arculus(
            factor_source_id_pub_key.to_bytes(),
        );
        stub.initialize_session()
            .reset_wallet()
            .select_wallet()
            .store_pin(pin.clone())
            .init_recover_wallet(mnemonic.word_count.discriminant() as i64)
            .seed_phrase_from_mnemonic_sentence(
                mnemonic.phrase().as_bytes().into(),
                seed_from_mnemonic_sentence_created_by_csdk.clone(),
            )
            .finish_recover_wallet(
                seed_from_mnemonic_sentence_created_by_csdk.clone(),
            )
            .verify_pin(pin.clone())
            .expect_read_factor_source_id(Some(factor_source_id_pub_key))
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut
            .configure_card_with_mnemonic(mnemonic.clone(), pin.clone())
            .await;

        pretty_assertions::assert_eq!(result, Ok(factor_source_id))
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
        let factor_source_id = FactorSourceIDFromHash::new_for_arculus(
            factor_source_id_pub_key.to_bytes(),
        );
        let factor_source = ArculusCardFactorSource::new(
            factor_source_id.clone(),
            ArculusCardHint::sample(),
        );

        stub.initialize_session()
            .expect_read_factor_source_id(Some(factor_source_id_pub_key))
            .expect_derive_public_key(
                path1.clone().to_hd_path(),
                Some(pub_key1),
            )
            .expect_derive_public_key(
                path2.clone().to_hd_path(),
                Some(pub_key2),
            )
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.derive_public_keys(factor_source, paths).await;

        let expect_key1 = HierarchicalDeterministicFactorInstance::new(
            factor_source_id.clone(),
            HierarchicalDeterministicPublicKey::new(pub_key1, path1),
        );
        let expect_key2 = HierarchicalDeterministicFactorInstance::new(
            factor_source_id.clone(),
            HierarchicalDeterministicPublicKey::new(pub_key2, path2),
        );
        pretty_assertions::assert_eq!(
            result,
            Ok(IndexSet::from([expect_key1, expect_key2]))
        )
    }

    #[actix_rt::test]
    async fn sign_hashes() {
        let transaction_intent = TransactionIntent::sample();
        let pk1 = Ed25519PrivateKey::sample();
        let pk2 = Ed25519PrivateKey::sample_other();

        let expected_signature_with_pub_key_1 =
            SignatureWithPublicKey::Ed25519 {
                public_key: pk1.public_key(),
                signature: pk1
                    .sign(&transaction_intent.transaction_intent_hash().hash),
            };

        let expected_signature_with_pub_key_2 =
            SignatureWithPublicKey::Ed25519 {
                public_key: pk2.public_key(),
                signature: pk2
                    .sign(&transaction_intent.transaction_intent_hash().hash),
            };

        let factor_source_id_pub_key = PublicKey::sample_ed25519();
        let factor_source_id = FactorSourceIDFromHash::new_for_arculus(
            factor_source_id_pub_key.to_bytes(),
        );
        let factor_source = ArculusCardFactorSource::new(
            factor_source_id.clone(),
            ArculusCardHint::sample(),
        );
        let pin = "123456".to_string();

        // replace sample instances with ones tied to our factor_source
        let path1 = DerivationPath::sample();
        let path2 = DerivationPath::sample_other();

        let hdfk1 = HierarchicalDeterministicPublicKey::new(
            expected_signature_with_pub_key_1.clone().public_key(),
            path1.clone(),
        );
        let hdfi1 = HierarchicalDeterministicFactorInstance::new(
            factor_source_id.clone(),
            hdfk1,
        );
        let instance1 = OwnedFactorInstance::new(
            AddressOfAccountOrPersona::sample_account_mainnet(),
            hdfi1,
        );

        let hdfk2 = HierarchicalDeterministicPublicKey::new(
            expected_signature_with_pub_key_2.clone().public_key(),
            path2.clone(),
        );
        let hdfi2 = HierarchicalDeterministicFactorInstance::new(
            factor_source_id.clone(),
            hdfk2,
        );
        let instance2 = OwnedFactorInstance::new(
            AddressOfAccountOrPersona::sample_account_mainnet_other(),
            hdfi2,
        );

        let transaction_intent = TransactionIntent::sample();
        let sign_request =
            TransactionSignRequestInput::<TransactionIntent>::new(
                transaction_intent.into(),
                factor_source_id.clone(),
                IndexSet::from([instance1.clone(), instance2.clone()]),
            );

        let input = IndexSet::from([sign_request.clone()]);

        let mut stub = ArculusWalletTestStub::new();

        stub.initialize_session()
            .expect_read_factor_source_id(Some(factor_source_id_pub_key))
            .verify_pin(pin.clone())
            .expect_to_sign_hash(
                sign_request.signature_inputs()[0].payload_id.clone().into(),
                instance1.factor_instance().derivation_path(),
                expected_signature_with_pub_key_1.clone(),
            )
            .expect_to_sign_hash(
                sign_request.signature_inputs()[1].payload_id.clone().into(),
                instance2.factor_instance().derivation_path(),
                expected_signature_with_pub_key_2.clone(),
            )
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut
            .sign(
                factor_source_id,
                NFCTagArculusInteractonPurpose::SignTransaction(factor_source),
                pin,
                input,
            )
            .await;

        let expected_result = IndexSet::from([
            HDSignature::new(
                sign_request.signature_inputs()[0].clone(),
                expected_signature_with_pub_key_1,
            )
            .unwrap(),
            HDSignature::new(
                sign_request.signature_inputs()[1].clone(),
                expected_signature_with_pub_key_2,
            )
            .unwrap(),
        ]);

        assert_eq!(result, Ok(expected_result));
    }
}
