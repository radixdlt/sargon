use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NetworkResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: BagOfBytes,
}
