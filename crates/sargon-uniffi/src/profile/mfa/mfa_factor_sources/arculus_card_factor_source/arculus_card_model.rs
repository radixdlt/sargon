use crate::prelude::*;

/// The model of a Arculus Card.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum ArculusCardModel {
    /// Arculus card model: "Arculus® Cold Storage Wallet",
    /// for more info [see][link].
    ///
    /// [link]: https://www.getarculus.com/products/arculus-cold-storage-wallet.html
    ArculusColdStorageWallet,
}