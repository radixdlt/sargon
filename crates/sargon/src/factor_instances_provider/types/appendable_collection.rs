use crate::prelude::*;
use std::borrow::Borrow;

pub trait AppendableCollection: FromIterator<Self::Element> {
    type Element;
    fn append<T: IntoIterator<Item = Self::Element>>(&mut self, iter: T);
}
impl<V: Eq + std::hash::Hash> AppendableCollection for IndexSet<V> {
    type Element = V;

    fn append<T: IntoIterator<Item = Self::Element>>(&mut self, iter: T) {
        self.extend(iter)
    }
}

impl AppendableCollection for FactorInstances {
    type Element = HierarchicalDeterministicFactorInstance;

    fn append<T: IntoIterator<Item = Self::Element>>(&mut self, iter: T) {
        // self.extend(iter)
        todo!()
    }
}

pub trait AppendableMap {
    type Key: Eq + std::hash::Hash + Clone;
    type AC: AppendableCollection;
    fn append_or_insert_to<
        I: IntoIterator<Item = <Self::AC as AppendableCollection>::Element>,
    >(
        &mut self,
        key: impl Borrow<Self::Key>,
        items: I,
    );

    fn append_or_insert_element_to(
        &mut self,
        key: impl Borrow<Self::Key>,
        element: <Self::AC as AppendableCollection>::Element,
    ) {
        self.append_or_insert_to(key.borrow(), [element]);
    }
}

impl<K, V> AppendableCollection for IndexMap<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    type Element = (K, V);

    fn append<T: IntoIterator<Item = Self::Element>>(&mut self, iter: T) {
        self.extend(iter)
    }
}

impl<K, V> AppendableMap for IndexMap<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: AppendableCollection,
{
    type Key = K;
    type AC = V;
    fn append_or_insert_to<
        I: IntoIterator<Item = <Self::AC as AppendableCollection>::Element>,
    >(
        &mut self,
        key: impl Borrow<Self::Key>,
        items: I,
    ) {
        let key = key.borrow();
        if let Some(existing) = self.get_mut(key) {
            existing.append(items);
        } else {
            self.insert(key.clone(), V::from_iter(items));
        }
    }
}

#[cfg(test)]
mod test_appendable_collection {
    use super::*;

    #[test]
    fn test_append_element() {
        type Sut = IndexMap<i8, IndexSet<u8>>;
        let mut map = Sut::new();
        map.append_or_insert_element_to(-3, 5);
        map.append_or_insert_element_to(-3, 6);
        map.append_or_insert_element_to(-3, 6);
        map.append_or_insert_to(-3, [42, 237]);
        map.append_or_insert_to(-9, [64, 128]);
        assert_eq!(
            map,
            Sut::from_iter([
                (-3, IndexSet::<u8>::from_iter([5, 6, 42, 237])),
                (-9, IndexSet::<u8>::from_iter([64, 128])),
            ])
        );
    }
}
