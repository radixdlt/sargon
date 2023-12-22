use super::{
    factor_source::FactorSource, factor_source_id::FactorSourceID,
    factor_source_kind::FactorSourceKind,
};

pub trait IsFactorSource: Into<FactorSource> + TryFrom<FactorSource> {
    fn factor_source_kind(&self) -> FactorSourceKind;
    fn factor_source_id(&self) -> FactorSourceID;
}
