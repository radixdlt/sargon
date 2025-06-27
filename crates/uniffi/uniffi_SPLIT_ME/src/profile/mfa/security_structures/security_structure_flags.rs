use crate::prelude::*;
use sargon::SecurityStructureFlag as InternalSecurityStructureFlag;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum SecurityStructureFlag {
    /// Used to mark a Security Shield as "main". We can only have one.
    Main,
}
