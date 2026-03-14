use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of user-managed external account addresses with custom names.
    AddressBook,
    AddressBookEntry
);

impl AddressBook {
    pub fn contains_address(&self, address: &AccountAddress) -> bool {
        self.contains_id(*address)
    }

    pub fn add_entry(&mut self, entry: AddressBookEntry) -> bool {
        self.try_insert_unique(entry).is_ok()
    }

    pub fn update_entry(
        &mut self,
        address: AccountAddress,
        name: DisplayName,
        note: Option<String>,
    ) -> bool {
        self.update_with(address, |entry| {
            entry.update_name_and_note(name, note.clone())
        })
    }

    pub fn remove_by_address(&mut self, address: &AccountAddress) -> bool {
        self.remove_id(address).is_some()
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
    fn contains_address() {
        let entry = AddressBookEntry::sample();
        let sut = SUT::just(entry.clone());
        assert!(sut.contains_address(&entry.address));
    }

    #[test]
    fn update_entry() {
        let entry = AddressBookEntry::sample();
        let mut sut = SUT::just(entry.clone());

        assert!(sut.update_entry(
            entry.address,
            DisplayName::sample_other(),
            Some("updated".to_owned())
        ));
        assert_eq!(
            sut.get_id(entry.address).unwrap().name,
            DisplayName::sample_other()
        );
        assert_eq!(
            sut.get_id(entry.address).unwrap().note,
            Some("updated".to_owned())
        );
    }

    #[test]
    fn update_missing_entry_returns_false() {
        let mut sut = SUT::new();
        assert!(!sut.update_entry(
            AccountAddress::sample(),
            DisplayName::sample(),
            None
        ));
    }

    #[test]
    fn remove_by_address() {
        let entry = AddressBookEntry::sample();
        let mut sut = SUT::just(entry.clone());
        assert!(sut.remove_by_address(&entry.address));
        assert!(!sut.contains_address(&entry.address));
        assert!(!sut.remove_by_address(&entry.address));
    }
}
