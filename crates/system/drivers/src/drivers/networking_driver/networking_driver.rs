use crate::prelude::*;

/// Trait for executing network requests, to be implemented by hosts, so that
/// Sargon can make network requests with some HTTP client.
#[async_trait::async_trait]
pub trait NetworkingDriver: Send + Sync {
    /// Method invoked by Sargon Rust side, telling an implementing type to
    /// execute a `NetworkRequest` and pass the Result back to Sargon Rust side.
    ///
    /// Either: `Err` or `Ok(NetworkResponse)`.
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse, CommonError>;
}
