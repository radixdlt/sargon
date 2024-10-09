use crate::prelude::*;

#[derive(Clone,  Debug, PartialEq, Eq, uniffi::Enum)]
pub enum VectorImageType {
    Svg,
    Pdf,
}