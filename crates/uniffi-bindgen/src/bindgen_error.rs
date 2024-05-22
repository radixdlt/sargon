use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum BindgenError {
    #[error("Failed to parse 'generate' subcommand args")]
    ParseGenerateSubcommandArgs,

    #[error("Failed to read {path}, reason: {reason}")]
    ReadFile { path: String, reason: String },

    #[error("Failed to write {path}, reason: {reason}")]
    WriteFile { path: String, reason: String },
}
