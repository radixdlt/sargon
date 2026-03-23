use crate::prelude::*;
use sargon::TokenPriceService as InternalTokenPriceService;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct TokenPriceService {
    pub base_url: Url,
}

delegate_debug_into!(TokenPriceService, InternalTokenPriceService);

#[uniffi::export]
pub fn new_token_price_service_sample() -> TokenPriceService {
    InternalTokenPriceService::sample().into()
}

#[uniffi::export]
pub fn new_token_price_service_sample_other() -> TokenPriceService {
    InternalTokenPriceService::sample_other().into()
}
