use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn arculus_card_read_firmware_version(&self) -> Result<String> {
        self.wrapped.arculus_card_read_firmware_version().await.into_result()
    }

    pub async fn read_card_factor_source_id(&self) -> Result<FactorSourceIDFromHash> {
        self.wrapped.read_card_factor_source_id().await.into_result()
    }

    pub async fn create_wallet_seed(&self, pin: String, word_count: i64) -> Result<Mnemonic> {
        self.wrapped.create_wallet_seed(pin, word_count).await.into_result()
    }

    pub async fn restore_wallet_seed(&self, mnemonic: Mnemonic, pin: String) -> Result<()> {
        self.wrapped.restore_wallet_seed(mnemonic.into_internal(), pin).await.into_result()
    }

    pub async fn derive_public_keys(&self, paths: Vec<DerivationPath>) -> Result<Vec<HierarchicalDeterministicPublicKey>> {
        self.wrapped.derive_public_keys(
            paths.into_iter().map(|path| path.into_internal()).collect::<sargon::IndexSet<_>>()
        ).await
        .map(|keys|
            keys.into_iter().map(|key| key).collect::<Vec<sargon::HierarchicalDeterministicPublicKey>>()
        )
        .into_result()
    }

    pub async fn sign_hash(&self, pin: String, hash: Hash, derivation_path: DerivationPath) -> Result<SignatureWithPublicKey> {
        self.wrapped.sign_hash(pin, hash.into_internal(), derivation_path.into_internal()).await.into_result()
    }
}