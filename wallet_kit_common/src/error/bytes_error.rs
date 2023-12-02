use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BytesError {
    #[error("String not hex")]
    StringNotHex,

    #[error("Invalid byte count, expected 32.")]
    InvalidByteCountExpected32,
}
