use crate::prelude::*;

use radix_engine::prelude::ToMetadataEntry as ScryptoToMetadataEntry;
use transaction::prelude::MetadataValue as ScryptoMetadataValue;

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
pub enum MetadataKey {
    #[display("account_type")]
    AccountType,

    #[display("owner_keys")]
    OwnerKeys,
}

impl From<MetadataKey> for String {
    fn from(value: MetadataKey) -> Self {
        value.to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MetadataValue {
    Str(MetadataValueStr),
}

impl MetadataValue {
    pub const DAPP_DEFINITION: Self =
        Self::Str(MetadataValueStr::DappDefinition);
}

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
pub enum MetadataValueStr {
    #[display("dapp definition")]
    DappDefinition,
}

impl ScryptoToMetadataEntry for MetadataValueStr {
    fn to_metadata_entry(self) -> Option<ScryptoMetadataValue> {
        Some(ScryptoMetadataValue::String(self.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dapp_def() {
        assert_eq!(
            MetadataValueStr::DappDefinition
                .to_metadata_entry()
                .unwrap(),
            ScryptoMetadataValue::String("dapp definition".to_owned())
        );
    }
}
