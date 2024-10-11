use crate::prelude::*;
use sargon::LocaleConfig as InternalLocaleConfig;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct LocaleConfig {
    pub decimal_separator: Option<String>,
    pub grouping_separator: Option<String>,
}

impl From<InternalLocaleConfig> for LocaleConfig {
    fn from(value: InternalLocaleConfig) -> Self {
        Self {
            decimal_separator: value.decimal_separator,
            grouping_separator: value.grouping_separator,
        }
    }
}

impl Into<InternalLocaleConfig> for LocaleConfig {
    fn into(self) -> InternalLocaleConfig {
        InternalLocaleConfig {
            decimal_separator: self.decimal_separator,
            grouping_separator: self.grouping_separator,
        }
    }
}
