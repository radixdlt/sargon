use crate::prelude::*;
use paste::*;
use radix_engine_common::types::{
    EntityType as ScryptoEntityType, NodeId as ScryptoNodeId,
};
use radix_engine_toolkit::models::canonical_address_types::{
    CanonicalAccessControllerAddress as RetAccessControllerAddress,
    CanonicalAccountAddress as RetAccountAddress,
    CanonicalAddress as RetIsAddressTrait,
    CanonicalAddressError as RetCanonicalAddressError,
    CanonicalComponentAddress as RetComponentAddress,
    CanonicalIdentityAddress as RetIdentityAddress,
    CanonicalPackageAddress as RetPackageAddress,
    CanonicalPoolAddress as RetPoolAddress,
    CanonicalResourceAddress as RetResourceAddress,
    CanonicalValidatorAddress as RetValidatorAddress,
    CanonicalVaultAddress as RetVaultAddress,
};

pub trait AddressViaRet: Sized {
    fn new(
        node_id: impl Into<ScryptoNodeId>,
        network_id: NetworkID,
    ) -> Result<Self>;
}

macro_rules! decl_ret_wrapped_address {
    ($addr_name:ty, $ret_addr:ty, $addr_uniffi_fn_name:ident) => {
        paste! {

            #[uniffi::export]
            pub fn [<new_ $addr_uniffi_fn_name _address>](bech32: String) -> Result<$addr_name> {
                $addr_name::try_from_bech32(&bech32)
            }

            #[uniffi::export]
            pub fn [<$addr_uniffi_fn_name _address_network_id>](address: &$addr_name) -> NetworkID {
                address.network_id()
            }

            #[uniffi::export]
            pub fn [<$addr_uniffi_fn_name _address_bech32_address>](address: &$addr_name) -> String {
                address.address()
            }

             /// UniFFI conversion for RET types which are DisplayFromStr using String as builtin.
            impl crate::UniffiCustomTypeConverter for $ret_addr {
                type Builtin = String;

                #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    val.parse::<Self>()
                    .map_err(|e| {
                        error!("Failed to UniFFI decode String from FFI via RET, RET error: {:?}", e);
                        CommonError::FailedToDecodeAddressFromBech32 { bad_value: val }
                    })
                    .map_err(|e| e.into())
                }

                #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
                fn from_custom(obj: Self) -> Self::Builtin {
                    obj.to_string()
                }
            }

            impl From<$ret_addr> for $addr_name {
                fn from(value: $ret_addr) -> Self {
                    Self { secret_magic: value }
                }
            }

            impl From<$addr_name> for $ret_addr {
                fn from(value: $addr_name) -> Self {
                    value.secret_magic
                }
            }

            impl $addr_name {
                pub fn address(&self) -> String {
                    self.to_string()
                }

                pub fn network_id(&self) -> NetworkID {
                    self.secret_magic.network_id().try_into().expect("Should have known all network ids")
                }

                pub fn entity_type(&self) -> ScryptoEntityType {
                    self.secret_magic.entity_type()
                }

                pub fn try_from_bech32(bech32: impl AsRef<str>) -> Result<Self> {
                    bech32.as_ref().parse::<$ret_addr>()
                    .map_err(|e| {
                        error!("Failed Bech32 decode String, RET error: {:?}", e);
                        CommonError::FailedToDecodeAddressFromBech32 { bad_value: bech32.as_ref().to_owned() }
                    })
                    .map(Into::<Self>::into)
                }
            }

            impl AddressViaRet for $addr_name {
                fn new(
                    node_id: impl Into<ScryptoNodeId>,
                    network_id: NetworkID,
                ) -> Result<Self, CommonError> {
                    let node_id: ScryptoNodeId = node_id.into();
                    $ret_addr::new(node_id.clone(), network_id.discriminant())
                    .map_err(|e| {
                        error!("Failed create address, from node and network_id, RET error: {:?}", e);
                        CommonError::FailedToCreateAddressViaRetAddressFromNodeIdAndNetworkID { node_id_as_hex: node_id.to_hex(), network_id }
                    })
                    .map(|i| Into::<$addr_name>::into(i))
                }
            }
        }
    };
}

decl_ret_wrapped_address!(
    AccessControllerAddress,
    RetAccessControllerAddress,
    accesscontroller
);
decl_ret_wrapped_address!(AccountAddress, RetAccountAddress, account);
decl_ret_wrapped_address!(ComponentAddress, RetComponentAddress, component);
decl_ret_wrapped_address!(IdentityAddress, RetIdentityAddress, identity);
decl_ret_wrapped_address!(PoolAddress, RetPoolAddress, pool);
decl_ret_wrapped_address!(PackageAddress, RetPackageAddress, package);
decl_ret_wrapped_address!(ResourceAddress, RetResourceAddress, resource);
decl_ret_wrapped_address!(ValidatorAddress, RetValidatorAddress, validator);
decl_ret_wrapped_address!(VaultAddress, RetVaultAddress, vault);

#[cfg(test)]
mod tests {
    use radix_engine_common::types::NodeId;

    use crate::prelude::*;

    #[test]
    fn account_address_from_invalid_node_id() {
        let unknown_node_id = NodeId::new(222, &[0xff; 29]);
        assert_eq!(
            <AccountAddress as AddressViaRet>::new(unknown_node_id, NetworkID::Mainnet),
            Err(CommonError::FailedToCreateAddressViaRetAddressFromNodeIdAndNetworkID {
                node_id_as_hex: "deffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(), 
                network_id: NetworkID::Mainnet,
            })
        );
    }
}
