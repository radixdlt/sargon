use crate::prelude::*;

pub struct NextDerivationEntityIndexCacheAnalyzingAssigner {
    cache: FactorInstancesCache,
}
impl NextDerivationEntityIndexCacheAnalyzingAssigner {
    pub fn new(cache: FactorInstancesCache) -> Self {
        Self { cache }
    }

    fn max(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        index_agnostic_path: IndexAgnosticPath,
    ) -> Option<HDPathComponent> {
        self.cache
            .max_index_for(factor_source_id, index_agnostic_path)
    }

    /// Returns the next index for the given `FactorSourceIDFromHash` and
    /// `IndexAgnosticPath`, by analyzing the cache. In case of read failure
    /// will this method return `Err`, if the cache did not contain any data for
    /// the given `FactorSourceIDFromHash` and `IndexAgnosticPath`, then `Ok(None)` is returned.
    ///
    /// If some index was found, this method returns `max + 1`.
    ///
    /// Can also fail if addition of one would overflow.
    pub fn next(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        index_agnostic_path: IndexAgnosticPath,
    ) -> Result<Option<HDPathComponent>> {
        let max = self.max(factor_source_id, index_agnostic_path);
        let Some(max) = max else { return Ok(None) };
        max.checked_add_one_to_global().map(Some)
    }
}
