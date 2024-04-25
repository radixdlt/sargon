use crate::prelude::*;

/// An HTTP client for consuming the Radix ⛩️ Gateway API ([docs]).
///
/// A `GatewayClient` needs a "network antenna" to be able to execute the
/// network requests - which is a trait that clients implement on the FFI side
/// (iOS/Android) an "installs" when initiating an instance of the `GatewayClient`.
///
/// The implementing FFI clients can then consume the Radix Gateway API to e.g.
/// fetch the XRD balance of an account address or submit a signed transaction.
///
/// [docs]: https://radix-babylon-gateway-api.redoc.ly/
#[derive(uniffi::Object)]
pub struct GatewayClient {
    /// An object implementing the `NetworkAntenna` traits, which iOS/Android
    /// clients pass into the constructor of this GatewayClient, so that it can
    /// execute network requests.
    pub http_client: HttpClient,

    /// The gateway this GatewayClient talks to, which is a (URL, NetworkID) tuple
    /// essentially.
    pub gateway: Gateway,
}

#[uniffi::export]
impl GatewayClient {
    /// Constructs a new `GatewayClient` with a NetworkAntenna for a specified
    /// `Gateway`.
    #[uniffi::constructor]
    pub fn with_gateway(
        network_antenna: Arc<dyn NetworkAntenna>,
        gateway: Gateway,
    ) -> Self {
        Self {
            http_client: HttpClient { network_antenna },
            gateway,
        }
    }

    /// Constructs a new `GatewayClient` with a NetworkAntenna for a specified
    /// network, by looking up an Radix DLT provided Gateway on that network.
    ///
    /// # Panics
    /// Panics if Radix DLT does not provide a Gateway for the specified
    /// `network_id` - e.g. will panic if you specify `NetworkID::Simulator` (duh).
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
        network_id: NetworkID,
    ) -> Self {
        Self::with_gateway(network_antenna, Gateway::from(network_id))
    }
}

#[cfg(test)]
impl From<()> for BagOfBytes {
    fn from(_value: ()) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use reqwest::Response;
    use std::time::Duration;

    const MAX: Duration = Duration::from_millis(10);

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GatewayClient;

    #[actix_rt::test]
    async fn test_submit_notarized_transaction_mock_duplicate() {
        let mock_antenna =
            MockAntenna::with_response(TransactionSubmitResponse {
                duplicate: true,
            });
        let sut = SUT::new(Arc::new(mock_antenna), NetworkID::Stokenet);
        let req =
            sut.submit_notarized_transaction(NotarizedTransaction::sample());
        let result = timeout(MAX, req).await.unwrap();
        assert_eq!(result, Err(CommonError::GatewaySubmitDuplicateTX { intent_hash: "txid_rdx198k527d5wt4ms5tvrdcu8089v4hptp7ztv388k539uzzvmw25ltsj7u4zz".to_owned() }));
    }
}
