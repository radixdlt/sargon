use std::borrow::Borrow;

use crate::prelude::*;

#[derive(Debug)]
pub struct KeyedInstances<K: Eq + std::fmt::Debug + std::hash::Hash + Clone>(
    pub IndexMap<K, FactorInstances>,
);

impl<K: Eq + std::fmt::Debug + std::hash::Hash + Clone> KeyedInstances<K> {
    pub fn validate_from_source(
        &self,
        factor_source_id: impl Borrow<FactorSourceIDFromHash>,
    ) -> Result<()> {
        if self
            .all_instances()
            .into_iter()
            .any(|f| f.factor_source_id != *factor_source_id.borrow())
        {
            return Err(CommonError::FactorSourceDiscrepancy);
        }
        Ok(())
    }

    pub fn remove(&mut self, key: impl Borrow<K>) -> Option<FactorInstances> {
        self.0.shift_remove(key.borrow())
    }
    pub fn all_instances(&self) -> FactorInstances {
        self.0
            .clone()
            .into_iter()
            .flat_map(|(_, v)| v.factor_instances())
            .collect::<FactorInstances>()
    }
    pub fn new(map: IndexMap<K, FactorInstances>) -> Self {
        Self(map)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<K: Eq + std::fmt::Debug + std::hash::Hash + Clone> IntoIterator for KeyedInstances<K> {
    type Item = <IndexMap<K, FactorInstances> as IntoIterator>::Item;
    type IntoIter = <IndexMap<K, FactorInstances> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub type InstancesByAgnosticPath = KeyedInstances<IndexAgnosticPath>;
pub type InstancesByDerivationPreset = KeyedInstances<DerivationPreset>;
impl InstancesByAgnosticPath {
    pub fn into_derivation_preset(self) -> InstancesByDerivationPreset {
        let map = self
            .into_iter()
            .map(|(k, v)| (DerivationPreset::try_from(k).unwrap(), v))
            .collect::<IndexMap<DerivationPreset, FactorInstances>>();
        InstancesByDerivationPreset::new(map)
    }
}

impl From<FactorInstances> for InstancesByAgnosticPath {
    fn from(value: FactorInstances) -> Self {
        let map = value
            .factor_instances()
            .into_iter()
            .into_group_map_by(|f| f.agnostic_path())
            .into_iter()
            .map(|(k, v)| (k, FactorInstances::from_iter(v)))
            .collect::<IndexMap<IndexAgnosticPath, FactorInstances>>();

        Self::new(map)
    }
}

impl From<FactorInstances> for InstancesByDerivationPreset {
    fn from(value: FactorInstances) -> Self {
        InstancesByAgnosticPath::from(value).into_derivation_preset()
    }
}
