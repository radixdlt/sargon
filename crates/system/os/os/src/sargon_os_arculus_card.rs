use crate::prelude::*;

impl SargonOS {
    pub async fn arculus_get_card_state(&self) -> Result<ArculusCardState> {
        self.clients
            .arculus_wallet_client
            .get_arculus_card_state()
            .await
    }

    pub async fn arculus_configure_card(
        &self,
    ) -> Result<FactorSourceIDFromHash> {
        let pin = "123456";
        let word_count = 24;

        let fs_id = self
            .clients
            .arculus_wallet_client
            .configure_card(pin.to_string(), word_count)
            .await?;

        self.clients
            .secure_storage
            .store_pin_for_factor_source_id(fs_id.clone(), pin.to_string())
            .await?;

        Ok(fs_id)
    }

    pub async fn arculus_configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
    ) -> Result<FactorSourceIDFromHash> {
        let pin = "123456";

        let fs_id = self
            .clients
            .arculus_wallet_client
            .configure_card_with_mnemonic(mnemonic, pin.to_string())
            .await?;

        self.clients
            .secure_storage
            .store_pin_for_factor_source_id(fs_id.clone(), pin.to_string())
            .await?;

        Ok(fs_id)
    }

    pub async fn arculus_card_derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        self.clients
            .arculus_wallet_client
            .derive_public_keys(factor_source, paths)
            .await
    }

    pub async fn arculus_card_sign<S: Signable>(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        purpose: NFCTagArculusInteractonPurpose,
        per_transaction: IndexSet<TransactionSignRequestInput<S>>,
    ) -> Result<IndexSet<HDSignature<S::ID>>> {
        let pin = self
            .clients
            .secure_storage
            .load_pin_for_factor_source_id(factor_source_id)
            .await?
            .ok_or(CommonError::Unknown)?;
        self.clients
            .arculus_wallet_client
            .sign(factor_source_id, purpose, pin, per_transaction)
            .await
    }

    pub async fn arculus_card_reset(&self) -> Result<()> {
        self.clients.arculus_wallet_client.reset_wallet().await
    }
}

pub trait ArculusCardFactorSourcePINStorage {
    async fn load_pin_for_factor_source_id(
        &self,
        id: FactorSourceIDFromHash,
    ) -> Result<Option<String>>;
    async fn store_pin_for_factor_source_id(
        &self,
        id: FactorSourceIDFromHash,
        pin: String,
    ) -> Result<()>;
}

impl ArculusCardFactorSourcePINStorage for SecureStorageClient {
    async fn load_pin_for_factor_source_id(
        &self,
        id: FactorSourceIDFromHash,
    ) -> Result<Option<String>> {
        self.load(SecureStorageKey::ArculusCardFactorSourcePIN {
            factor_source_id: id,
        })
        .await
    }

    async fn store_pin_for_factor_source_id(
        &self,
        id: FactorSourceIDFromHash,
        pin: String,
    ) -> Result<()> {
        self.save(
            SecureStorageKey::ArculusCardFactorSourcePIN {
                factor_source_id: id,
            },
            &pin,
        )
        .await
    }
}
