use shaku::{Component, HasProvider};

use crate::prelude::*;

trait ApplyShieldTransactionsCommiterModule:
    HasProvider<dyn ApplyShieldTransactionsCommiter>
{
}

module! {
    ApplyShieldTransactionsCommiterModuleImpl: ApplyShieldTransactionsCommiterModule {
        components = [#[lazy] OsFactoryImpl],
        providers = [
            ApplyShieldTransactionsCommiterImpl,
            ApplyShieldTransactionsBuilderImpl,
            ApplyShieldTransactionsSignerImpl,
            ApplyShieldTransactionsEnqueuerImpl,
        ]
    }
}

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsCommiter: Send + Sync {
    async fn commit(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<IndexSet<TransactionIntentHash>>;
}

/// Builds, signs and enqueues the transaction intents of manifests
/// which applies a shields to entities.
#[derive(Provider)]
#[shaku(interface = ApplyShieldTransactionsCommiter)]
pub struct ApplyShieldTransactionsCommiterImpl {
    #[shaku(provide)]
    builder: Box<dyn ApplyShieldTransactionsBuilder>,

    #[shaku(provide)]
    signer: Box<dyn ApplyShieldTransactionsSigner>,

    #[shaku(provide)]
    enqueuer: Box<dyn ApplyShieldTransactionsEnqueuer>,
}

pub trait OsFactory: shaku::Interface {
    fn os(&self) -> &'static SargonOS;
}

#[derive(Component)]
#[shaku(interface = OsFactory)]
pub struct OsFactoryImpl {
    os: &'static SargonOS,
}
impl OsFactory for OsFactoryImpl {
    fn os(&self) -> &'static SargonOS {
        self.os
    }
}

impl ApplyShieldTransactionsCommiterImpl {
    fn live(os: &'static SargonOS) -> Box<dyn ApplyShieldTransactionsCommiter> {
        let module = Arc::new(
            ApplyShieldTransactionsCommiterModuleImpl::builder()
                .with_component_parameters::<OsFactoryImpl>(
                    OsFactoryImplParameters { os },
                )
                .build(),
        );

        module.provide().unwrap()
    }

    pub fn new(
        os: &'static SargonOS,
    ) -> Box<dyn ApplyShieldTransactionsCommiter> {
        Self::live(os)
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsCommiter for ApplyShieldTransactionsCommiterImpl {
    /// Builds, signs and enqueues the transaction intents of manifests
    /// which applies a shields to entities.
    async fn commit(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<IndexSet<TransactionIntentHash>> {
        let payload_to_sign = self
            .builder
            .build_payload_to_sign(network_id, manifest_and_payer_tuples)
            .await?;

        // Try to sign all applications - we will "the best" of the 5 manifests for securified entities
        // This step **also notarized** the signed intents.
        let signed_transactions = self
            .signer
            .sign_transaction_intents(payload_to_sign)
            .await?;

        // Enqueue all signed transactions
        let transaction_ids = self
            .enqueuer
            .enqueue_signed_transactions(signed_transactions)
            .await?;

        // Return the TransactionIntentHashes
        Ok(transaction_ids)
    }
}
