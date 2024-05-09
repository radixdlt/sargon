use indexmap::IndexSet;

use crate::prelude::*;

impl PersonaData {
    pub fn ids_of_entries(&self) -> IndexSet<PersonaDataEntryID> {
        let mut full_ids = IndexSet::<PersonaDataEntryID>::new();
        if let Some(name) = &self.name {
            let _ = full_ids.insert(name.id);
        };
        full_ids.extend(self.email_addresses.ids());
        full_ids.extend(self.phone_numbers.ids());
        full_ids
    }
}
