use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid Account Address '{0}'.")]
    InvalidAccountAddress(String),
}
