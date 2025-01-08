use crate::prelude::*;
use futures::future::{BoxFuture, Future};

/// A handle for consuming used instances that are present in the cache, recently read from the cache
/// but not eagerly consumed since we depended on a subsequent operation to succeed before we would
/// consume them.
pub struct InstancesInCacheConsumer {
    do_consume:
        Box<dyn Fn() -> BoxFuture<'static, Result<()>> + Send + 'static>,
}

impl InstancesInCacheConsumer {
    pub(super) fn new<T, F>(f: T) -> Self
    where
        T: Send + Sync + 'static + Fn() -> F,
        F: Future<Output = Result<()>> + Send + 'static,
    {
        InstancesInCacheConsumer {
            do_consume: Box::new(move || Box::pin(f())),
        }
    }

    /// Consumes FactorInstances from the FactorInstancesCache, call this method
    /// once you have successfully used the FactorInstance(s), e.g. saved a newly
    /// created virtual entity into Profile, or successfully secured an entity,
    /// that is submitted a transaction updating the on chain rules and also
    /// successfully updated it in Profile and saved Profile to secure storage.
    pub async fn consume(self) -> Result<()> {
        (self.do_consume)().await
    }
}
