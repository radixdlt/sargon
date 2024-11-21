use crate::prelude::*;

/// A collection of `HierarchicalDeterministicFactorInstance` derived from a
/// factor source.
#[derive(Debug)]
pub(crate) struct Keyring {
    pub(crate) factor_source_id: FactorSourceIDFromHash,
    pub(crate) paths: IndexSet<DerivationPath>,
    derived: RwLock<IndexSet<HierarchicalDeterministicFactorInstance>>,
}

impl Keyring {
    pub fn clone_snapshot(&self) -> Self {
        Self {
            factor_source_id: self.factor_source_id,
            paths: self.paths.clone(),
            derived: RwLock::new(self.derived.try_read().unwrap().clone()),
        }
    }
    pub(crate) fn new(
        factor_source_id: FactorSourceIDFromHash,
        paths: IndexSet<DerivationPath>,
    ) -> Self {
        Self {
            factor_source_id,
            paths,
            derived: RwLock::new(IndexSet::new()),
        }
    }
    pub(crate) fn factors(
        &self,
    ) -> IndexSet<HierarchicalDeterministicFactorInstance> {
        self.derived.try_read().unwrap().clone()
    }

    pub(crate) fn process_response(
        &self,
        response: IndexSet<HierarchicalDeterministicFactorInstance>,
    ) {
        assert!(response.iter().all(|f| f.factor_source_id
            == self.factor_source_id
            && !self
                .derived
                .try_read()
                .unwrap()
                .iter()
                .any(|x| x.public_key == f.public_key)));

        self.derived.try_write().unwrap().extend(response)
    }
}
