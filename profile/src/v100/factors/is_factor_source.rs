use crate::prelude::*;

pub trait BaseIsFactorSource:
    Into<FactorSource> + TryFrom<FactorSource>
{
    fn factor_source_kind(&self) -> FactorSourceKind;
    fn factor_source_id(&self) -> FactorSourceID;
}

pub trait IsFactorSource: BaseIsFactorSource {
    fn kind() -> FactorSourceKind;
}
