use crate::prelude::*;

pub trait BaseIsFactorSource:
    Into<FactorSource> + TryFrom<FactorSource> + Clone
{
    fn factor_source_kind(&self) -> FactorSourceKind;
    fn factor_source_id(&self) -> FactorSourceID;

    fn id_from_hash(&self) -> FactorSourceIDFromHash {
        self.factor_source_id()
            .into_hash()
            .map_err(|_| CommonError::FactorSourceIDNotFromHash)
            .unwrap()
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

pub trait IsFactorSource: BaseIsFactorSource {
    fn kind() -> FactorSourceKind;
}
