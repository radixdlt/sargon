use crate::prelude::*;
use sargon::NetworkResponse as InternalNetworkResponse;

#[derive(Clone, PartialEq, Eq, InternalConversionV2, uniffi::Record)]
pub struct NetworkResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: BagOfBytes,
}
