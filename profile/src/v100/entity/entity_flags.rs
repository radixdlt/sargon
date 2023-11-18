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

    /// Instantiates a flag collection with the provided Vec<Flag>,
    /// removing any duplicates from `flags` if any.
    pub fn with_flags(flags: Vec<EntityFlag>) -> Self {
        Self(BTreeSet::from_iter(flags))
    }

    /// Instantiates a flag collection with the provided single flag
    pub fn with_flag(flag: EntityFlag) -> Self {
        Self::with_flags(vec![flag])
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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use wallet_kit_test_utils::json::{
        assert_eq_after_json_roundtrip, assert_json_roundtrip,
        assert_json_value_eq_after_roundtrip, assert_json_value_ne_after_roundtrip,
    };

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
    fn new_with_duplicates_of_f_contains_only_f() {
        assert_eq!(
            EntityFlags::with_flags(vec![EntityFlag::DeletedByUser, EntityFlag::DeletedByUser])
                .len(),
            1
        );
    }

    #[test]
    fn new_empty_insert_f_contains_f() {
        let mut sut = EntityFlags::default();
        sut.insert_flag(EntityFlag::DeletedByUser);
        assert!(sut.contains(&EntityFlag::DeletedByUser));
    }

    #[test]
    fn json_roundtrip_non_empty() {
        let model = EntityFlags::with_flag(EntityFlag::DeletedByUser);

        assert_json_value_eq_after_roundtrip(&model, json!(vec!["deletedByUser"]));

        assert_json_roundtrip(&model);
        assert_json_value_ne_after_roundtrip(&model, json!(Vec::<String>::new()));
    }

    #[test]
    fn json_roundtrip_empty() {
        let model = EntityFlags::default();

        let json = json!(Vec::<String>::new());
        assert_json_value_eq_after_roundtrip(&model, json);
        assert_json_roundtrip(&model);

        assert_json_value_ne_after_roundtrip(&model, json!(vec!["deletedByUser"]));
    }
}
