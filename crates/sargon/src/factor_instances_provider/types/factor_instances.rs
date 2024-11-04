use crate::prelude::*;

/// A collection of factor instances.
#[derive(Default, Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("FIS[{:?}]", self.factor_instances)]
pub struct FactorInstances {
    #[allow(dead_code)]
    __hidden: HiddenConstructor,
    factor_instances: IndexSet<HierarchicalDeterministicFactorInstance>,
}

impl FactorInstances {
    pub fn extend(
        &mut self,
        instances: impl IntoIterator<Item = HierarchicalDeterministicFactorInstance>,
    ) {
        let instances = instances.into_iter().collect::<IndexSet<_>>(); // remove duplicates
        self.factor_instances.extend(instances);
    }
    pub fn shift_remove_index(
        &mut self,
        index: usize,
    ) -> HierarchicalDeterministicFactorInstance {
        self.factor_instances
            .shift_remove_index(index)
            .expect("correct index")
    }
    pub fn first(&self) -> Option<HierarchicalDeterministicFactorInstance> {
        self.factor_instances.first().cloned()
    }
    pub fn split_at(self, mid: usize) -> (Self, Self) {
        let instances = self.factor_instances.into_iter().collect_vec();
        let (head, tail) = instances.split_at(mid);
        (Self::from(head), Self::from(tail))
    }
}
impl From<&[HierarchicalDeterministicFactorInstance]> for FactorInstances {
    fn from(value: &[HierarchicalDeterministicFactorInstance]) -> Self {
        Self::from(
            IndexSet::<HierarchicalDeterministicFactorInstance>::from_iter(
                value.iter().cloned(),
            ),
        )
    }
}
impl From<IndexSet<HierarchicalDeterministicFactorInstance>>
    for FactorInstances
{
    fn from(
        instances: IndexSet<HierarchicalDeterministicFactorInstance>,
    ) -> Self {
        Self::new(instances)
    }
}

impl From<FactorInstances>
    for IndexSet<HierarchicalDeterministicFactorInstance>
{
    fn from(value: FactorInstances) -> Self {
        value.factor_instances()
    }
}
impl FactorInstances {
    pub fn append(
        &mut self,
        instances: impl Into<IndexSet<HierarchicalDeterministicFactorInstance>>,
    ) {
        let to_append: IndexSet<_> = instances.into();
        self.factor_instances.extend(to_append);
    }

    pub fn is_empty(&self) -> bool {
        self.factor_instances.is_empty()
    }

    pub fn len(&self) -> usize {
        self.factor_instances.len()
    }
}

impl IntoIterator for FactorInstances {
    type Item = HierarchicalDeterministicFactorInstance;
    type IntoIter = <IndexSet<HierarchicalDeterministicFactorInstance> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.factor_instances().into_iter()
    }
}

impl FromIterator<HierarchicalDeterministicFactorInstance> for FactorInstances {
    fn from_iter<
        I: IntoIterator<Item = HierarchicalDeterministicFactorInstance>,
    >(
        iter: I,
    ) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl FactorInstances {
    pub fn new(
        factor_instances: IndexSet<HierarchicalDeterministicFactorInstance>,
    ) -> Self {
        Self {
            __hidden: HiddenConstructor,
            factor_instances,
        }
    }

    pub fn just(
        factor_instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::new(IndexSet::just(factor_instance))
    }

    pub fn factor_instances(
        &self,
    ) -> IndexSet<HierarchicalDeterministicFactorInstance> {
        self.factor_instances.clone()
    }
}

impl HasSampleValues for FactorInstances {
    fn sample() -> Self {
        Self::new(IndexSet::from_iter([
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0),
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(1),
        ]))
    }

    fn sample_other() -> Self {
        Self::new(IndexSet::from_iter([
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(2),
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(3),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = FactorInstances;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
