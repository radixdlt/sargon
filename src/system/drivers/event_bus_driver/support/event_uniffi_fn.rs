use crate::prelude::*;

#[uniffi::export]
pub fn event_kind(event: &Event) -> EventKind {
    event.kind()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Event;

    #[test]
    fn test_kind() {
        assert_eq!(event_kind(&SUT::Booted), EventKind::Booted);
    }
}
