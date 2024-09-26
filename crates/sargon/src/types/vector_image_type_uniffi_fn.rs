use crate::prelude::*;
use crate::types::*;

#[uniffi::export]
pub fn vector_image_type_url_extension(image_type: VectorImageType) -> String {
    image_type.url_extension().to_string()
}

#[uniffi::export]
pub fn vector_image_type_data_url_type(image_type: VectorImageType) -> String {
    image_type.data_url_type().to_string()
}

#[uniffi::export]
pub fn new_vector_image_type_sample() -> VectorImageType {
    VectorImageType::sample()
}

#[uniffi::export]
pub fn new_vector_image_type_sample_other() -> VectorImageType {
    VectorImageType::sample_other()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = VectorImageType;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_vector_image_type_sample());
        assert_eq!(SUT::sample_other(), new_vector_image_type_sample_other());
    }

    #[test]
    fn test_vector_image_type_url_extension() {
        let sut = SUT::sample();
        assert_eq!(sut.url_extension(), vector_image_type_url_extension(sut));
    }

    #[test]
    fn test_vector_image_type_data_url_type() {
        let sut = SUT::sample();
        assert_eq!(sut.data_url_type(), vector_image_type_data_url_type(sut));
    }
}
