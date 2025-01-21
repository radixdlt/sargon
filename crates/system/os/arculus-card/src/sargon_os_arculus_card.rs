use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsArculusCard {
    async fn arculus_get_card_state(&self) -> Result<ArculusCardState>;
    async fn arculus_configure_card(
        &self
    ) -> Result<FactorSourceIDFromHash>;

    async fn arculus_configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
    ) -> Result<FactorSourceIDFromHash>;

    async fn arculus_card_derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>>;

    async fn arculus_card_sign_hashes(
        &self,
        factor_source: ArculusCardFactorSource,
        hashes: IndexMap<Hash, IndexSet<DerivationPath>>,
    ) -> Result<IndexMap<Hash, IndexSet<SignatureWithPublicKey>>>;

    async fn arculus_card_sign_hash(
        &self,
        factor_source: ArculusCardFactorSource,
        hash: Hash,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<SignatureWithPublicKey>>;

    async fn arculus_card_reset(&self) -> Result<()>;
}

#[async_trait::async_trait]
impl OsArculusCard for SargonOS {
    async fn arculus_get_card_state(&self) -> Result<ArculusCardState> {
        self.arculus_get_card_state().await
    }

    async fn arculus_configure_card(
        &self,
    ) -> Result<FactorSourceIDFromHash> {
        self.arculus_configure_card().await
    }

    async fn arculus_configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic
    ) -> Result<FactorSourceIDFromHash> {
        self.arculus_configure_card_with_mnemonic(mnemonic).await
    }

    async fn arculus_card_derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        self.arculus_card_derive_public_keys(factor_source, paths).await
    }

    async fn arculus_card_sign_hashes(
        &self,
        factor_source: ArculusCardFactorSource,
        hashes: IndexMap<Hash, IndexSet<DerivationPath>>,
    ) -> Result<IndexMap<Hash, IndexSet<SignatureWithPublicKey>>> {
        self.arculus_card_sign_hashes(factor_source, hashes)
            .await
    }

    async fn arculus_card_sign_hash(
        &self,
        factor_source: ArculusCardFactorSource,
        hash: Hash,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<SignatureWithPublicKey>> {
        self.arculus_card_sign_hash(factor_source, hash, paths).await
    }

    async fn arculus_card_reset(&self) -> Result<()> {
        self.arculus_card_reset().await
    }
}
