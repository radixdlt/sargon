use crate::prelude::*;

#[uniffi::export]
pub fn event_kind(event: &Event) -> EventKind {
    event.kind()
}
