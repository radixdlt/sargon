use crate::prelude::*;

#[cfg(any(test, feature = "mock"))]
use mockall::automock;

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
        transactions: IdentifiedVecOf<SignableWithEntities<TransactionIntent>>,
        purpose: SigningPurpose,
    ) -> Box<dyn TransactionIntentSignaturesCollector>;
}

pub struct DefaultTransactionIntentSignaturesCollectorBuilder {
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    all_profile_factor_sources: IndexSet<FactorSource>,
}

impl DefaultTransactionIntentSignaturesCollectorBuilder {
    pub fn new(
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
        all_profile_factor_sources: IndexSet<FactorSource>
    ) -> Self {
        Self { 
            interactor,
            all_profile_factor_sources 
        }
    }
}

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
        transactions: IdentifiedVecOf<SignableWithEntities<TransactionIntent>>,
        purpose: SigningPurpose,
    ) -> Box<dyn TransactionIntentSignaturesCollector> {
        Box::new(DefaultTransactionIntentSignaturesCollector {
            inner: SignaturesCollector::with(
                SigningFinishEarlyStrategy::default(),
                self.all_profile_factor_sources.clone(),
                transactions,
                self.interactor.clone(),
                purpose,
            ),
        })
    }
}