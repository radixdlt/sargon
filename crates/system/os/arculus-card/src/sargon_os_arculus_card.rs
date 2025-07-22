use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsArculusCard {
    async fn arculus_get_card_info(&self) -> Result<ArculusCardInfo>;

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
    async fn arculus_get_card_info(&self) -> Result<ArculusCardInfo> {
        self.arculus_get_card_info().await
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
