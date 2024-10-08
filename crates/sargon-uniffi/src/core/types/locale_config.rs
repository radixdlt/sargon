use crate::prelude::*;
use sargon::LocaleConfig as InternalLocaleConfig;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct LocaleConfig {
    pub decimal_separator: Option<String>,
    pub grouping_separator: Option<String>,
}

impl From<InternalLocaleConfig> for LocaleConfig {
    fn from(value: InternalLocaleConfig) -> Self {
        Self {
            decimal_separator: value.decimal_separator.map(|s| s.0),
            grouping_separator: value.grouping_separator.map(|s| s.0),
        }
    }
}

impl Into<InternalLocaleConfig> for LocaleConfig {
    fn into(self) -> InternalLocaleConfig {
        InternalLocaleConfig {
            decimal_separator: self.decimal_separator.map(|s| s.into()),
            grouping_separator: self.grouping_separator.map(|s| s.into()),
        }
    }
}