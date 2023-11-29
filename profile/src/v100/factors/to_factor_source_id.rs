use super::factor_source_id::FactorSourceID;

pub trait ToFactorSourceID {
    fn embed(&self) -> FactorSourceID;
}
