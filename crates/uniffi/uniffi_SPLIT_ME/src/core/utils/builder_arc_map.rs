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

pub(crate) fn builder_arc_map_result<T, F>(
    arc: Arc<T>,
    callback: F,
) -> Result<Arc<T>>
where
    T: Clone,
    F: FnOnce(&mut T) -> Result<()>,
{
    let mut this = Arc::try_unwrap(arc).unwrap_or_else(|x| (*x).clone());
    callback(&mut this)?;
    Ok(Arc::new(this))
}
