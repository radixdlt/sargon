
use crate::prelude::*;

pub struct RadixNameService {
    pub(crate) config: RadixNameServiceConfig,
    pub(crate) gateway_client: GatewayClient,
}

impl RadixNameService {
    fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        config: RadixNameServiceConfig,
        network_id: NetworkID,
    ) -> Self {
        let gateway_client = GatewayClient::new(networking_driver, network_id);
        Self {
            config,
            gateway_client,
        }
    }

    pub fn new_xrd_domains(
        networking_driver: Arc<dyn NetworkingDriver>,
        network_id: NetworkID,
    ) -> Result<Self> {
        match Self::xrd_domains_config().get(&network_id) {
            Some(config) => {
                Ok(Self::new(networking_driver, config.clone(), network_id))
            }
            None => Err(CommonError::RnsUnsupportedNetwork {
                network: network_id.discriminant(),
            }),
        }
    }

    fn xrd_domains_config() -> HashMap<NetworkID, RadixNameServiceConfig> {
        HashMap::from([
            (
                NetworkID::Mainnet,
                RadixNameServiceConfig::xrd_domains_mainnet(),
            ),
            (
                NetworkID::Stokenet,
                RadixNameServiceConfig::xrd_domains_stokenet(),
            ),
        ])
    }
}

#[derive(Clone)]
pub(crate) struct RadixNameServiceConfig {
    pub(crate) domains_collection_address: NonFungibleResourceAddress,
    pub(crate) records_collection_address: NonFungibleResourceAddress,
}

impl RadixNameServiceConfig {
    fn new(
        domains_collection_address: NonFungibleResourceAddress,
        records_collection_address: NonFungibleResourceAddress,
    ) -> Self {
        Self {
            domains_collection_address,
            records_collection_address,
        }
    }

    pub(crate) fn xrd_domains_mainnet() -> Self {
        Self::new(
            NonFungibleResourceAddress::from_str("resource_rdx1n2dd0w53zpdlqdz65vpymygj8a60vqnggyuxfpfdldjmy2224x020q").unwrap(),
            NonFungibleResourceAddress::from_str("resource_rdx1nf7lt68zan0fvlfqqrtnxasxjmv877ncnr2kpdl69t076sw4whjc27").unwrap(),
        )
    }

    pub(crate) fn xrd_domains_stokenet() -> Self {
        Self::new(
            NonFungibleResourceAddress::from_str("resource_tdx_2_1n2leg5zgd0cw3766mdae43jg8dvp2h4x08rjjcrf3qrta8lhfjt7wq").unwrap(),
            NonFungibleResourceAddress::from_str("resource_tdx_2_1ng2r922evyvtzhdfdh4r2nqznw4zwkfesed296aclc5xqfr857t8mz").unwrap(),
        )
    }
}
