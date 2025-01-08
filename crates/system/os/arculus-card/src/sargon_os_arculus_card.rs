use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsArculusCard {
    async fn arculus_card_read_firmware_version(&self) -> Result<String>;
    async fn read_card_factor_source_id(
        &self,
    ) -> Result<FactorSourceIDFromHash>;
    async fn create_wallet_seed(
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
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>>;
    async fn sign_hash(
        &self,
        pin: String,
        hash: Hash,
        derivation_path: DerivationPath,
    ) -> Result<SignatureWithPublicKey>;
}


#[async_trait::async_trait]
impl OsArculusCard for SargonOS {
    async fn arculus_card_read_firmware_version(&self) -> Result<String> {
        self.arculus_card_read_firmware_version().await
    }

    async fn read_card_factor_source_id(
        &self,
    ) -> Result<FactorSourceIDFromHash> {
        self.read_card_factor_source_id()
            .await
    }

    async fn create_wallet_seed(
        &self,
        pin: String,
        word_count: i64,
    ) -> Result<Mnemonic> {
        self.create_wallet_seed(pin, word_count)
            .await
    }

    async fn restore_wallet_seed(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<()> {
        self.restore_wallet_seed(mnemonic, pin)
            .await
    }

    async fn derive_public_keys(
        &self,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        self.derive_public_keys(paths)
            .await
    }

    async fn sign_hash(
        &self,
        pin: String,
        hash: Hash,
        derivation_path: DerivationPath,
    ) -> Result<SignatureWithPublicKey> {
        self.sign_hash(pin, hash, derivation_path)
            .await
    }
}
