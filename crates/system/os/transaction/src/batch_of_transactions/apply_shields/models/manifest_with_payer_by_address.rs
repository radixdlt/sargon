use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestWithPayerByAddress {
    pub payer: Option<AccountAddress>,
    pub manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
}

impl ManifestWithPayerByAddress {
    pub fn new(
        manifest: TransactionManifest,
        payer: impl Into<Option<AccountAddress>>,
        estimated_xrd_fee: Decimal,
    ) -> Self {
        Self {
            payer: payer.into(),
            manifest,
            estimated_xrd_fee,
        }
    }
}
