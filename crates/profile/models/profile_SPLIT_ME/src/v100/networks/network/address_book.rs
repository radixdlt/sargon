use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of user-managed addresses with custom names.
    AddressBook,
    AddressBookEntry
);

impl AddressBook {
    pub fn contains(&self, target: &AddressBookEntryTarget) -> bool {
        self.contains_id(target.clone())
    }

    pub fn add_entry(&mut self, entry: AddressBookEntry) -> bool {
        self.try_insert_unique(entry).is_ok()
    }

    pub fn update_entry(
        &mut self,
        target: AddressBookEntryTarget,
        name: DisplayName,
        note: Option<String>,
    ) -> bool {
        self.update_with(target, |entry| {
            entry.update_name_and_note(name, note.clone())
        })
    }

    pub fn remove(&mut self, target: &AddressBookEntryTarget) -> bool {
        self.remove_id(target).is_some()
    }
}

/// Writes an [`AddressBook`] as two lists in the Profile, flattened into its
/// network: entries for addresses under `address_book`, as before
/// non-fungibles could be named, and entries for non-fungibles under
/// `address_book_non_fungibles`. Wallets from before can't read an entry for a
/// non-fungible, and would fail to load the whole Profile if one were in
/// `address_book`; a key they don't know they skip.
pub mod address_book_in_two_lists {
    use super::*;
    use serde::{Deserializer, Serializer};

    #[derive(Serialize, Deserialize, Default)]
    struct Lists {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        address_book: Vec<AddressBookEntry>,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        address_book_non_fungibles: Vec<AddressBookEntry>,
    }

    pub fn serialize<S>(
        address_book: &AddressBook,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (addresses, non_fungibles) =
            address_book.iter().partition(|entry| {
                matches!(entry.target, AddressBookEntryTarget::GlobalAddress { .. })
            });
        Lists {
            address_book: addresses,
            address_book_non_fungibles: non_fungibles,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<AddressBook, D::Error>
    where
        D: Deserializer<'de>,
    {
        let lists = Lists::deserialize(deserializer)?;
        Ok(lists
            .address_book
            .into_iter()
            .chain(lists.address_book_non_fungibles)
            .collect())
    }
}

impl HasSampleValues for AddressBook {
    fn sample() -> Self {
        Self::just(AddressBookEntry::sample())
    }

    fn sample_other() -> Self {
        Self::just(AddressBookEntry::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AddressBook;

    #[test]
    fn add_entry_rejects_duplicates() {
        let mut sut = SUT::new();
        let entry = AddressBookEntry::sample();

        assert!(sut.add_entry(entry.clone()));
        assert!(!sut.add_entry(entry));
    }

    #[test]
    fn contains() {
        let entry = AddressBookEntry::sample();
        let sut = SUT::just(entry.clone());
        assert!(sut.contains(&entry.target));
        assert!(!sut.contains(&AddressBookEntryTarget::sample_other()));
    }

    #[test]
    fn update_entry() {
        let entry = AddressBookEntry::sample();
        let mut sut = SUT::just(entry.clone());

        assert!(sut.update_entry(
            entry.target.clone(),
            DisplayName::sample_other(),
            Some("updated".to_owned())
        ));
        assert_eq!(
            sut.get_id(entry.target.clone()).unwrap().name,
            DisplayName::sample_other()
        );
        assert_eq!(
            sut.get_id(entry.target.clone()).unwrap().note,
            Some("updated".to_owned())
        );
    }

    #[test]
    fn update_missing_entry_returns_false() {
        let mut sut = SUT::new();
        assert!(!sut.update_entry(
            AddressBookEntryTarget::sample(),
            DisplayName::sample(),
            None
        ));
    }

    #[test]
    fn remove() {
        let entry = AddressBookEntry::sample();
        let mut sut = SUT::just(entry.clone());
        assert!(sut.remove(&entry.target));
        assert!(!sut.contains(&entry.target));
        assert!(!sut.remove(&entry.target));
    }
}
