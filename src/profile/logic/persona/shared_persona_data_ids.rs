use crate::prelude::*;
use indexmap::IndexSet;

impl SharedPersonaData {
    pub fn ids_of_entries(&self) -> IndexSet<PersonaDataEntryID> {
        let mut full_ids = IndexSet::<PersonaDataEntryID>::new();
        if let Some(name) = &self.name {
            let _ = full_ids.insert(*name);
        };
        if let Some(email_addresses) = &self.email_addresses {
            full_ids.extend(email_addresses.ids.into_iter());
        }
        if let Some(phone_numbers) = &self.phone_numbers {
            full_ids.extend(phone_numbers.ids.into_iter());
        }
        full_ids
    }
}
