use indexmap::{IndexMap, IndexSet};
use std::collections::{HashMap, HashSet};

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

pub trait JustKV<Key, Value> {
    fn kv(key: Key, value: Value) -> Self;
}
impl<K: std::hash::Hash + Eq, V> JustKV<K, V> for IndexMap<K, V> {
    fn kv(key: K, value: V) -> Self {
        Self::from_iter([(key, value)])
    }
}
impl<K: std::hash::Hash + Eq, V> JustKV<K, V> for HashMap<K, V> {
    fn kv(key: K, value: V) -> Self {
        Self::from_iter([(key, value)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum BlackOrWhite {
        Black,
    }

    #[test]
    fn test_index_set() {
        assert_eq!(
            IndexSet::just(BlackOrWhite::Black),
            IndexSet::<_>::from_iter([BlackOrWhite::Black])
        )
    }

    #[test]
    fn test_hash_set() {
        assert_eq!(
            HashSet::just(BlackOrWhite::Black),
            HashSet::<_>::from_iter([BlackOrWhite::Black])
        )
    }

    #[test]
    fn test_index_map() {
        assert_eq!(
            IndexMap::just((1u8, BlackOrWhite::Black)),
            IndexMap::<_, _>::from_iter([(1u8, BlackOrWhite::Black)])
        )
    }

    #[test]
    fn test_hash_map() {
        assert_eq!(
            HashMap::just((1u8, BlackOrWhite::Black)),
            HashMap::<_, _>::from_iter([(1u8, BlackOrWhite::Black)])
        )
    }
}
