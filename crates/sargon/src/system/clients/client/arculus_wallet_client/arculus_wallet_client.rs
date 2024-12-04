use std::future::Future;

use indexmap::IndexSet;
use radix_engine_interface::freeze_roles;

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
    pub(crate) nfc_tag_driver: Arc<dyn NFCTagDriver>
}

impl ArculusWalletClient {
    pub fn new(csdk_driver: Arc<dyn ArculusCSDKDriver>, nfc_tag_driver: Arc<dyn NFCTagDriver>) -> Self {
        Self {
            csdk_driver,
            nfc_tag_driver,
        }
    }
}

impl ArculusWalletClient {
    async fn start_arculus_wallet_session(&self) -> Result<ArculusWalletPointer> {
        let aid: Vec<u8> = vec![0x41, 0x52, 0x43, 0x55, 0x4C, 0x55, 0x53, 0x01, 0x01, 0x57];
        let wallet= self.csdk_driver.wallet_init();
        self.select_card_io(wallet, aid.into()).await?;
        self.init_encrypted_session_io(wallet).await?;

        Ok(wallet)
    }

    async fn execute_card_operation<Response, Op, Fut>(&self, op: Op) -> Result<Response> 
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
    pub async fn read_card_firmware_version(&self) -> Result<String> {
        let version: BagOfBytes = self.execute_card_operation(
             |wallet| self.get_firmware_version_io(wallet)
        ).await?;

        Ok(version.bytes.iter().map(|byte| byte.to_string()).collect::<Vec<String>>().join("."))
    }

    pub async fn read_card_factor_source_id(&self) -> Result<FactorSourceIDFromHash> {
        self.execute_card_operation(|wallet| 
            self._read_card_factor_source_id(wallet)
        ).await
    }

    pub async fn create_wallet_seed(&self, pin: String, word_count: u8) -> Result<Mnemonic> {
        self.execute_card_operation(|wallet| {
           self._create_wallet_seed(wallet, pin, word_count)
        }).await
    }

    pub async fn restore_wallet_seed(&self, mnemonic: Mnemonic, pin: String) -> Result<()> {
        self.execute_card_operation(|wallet|
            self._restore_wallet_seed(wallet, mnemonic, pin)
        ).await
    }

    pub async fn derive_public_keys(&self, paths: IndexSet<DerivationPath>) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        self.execute_card_operation(|wallet|
            self._derive_public_keys(wallet, paths)
        ).await
    }
}

impl ArculusWalletClient {
    pub async fn _derive_public_keys(&self, wallet: ArculusWalletPointer, paths: IndexSet<DerivationPath>) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        let mut keys = IndexSet::new();

    for path in paths {
        // Each task starts only after the previous one completes
        let result = self.get_public_key_by_path_io(wallet, path.clone()).await?;
        let public_key = PublicKey::ed25519_from_bytes(&result)?;
        let key = HierarchicalDeterministicPublicKey::new(public_key, path);
        keys.insert(key);
    }

        Ok(keys)
    }

    pub async fn _restore_wallet_seed(&self, wallet: ArculusWalletPointer, mnemonic: Mnemonic, pin: String) -> Result<()> {
        let pin_len = pin.len() as u8;
        let seed_length = mnemonic.word_count.discriminant();

        self.reset_wallet_io(wallet).await?;
        self.store_pin_io(wallet, pin.clone(), pin_len).await?;
        self.init_recover_wallet_io(wallet, seed_length).await?;
        let seed = self.csdk_driver.seed_phrase_from_mnemonic_sentence(wallet,  mnemonic.phrase().as_bytes().into(), seed_length, None, 0)?;
        self.finish_recover_wallet_io(wallet, seed, seed_length).await?;
        self.verify_pin_io(wallet, pin, pin_len).await?;

        Ok(())
    }

    async fn _read_card_factor_source_id(&self, wallet: ArculusWalletPointer) -> Result<FactorSourceIDFromHash> {
        let gguid_raw = self.get_gguid_io(wallet).await?;

        let bytes = Exactly32Bytes::try_from(gguid_raw)?;

        Ok(FactorSourceIDFromHash::new(FactorSourceKind::ArculusCard, bytes))
    }

    async fn _create_wallet_seed(&self, wallet: ArculusWalletPointer, pin: String, word_count: u8) -> Result<Mnemonic> {
        let pin_len = pin.len() as u8;

        self.reset_wallet_io(wallet).await?;
        self.store_pin_io(wallet, pin.clone(), pin_len).await?;
        let words = self.create_wallet_seed_io(wallet, word_count).await?;
        self.init_recover_wallet_io(wallet, word_count).await?;

        let seed = self.csdk_driver.seed_phrase_from_mnemonic_sentence(wallet, words.clone(), word_count, None, 0)?;

        self.finish_recover_wallet_io(wallet, seed, word_count).await?;
        self.verify_pin_io(wallet, pin, pin_len).await?;

        let phrase = String::from_utf8(words.to_vec()).unwrap();
       
        Mnemonic::from_phrase(&phrase)
    }
}

// impl ArculusWalletClient {
//     fn select_wallet_io(&self) -> Result<BagOfBytes> {
//         let raw_response = self.do_card_io(self.select_wallet_request(BagOfBytes::sample_aced())?)?;
//         let response = self.select_wallet_response(raw_response)
//         // parse status
//         let raw_encrypted_session_response = self.do_card_io(self.init_encrypted_session_request()?)?;
//         let status_res = self.init_encrypted_session_response(raw_encrypted_session_response)?;

//         let firmware_raw_response = self.do_card_io(self.get_firmware_version_request()?)?;
//         let firmware_response = self.get_firmware_version_response(firmware_raw_response)?;
//         let str = firmware_raw_response.to_hex();

        
//     }
// }

