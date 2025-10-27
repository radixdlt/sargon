use crate::prelude::*;

#[cfg(any(test, feature = "mock"))]
use mockall::automock;

/// Driver to interact with natice Arculus CSDK library
#[cfg_attr(any(test, feature = "mock"), automock)]
#[async_trait::async_trait]
pub trait TransactionIntentSignaturesCollector: Send {
    async fn collect_signatures(
        self: Box<Self>,
    ) -> Result<SignaturesOutcome<TransactionIntentHash>>;
}

/// Driver to interact with natice Arculus CSDK library
#[cfg_attr(any(test, feature = "mock"), automock)]
pub trait TransactionIntentSignaturesCollectorBuilder: Send + Sync {
    fn build(
        &self,
        finish_early_strategy: SigningFinishEarlyStrategy,
        profile_factor_sources: IndexSet<FactorSource>,
        transactions: IdentifiedVecOf<SignableWithEntities<TransactionIntent>>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        purpose: SigningPurpose,
    ) -> Box<dyn TransactionIntentSignaturesCollector>;
}

#[derive(Default)]
pub struct DefaultTransactionIntentSignaturesCollectorBuilder;

pub struct DefaultTransactionIntentSignaturesCollector {
    inner: SignaturesCollector<TransactionIntent>,
}

#[async_trait::async_trait]
impl TransactionIntentSignaturesCollector
    for DefaultTransactionIntentSignaturesCollector
{
    async fn collect_signatures(
        self: Box<Self>,
    ) -> Result<SignaturesOutcome<TransactionIntentHash>> {
        self.inner.collect_signatures().await
    }
}

impl TransactionIntentSignaturesCollectorBuilder
    for DefaultTransactionIntentSignaturesCollectorBuilder
{
    fn build(
        &self,
        finish_early_strategy: SigningFinishEarlyStrategy,
        profile_factor_sources: IndexSet<FactorSource>,
        transactions: IdentifiedVecOf<SignableWithEntities<TransactionIntent>>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        purpose: SigningPurpose,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        Box::new(DefaultTransactionIntentSignaturesCollector {
            inner: SignaturesCollector::with(
                finish_early_strategy,
                profile_factor_sources,
                transactions,
                interactor,
                purpose,
            ),
        })
    }
}