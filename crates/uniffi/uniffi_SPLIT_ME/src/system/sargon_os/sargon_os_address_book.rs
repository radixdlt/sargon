use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub fn address_book_on_current_network(
        &self,
    ) -> Result<Vec<AddressBookEntry>> {
        let entries: sargon::AddressBook = self
            .wrapped
            .address_book_on_current_network()
            .into_result()?;
        Ok(entries.into_iter().map(Into::into).collect())
    }

    pub fn address_book_entry_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<AddressBookEntry> {
        let entry: sargon::AddressBookEntry = self
            .wrapped
            .address_book_entry_by_address(address.into_internal())
            .into_result()?;
        Ok(entry.into())
    }
}

#[uniffi::export]
impl SargonOS {
    pub async fn add_address_book_entry(
        &self,
        address: AccountAddress,
        name: DisplayName,
        note: Option<String>,
    ) -> Result<bool> {
        self.wrapped
            .add_address_book_entry(
                address.into_internal(),
                name.into_internal(),
                note,
            )
            .await
            .into_result()
    }

    pub async fn update_address_book_entry(
        &self,
        address: AccountAddress,
        name: DisplayName,
        note: Option<String>,
    ) -> Result<bool> {
        self.wrapped
            .update_address_book_entry(
                address.into_internal(),
                name.into_internal(),
                note,
            )
            .await
            .into_result()
    }

    pub async fn delete_address_book_entry(
        &self,
        address: AccountAddress,
    ) -> Result<bool> {
        self.wrapped
            .delete_address_book_entry(address.into_internal())
            .await
            .into_result()
    }
}
