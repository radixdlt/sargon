use crate::prelude::*;

pub(crate) static ALL_ACCOUNT_ADDRESSES_SAMPLES: Lazy<[AccountAddress; 10]> =
    Lazy::new(|| {
        [
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
            AccountAddress::random(NetworkID::Mainnet),
        ]
    });

impl AccountAddress {
    pub(crate) fn sample_at(index: usize) -> Self {
        ALL_ACCOUNT_ADDRESSES_SAMPLES[index].clone()
    }
}
