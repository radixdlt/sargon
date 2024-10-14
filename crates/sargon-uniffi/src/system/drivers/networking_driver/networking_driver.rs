use crate::prelude::*;
use sargon::NetworkRequest as InternalNetworkRequest;
use sargon::NetworkResponse as InternalNetworkResponse;
use sargon::NetworkingDriver as InternalNetworkingDriver;
use sargon::Result as InternalResult;

/// Trait for executing network requests, to be implemented by hosts, so that
/// Sargon can make network requests with some HTTP client.
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NetworkingDriver: Send + Sync + std::fmt::Debug {
    /// Method invoked by Sargon Rust side, telling an implementing type to
    /// execute a `NetworkRequest` and pass the Result back to Sargon Rust side.
    ///
    /// Either: `Err` or `Ok(NetworkResponse)`.
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse, CommonError>;
}

#[derive(Debug)]
pub struct NetworkingDriverAdapter {
    pub wrapped: Arc<dyn NetworkingDriver>,
}

#[async_trait::async_trait]
impl InternalNetworkingDriver for NetworkingDriverAdapter {
    async fn execute_network_request(
        &self,
        request: InternalNetworkRequest,
    ) -> InternalResult<InternalNetworkResponse> {
            self.wrapped.execute_network_request(request.into()).await.into_internal_result()
    }
}
