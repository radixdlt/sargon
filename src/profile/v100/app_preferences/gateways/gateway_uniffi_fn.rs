use crate::prelude::*;

#[uniffi::export]
pub fn new_gateway_sample() -> Gateway {
    Gateway::sample()
}

#[uniffi::export]
pub fn new_gateway_sample_other() -> Gateway {
    Gateway::sample_other()
}

#[uniffi::export]
pub fn new_gateway_for_network_id(network_id: NetworkID) -> Gateway {
    Gateway::from(network_id)
}

#[uniffi::export]
pub fn gateway_mainnet() -> Gateway {
    Gateway::mainnet()
}

#[uniffi::export]
pub fn gateway_stokenet() -> Gateway {
    Gateway::stokenet()
}

#[uniffi::export]
pub fn new_gateway_with_url_on_network(
    url: String,
    network_id: NetworkID,
) -> Result<Gateway> {
    Gateway::new(url, network_id)
}

#[uniffi::export]
pub fn gateway_wellknown_gateways() -> Vec<Gateway> {
    Gateway::wellknown()
}

#[uniffi::export]
pub fn gateway_is_wellknown(gateway: &Gateway) -> bool {
    gateway.is_wellknown()
}

#[uniffi::export]
pub fn gateway_to_string(gateway: &Gateway) -> String {
    gateway.to_string()
}

#[uniffi::export]
pub fn gateway_id(gateway: &Gateway) -> Url {
    gateway.id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Gateway;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_gateway_sample(),
                new_gateway_sample_other(),
                // duplicates should get removed
                new_gateway_sample(),
                new_gateway_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_to_string() {
        let sut = SUT::sample();
        assert_eq!(gateway_to_string(&sut), sut.to_string());
    }

    #[test]
    fn test_id() {
        let sut = SUT::sample();
        assert_eq!(gateway_id(&sut), sut.id());
    }

    #[test]
    fn test_gateway_mainnet() {
        assert_eq!(gateway_mainnet(), SUT::mainnet());
    }

    #[test]
    fn test_gateway_stokenet() {
        assert_eq!(gateway_stokenet(), SUT::stokenet());
    }

    #[test]
    fn test_new_gateway_with_url_on_network() {
        assert_eq!(
            new_gateway_with_url_on_network(
                "https://mainnet.radixdlt.com/".to_owned(),
                NetworkID::Mainnet
            )
            .unwrap(),
            SUT::mainnet()
        );
    }

    #[test]
    fn test_gateway_wellknown_gateways() {
        assert_eq!(gateway_wellknown_gateways(), SUT::wellknown())
    }

    #[test]
    fn test_gateway_is_wellknown() {
        assert!(gateway_is_wellknown(&SUT::sample()))
    }

    #[test]
    fn test_new_gateway_for_network_id() {
        assert_eq!(
            new_gateway_for_network_id(NetworkID::Mainnet),
            SUT::mainnet()
        )
    }
}
