use crate::prelude::*;
use sargon::ChangeSource as InternalChangeSource;

/// Created by the visitor, generally references a particular instruction,
/// or maybe an initial YIELD_TO_PARENT.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum ChangeSource {
    InitialYieldFromParent,
    Invocation { instruction_index: u64 },
    NewBucket { instruction_index: u64 },
    Assertion { instruction_index: u64 },
}
