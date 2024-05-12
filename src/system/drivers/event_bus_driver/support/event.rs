use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum EventKind {
    Booted,
    ProfileSaved,
    AddedAccount,
    AddedAccounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum ProfileChange {
    AddedAccount { address: AccountAddress },
    AddedAccounts { addresses: Vec<AccountAddress> },
}

impl HasEventKind for ProfileChange {
    fn kind(&self) -> EventKind {
        match self {
            Self::AddedAccount { address: _ } => EventKind::AddedAccount,
            Self::AddedAccounts { addresses: _ } => EventKind::AddedAccounts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum Event {
    Booted,
    ProfileSaved,
    ProfileChanged { change: ProfileChange },
}

pub trait HasEventKind {
    fn kind(&self) -> EventKind;
}

impl HasEventKind for Event {
    fn kind(&self) -> EventKind {
        match self {
            Self::Booted => EventKind::Booted,
            Self::ProfileSaved => EventKind::ProfileSaved,
            Self::ProfileChanged { change } => change.kind(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct EventNotification {
    pub id: Uuid,
    pub event: Event,
    pub timestamp: Timestamp,
}

impl EventNotification {
    pub fn new(event: Event) -> Self {
        Self {
            id: id(),
            event,
            timestamp: now(),
        }
    }
}
