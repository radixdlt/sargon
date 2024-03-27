use crate::prelude::*;

pub trait AddressViaRet: Sized {
    fn new(
        node_id: impl Into<ScryptoNodeId>,
        network_id: NetworkID,
    ) -> Result<Self>;
}

/// Helps with unit testing, so that we do not need to explicitly specify each
/// (Sargon) Address types corresponding RET address type, but can use, e.g.
/// `AccountAddress::RetAddress` instead of `radix_engine_toolkit::models::canonical_address_types::CanonicalAccountAddress`
pub(crate) trait FromRetAddress {
    type RetAddress;
}

pub(crate) fn format_string(
    s: impl AsRef<str>,
    start: usize,
    end: usize,
) -> String {
    let s = s.as_ref();
    let prefix = &s[0..start];
    let suffix = suffix_str(end, s);
    format!("{}...{}", prefix, suffix)
}

pub trait IntoScryptoAddress {
    fn scrypto(&self) -> ScryptoGlobalAddress;
    fn network_id(&self) -> NetworkID;
}

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_ret_wrapped_address {
    (
        $(
            #[doc = $expr: expr]
        )*
        $address_type:ident
    ) => {
        paste! {
            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                derive_more::FromStr,
                derive_more::Display,
                derive_more::Debug,
                SerializeDisplay,
                DeserializeFromStr,
                uniffi::Record,
            )]
            #[display("{secret_magic}")]
            #[debug("{secret_magic}")]
            pub struct [< $address_type:camel Address >] {
                pub(crate) secret_magic: [< Ret $address_type:camel Address >], // Do NOT add comments above
            }

            #[uniffi::export]
            pub fn [<new_ $address_type:snake _address>](bech32: String) -> Result<[< $address_type:camel Address >]> {
                [< $address_type:camel Address >]::try_from_bech32(&bech32)
            }


            /// Returns a new address, with the same node_id, but using `network_id` as
            /// network.
            #[uniffi::export]
            pub fn [<$address_type:snake _address_map_to_network>](address: &[< $address_type:camel Address >], network_id: NetworkID) -> [< $address_type:camel Address >] {
                address.map_to_network(network_id)
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_network_id>](address: &[< $address_type:camel Address >]) -> NetworkID {
                address.network_id()
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_bech32_address>](address: &[< $address_type:camel Address >]) -> String {
                address.address()
            }

            #[uniffi::export]
            pub fn [<$address_type:snake _address_formatted>](address: &[< $address_type:camel Address >], format: AddressFormat) -> String {
                address.formatted(format)
            }

            uniffi::custom_type!([< Ret $address_type:camel Address >], String);

             /// UniFFI conversion for RET types which are DisplayFromStr using String as builtin.
            impl crate::UniffiCustomTypeConverter for [< Ret $address_type:camel Address >] {
                type Builtin = String;

                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    val.parse::<Self>()
                    .map_err(|_| {
                        CommonError::FailedToDecodeAddressFromBech32 { bad_value: val }.into()
                    })
                }

                fn from_custom(obj: Self) -> Self::Builtin {
                    obj.to_string()
                }
            }

            impl From<[< Ret $address_type:camel Address >]> for [< $address_type:camel Address >] {
                fn from(value: [< Ret $address_type:camel Address >]) -> Self {
                    Self { secret_magic: value }
                }
            }

            #[cfg(test)]
            impl From<&str> for [< $address_type:camel Address >] {
                /// TEST ONLY
                fn from(value: &str) -> Self {
                    value.parse().expect(&format!("Test failed since the passed in str is not a valid address: '{}'", value))
                }
            }

            impl FromRetAddress for [< $address_type:camel Address >] {
                type RetAddress = [< Ret $address_type:camel Address >];
            }

            impl From<[< $address_type:camel Address >]> for ScryptoGlobalAddress {
                fn from(value: [< $address_type:camel Address >]) -> ScryptoGlobalAddress {
                    value.scrypto()
                }
            }

            impl Ord for [< $address_type:camel Address >] {
                fn cmp(&self, other: &Self) -> Ordering {
                    self.address().cmp(&other.address())
                }
            }

            impl PartialOrd for [< $address_type:camel Address >] {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }

            impl [< $address_type:camel Address >] {

                pub fn formatted(&self, format: AddressFormat) -> String {
                    match format {
                        AddressFormat::Default => format_string(self.address(), 4, 6),
                        AddressFormat::Full | AddressFormat::Raw => self.address(),
                    }
                }

                pub(crate) fn scrypto(&self) -> ScryptoGlobalAddress {
                    ScryptoGlobalAddress::try_from(self.node_id())
                    .expect("Should always be able to convert a Sargon Address into radix engine 'GlobalAddress'.")
                }
                pub(crate) fn node_id(&self) -> ScryptoNodeId {
                    self.secret_magic.node_id()
                }

                /// Returns a new address, with the same node_id, but using `network_id` as
                /// network.
                pub fn map_to_network(&self, network_id: NetworkID) -> Self {
                    if network_id == self.network_id() {
                        return *self
                    }
                    <Self as AddressViaRet>::new(self.node_id(), network_id).expect("Should always be able to map an address to a different network.")
                }

                pub fn address(&self) -> String {
                    self.to_string()
                }

                pub fn entity_type(&self) -> ScryptoEntityType {
                    self.secret_magic.entity_type()
                }

                pub fn try_from_bech32(bech32: impl AsRef<str>) -> Result<Self> {
                    bech32.as_ref().parse::<[< Ret $address_type:camel Address >]>()
                    .map_err(|e| {
                        error!("Failed Bech32 decode String, RET error: {:?}", e);
                        CommonError::FailedToDecodeAddressFromBech32 { bad_value: bech32.as_ref().to_owned() }
                    })
                    .map(Self::from)
                }
            }

            impl From<[< $address_type:camel Address >]> for ScryptoManifestValue {
                fn from(value: [< $address_type:camel Address >]) -> ScryptoManifestValue {
                    ScryptoManifestValue::Custom {
                        value: ScryptoManifestCustomValue::Address(
                            ScryptoManifestAddress::Static(
                                value.node_id(),
                            ),
                        ),
                    }
                }
            }

            #[cfg(test)]
            mod [<uniffi_tests_of_ $address_type:snake>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = [< $address_type:camel Address >];

                #[test]
                fn map_to_network() {
                    let sut = SUT::sample();
                    assert_eq!([<$address_type:snake _address_map_to_network>](&sut, sut.network_id()), sut); // unchanged
                }
            }

            impl IntoScryptoAddress for [< $address_type:camel Address >] {
                fn scrypto(&self) -> ScryptoGlobalAddress {
                    ScryptoGlobalAddress::try_from(self.node_id())
                    .expect("Should always be able to convert a Sargon Address into radix engine 'GlobalAddress'.")
                }

                fn network_id(&self) -> NetworkID {
                    self.secret_magic.network_id().try_into().expect("Should have known all network ids")
                }
            }

            impl AddressViaRet for [< $address_type:camel Address >] {
                fn new(
                    node_id: impl Into<ScryptoNodeId>,
                    network_id: NetworkID,
                ) -> Result<Self, CommonError> {
                    let node_id: ScryptoNodeId = node_id.into();
                    [< Ret $address_type:camel Address >]::new(node_id.clone(), network_id.discriminant())
                    .map_err(|e| {
                        error!("Failed create address, from node and network_id, RET error: {:?}", e);
                        CommonError::FailedToCreateAddressViaRetAddressFromNodeIdAndNetworkID { node_id_as_hex: node_id.to_hex(), network_id }
                    })
                    .map(|i| [< $address_type:camel Address >]::from(i))
                }


            }

            impl TryInto<ScryptoDynamicGlobalAddress> for &[< $address_type:camel Address >] {
                type Error = crate::CommonError;

                fn try_into(
                    self,
                ) -> Result<ScryptoDynamicGlobalAddress, Self::Error> {
                    Ok(ScryptoDynamicGlobalAddress::Static(self.scrypto()))
                }
            }
        }
    };
}

