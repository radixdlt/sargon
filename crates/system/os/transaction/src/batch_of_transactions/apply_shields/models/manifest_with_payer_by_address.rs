use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestWithPayerByAddress {
    /// In case of entity applying shield in `manifest` is an Account, then
    /// `payer` might be the same Account.
    pub payer: AccountAddress,
    pub manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub fee_tip_percentage: Option<u16>,
}

impl ManifestWithPayerByAddress {
    pub fn with(
        manifest: TransactionManifest,
        estimated_xrd_fee: Decimal,
        payer: impl Into<AccountAddress>,
        fee_tip_percentage: impl Into<Option<u16>>,
    ) -> Self {
        Self {
            payer: payer.into(),
            manifest,
            estimated_xrd_fee,
            fee_tip_percentage: fee_tip_percentage.into(),
        }
    }

    pub fn new(
        manifest: TransactionManifest,
        estimated_xrd_fee: Decimal,
        payer: impl Into<AccountAddress>,
    ) -> Self {
        Self::with(manifest, estimated_xrd_fee, payer, None)
    }
}
