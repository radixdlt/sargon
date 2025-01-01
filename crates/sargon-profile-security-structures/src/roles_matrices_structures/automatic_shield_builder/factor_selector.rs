use crate::prelude::*;

/// A tiny enum to make it possible to filter FactorSources on either
/// FactorSourceCategory or FactorSourceKind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorSelector {
    Category(FactorSourceCategory),
    Kind(FactorSourceKind),
}
