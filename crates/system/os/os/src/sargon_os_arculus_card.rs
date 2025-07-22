use crate::prelude::*;

impl SargonOS {
    pub async fn arculus_get_card_info(&self) -> Result<ArculusCardInfo> {
        self.clients
            .arculus_wallet_client
            .get_arculus_card_info()
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

    pub async fn arculus_card_reset(&self) -> Result<()> {
        self.clients.arculus_wallet_client.reset_wallet().await
    }
}
