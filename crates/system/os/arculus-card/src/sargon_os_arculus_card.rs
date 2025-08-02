use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsArculusCard {
    async fn validate_min_firmware_version(
        &self,
    ) -> Result<ArculusMinFirmwareVersionRequirement>;

    async fn arculus_configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash>;

    async fn arculus_card_derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>;

    async fn arculus_card_sign<S: Signable>(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        purpose: NFCTagArculusInteractonPurpose,
        pin: String,
        per_transaction: IndexSet<TransactionSignRequestInput<S>>,
    ) -> Result<IndexSet<HDSignature<S::ID>>>;

    async fn verify_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
    ) -> Result<()>;

    async fn set_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        old_pin: String,
        new_pin: String,
    ) -> Result<()>;

    async fn arculus_card_reset(&self) -> Result<()>;
}

#[async_trait::async_trait]
impl OsArculusCard for SargonOS {
    async fn validate_min_firmware_version(
        &self,
    ) -> Result<ArculusMinFirmwareVersionRequirement> {
        self.validate_min_firmware_version().await
    }

    async fn arculus_configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        self.arculus_configure_card_with_mnemonic(mnemonic, pin)
            .await
    }

    async fn arculus_card_derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        self.arculus_card_derive_public_keys(factor_source, paths)
            .await
    }

    async fn arculus_card_sign<S: Signable>(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        purpose: NFCTagArculusInteractonPurpose,
        pin: String,
        per_transaction: IndexSet<TransactionSignRequestInput<S>>,
    ) -> Result<IndexSet<HDSignature<S::ID>>> {
        self.arculus_card_sign(factor_source_id, purpose, pin, per_transaction)
            .await
    }

    async fn verify_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
    ) -> Result<()> {
        self.verify_card_pin(factor_source, pin).await
    }

    async fn set_card_pin(
        &self,
        factor_source: ArculusCardFactorSource,
        old_pin: String,
        new_pin: String,
    ) -> Result<()> {
        self.set_card_pin(factor_source, old_pin, new_pin).await
    }

    async fn arculus_card_reset(&self) -> Result<()> {
        self.arculus_card_reset().await
    }
}
