use crate::prelude::*;

impl<V: std::hash::Hash + PartialEq + Eq + Clone> From<IndexSet<V>>
    for OrderedSet<V>
{
    fn from(value: IndexSet<V>) -> Self {
        Self(value)
    }
}
impl<V: std::hash::Hash + PartialEq + Eq + Clone> std::ops::Deref
    for OrderedSet<V>
{
    type Target = IndexSet<V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V: std::hash::Hash + PartialEq + Eq + Clone> Default for OrderedSet<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: std::hash::Hash + PartialEq + Eq + Clone> std::hash::Hash
    for OrderedSet<V>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let vec = self.0.clone().into_iter().collect_vec();
        vec.hash(state)
    }
}
