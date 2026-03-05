use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns a Radix Connect mobile client managed by this SargonOS wrapper.
    ///
    /// The client resolves relay URL lazily from profile app preferences on each
    /// send operation, so hosts do not need to recreate it after relay changes.
    #[uniffi::method]
    pub fn radix_connect_mobile(&self) -> RadixConnectMobile {
        let mobile = self.wrapped.radix_connect_mobile();
        RadixConnectMobile::from_internal(mobile)
    }
}
