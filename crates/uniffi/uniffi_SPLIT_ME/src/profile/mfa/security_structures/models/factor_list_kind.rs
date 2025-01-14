use crate::prelude::*;
use sargon::FactorListKind as InternalFactorListKind;

/// A kind of factor list, either threshold, or override kind.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum FactorListKind {
    Threshold,
    Override,
}
