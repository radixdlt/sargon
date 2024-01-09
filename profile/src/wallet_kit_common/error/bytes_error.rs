use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, uniffi::Error)]
#[uniffi(flat_error)]
pub enum BytesError {
    #[error("String not hex")]
    StringNotHex,

    #[error("Invalid byte count, expected 32.")]
    InvalidByteCountExpected32,
}
