use crate::prelude::*;

pub trait BaseBaseIsFactorSource {
    fn factor_source_kind(&self) -> FactorSourceKind;
    fn factor_source_id(&self) -> FactorSourceID;

    fn id_from_hash(&self) -> FactorSourceIDFromHash {
        FactorSourceIDFromHash::try_from(self.factor_source_id()).unwrap()
    }

    fn common_properties(&self) -> FactorSourceCommon;
    fn set_common_properties(&mut self, updated: FactorSourceCommon);

    fn supports_babylon(&self) -> bool {
        self.common_properties().supports_babylon()
    }
    fn supports_olympia(&self) -> bool {
        self.common_properties().supports_olympia()
    }

    fn name(&self) -> String;

    fn category(&self) -> FactorSourceCategory {
        self.factor_source_kind().category()
    }
}

pub trait BaseIsFactorSource:
    BaseBaseIsFactorSource + Into<FactorSource> + TryFrom<FactorSource> + Clone
{
}

impl<
        T: BaseBaseIsFactorSource
            + Into<FactorSource>
            + TryFrom<FactorSource>
            + Clone,
    > BaseIsFactorSource for T
{
}

pub trait IsFactorSource: BaseIsFactorSource {
    fn kind() -> FactorSourceKind;
}
