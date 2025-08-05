use crate::prelude::*;

impl SargonOS {
    pub async fn arculus_validate_min_firmware_version(
        &self,
    ) -> Result<ArculusMinFirmwareVersionRequirement> {
        self.clients
            .arculus_wallet_client
            .validate_min_firmware_version()
            .await
    }

    pub async fn arculus_configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        let fs_id = self
            .clients
            .arculus_wallet_client
            .configure_card_with_mnemonic(mnemonic, pin.to_string())
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
        pin: String,
        per_transaction: IndexSet<TransactionSignRequestInput<S>>,
    ) -> Result<IndexSet<HDSignature<S::ID>>> {
        self.clients
            .arculus_wallet_client
            .sign(factor_source_id, purpose, pin, per_transaction)
            .await
    }

    pub async fn verify_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
    ) -> Result<()> {
        self.clients
            .arculus_wallet_client
            .verify_card_pin(factor_source, pin)
            .await
    }

    pub async fn set_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        old_pin: String,
        new_pin: String,
    ) -> Result<()> {
        self.clients
            .arculus_wallet_client
            .set_card_pin(factor_source, old_pin, new_pin)
            .await
    }

    pub async fn arculus_card_reset(&self) -> Result<()> {
        self.clients.arculus_wallet_client.reset_wallet().await
    }
}
