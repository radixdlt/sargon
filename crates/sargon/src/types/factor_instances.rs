use std::ops::DerefMut;

use crate::prelude::*;

pub struct FactorInstances(IndexSet<HierarchicalDeterministicFactorInstance>);

impl FactorInstances {
    pub fn new(
        factors: impl IntoIterator<Item = HierarchicalDeterministicFactorInstance>,
    ) -> Self {
        Self(factors.into_iter().collect::<IndexSet<_>>())
    }
}

impl IntoIterator for FactorInstances {
    type Item = HierarchicalDeterministicFactorInstance;
    type IntoIter = <IndexSet<HierarchicalDeterministicFactorInstance> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<HierarchicalDeterministicFactorInstance> for FactorInstances {
    fn from_iter<
        I: IntoIterator<Item = HierarchicalDeterministicFactorInstance>,
    >(
        iter: I,
    ) -> Self {
        Self::new(iter)
    }
}

impl Deref for FactorInstances {
    type Target = IndexSet<HierarchicalDeterministicFactorInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FactorInstances {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
