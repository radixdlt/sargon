use serde_json::value::Index;
use std::future::Future;

pub use crate::prelude::*;

#[derive(Debug)]
pub struct ArculusCSDKClient {
    /// An object implementing the `NetworkingDriver` traits, which iOS/Android
    /// clients pass into the constructor of this GatewayClient, so that it can
    /// execute network requests.
    pub driver: Arc<dyn ArculusCSDKDriver>,
}

#[derive(Debug)]
pub struct ArculusWalletClient {
    pub(crate) csdk_driver: Arc<dyn ArculusCSDKDriver>,
    pub(crate) nfc_tag_driver: Arc<dyn NFCTagDriver>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArculusCardInfo {
    pub firmware_version: String,
    pub gguid: Uuid,
    pub factor_source_id: Option<FactorSourceIDFromHash>,
}

impl ArculusCardInfo {
    pub fn new(
        firmware_version: String,
        gguid: Uuid,
        factor_source_id: Option<FactorSourceIDFromHash>,
    ) -> Self {
        Self {
            firmware_version,
            gguid,
            factor_source_id,
        }
    }
}

impl HasSampleValues for ArculusCardInfo {
    fn sample() -> Self {
        Self::new(
            "2.2.7.0".to_string(),
            Uuid::sample(),
            Some(FactorSourceIDFromHash::sample_arculus()),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            "2.2.7.6".to_string(),
            Uuid::sample(),
            Some(FactorSourceIDFromHash::sample_arculus_other()),
        )
    }
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

impl ArculusWalletClient {
    const AID: [u8; 10] =
        [0x41, 0x52, 0x43, 0x55, 0x4C, 0x55, 0x53, 0x01, 0x01, 0x57];

    pub(crate) async fn execute_card_operation<Response, Op, Fut>(
        &self,
        op: Op,
    ) -> Result<Response>
    where
        Op: FnOnce(ArculusWalletPointer) -> Fut,
        Fut: Future<Output = Result<Response>>,
    {
        self.nfc_tag_driver.start_session().await?;
        let wallet = self.start_arculus_wallet_session().await?;

        let result = op(wallet.clone()).await;

        self.nfc_tag_driver.end_session().await;
        self.csdk_driver.wallet_free(wallet);

        result
    }
}

impl ArculusWalletClient {
    pub async fn get_card_info(&self) -> Result<ArculusCardInfo> {
        self.execute_card_operation(|wallet| self._get_card_info(wallet))
            .await
    }

    pub async fn get_factor_source_id(&self) -> Result<FactorSourceIDFromHash> {
        self.execute_card_operation(|wallet| {
            self._read_card_factor_source_id(wallet)
        })
        .await
    }

    pub async fn create_wallet_seed(
        &self,
        pin: String,
        word_count: i64,
    ) -> Result<Mnemonic> {
        self.execute_card_operation(|wallet| {
            self._create_wallet_seed(wallet, pin, word_count)
        })
        .await
    }

    pub async fn restore_wallet_seed(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<()> {
        self.execute_card_operation(|wallet| {
            self._restore_wallet_seed(wallet, mnemonic, pin)
        })
        .await
    }

    pub async fn derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        self.execute_card_operation(|wallet| {
            self._derive_public_keys(wallet, factor_source, paths)
        })
        .await
    }
}

impl ArculusWalletClient {
    async fn start_arculus_wallet_session(
        &self,
    ) -> Result<ArculusWalletPointer> {
        let wallet = self.csdk_driver.wallet_init();
        let card_aid_to_select =
            BagOfBytes::from(ArculusWalletClient::AID.to_vec());
        let card_aid = self
            .select_card_io(wallet, card_aid_to_select.clone())
            .await?;

        if card_aid != card_aid_to_select {
            self.select_card_io(wallet, card_aid).await?;
        }

        self.init_encrypted_session_io(wallet).await?;

        Ok(wallet)
    }

    async fn _get_card_info(
        &self,
        wallet: ArculusWalletPointer,
    ) -> Result<ArculusCardInfo> {
        let firmware_version = self
            .get_firmware_version_io(wallet)
            .await?
            .to_vec()
            .iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join(".");
        let gguid = self.get_gguid_io(wallet).await?;
        let factor_source_id = self._read_card_factor_source_id(wallet).await?;

        Ok(ArculusCardInfo {
            firmware_version,
            gguid: Uuid::from_str(&gguid.to_hex()).unwrap(),
            factor_source_id: Some(factor_source_id),
        })
    }

    pub(crate) async fn validate_factor_source(
        &self,
        wallet: ArculusWalletPointer,
        factor_source: ArculusCardFactorSource,
    ) -> Result<()> {
        let on_card_factor_source_id =
            self._read_card_factor_source_id(wallet).await?;

        if on_card_factor_source_id != factor_source.id {
            return Err(CommonError::AESDecryptionFailed);
        }

        Ok(())
    }

    pub(crate) async fn _derive_public_keys(
        &self,
        wallet: ArculusWalletPointer,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        self.validate_factor_source(wallet, factor_source).await?;

        let mut keys = IndexSet::new();

        for path in paths {
            let public_key =
                self._derive_public_key(wallet, path.clone()).await?;
            let key = HierarchicalDeterministicPublicKey::new(
                public_key.into(),
                path,
            );
            keys.insert(key);
        }

        Ok(keys)
    }

    pub(crate) async fn _derive_public_key(
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

    async fn _restore_wallet_seed(
        &self,
        wallet: ArculusWalletPointer,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<()> {
        let seed_length = mnemonic.word_count.discriminant() as i64;

        self.reset_wallet_io(wallet).await?;
        self.store_pin_io(wallet, pin.clone()).await?;
        self.init_recover_wallet_io(wallet, seed_length).await?;
        let seed = self.csdk_driver.seed_phrase_from_mnemonic_sentence(
            wallet,
            mnemonic.phrase().as_bytes().into(),
            None,
        )?;
        self.finish_recover_wallet_io(wallet, seed).await?;
        self.verify_pin_io(wallet, pin).await?;

        Ok(())
    }

    async fn _read_card_factor_source_id(
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

    async fn _create_wallet_seed(
        &self,
        wallet: ArculusWalletPointer,
        pin: String,
        word_count: i64,
    ) -> Result<Mnemonic> {
        self.reset_wallet_io(wallet).await?;
        self.store_pin_io(wallet, pin.clone()).await?;
        let words = self.create_wallet_seed_io(wallet, word_count).await?;
        self.init_recover_wallet_io(wallet, word_count).await?;

        let seed = self.csdk_driver.seed_phrase_from_mnemonic_sentence(
            wallet,
            words.clone(),
            None,
        )?;

        self.finish_recover_wallet_io(wallet, seed).await?;
        self.verify_pin_io(wallet, pin).await?;

        let phrase = String::from_utf8(words.to_vec()).unwrap();

        Mnemonic::from_phrase(&phrase.remove_last())
    }
}
