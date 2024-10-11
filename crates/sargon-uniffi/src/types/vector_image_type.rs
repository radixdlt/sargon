use crate::prelude::*;
use sargon::VectorImageType as InternalVectorImageType;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum VectorImageType {
    Svg,
    Pdf,
}

impl From<InternalVectorImageType> for VectorImageType {
    fn from(value: InternalVectorImageType) -> Self {
        match value {
            InternalVectorImageType::Svg => VectorImageType::Svg,
            InternalVectorImageType::Pdf => VectorImageType::Pdf,
        }
    }
}

impl Into<InternalVectorImageType> for VectorImageType {
    fn into(self) -> InternalVectorImageType {
        match self {
            VectorImageType::Svg => InternalVectorImageType::Svg,
            VectorImageType::Pdf => InternalVectorImageType::Pdf,
        }
    }
}