decl_ret_wrapped_address!(
    /// Address to an AccessController that controls an Account or Identity (Persona),
    /// it said entity has been "securified", e.g.:
    /// `"accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a"`
    ///
    /// When a user applies a SecurityStructureConfiguration for the first time on a
    /// non-securified entity (and signs and submit the resulting TX) said entity is
    /// "assigned" an AccessControllerAddress by the network.
    ///
    /// An `AccessControllerAddress` has the [Scrypto's `EntityType`][entt] `GlobalAccessController`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalAccessControllerAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L247-L248
    accessController
);
decl_ret_wrapped_address!(
    /// Human readable address of an account. Always starts with `"account_"``, for example:
    ///
    /// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
    ///
    /// Most commonly the user will see this address in its abbreviated
    /// form which is:
    ///
    /// `acco...please`
    ///
    /// Addresses are checksummed, as per Bech32. **Only** *Account* addresses starts with
    /// the prefix `account_`.
    ///
    /// There are fundamentally three different sub-types ([Scrypto's `EntityType`][entt]) of AccountAddresses:
    /// * GlobalAccount
    /// * GlobalVirtualSecp256k1Account
    /// * GlobalVirtualEd25519Account
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalAccountAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L224-L228
    account
);
decl_ret_wrapped_address!(
    /// An address to some On-Ledger (OnNetwork) component, e.g. a Dapp, being an instantiation
    /// of some Scrypto blueprint, e.g:
    /// `"component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"`
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of ComponentAddress:
    /// * GlobalGenericComponent
    /// * InternalGenericComponent
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalComponentAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L243-L246
    component
);
decl_ret_wrapped_address!(
    /// Human readable address of an identity, which are used by Personas. Always starts with
    /// the prefix `"identity_"`, for example:
    ///
    /// `identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j`
    ///
    /// Addresses are checksummed, as per Bech32. **Only** *Identity* addresses starts with
    /// the prefix `"identity_"`.
    ///
    /// There are fundamentally three different sub-types ([Scrypto's `EntityType`][entt]) of IdentityAddresses:
    /// * GlobalIdentity,
    /// * GlobalVirtualSecp256k1Identity,
    /// * GlobalVirtualEd25519Identity
    ///
    /// ```
    /// extern crate sargon;
    /// use sargon::prelude::*;
    ///
    /// assert_eq!(
    ///     "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j".parse::<IdentityAddress>().unwrap().network_id(),
    ///     NetworkID::Mainnet
    /// );
    /// ```
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalIdentityAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L229-L234
    identity
);
decl_ret_wrapped_address!(
    /// Addresses identifying an OnLedger (OnNetwork) Liquidity Pool (LP) of tokens that users can contribute
    /// Liquidity too, e.g.:
    /// `"pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"`
    ///
    /// Typically users contribute to Liquidity Pools by using a Dapp and the Radix Wallet.
    ///
    /// There are fundamentally three different sub-types ([Scrypto's `EntityType`][entt]) of PoolAddresses:
    /// * GlobalOneResourcePool
    /// * GlobalTwoResourcePool
    /// * GlobalMultiResourcePool
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalPoolAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L256-L261
    pool
);
decl_ret_wrapped_address!(
    /// The unique address identifying a package - which is a collection of blueprints on Ledger, e.g.:
    /// `"package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet"`
    ///
    /// PackageAddress has [Scrypto's `EntityType`][entt] type `GlobalPackage`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalPackageAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L241C29-L241C42
    package
);
decl_ret_wrapped_address!(
    /// Addresses identifying an asset, either fungible (Token) or non-fungible (NFT), on the Radix network, e.g.
    /// `"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"`
    /// Being the unique identifier of the Radix Token, the Rad, on mainnet.
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of ResourceAddresses:
    /// * GlobalFungibleResourceManager
    /// * GlobalNonFungibleResourceManager
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalResourceAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L236-L239
    resource
);
decl_ret_wrapped_address!(
    /// Address to a Validator that secures the network by validating transactions, users can stake to these
    /// validators (Delegated Proof of Stake) by using the Dashboard and sending a TX to the Radix Wallet to sign;
    /// e.g.:
    /// `"validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"`
    ///
    /// A `ValidatorAddress` has the [Scrypto's `EntityType`][entt] `GlobalValidator`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalValidatorAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L249-L250
    validator
);
decl_ret_wrapped_address!(
    /// Addresses to a specific vault, owned by a user, holding asset of one kind, either fungible or non-fungible.
    /// Identities cannot own assets so they do not have vaults, but Accounts do, e.g.:
    /// `"internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"`
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of VaultAddresses:
    /// * InternalFungibleVault
    /// * InternalNonFungibleVault
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalVaultAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L251-L255
    vault
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_address_from_invalid_node_id() {
        let unknown_node_id = ScryptoNodeId::new(222, &[0xff; 29]);
        assert_eq!(
            <AccountAddress as AddressViaRet>::new(unknown_node_id, NetworkID::Mainnet),
            Err(CommonError::FailedToCreateAddressViaRetAddressFromNodeIdAndNetworkID {
                node_id_as_hex: "deffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(), 
                network_id: NetworkID::Mainnet,
            })
        );
    }
}
