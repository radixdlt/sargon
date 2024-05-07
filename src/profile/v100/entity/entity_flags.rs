use crate::prelude::*;

/// An order set of `EntityFlag`s used to describe certain Off-ledger
/// user state about Accounts or Personas, such as if an entity is
/// marked as hidden or not.
pub type EntityFlags = OrderedMap<EntityFlag>;

#[uniffi::export]
pub fn new_entity_flags_sample() -> EntityFlags {
    EntityFlags::sample()
}

#[uniffi::export]
pub fn new_entity_flags_sample_other() -> EntityFlags {
    EntityFlags::sample_other()
}

impl Identifiable for EntityFlag {
    type ID = Self;

    fn id(&self) -> Self::ID {
        *self
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
        self.insert(flag).is_none()
    }

    pub fn remove_flag(&mut self, flag: &EntityFlag) -> Option<EntityFlag> {
        self.remove(flag)
    }
}

impl HasSampleValues for EntityFlags {
    fn sample() -> Self {
        Self::from_iter([EntityFlag::sample()])
    }

    fn sample_other() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntityFlags;

    #[test]
    fn empty_by_default() {
        assert_eq!(SUT::default(), SUT::new())
    }

    #[test]
    fn default_does_not_contain_deleted_by_user() {
        assert!(!SUT::default().contains(&EntityFlag::DeletedByUser));
    }

    #[test]
    fn new_with_f_contains_f() {
        assert!(SUT::just(EntityFlag::DeletedByUser)
            .contains(&EntityFlag::DeletedByUser));
    }

    #[test]
    fn remove_existing_flag() {
        assert!(SUT::just(EntityFlag::DeletedByUser)
            .remove_flag(&EntityFlag::DeletedByUser)
            .is_some());
    }

    #[test]
    fn remove_non_existing_flag() {
        assert_eq!(
            SUT::default().remove_flag(&EntityFlag::DeletedByUser),
            None
        );
        // does not exist
    }

    #[test]
    fn new_with_duplicates_of_f_contains_only_f() {
        assert_eq!(
            SUT::from_iter([
                EntityFlag::DeletedByUser,
                EntityFlag::DeletedByUser
            ])
            .len(),
            1
        );
    }

    #[test]
    fn new_empty_insert_f_contains_f() {
        let mut sut = SUT::default();
        sut.insert_flag(EntityFlag::DeletedByUser);
        assert!(sut.contains(&EntityFlag::DeletedByUser));
    }

    #[test]
    fn json_roundtrip_non_empty() {
        let model = SUT::just(EntityFlag::DeletedByUser);

        assert_json_value_eq_after_roundtrip(
            &model,
            json!(vec!["deletedByUser"]),
        );

        assert_json_roundtrip(&model);
        assert_json_value_ne_after_roundtrip(
            &model,
            json!(Vec::<String>::new()),
        );
    }

    #[test]
    fn json_roundtrip_empty() {
        let model = SUT::default();

        let json = json!(Vec::<String>::new());
        assert_json_value_eq_after_roundtrip(&model, json);
        assert_json_roundtrip(&model);

        assert_json_value_ne_after_roundtrip(
            &model,
            json!(vec!["deletedByUser"]),
        );
    }
}
