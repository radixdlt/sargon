use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::entity_flag::EntityFlag;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityFlags(BTreeSet<EntityFlag>);

impl EntityFlags {
    /// Instantiates an empty collection of entity flags.
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    /// Instantiates a flag collection with the provided flags,
    /// removing any duplicates from `flags` if any.
    pub fn with_flags<I>(flags: I) -> Self
    where
        I: Iterator<Item = EntityFlag>,
    {
        Self(BTreeSet::from_iter(flags))
    }

    /// Instantiates a flag collection with the provided single flag
    pub fn with_flag(flag: EntityFlag) -> Self {
        Self::with_flags(vec![flag].into_iter())
    }
}

impl Default for EntityFlags {
    /// Instantiates an empty collection of entity flags.
    fn default() -> Self {
        Self::new()
    }
}

impl EntityFlags {
    /// Adds a flag to the set of flags.
    ///
    /// Returns whether the flag was newly inserted. That is:
    ///
    /// If the set did not previously contain an equal flag, true is returned.
    /// If the set already contained an equal flag, false is returned, and the entry is not updated.
    pub fn insert_flag(&mut self, flag: EntityFlag) -> bool {
        self.0.insert(flag)
    }

    /// If the set contains a flag equal to `flag`, removes it from the set and drops it.
    /// Returns whether such a flag was present.
    pub fn remove_flag(&mut self, flag: &EntityFlag) -> bool {
        self.0.remove(flag)
    }

    pub fn contains(&self, flag: &EntityFlag) -> bool {
        self.0.contains(flag)
    }
}

#[cfg(test)]
mod tests {
    use crate::v100::entity::{entity_flag::EntityFlag, entity_flags::EntityFlags};

    #[test]
    fn empty_by_default() {
        assert_eq!(EntityFlags::default(), EntityFlags::new())
    }
    #[test]
    fn default_does_not_contain_deleted_by_user() {
        assert!(!EntityFlags::default().contains(&EntityFlag::DeletedByUser));
    }

    #[test]
    fn new_with_f_contains_f() {
        assert!(
            EntityFlags::with_flag(EntityFlag::DeletedByUser).contains(&EntityFlag::DeletedByUser)
        );
    }

    #[test]
    fn new_empty_insert_f_contains_f() {
        let mut sut = EntityFlags::default();
        sut.insert_flag(EntityFlag::DeletedByUser);
        assert!(sut.contains(&EntityFlag::DeletedByUser));
    }
}
