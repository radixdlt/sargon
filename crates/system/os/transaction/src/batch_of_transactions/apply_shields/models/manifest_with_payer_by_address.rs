use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestWithPayerByAddress {
    /// `None` is invalid if `entity_applying_shield` is a Persona.
    /// Some(Account) if `entity_applying_shield` is an Account means "use this other account instead"
    /// None if `entity_applying_shield` is an Account means "use the account applying the shield"
    pub payer: Option<AccountAddress>,
    pub manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
}
