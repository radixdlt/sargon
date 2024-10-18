use crate::prelude::*;
use sargon::AppDisplay as InternalAppDisplay;

/// Settings related to displaying of information to the user inside the app.
///
/// **N.B. neither of these settings are in fact not yet used by clients.**
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AppDisplay {
    /// If we should show the aggregate value of users portfolio in fiat currency
    /// of hide it.
    pub is_currency_amount_visible: bool,

    /// Which fiat currency the prices are measured in.
    pub fiat_currency_price_target: FiatCurrency,
}
