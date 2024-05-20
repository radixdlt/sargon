use crate::prelude::*;
use strum::*;

#[derive(Debug, PartialEq, Eq, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum MetadataKey {
    AccountType,
    OwnerKeys,
    Name,
    Symbol,
    IconUrl,
    Description,
    Tags,
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

    #[test]
    fn metadata_values() {
        use MetadataKey::*;
        let eq = |v: MetadataKey, e| {
            assert_eq!(v.to_string(), e);
            assert_eq!(MetadataKey::from_str(e).unwrap(), v);
        };

        eq(AccountType, "account_type");
        eq(OwnerKeys, "owner_keys");
        eq(Name, "name");
        eq(Symbol, "symbol");
        eq(IconUrl, "icon_url");
        eq(Description, "description");
        eq(Tags, "tags");
    }
}
