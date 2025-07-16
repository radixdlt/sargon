use std::{
    fmt::format,
    future::{self, Future},
};

pub use crate::prelude::*;

#[derive(Debug)]
pub struct ArculusWalletClient {
    pub(crate) csdk_driver: Arc<dyn ArculusCSDKDriver>,
    pub(crate) nfc_tag_driver: Arc<dyn NFCTagDriver>,
}

impl ArculusWalletClient {
    pub fn new(
        csdk_driver: Arc<dyn ArculusCSDKDriver>,
        nfc_tag_driver: Arc<dyn NFCTagDriver>,
    ) -> Self {
        Self {
            csdk_driver,
            nfc_tag_driver,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArculusCardState {
    NotConfigured,
    Configured(FactorSourceIDFromHash),
}

impl HasSampleValues for ArculusCardState {
    fn sample() -> Self {
        ArculusCardState::NotConfigured
    }

    fn sample_other() -> Self {
        ArculusCardState::Configured(FactorSourceIDFromHash::sample())
    }
}

impl ArculusWalletClient {
    pub async fn get_arculus_card_state(&self) -> Result<ArculusCardState> {
        self.execute_card_operation(
            NFCTagDriverPurpose::Arculus(
                NFCTagArculusInteractonPurpose::IdentifyingCard,
            ),
            |wallet| self._get_arculus_card_state(wallet),
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
        self.execute_card_operation(
            NFCTagDriverPurpose::Arculus(purpose),
            |wallet| self._sign(wallet, factor_source_id, pin, per_transaction),
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

    pub async fn configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        self.execute_card_operation(
            NFCTagDriverPurpose::Arculus(
                NFCTagArculusInteractonPurpose::ConfiguringCardMnemonic,
            ),
            |wallet| self._restore_wallet_seed(wallet, mnemonic, pin),
        )
        .await
    }

    pub async fn configure_card(
        &self,
        pin: String,
        word_count: i64,
    ) -> Result<FactorSourceIDFromHash> {
        self.execute_card_operation(
            NFCTagDriverPurpose::Arculus(
                NFCTagArculusInteractonPurpose::ConfiguringCardMnemonic,
            ),
            |wallet| self._create_wallet_seed(wallet, pin, word_count),
        )
        .await
    }

    pub async fn derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        self.execute_card_operation(
            NFCTagDriverPurpose::Arculus(
                NFCTagArculusInteractonPurpose::DerivingPublicKeys(
                    factor_source.clone(),
                ),
            ),
            |wallet| self._derive_public_keys(wallet, factor_source.id, paths),
        )
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
            let result = op(wallet.clone()).await;
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
    pub async fn _sign<S: Signable>(
        &self,
        wallet: ArculusWalletPointer,
        factor_source_id: FactorSourceIDFromHash,
        pin: String,
        per_transaction: IndexSet<TransactionSignRequestInput<S>>,
    ) -> Result<IndexSet<HDSignature<S::ID>>> {
        self.validate_factor_source(wallet, factor_source_id)
            .await?;
        self.verify_pin_io(wallet, pin.clone()).await?;

        let mut signatures = IndexSet::new();

        let signature_inputs: Vec<HDSignatureInput<S::ID>> = per_transaction
            .iter()
            .flat_map(|transaction| transaction.signature_inputs())
            .collect();

        for signature_input in signature_inputs {
            let signature = self
                .sign_hash_path(
                    wallet.clone(),
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

    async fn _get_arculus_card_state(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<ArculusCardState> {
        let id_result = self._get_factor_source_id(wallet).await;
        match id_result {
            Ok(id) => Ok(ArculusCardState::Configured(id)),
            Err(CommonError::ArculusCSDKFailedToCreateGetPublicKeyByPathResponse) => Ok(ArculusCardState::NotConfigured),
            Err(e) => Err(e)
        }
    }

    async fn _create_wallet_seed(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
        word_count: i64,
    ) -> Result<FactorSourceIDFromHash> {
        self.setup_seed(wallet, pin, word_count, || {
            self.create_wallet_seed_io(wallet, word_count)
        })
        .await
    }

    async fn _restore_wallet_seed(
        &self,
        wallet: ArculusWalletPointer,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        self.setup_seed(
            wallet,
            pin,
            mnemonic.word_count.discriminant() as i64,
            || future::ready(Ok(mnemonic.phrase().as_bytes().into())),
        )
        .await
    }

    /// Configures the given mnemonic on the card.
    /// The mnemonic is either provided by the user on restore or it is generated by the card on create new seed.
    async fn setup_seed<MnemonicProvider, Fut>(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
        word_count: i64,
        mnemonic_provider: MnemonicProvider,
    ) -> Result<FactorSourceIDFromHash>
    where
        MnemonicProvider: FnOnce() -> Fut,
        Fut: Future<Output = Result<BagOfBytes>>,
    {
        // First reset the Wallet to prepare it for a new seed
        self.reset_wallet_io(wallet).await?;

        // Select the card again after reset
        self.select_wallet(wallet).await?;
        self.store_pin_io(wallet, pin.clone()).await?;

        // The
        self.init_recover_wallet_io(wallet, word_count).await?;

        let mnemonic_sentence = mnemonic_provider().await?;

        // Generate the seed with the CSDK from the Arculus Card generated words
        let seed = self.csdk_driver.seed_phrase_from_mnemonic_sentence(
            wallet,
            mnemonic_sentence,
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

impl ArculusWalletClient {
    async fn _derive_public_keys(
        &self,
        wallet: ArculusWalletPointer,
        factor_source_id: FactorSourceIDFromHash,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        self.validate_factor_source(wallet, factor_source_id.clone())
            .await?;

        let mut keys = IndexSet::new();
        let number_of_total_paths = paths.len();
        self.nfc_tag_driver
            .set_message("Deriving public keys, pregress 0%".to_string())
            .await;
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

            let progress =
                (keys.len() as f32 / number_of_total_paths as f32) * 100 as f32;
            self.nfc_tag_driver
                .set_message(format!(
                    "Deriving public keys, progress {:?}%",
                    progress.floor()
                ))
                .await;
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

impl ArculusWalletClient {
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
            public_key: public_key,
            signature: signature,
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
        let card_state = self._get_arculus_card_state(wallet).await;

        match card_state {
            Ok(ArculusCardState::NotConfigured) => {
                return Err(CommonError::ArculusCardNotConfigured);
            }
            Ok(ArculusCardState::Configured(on_card_factor_source_id)) => {
                if on_card_factor_source_id != factor_source_id {
                    return Err(
                        CommonError::ArculusCardFactorSourceIdMissmatch,
                    );
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
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

        fn init_encrypted_session(&mut self) -> &mut Self {
            let request = BagOfBytes::random();
            let nfc_card_response = BagOfBytes::random();
            self.csdk_driver
                .expect_init_encrypted_session_request()
                .with(eq(self.wallet_pointer.clone()))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_card_response.clone());
            self.csdk_driver
                .expect_init_encrypted_session_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_response))
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
            self.end_nfc_session().free_wallet()
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
                .return_const(ArculusWalletCSDKResponseStatus::Ok as i32);

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
            let nfc_card_reponse = BagOfBytes::random();

            self.csdk_driver
                .expect_init_recover_wallet_request()
                .with(eq(self.wallet_pointer.clone()), eq(seed_words_count))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_card_reponse.clone());
            self.csdk_driver
                .expect_init_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_reponse))
                .once()
                .in_sequence(&mut self.sequence)
                .return_const(ArculusWalletCSDKResponseStatus::Ok as i32);

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
                .return_const(Some(request.clone()));
            self.nfc_send_receive(request, nfc_card_reponse.clone());
            self.csdk_driver
                .expect_finish_recover_wallet_response()
                .with(eq(self.wallet_pointer.clone()), eq(nfc_card_reponse))
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
    async fn get_card_state_failed_derive_factor_source_id() {
        let mut stub = ArculusWalletTestStub::new();

        stub.initialize_session()
            .expect_read_factor_source_id(None)
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.get_arculus_card_state().await;

        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusCardState::NotConfigured)
        );
    }

    #[actix_rt::test]
    async fn get_card_state() {
        let mut stub = ArculusWalletTestStub::new();

        let factor_source_id_pub_key = PublicKey::sample_ed25519();
        let factor_source_id = FactorSourceIDFromHash::new_for_arculus(
            factor_source_id_pub_key.to_bytes(),
        );

        stub.initialize_session()
            .expect_read_factor_source_id(Some(factor_source_id_pub_key))
            .end_session();

        let sut = ArculusWalletClient::new(
            Arc::new(stub.csdk_driver),
            Arc::new(stub.nfc_tag_driver),
        );

        let result = sut.get_arculus_card_state().await;

        pretty_assertions::assert_eq!(
            result,
            Ok(ArculusCardState::Configured(factor_source_id))
        );
    }

    #[actix_rt::test]
    async fn create_wallet_seed() {
        let pin = "123456".to_string();
        let word_count = 24;
        let mnemonic_sentence_create_by_card = BagOfBytes::from_hex("6578616d706c652070656c6963616e207370656e6420646172696e6720696e6d61746520676c616420746f6e6520636f6c756d6e2064657061727420726f736520736f727420706561722062726965662063656c657279206e6f7465206f6c796d706963206d697373207370617469616c206f626a65637420756e69666f726d207265736f7572636520646973706c617920666973682073686f7000").unwrap();
        let seed_from_mnemonic_sentence_created_by_csdk = BagOfBytes::from_hex("8b10dc86df2ff3d44f8c350fc0a7bf5c7eeb27f74e529d90eea280fb1ea9b62b679328235b6eca5de27c89e2bc655d38385d7c3c0a3543845938ce9f1ee4c69951a1e3652c8b2aac1b013c3971c92acff03f32592d62999f4a96f06a2b1b8e542d9a91103a36f55752e7c2230f04399c1f69907bf814c6938f8a69c6410bc8789a60825588362f333be01b5b60dca6bf7eb998da864a3e86fe4979f5d051baa597d45fef318ae1ad8b68a81645d60eeb12a2b63fc90b8f7506d0e14957e0249a").unwrap();

        let factor_source_id_pub_key = PublicKey::sample_ed25519();
        let factor_source_id = FactorSourceIDFromHash::new_for_arculus(
            factor_source_id_pub_key.to_bytes(),
        );

        let mut stub = ArculusWalletTestStub::new();

        stub.initialize_session()
            .reset_wallet()
            .select_wallet()
            .store_pin(pin.clone())
            .init_recover_wallet(word_count)
            .create_wallet_seed(
                word_count,
                mnemonic_sentence_create_by_card.clone(),
            )
            .seed_phrase_from_mnemonic_sentence(
                mnemonic_sentence_create_by_card.clone(),
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

        let result = sut.create_wallet_seed(pin, 24).await;

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
            factor_source_id,
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
            .expect_read_factor_source_id(Some(factor_source_id_pub_key))
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
}
