use crate::prelude::*;

pub trait BaseIsFactorSource:
    Into<FactorSource>
    + TryFrom<FactorSource>
    + Clone
    + HasFactorSourceKindObjectSafe
{
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
}

impl<T: IsFactorSource> HasFactorSourceKindObjectSafe for T {
    fn get_factor_source_kind(&self) -> FactorSourceKind {
        Into::<FactorSource>::into(self.clone()).get_factor_source_kind()
    }
}

pub trait IsFactorSource: BaseIsFactorSource + HasFactorSourceKind {}

impl<T: BaseIsFactorSource + HasFactorSourceKind> IsFactorSource for T {}
