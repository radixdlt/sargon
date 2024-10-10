use sargon::AddressFormat as InternalAddressFormat;


#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum AddressFormat {
    Full,
    Raw,
    Default,
}

impl From<InternalAddressFormat> for AddressFormat {
    fn from(value: InternalAddressFormat) -> Self {
        match value {
            InternalAddressFormat::Full => AddressFormat::Full,
            InternalAddressFormat::Raw => AddressFormat::Raw,
            InternalAddressFormat::Default => AddressFormat::Default,
        }
    }
}

impl Into<InternalAddressFormat> for AddressFormat {
    fn into(self) -> InternalAddressFormat {
        match self {
            AddressFormat::Full => InternalAddressFormat::Full,
            AddressFormat::Raw => InternalAddressFormat::Raw,
            AddressFormat::Default => InternalAddressFormat::Default,
        }
    }
}
