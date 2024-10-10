use crate::prelude::*;

#[derive(Clone,   PartialEq, Eq, uniffi::Enum)]
pub enum VectorImageType {
    Svg,
    Pdf,
}