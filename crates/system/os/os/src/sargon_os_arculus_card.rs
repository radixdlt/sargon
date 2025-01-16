use crate::prelude::*;

impl SargonOS {
    pub async fn arculus_get_card_state(&self) -> Result<ArculusCardState> {
        self.clients
            .arculus_wallet_client
            .get_arculus_card_state()
            .await
    }

    pub async fn arculus_card_create_wallet_seed(
        &self,
        pin: String,
        word_count: i64,
    ) -> Result<FactorSourceIDFromHash> {
        self.clients
            .arculus_wallet_client
            .create_wallet_seed(pin, word_count)
            .await
    }

    pub async fn arculus_card_restore_wallet_seed(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        self.clients
            .arculus_wallet_client
            .restore_wallet_seed(mnemonic, pin)
            .await
    }

    pub async fn arculus_card_derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicPublicKey>> {
        self.clients
            .arculus_wallet_client
            .derive_public_keys(factor_source, paths)
            .await
    }

    pub async fn arculus_card_sign_hashes(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
        hashes: IndexMap<Hash, IndexSet<DerivationPath>>,
    ) -> Result<IndexMap<Hash, IndexSet<SignatureWithPublicKey>>> {
        self.clients
            .arculus_wallet_client
            .sign_hashes(factor_source, pin, hashes)
            .await
    }
}
