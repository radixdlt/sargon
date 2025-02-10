use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestWithPayerByAddress {
    /// In case of entity applying shield in `manifest` is an Account, then
    /// `payer` might be the same Account.
    pub payer: AccountAddress,
    pub manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
}

impl ManifestWithPayerByAddress {
    pub fn new(
        manifest: TransactionManifest,
        estimated_xrd_fee: Decimal,
        payer: impl Into<AccountAddress>,
    ) -> Self {
        Self {
            payer: payer.into(),
            manifest,
            estimated_xrd_fee,
        }
    }
}
