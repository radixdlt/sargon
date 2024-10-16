use sbor::prelude::indexmap::IndexSet;

use crate::prelude::*;

pub struct HDFactorSources(IndexMap<FactorSourceIDFromHash, FactorSource>);

impl HDFactorSources {
    pub fn new(factors: impl IntoIterator<Item = FactorSource>) -> Self {
        Self(
            factors
                .into_iter()
                .filter_map(|fs| {
                    fs.factor_source_id().as_hash().cloned().map(|id| (id, fs))
                })
                .collect::<IndexMap<FactorSourceIDFromHash, FactorSource>>(),
        )
    }
}

impl IntoIterator for HDFactorSources {
    type Item = (FactorSourceIDFromHash, FactorSource);
    type IntoIter =
        <IndexMap<FactorSourceIDFromHash, FactorSource> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<FactorSource> for HDFactorSources {
    fn from_iter<I: IntoIterator<Item = FactorSource>>(iter: I) -> Self {
        Self::new(iter)
    }
}

impl Deref for HDFactorSources {
    type Target = IndexMap<FactorSourceIDFromHash, FactorSource>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
