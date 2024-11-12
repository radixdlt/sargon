use crate::prelude::*;
use futures::future::{BoxFuture, Future};

pub struct InstancesInCacheConsumer {
    do_consume:
        Box<dyn Fn() -> BoxFuture<'static, Result<()>> + Send + 'static>,
}
unsafe impl Sync for InstancesInCacheConsumer {}
unsafe impl Send for InstancesInCacheConsumer {}

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

    pub async fn consume(self) -> Result<()> {
        (self.do_consume)().await
    }
}
