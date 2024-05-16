use radix_rust::prelude::IndexMap;

use crate::prelude::*;

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> IdentifiedVecOf<V> {
    #[inline]
    pub fn iter(&self) -> IdentifiedVecOfIterator<V> {
        IdentifiedVecOfIterator {
            ordered_map: self,
            index: 0,
        }
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> FromIterator<V>
    for IdentifiedVecOf<V>
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut map = IndexMap::<<V as Identifiable>::ID, V>::new();
        for item in iter {
            let _ = map.insert(item.id(), item);
        }
        Self::from(map)
    }
}

impl<'a, V: Debug + PartialEq + Eq + Clone + Identifiable> IntoIterator
    for &'a IdentifiedVecOf<V>
{
    type Item = V;
    type IntoIter = IdentifiedVecOfIterator<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        IdentifiedVecOfIterator {
            ordered_map: self,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifiedVecOfIterator<
    'a,
    V: Debug + PartialEq + Eq + Clone + Identifiable,
> {
    ordered_map: &'a IdentifiedVecOf<V>,
    index: usize,
}

impl<'a, V: Debug + PartialEq + Eq + Clone + Identifiable> Iterator
    for IdentifiedVecOfIterator<'a, V>
{
    type Item = V;

    fn next(&mut self) -> Option<V> {
        if self.index < self.ordered_map.len() {
            let elem = self.ordered_map.0.get_index(self.index);
            self.index += 1;
            elem.map(|pair| pair.1.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::User;
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn into_from_iter() {
        let sut = SUT::sample();
        let iter = sut.into_iter();
        let from_iter = SUT::from_iter(iter);
        assert_eq!(from_iter, sut)
    }

    #[test]
    fn iter() {
        let sut = SUT::sample_other();
        assert_eq!(
            sut.into_iter().collect_vec(),
            vec![User::bob(), User::david(), User::frank()]
        )
    }
}
