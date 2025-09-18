use crate::prelude::*;

impl ArculusWalletClient {
    pub async fn do_card_io<Response, F>(
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

    pub async fn do_card_io_validate_status<F>(
        &self,
        command: BagOfBytes,
        response_map: F,
    ) -> Result<()>
    where
        F: FnOnce(BagOfBytes) -> i32,
    {
        let nfc_response = self.nfc_tag_driver.send_receive(command).await?;
        let status_code = response_map(nfc_response);

        let status = ArculusWalletCSDKResponseStatus::try_from(status_code)?;
        status.as_result()
    }

    pub async fn do_chainned_card_io<Response, F>(
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
    /// Gets the public key by path on the card
    pub async fn get_public_key_by_path_io(
        &self,
        wallet: ArculusWalletPointer,
        path: HDPath,
        curve: CardCurve,
    ) -> Result<BagOfBytes> {
        let request = self
            .csdk_driver
            .get_public_key_by_path_request(
                wallet,
                path.to_bip32_string().into_bytes().into(),
                curve.val(),
            )
            .ok_or(
                CommonError::ArculusCSDKFailedToCreateGetPublicKeyByPathRequest,
            )?;
        self.do_card_io(request, |response| {
            self.csdk_driver
                .get_public_key_by_path_response(wallet, response)
                .ok_or(CommonError::ArculusCSDKFailedToParseGetPublicKeyByPathResponse)
        })
        .await
    }

    /// Selects the given wallet on the card
    pub async fn select_card_io(
        &self,
        wallet: ArculusWalletPointer,
        aid: BagOfBytes,
    ) -> Result<BagOfBytes> {
        let request = self
            .csdk_driver
            .select_wallet_request(wallet, aid.clone())
            .ok_or(CommonError::ArculusCSDKFailedToCreateSelectWalletRequest)?;
        self.do_card_io(request, |response| {
            self.csdk_driver
                .select_wallet_response(wallet, response)
                .ok_or(
                    CommonError::ArculusCSDKFailedToParseSelectWalletResponse,
                )
        })
        .await
    }

    /// Gets the card gguid
    pub async fn get_gguid_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<BagOfBytes> {
        let request = self
            .csdk_driver
            .get_gguid_request(wallet)
            .ok_or(CommonError::ArculusCSDKFailedToCreateGetGguidRequest)?;
        self.do_card_io(request, |response| {
            self.csdk_driver
                .get_gguid_response(wallet, response)
                .ok_or(CommonError::ArculusCSDKFailedToParseGetGguidResponse)
        })
        .await
    }

    /// Gets the card firmware version
    pub async fn get_firmware_version_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<BagOfBytes> {
        let request = self
            .csdk_driver
            .get_firmware_version_request(wallet)
            .ok_or(
                CommonError::ArculusCSDKFailedToCreateGetFirmwareVersionRequest,
            )?;
        self.do_card_io(request, |response| {
            self.csdk_driver
                .get_firmware_version_response(wallet, response)
                .ok_or(CommonError::ArculusCSDKFailedToParseGetFirmwareVersionResponse)
        })
        .await
    }

    /// Stores the pin on the card
    pub async fn store_pin_io(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Result<()> {
        let request = self
            .csdk_driver
            .store_data_pin_request(wallet, pin)
            .ok_or(CommonError::ArculusCSDKFailedToCreateStoreDataPinRequest)?;
        self.do_card_io_validate_status(request, |response| {
            self.csdk_driver.store_data_pin_response(wallet, response)
        })
        .await
    }

    /// Verifies the pin on the card
    pub async fn verify_pin_io(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
    ) -> Result<ArculusVerifyPINResponse> {
        let request = self
            .csdk_driver
            .verify_pin_request(wallet, pin)
            .ok_or(CommonError::ArculusCSDKFailedToCreateVerifyPinRequest)?;
        self.do_card_io(request, |response| {
            Ok(self.csdk_driver.verify_pin_response(wallet, response))
        })
        .await
    }

    /// Initializes the encrypted session on the card
    pub async fn init_encrypted_session_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<()> {
        let request = self
            .csdk_driver
            .init_encrypted_session_request(wallet)
            .ok_or(
            CommonError::ArculusCSDKFailedToCreateInitEncryptedSessionRequest,
        )?;
        self.do_card_io_validate_status(request, |response| {
            self.csdk_driver
                .init_encrypted_session_response(wallet, response)
        })
        .await
    }

    /// Creates the new wallet seed for the seed phrase word count
    pub async fn create_wallet_seed_io(
        &self,
        wallet: ArculusWalletPointer,
        word_count: i64,
    ) -> Result<BagOfBytes> {
        let request = self
            .csdk_driver
            .create_wallet_seed_request(wallet, word_count)
            .ok_or(
                CommonError::ArculusCSDKFailedToCreateCreateWalletSeedRequest,
            )?;
        self.do_card_io(request, |response| {
            self.csdk_driver
                .create_wallet_seed_response(wallet, response)
                .ok_or(CommonError::ArculusCSDKFailedToParseCreateWalletSeedResponse)
        })
        .await
    }

    /// Resets the wallet on the card
    pub async fn reset_wallet_io(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<()> {
        let request = self
            .csdk_driver
            .reset_wallet_request(wallet)
            .ok_or(CommonError::ArculusCSDKFailedToCreateResetWalletRequest)?;
        self.do_card_io_validate_status(request, |response| {
            self.csdk_driver.reset_wallet_response(wallet, response)
        })
        .await
    }

    /// Initializes the wallet recovery process
    pub async fn init_recover_wallet_io(
        &self,
        wallet: ArculusWalletPointer,
        seed_words_count: i64,
    ) -> Result<()> {
        let request = self
            .csdk_driver
            .init_recover_wallet_request(wallet, seed_words_count)
            .ok_or(
                CommonError::ArculusCSDKFailedToCreateInitRecoverWalletRequest,
            )?;
        self.do_card_io_validate_status(request, |response| {
            self.csdk_driver
                .init_recover_wallet_response(wallet, response)
        })
        .await
    }

    /// Finishes the wallet recovery process
    pub async fn finish_recover_wallet_io(
        &self,
        wallet: ArculusWalletPointer,
        seed: BagOfBytes,
    ) -> Result<()> {
        let request = self
            .csdk_driver
            .finish_recover_wallet_request(wallet, seed)
            .ok_or(CommonError::ArculusCSDKFailedToCreateFinishRecoverWalletRequest)?;
        self.do_card_io_validate_status(request, |response| {
            self.csdk_driver
                .finish_recover_wallet_response(wallet, response)
        })
        .await
    }

    /// Signs the hash by path on the card
    pub async fn sign_hash_path_io(
        &self,
        wallet: ArculusWalletPointer,
        path: HDPath,
        hash: Hash,
        curve: CardCurve,
        algorithm: CardAlgorithm,
    ) -> Result<Ed25519Signature> {
        let request = self
            .csdk_driver
            .sign_hash_path_request(
                wallet,
                path.to_bip32_string().into_bytes().into(),
                curve.val(),
                algorithm.val(),
                hash.bytes().into(),
            )
            .ok_or(CommonError::ArculusCSDKFailedToCreateSignHashPathRequest)?;
        let signature_bytes = self
            .do_chainned_card_io(request, |response| {
                self.csdk_driver
                    .sign_hash_path_response(wallet, response)
                    .ok_or(
                    CommonError::ArculusCSDKFailedToParseSignHashPathResponse,
                )
            })
            .await?;

        Ed25519Signature::try_from(signature_bytes)
    }
}
