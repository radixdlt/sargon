use crate::prelude::*;
use sargon::LocaleConfig as InternalLocaleConfig;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct LocaleConfig {
    pub decimal_separator: Option<String>,
    pub grouping_separator: Option<String>,
}
