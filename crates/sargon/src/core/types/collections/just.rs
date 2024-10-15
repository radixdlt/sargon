use crate::prelude::*;

pub trait Just<Item> {
    fn just(item: Item) -> Self;
}
impl<T: std::hash::Hash + Eq> Just<T> for IndexSet<T> {
    fn just(item: T) -> Self {
        Self::from_iter([item])
    }
}
impl<T: std::hash::Hash + Eq> Just<T> for HashSet<T> {
    fn just(item: T) -> Self {
        Self::from_iter([item])
    }
}
impl<K: std::hash::Hash + Eq, V> Just<(K, V)> for IndexMap<K, V> {
    fn just(item: (K, V)) -> Self {
        Self::from_iter([item])
    }
}
impl<K: std::hash::Hash + Eq, V> Just<(K, V)> for HashMap<K, V> {
    fn just(item: (K, V)) -> Self {
        Self::from_iter([item])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_set() {
        assert_eq!(
            IndexSet::just(FactorSourceKind::Device),
            IndexSet::<FactorSourceKind>::from_iter([FactorSourceKind::Device])
        )
    }

    #[test]
    fn test_hash_set() {
        assert_eq!(
            HashSet::just(FactorSourceKind::Device),
            HashSet::<FactorSourceKind>::from_iter([FactorSourceKind::Device])
        )
    }

    #[test]
    fn test_index_map() {
        assert_eq!(
            IndexMap::just((
                FactorSourceKind::Device,
                FactorSource::sample_device()
            )),
            IndexMap::<FactorSourceKind, FactorSource>::from_iter([(
                FactorSourceKind::Device,
                FactorSource::sample_device()
            )])
        )
    }

    #[test]
    fn test_hash_map() {
        assert_eq!(
            HashMap::just((
                FactorSourceKind::Device,
                FactorSource::sample_device()
            )),
            HashMap::<FactorSourceKind, FactorSource>::from_iter([(
                FactorSourceKind::Device,
                FactorSource::sample_device()
            )])
        )
    }
}
