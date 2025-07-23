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

    async fn arculus_card_reset(&self) -> Result<()> {
        self.arculus_card_reset().await
    }
}
