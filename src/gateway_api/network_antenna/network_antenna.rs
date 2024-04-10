use crate::prelude::*;

/// Trait for executing network requests, to be  implemented on the FFI side
/// (iOS/Android), so that Sargon can with some HTTP client perform make network
/// requests.
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NetworkAntenna: Send + Sync {
    /// Method invoked by Sargon Rust side, telling an implementing type to
    /// execute a `NetworkRequest` and pass the Result back to Sargon Rust side.
    ///
    /// Either: `Err` or `Ok(NetworkResponse)`.
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkResponse, CommonError>;
}
