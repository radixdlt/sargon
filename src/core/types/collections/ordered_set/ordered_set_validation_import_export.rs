use std::any::TypeId;

use crate::prelude::*;

pub(super) fn import_ordered_set_from<V>(
    iter: impl IntoIterator<Item = V>,
) -> Result<OrderedSet<V>>
where
    V: Eq + Clone + std::hash::Hash + 'static,
{
    let mut set = IndexSet::new();

    for item in iter {
        let (index, inserted) = set.insert_full(item);
        if !inserted {
            return Err(CommonError::ItemAlreadyExist {
                index_of_existing: index as u64,
            });
        }
    }
    let ordered_set = OrderedSet::from(set);
    export_ordered_set(&ordered_set)?;
    Ok(ordered_set)
}

pub(super) fn export_ordered_set<V>(
    set: &OrderedSet<V>,
) -> Result<&OrderedSet<V>>
where
    V: Eq + Clone + std::hash::Hash + 'static,
{
    Ok(set)
}
