use crate::prelude::*;
use crate::types::*;

#[uniffi::export]
pub fn image_url_utils_is_vector_image(
    url: &str,
    image_type: VectorImageType,
) -> bool {
    sargon::is_vector_image(url, image_type.into_internal())
}

#[uniffi::export]
pub fn image_url_utils_make_image_url(
    url: &str,
    image_service_url: &str,
    width: u32,
    height: u32,
) -> Result<Url> {
    sargon::make_image_url(url, image_service_url, width, height).into_result()
}
