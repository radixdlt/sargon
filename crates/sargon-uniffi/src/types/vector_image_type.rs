use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum VectorImageType {
    Svg,
    Pdf,
}