use crate::prelude::*;
use sargon::VectorImageType as InternalVectorImageType;

#[derive(Clone, PartialEq, Eq, InternalConversionV2, uniffi::Enum)]
pub enum VectorImageType {
    Svg,
    Pdf,
}
