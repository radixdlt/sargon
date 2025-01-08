use crate::prelude::*;

pub trait HasFactorSources {
    fn factor_sources(&self) -> IndexSet<FactorSource>;
}
