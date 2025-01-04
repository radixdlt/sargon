use crate::prelude::*;

pub(crate) fn builder_arc_map<T, F>(arc: Arc<T>, callback: F) -> Arc<T>
where
    T: Clone,
    F: FnOnce(&mut T),
{
    let mut this = Arc::try_unwrap(arc).unwrap_or_else(|x| (*x).clone());
    callback(&mut this);
    Arc::new(this)
}
