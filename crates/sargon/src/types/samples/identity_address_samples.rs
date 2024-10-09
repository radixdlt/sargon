use crate::prelude::*;

pub(crate) static ALL_IDENTITY_ADDRESSES_SAMPLES: Lazy<[IdentityAddress; 10]> =
    Lazy::new(|| {
        [
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
            IdentityAddress::random(NetworkID::Mainnet),
        ]
    });

impl IdentityAddress {
    pub(crate) fn sample_at(index: usize) -> Self {
        ALL_IDENTITY_ADDRESSES_SAMPLES[index].clone()
    }
}
