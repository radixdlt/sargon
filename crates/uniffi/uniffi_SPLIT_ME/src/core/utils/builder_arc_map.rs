use crate::prelude::*;
use std::future::Future;

pub(crate) fn builder_arc_map<T, F>(arc: Arc<T>, callback: F) -> Arc<T>
where
    T: Clone,
    F: FnOnce(&mut T),
{
    let mut this = Arc::try_unwrap(arc).unwrap_or_else(|x| (*x).clone());
    callback(&mut this);
    Arc::new(this)
}

pub(crate) async fn builder_arc_map_future_result<T, F, Fut>(
    arc: Arc<T>,
    callback: F,
) -> Result<Arc<T>>
where
    T: Clone,
    F: FnOnce(&mut T) -> Fut,
    Fut: Future<Output = Result<()>>,
{
    let mut this = Arc::try_unwrap(arc).unwrap_or_else(|x| (*x).clone());
    callback(&mut this).await?;
    Ok(Arc::new(this))
}
