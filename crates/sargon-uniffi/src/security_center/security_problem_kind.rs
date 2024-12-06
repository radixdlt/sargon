use crate::prelude::*;
use sargon::SecurityProblemKind as InternalSecurityProblemKind;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
/// An enum describing the different types of Security Problems the Wallet can encounter.
pub enum SecurityProblemKind {
    SecurityFactors,

    ConfigurationBackup,

    SecurityShields,
}
