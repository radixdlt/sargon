use crate::prelude::*;
use sargon::VectorImageType as InternalVectorImageType;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum VectorImageType {
    Svg,
    Pdf,
}

#[uniffi::export]
pub fn vector_image_type_url_extension(image_type: VectorImageType) -> String {
    image_type.into_internal().url_extension().to_string()
}

#[uniffi::export]
pub fn vector_image_type_data_url_type(image_type: VectorImageType) -> String {
    image_type.into_internal().data_url_type().to_string()
}

#[uniffi::export]
pub fn new_vector_image_type_sample() -> VectorImageType {
    InternalVectorImageType::sample().into()
}

#[uniffi::export]
pub fn new_vector_image_type_sample_other() -> VectorImageType {
    InternalVectorImageType::sample_other().into()
}