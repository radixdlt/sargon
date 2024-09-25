use crate::prelude::*;
use crate::types::*;

pub fn is_vector_image(url: &Url, image_type: VectorImageType) -> bool {
    let url_string = url.to_string();
    let query_parameters = url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<String, String>>();
    let image_url_string = query_parameters
        .get("imageOrigin")
        .map(|s| s.as_str())
        .unwrap_or(&url_string);

    image_url_string
        .starts_with(&format!("data:image/{}", image_type.data_url_type()))
        || image_url_string
            .to_lowercase()
            .ends_with(image_type.url_extension())
}

pub fn make_image_url(
    url: &str,
    image_service_url: &str,
    width: u32,
    height: u32,
) -> Result<Url> {
    const MIN_SIZE: u32 = 64;

    let parsed_url = parse_url(url)?;
    let image_origin = url_encode(url);
    let image_size =
        format!("{}x{}", width.max(MIN_SIZE), height.max(MIN_SIZE));
    let mut image_url = format!(
        "{}/?imageOrigin={}&imageSize={}",
        image_service_url, image_origin, image_size
    );

    if is_vector_image(&parsed_url, VectorImageType::Svg) {
        image_url.push_str("&format=png");
    }

    parse_url(image_url)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_vector_image() {
        let svg_url = parse_url("https://svgshare.com/i/U7z.svg").unwrap();
        let pdf_url = parse_url("https://example.com/image.pdf").unwrap();
        let svg_data_url = parse_url(
            "data:image/svg+xml,%3Csvg%20viewBox%3D%220%200%201000%201000%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%3E%0A%3Cpolygon%20fill%3D%22hsla%2890%2C99%25%2C52%25%2C1%29%22%20points%3D%220%2C%200%2C%201000%2C%201000%2C%200%2C%201000%22%20transform%3D%22scale%28-1%2C1%29%20translate%28-1000%29%22%2F%3E%0A%3Cpolygon%20fill%3D%22hsla%28199%2C90%25%2C64%25%2C1%29%22%20points%3D%221000%2C%201000%2C%201000%2C%200%2C%200%2C%200%22%20transform%3D%22scale%28-1%2C1%29%20translate%28-1000%29%22%2F%3E%0A%3Cpath%20d%3D%22M1000%2C229%20A1000%2C1000%2C0%2C0%2C0%2C229%2C1000%20L1000%2C1000%20z%22%20fill%3D%22hsla%28140%2C98%25%2C61%25%2C1%29%22%2F%3E%0A%3Cpath%20d%3D%22M392%2C500%20L608%2C500%20M500%2C392%20L500%2C608%22%20stroke%3D%22hsla%2847%2C92%25%2C61%25%2C1%29%22%20stroke-width%3D%2272%22%2F%3E%0A%3C%2Fsvg%3E"
        )
        .unwrap();
        let pdf_data_url = parse_url("data:image/pdf,dummydata").unwrap();

        assert!(is_vector_image(&svg_url, VectorImageType::Svg));
        assert!(is_vector_image(&pdf_url, VectorImageType::Pdf));
        assert!(is_vector_image(&svg_data_url, VectorImageType::Svg));
        assert!(is_vector_image(&pdf_data_url, VectorImageType::Pdf));
    }

    #[test]
    fn test_is_vector_image_with_image_origin_url() {
        let url = parse_url("https://image-service-dev.extratools.works/?imageOrigin=https%3A%2F%2Fsvgshare.com%2Fi%2FU7z.svg&imageSize=1024x1024&format=png").unwrap();

        assert!(is_vector_image(&url, VectorImageType::Svg));
    }

    #[test]
    fn test_is_vector_image_with_image_origin_data_url() {
        let data_url = parse_url(
            "https://image-service-dev.extratools.works/?imageOrigin=data%3Aimage%2Fsvg%2Bxml%2C%253Csvg%2520viewBox%253D%25220%25200%25201000%25201000%2522%2520xmlns%253D%2522http%253A%252F%252Fwww.w3.org%252F2000%252Fsvg%2522%253E%250A%253Cpolygon%2520fill%253D%2522hsla%252890%252C99%2525%252C52%2525%252C1%2529%2522%2520points%253D%25220%252C%25200%252C%25201000%252C%25201000%252C%25200%252C%25201000%2522%2520transform%253D%2522scale%2528-1%252C1%2529%2520translate%2528-1000%2529%2522%252F%253E%250A%253Cpolygon%2520fill%253D%2522hsla%2528199%252C90%2525%252C64%2525%252C1%2529%2522%2520points%253D%25221000%252C%25201000%252C%25201000%252C%25200%252C%25200%252C%25200%2522%2520transform%253D%2522scale%2528-1%252C1%2529%2520translate%2528-1000%2529%2522%252F%253E%250A%253Cpath%2520d%253D%2522M1000%252C229%2520A1000%252C1000%252C0%252C0%252C0%252C229%252C1000%2520L1000%252C1000%2520z%2522%2520fill%253D%2522hsla%2528140%252C98%2525%252C61%2525%252C1%2529%2522%252F%253E%250A%253Cpath%2520d%253D%2522M392%252C500%2520L608%252C500%2520M500%252C392%2520L500%252C608%2522%2520stroke%253D%2522hsla%252847%252C92%2525%252C61%2525%252C1%2529%2522%2520stroke-width%253D%252272%2522%252F%253E%250A%253C%252Fsvg%253E&imageSize=1024x1024&format=png"
        )
        .unwrap();

        assert!(is_vector_image(&data_url, VectorImageType::Svg));
    }

    #[test]
    fn test_make_image_url_svg_url() {
        let image_service_url = "https://image-service-dev.extratools.works";
        let image_origin_url = "https://svgshare.com/i/U7z.svg";

        pretty_assertions::assert_eq!(
            make_image_url(image_origin_url, image_service_url, 1024, 1024).unwrap().to_string(),
            "https://image-service-dev.extratools.works/?imageOrigin=https%3A%2F%2Fsvgshare.com%2Fi%2FU7z.svg&imageSize=1024x1024&format=png".to_string()
        );
    }

    #[test]
    fn test_make_image_url_svg_data_url() {
        let image_service_url = "https://image-service-dev.extratools.works";
        let image_origin_data_url = "data:image/svg+xml,%3Csvg%20viewBox%3D%220%200%201000%201000%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%3E%0A%3Cpolygon%20fill%3D%22hsla%2890%2C99%25%2C52%25%2C1%29%22%20points%3D%220%2C%200%2C%201000%2C%201000%2C%200%2C%201000%22%20transform%3D%22scale%28-1%2C1%29%20translate%28-1000%29%22%2F%3E%0A%3Cpolygon%20fill%3D%22hsla%28199%2C90%25%2C64%25%2C1%29%22%20points%3D%221000%2C%201000%2C%201000%2C%200%2C%200%2C%200%22%20transform%3D%22scale%28-1%2C1%29%20translate%28-1000%29%22%2F%3E%0A%3Cpath%20d%3D%22M1000%2C229%20A1000%2C1000%2C0%2C0%2C0%2C229%2C1000%20L1000%2C1000%20z%22%20fill%3D%22hsla%28140%2C98%25%2C61%25%2C1%29%22%2F%3E%0A%3Cpath%20d%3D%22M392%2C500%20L608%2C500%20M500%2C392%20L500%2C608%22%20stroke%3D%22hsla%2847%2C92%25%2C61%25%2C1%29%22%20stroke-width%3D%2272%22%2F%3E%0A%3C%2Fsvg%3E";

        pretty_assertions::assert_eq!(
            make_image_url(image_origin_data_url, image_service_url, 1024, 1024).unwrap().to_string(),
            "https://image-service-dev.extratools.works/?imageOrigin=data%3Aimage%2Fsvg%2Bxml%2C%253Csvg%2520viewBox%253D%25220%25200%25201000%25201000%2522%2520xmlns%253D%2522http%253A%252F%252Fwww.w3.org%252F2000%252Fsvg%2522%253E%250A%253Cpolygon%2520fill%253D%2522hsla%252890%252C99%2525%252C52%2525%252C1%2529%2522%2520points%253D%25220%252C%25200%252C%25201000%252C%25201000%252C%25200%252C%25201000%2522%2520transform%253D%2522scale%2528-1%252C1%2529%2520translate%2528-1000%2529%2522%252F%253E%250A%253Cpolygon%2520fill%253D%2522hsla%2528199%252C90%2525%252C64%2525%252C1%2529%2522%2520points%253D%25221000%252C%25201000%252C%25201000%252C%25200%252C%25200%252C%25200%2522%2520transform%253D%2522scale%2528-1%252C1%2529%2520translate%2528-1000%2529%2522%252F%253E%250A%253Cpath%2520d%253D%2522M1000%252C229%2520A1000%252C1000%252C0%252C0%252C0%252C229%252C1000%2520L1000%252C1000%2520z%2522%2520fill%253D%2522hsla%2528140%252C98%2525%252C61%2525%252C1%2529%2522%252F%253E%250A%253Cpath%2520d%253D%2522M392%252C500%2520L608%252C500%2520M500%252C392%2520L500%252C608%2522%2520stroke%253D%2522hsla%252847%252C92%2525%252C61%2525%252C1%2529%2522%2520stroke-width%253D%252272%2522%252F%253E%250A%253C%252Fsvg%253E&imageSize=1024x1024&format=png".to_string()
        );
    }

    #[test]
    fn test_make_image_url_not_svg() {
        let image_service_url = "https://image-service-dev.extratools.works";
        let image_origin_url = "https://sgo4bmuvgu4t24bvdcfbndmnxigspezdsnzoevon2jb5odru7auq.arweave.net/kZ3AspU1OT1wNRiKFo2Nug0nkyOTcuJVzdJD1w40-Ck";

        pretty_assertions::assert_eq!(
            make_image_url(image_origin_url, image_service_url, 1024, 1024).unwrap().to_string(),
            "https://image-service-dev.extratools.works/?imageOrigin=https%3A%2F%2Fsgo4bmuvgu4t24bvdcfbndmnxigspezdsnzoevon2jb5odru7auq.arweave.net%2FkZ3AspU1OT1wNRiKFo2Nug0nkyOTcuJVzdJD1w40-Ck&imageSize=1024x1024".to_string()
        );
    }
}
