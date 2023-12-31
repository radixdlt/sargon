use identified_vec::{Identifiable, IsIdentifiedVec};

use crate::IdentifiedVecVia;

use super::entity_flag::EntityFlag;

/// An order set of `EntityFlag`s used to describe certain Off-ledger
/// user state about Accounts or Personas, such as if an entity is
/// marked as hidden or not.
pub type EntityFlags = IdentifiedVecVia<EntityFlag>;

impl Identifiable for EntityFlag {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl EntityFlags {
    /// Instantiates a flag collection with the provided Vec<Flag>,
    /// removing any duplicates from `flags` if any.
    pub fn with_flags<I>(flags: I) -> Self
    where
        I: IntoIterator<Item = EntityFlag>,
    {
        Self::from_iter(flags)
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
        self.append(flag).0
    }

    pub fn remove_flag(&mut self, flag: &EntityFlag) -> Option<EntityFlag> {
        self.remove(flag)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };
    use identified_vec::IsIdentifiedVec;
    use serde_json::json;

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
        assert!(EntityFlags::with_flag(EntityFlag::DeletedByUser)
            .contains(&EntityFlag::DeletedByUser));
    }

    #[test]
    fn remove_existing_flag() {
        assert!(EntityFlags::with_flag(EntityFlag::DeletedByUser)
            .remove_flag(&EntityFlag::DeletedByUser)
            .is_some());
    }

    #[test]
    fn remove_non_existing_flag() {
        assert!(!EntityFlags::default()
            .remove_flag(&EntityFlag::DeletedByUser)
            .is_none());
        // does not exist
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
