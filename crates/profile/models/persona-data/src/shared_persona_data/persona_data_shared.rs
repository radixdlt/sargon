use crate::prelude::*;

impl PersonaData {
    pub fn shared_everything(&self) -> SharedPersonaData {
        SharedPersonaData::new(
            self.name.clone().map(|x| x.id),
            SharedToDappWithPersonaIDsOfPersonaDataEntries::exactly(
                self.email_addresses.ids().into_iter().cloned(),
            ),
            SharedToDappWithPersonaIDsOfPersonaDataEntries::exactly(
                self.phone_numbers.ids().into_iter().cloned(),
            ),
        )
    }
}
