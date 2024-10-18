use crate::prelude::*;
use sargon::ArculusCardModel as InternalArculusCardModel;

/// The model of a Arculus Card.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum ArculusCardModel {
    /// Arculus card model: "Arculus® Cold Storage Wallet",
    /// for more info [see][link].
    ///
    /// [link]: https://www.getarculus.com/products/arculus-cold-storage-wallet.html
    ArculusColdStorageWallet,
}
