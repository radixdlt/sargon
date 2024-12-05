use crate::prelude::*;

impl SargonOS {
    pub async fn arculus_card_read_firmware_version(&self) -> Result<String> {
        self.clients.arculus_wallet_client.read_card_firmware_version().await
    }

    pub async fn read_card_factor_source_id(&self) -> Result<FactorSourceIDFromHash> {
        self.clients.arculus_wallet_client.read_card_factor_source_id().await
    }

    pub async fn create_wallet_seed(&self, pin: String, word_count: i64) -> Result<Mnemonic> {
        self.clients.arculus_wallet_client.create_wallet_seed(pin, word_count).await
    }

    pub async fn restore_wallet_seed(&self, mnemonic: Mnemonic, pin: String) -> Result<()> {
        self.clients.arculus_wallet_client.restore_wallet_seed(mnemonic, pin).await
    }

    pub async fn derive_public_keys(&self, paths: IndexSet<DerivationPath>) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        self.clients.arculus_wallet_client.derive_public_keys(paths).await
    }

    pub async fn sign_hash(&self, pin: String, hash: Hash, derivation_path: DerivationPath) -> Result<SignatureWithPublicKey> {
        self.clients.arculus_wallet_client.sign_hash(pin, hash, derivation_path).await
    }
}