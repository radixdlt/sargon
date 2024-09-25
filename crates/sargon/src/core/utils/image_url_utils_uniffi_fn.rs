use crate::prelude::*;
use crate::types::*;

#[uniffi::export]
pub fn image_url_utils_is_vector_image(
    url: &Url,
    image_type: VectorImageType,
) -> bool {
    is_vector_image(url, image_type)
}

#[uniffi::export]
pub fn image_url_utils_make_image_url(
    url: &str,
    image_service_url: &str,
    width: u32,
    height: u32,
) -> Result<Url> {
    make_image_url(url, image_service_url, width, height)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_image_url_utils_is_vector_image() {
        let url = parse_url("https://svgshare.com/i/U7z.svg").unwrap();
        let image_type = VectorImageType::Svg;

        assert_eq!(
            is_vector_image(&url, image_type),
            image_url_utils_is_vector_image(&url, image_type)
        )
    }

    #[test]
    fn test_image_url_utils_make_image_url() {
        let url = "https://svgshare.com/i/U7z.svg";
        let image_service_url = "https://image-service-dev.extratools.works";
        let width = 1024;
        let height = 1024;

        assert_eq!(
            make_image_url(url, image_service_url, width, height),
            image_url_utils_make_image_url(
                url,
                image_service_url,
                width,
                height
            )
        )
    }
}
