use crate::prelude::*;
use sargon::AppearanceID as InternalAppearanceID;

#[derive(
    
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct AppearanceID {
    pub value: u8,
}

impl From<InternalAppearanceID> for AppearanceID {
    fn from(value: InternalAppearanceID) -> Self {
        Self { value: value.value }
    }
}

impl Into<InternalAppearanceID> for AppearanceID {
    fn into(self) -> InternalAppearanceID {
        InternalAppearanceID { value: self.value }
    }
}

#[uniffi::export]
pub fn new_appearance_id(validating: u8) -> Result<AppearanceID> {
    InternalAppearanceID::new(validating).map_result()
}

#[uniffi::export]
pub fn new_appearance_id_from_number_of_accounts_on_network(
    count: u64,
) -> AppearanceID {
    InternalAppearanceID::from_number_of_accounts_on_network(count as usize).into()
}

#[uniffi::export]
pub fn new_appearance_id_sample() -> AppearanceID {
    InternalAppearanceID::sample().into()
}

#[uniffi::export]
pub fn new_appearance_id_sample_other() -> AppearanceID {
    InternalAppearanceID::sample_other().into()
}

#[uniffi::export]
pub fn appearance_ids_all() -> Vec<AppearanceID> {
    InternalAppearanceID::all().into_vec()
}