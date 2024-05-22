use crate::prelude::*;
use log::LevelFilter;

/// A trait used to mark that a values is safe to (debug) log, and
/// which debug representation of it to use.
///
/// You MUST NOT return sensitive information by implementing types,
/// e.g. PrivateKeys or Mnemonics.
pub trait SafeToLog {
    /// A safe to log representation of a type, MUST NOT contains sensitive information.
    fn non_sensitive(&self) -> impl std::fmt::Debug;
}
