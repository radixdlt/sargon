use crate::{factor_instances_cache::FICStorage, prelude::*};

pub type DenseKeyStorage = IndexMap<
    FactorSourceIDFromHashDenseKey,
    IndexMap<IndexAgnosticPath, IndexSet<HierarchicalDeterministicPublicKey>>,
>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FactorInstancesCacheSnapshot(pub DenseKeyStorage);
impl FactorInstancesCacheSnapshot {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl From<FactorInstancesCacheSnapshot> for FICStorage {
    fn from(value: FactorInstancesCacheSnapshot) -> Self {
        value
            .0
            .into_iter()
            .map(|(k, v)| {
                let key = FactorSourceIDFromHash::from(k);
                let value = v.into_iter().map(|(l, u)| {
                    (
                        l,
                        u.into_iter()
                            .map(|x| HierarchicalDeterministicFactorInstance::new(key, x))
                            .collect::<FactorInstances>(),
                    )
                }).collect::<IndexMap<IndexAgnosticPath, FactorInstances>>();
                (key, value)
            })
            .collect::<FICStorage>()
    }
}
impl From<FactorInstancesCacheSnapshot> for FactorInstancesCache {
    fn from(value: FactorInstancesCacheSnapshot) -> Self {
        Self::with_storage(FICStorage::from(value))
    }
}
impl From<FICStorage> for FactorInstancesCacheSnapshot {
    fn from(value: FICStorage) -> Self {
        Self(
            value
                .into_iter()
                .map(|(k, v)| {
                    let key = FactorSourceIDFromHashDenseKey::from(k);
                    let value = v
                        .into_iter()
                        .map(|(l, u)| {
                            (
                                    l,
                                    u.into_iter()
                                        .map(|x| x.hd_public_key())
                                        .collect::<IndexSet<
                                        HierarchicalDeterministicPublicKey,
                                    >>(),
                                )
                        })
                        .collect::<IndexMap<
                            IndexAgnosticPath,
                            IndexSet<HierarchicalDeterministicPublicKey>,
                        >>();
                    (key, value)
                })
                .collect::<DenseKeyStorage>(),
        )
    }
}
