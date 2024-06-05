use crate::prelude::*;

impl<V: std::hash::Hash + PartialEq + Eq + Clone> FromIterator<V>
    for OrderedSet<V>
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut set = IndexSet::<V>::new();
        for item in iter {
            let _ = set.insert(item);
        }
        Self::from(set)
    }
}

impl<V: std::hash::Hash + PartialEq + Eq + Clone> IntoIterator
    for OrderedSet<V>
{
    type Item = V;
    type IntoIter = <IndexSet<V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<V: std::hash::Hash + PartialEq + Eq + Clone> IntoIterator
    for &OrderedSet<V>
{
    type Item = V;
    type IntoIter = <IndexSet<V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}