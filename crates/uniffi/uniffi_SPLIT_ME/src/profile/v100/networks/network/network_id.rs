use crate::prelude::*;
use sargon::NetworkID as InternalNetworkID;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
#[repr(u8)]
pub enum NetworkID {
    /// Mainnet (0x01 / 0d01)
    ///
    /// The Radix public network.
    ///
    /// https://github.com/radixdlt/radixdlt-scrypto/blob/v1.0.1/radix-engine-common/src/network/mod.rs#L79
    Mainnet = 0x01,

    /// Stokenet (0x02 / 0d02)
    ///
    /// The public testnet for Radix.
    ///
    /// https://github.com/radixdlt/radixdlt-scrypto/blob/v1.0.1/radix-engine-common/src/network/mod.rs#L71
    Stokenet = 0x02,

    /// Adapanet (0x0a / 0d10
    Adapanet = 0x0a,

    /// Nebunet (0x0b / 0d11 )
    ///
    /// The first Betanet of Babylon
    Nebunet = 0x0b,

    /// Kisharnet (0x0c / 0d12)
    ///
    /// The first release candidate of Babylon (RCnet v1)
    Kisharnet = 0x0c,

    /// Ansharnet (0x0d / 0d13)
    ///
    /// The second release candidate of Babylon (RCnet v2)
    Ansharnet = 0x0d,

    /// Zabanet (0x0e / 0d14)
    ///
    /// The third release candidate of Babylon (RCnet v3)
    Zabanet = 0x0e,

    /// Enkinet (0x21 / 0d33)
    ///
    /// https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L94
    Enkinet = 0x21,

    /// Hammunet
    /// https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L95
    /// Decimal value: 34
    Hammunet = 0x22,

    /// Nergalnet
    /// https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L96
    /// Decimal value: 35
    Nergalnet = 0x23,

    /// Mardunet
    /// https://github.com/radixdlt/babylon-node/blob/main/common/src/main/java/com/radixdlt/networks/Network.java#L97
    /// Decimal value: 36
    Mardunet = 0x24,

    /// Simulator (0xf2 / 0d242)
    Simulator = 242,
}

delegate_display_debug_into!(NetworkID, InternalNetworkID);

#[uniffi::export]
pub fn new_network_id_from_discriminant(discriminant: u8) -> Result<NetworkID> {
    InternalNetworkID::try_from(discriminant).into_result()
}

#[uniffi::export]
pub fn network_id_to_string(id: NetworkID) -> String {
    id.into_internal().logical_name()
}

#[uniffi::export]
pub fn network_id_discriminant(id: NetworkID) -> u8 {
    id.into_internal().discriminant()
}

#[uniffi::export]
pub fn network_ids_all() -> Vec<NetworkID> {
    InternalNetworkID::all().into_type()
}
