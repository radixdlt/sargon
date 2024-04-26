// use super::relay_service::Service as RelayService;
use super::deep_link_parsing::RadixConnectMobileConnectRequest;
use crate::prelude::*;

// /// The Radix Connect Mobile client.
// /// This is the object that will be used by the mobile app to handle interactions sent over Radix Connect Relay.
// #[derive(uniffi::Object)]
// pub struct RadixConnectMobile {
//     relay_service: RelayService,
// }

// impl RadixConnectMobile {
//     #[uniffi::constructor]
//     pub fn new(network_antenna: Arc<dyn NetworkAntenna>) -> Self {
//         Self {
//             relay_service: RelayService::new(HttpClient { network_antenna }),
//         }
//     }
// }

#[uniffi::export]
pub fn new_mobile_connect_request(
    url: String,
) -> Result<RadixConnectMobileConnectRequest> {
    RadixConnectMobileConnectRequest::from_str(url.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_mobile_connect_request() {
        let uuid = Uuid::new_v4().to_string();
        let connect_url = format!("https://d1rxdfxrfmemlj.cloudfront.net/?sessionId={}&origin=radix%3A%2F%2Fapp", uuid);
        assert!(new_mobile_connect_request(connect_url).is_ok());
    }
}
