use crate::prelude::*;

use radix_engine::system::system_modules::execution_trace::ResourceSpecifier as ScryptoResourceSpecifier;

impl From<(ScryptoResourceSpecifier, NetworkID)> for ResourceAddress {
    fn from(value: (ScryptoResourceSpecifier, NetworkID)) -> Self {
        let (ret, network_id) = value;
        match ret {
            ScryptoResourceSpecifier::Amount(resource_address, _) => {
                (resource_address, network_id).into()
            }
            ScryptoResourceSpecifier::Ids(resource_address, _) => {
                (resource_address, network_id).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_address_from_scrypto_resource_specifier_amount_mainnet() {
        type SUT = ResourceAddress;

        let exp = SUT::sample_mainnet_candy();
        let ret =
            ScryptoResourceSpecifier::Amount(exp.clone().into(), 0.into());
        assert_eq!(SUT::from((ret.clone(), NetworkID::Mainnet)), exp.clone());

        // Not equals on wrong network.
        assert_ne!(SUT::from((ret.clone(), NetworkID::Stokenet)), exp.clone());
    }

    #[test]
    fn resource_address_from_scrypto_resource_specifier_amount_stokenet() {
        type SUT = ResourceAddress;

        let exp = SUT::sample_stokenet_gum();
        let ret =
            ScryptoResourceSpecifier::Amount(exp.clone().into(), 0.into());
        assert_eq!(SUT::from((ret.clone(), NetworkID::Stokenet)), exp.clone());

        // Not equals on wrong network.
        assert_ne!(SUT::from((ret.clone(), NetworkID::Mainnet)), exp.clone());
    }

    #[test]
    fn resource_address_from_scrypto_resource_specifier_ids_mainnet() {
        type SUT = ResourceAddress;

        let exp = SUT::sample_mainnet_candy();
        let ret = ScryptoResourceSpecifier::Ids(exp.clone().into(), [].into());
        assert_eq!(SUT::from((ret.clone(), NetworkID::Mainnet)), exp.clone());

        // Not equals on wrong network.
        assert_ne!(SUT::from((ret.clone(), NetworkID::Stokenet)), exp.clone());
    }

    #[test]
    fn resource_address_from_scrypto_resource_specifier_ids_stokenet() {
        type SUT = ResourceAddress;

        let exp = SUT::sample_stokenet_gum();
        let ret = ScryptoResourceSpecifier::Ids(exp.clone().into(), [].into());
        assert_eq!(SUT::from((ret.clone(), NetworkID::Stokenet)), exp.clone());

        // Not equals on wrong network.
        assert_ne!(SUT::from((ret.clone(), NetworkID::Mainnet)), exp.clone());
    }
}
