use crate::prelude::*;

/// The report that gathers the different actions performed on profile after sync completes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitySyncOutcome {
    pub actions_performed: IndexSet<EntitySyncActionPerformed>,
}

impl HasSampleValues for EntitySyncOutcome {
    fn sample() -> Self {
        Self::new(IndexSet::just(EntitySyncActionPerformed::sample()))
    }

    fn sample_other() -> Self {
        Self::no_action()
    }
}

impl EntitySyncOutcome {
    pub fn new(actions: IndexSet<EntitySyncActionPerformed>) -> Self {
        Self {
            actions_performed: actions,
        }
    }

    pub fn no_action() -> Self {
        Self::new(IndexSet::new())
    }
}

/// An action that implies a synchronisation needs to be applied in an entity in the local profile.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub(crate) enum EntitySyncAction {
    ToTombstone(AccountAddress),
    ToSecurify(AddressOfAccountOrPersona, AccessControllerAddress),
}

/// The kinds of sync actions performed on entities in profile.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum EntitySyncActionPerformed {
    SomeEntitiesTombstoned,
    SomeEntitiesSecurified,
}

impl HasSampleValues for EntitySyncActionPerformed {
    fn sample() -> Self {
        Self::SomeEntitiesTombstoned
    }

    fn sample_other() -> Self {
        Self::SomeEntitiesSecurified
    }
}

#[cfg(test)]
mod entity_sync_outcome_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntitySyncOutcome;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_no_action() {
        assert_eq!(EntitySyncOutcome::no_action().actions_performed.len(), 0)
    }
}

#[cfg(test)]
mod entity_sync_action_performed_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntitySyncActionPerformed;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
