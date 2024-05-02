// use super::relay_service::Service as RelayService;
use super::deep_link_parsing::*;
use crate::prelude::*;

/// The Radix Connect Mobile client.
/// This is the object that will be used by the mobile app to handle interactions sent over Radix Connect Relay.
// #[derive(uniffi::Object)]
// pub struct RadixConnectMobile {}

// // Provisional API
// #[uniffi::export]
// impl RadixConnectMobile {
//     // RadixConnectMobile should require a NetworkAntenna and a SecureStorageDriver from the Wallet.
//     // The internal components, such as RadixConnectRelayService will be created by the RadixConnectMobile.
//     #[uniffi::constructor]
//     pub fn new(
//         _network_antenna: Arc<dyn NetworkAntenna>,
//         _secure_storage: Arc<dyn SecureStorageDriver>,
//     ) -> Self {
//         todo!()
//     }

//     #[uniffi::method]
//     pub fn handle_linking_request(
//         &self,
//         _request: RadixConnectMobileLinkRequest,
//     ) -> Result<Url> {
//         todo!()
//     }

//     #[uniffi::method]
//     pub fn handle_dapp_interaction_request(
//         &self,
//         _request: RadixConnectMobileDappRequest,
//     ) -> Result<DappToWalletInteraction> {
//         todo!()
//     }

//     #[uniffi::method]
//     pub fn send_dapp_interaction_response(
//         &self,
//         _response: WalletToDappInteractionResponse,
//     ) -> Result<Url> {
//         todo!()
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
