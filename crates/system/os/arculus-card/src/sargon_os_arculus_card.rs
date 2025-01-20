use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsArculusCard {
    async fn arculus_get_card_state(&self) -> Result<ArculusCardState>;
    async fn arculus_card_create_wallet_seed(
        &self,
        pin: String,
        word_count: i64,
    ) -> Result<Mnemonic>;
    async fn restore_wallet_seed(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<()>;
    async fn derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>>;
    async fn arculus_card_sign_hashes(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
        hashes: IndexMap<Hash, IndexSet<DerivationPath>>,
    ) -> Result<IndexMap<Hash, IndexSet<SignatureWithPublicKey>>>;
}

#[async_trait::async_trait]
impl OsArculusCard for SargonOS {
    async fn arculus_get_card_state(&self) -> Result<ArculusCardState> {
        self.arculus_get_card_state().await
    }

    async fn arculus_card_create_wallet_seed(
        &self,
        pin: String,
        word_count: i64,
    ) -> Result<FactorSourceIDFromHash> {
        self.arculus_card_create_wallet_seed(pin, word_count).await
    }

    async fn restore_wallet_seed(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<()> {
        self.restore_wallet_seed(mnemonic, pin).await
    }

    async fn derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        self.arculus_card_derive_public_keys(factor_source, paths).await
    }

    async fn arculus_card_sign_hashes(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
        hashes: IndexMap<Hash, IndexSet<DerivationPath>>,
    ) -> Result<IndexMap<Hash, IndexSet<SignatureWithPublicKey>>> {
        self.arculus_card_sign_hashes(factor_source, pin, hashes)
            .await
    }
}
