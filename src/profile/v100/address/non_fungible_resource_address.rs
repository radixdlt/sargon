use crate::prelude::*;
use paste::*;
use std::ops::Deref;

macro_rules! decl_specialized_address {
    (
        $(
            #[doc = $expr: expr]
        )*
        $specialized_address_type: ident,
        $base_addr: ty,
        $validate: ident,
        $validation_err: ident
    ) => {

        paste! {
            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                PartialEq,
                Eq,
                Hash,
                derive_more::Display,
                derive_more::Debug,
                derive_more::FromStr,
                SerializeDisplay,
                DeserializeFromStr,
                uniffi::Record,
            )]
            #[debug("{:?}", self.secret_magic)]
            pub struct $specialized_address_type {
                secret_magic: $base_addr
            }

            /// Tries to bech32 decode the string into a specialized address.
            #[uniffi::export]
            pub fn [< new_ $specialized_address_type:snake >](bech32: String) -> Result<$specialized_address_type> {
                $base_addr::try_from_bech32(&bech32).and_then(TryInto::<$specialized_address_type>::try_into)
            }

            /// Returns the bech32 encoding of this address
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _bech32_address >](address: &$specialized_address_type) -> String {
                address.to_string()
            }

            /// Returns the network id this address
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _network_id >](address: &$specialized_address_type) -> NetworkID {
                address.secret_magic.network_id()
            }

            impl $specialized_address_type {
                pub fn new(address: $base_addr) -> Result<Self> {
                    if <$base_addr>::$validate(&address) {
                        Ok(Self {
                            secret_magic: address
                        })
                    } else {
                        Err(CommonError::$validation_err)
                    }
                }
            }

            impl TryFrom<$base_addr> for $specialized_address_type {
                type Error = CommonError;

                fn try_from(value: $base_addr) -> Result<Self> {
                    $specialized_address_type::new(value)
                }
            }

            impl Deref for $specialized_address_type {
                type Target = $base_addr;

                fn deref(&self) -> &Self::Target {
                    &self.secret_magic
                }
            }

            impl From<$specialized_address_type> for $base_addr {
                fn from(value: $specialized_address_type) -> Self {
                    value.secret_magic
                }
            }
        }
    };
}

decl_specialized_address!(
    /// NonFungibleResourceAddress is a specialized ResourceAddress for resources
    /// which are non fungible, it ALWAYS has an `'n'` after bech32 separator `'1'`, e.g.:
    /// `"resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"`.
    ///
    /// As opposed to a fungible resource address, e.g. that of XRD which has `'t'`
    /// after bech32 separator `'1'`, see:
    /// `"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"`
    ///
    /// This means that given an instance of `NonFungibleResourceAddress`, it is
    /// guaranteed that its entity type is [`::GlobalNonFungibleResourceManager`],
    /// and not `::GlobalFungibleResourceManager`.
    ///
    /// This type can safely be used with [`StakeClaim`]s, unfortunately since Radix Engine
    /// and/or network does not validate the resource address of a `NonFungibleGlobalId`,
    /// we cannot use this for that type.
    NonFungibleResourceAddress,
    ResourceAddress,
    is_non_fungible,
    FungibleResourceAddressNotAcceptedInNonFungibleContext
);

impl HasSampleValues for NonFungibleResourceAddress {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

impl NonFungibleResourceAddress {
    pub fn sample_mainnet() -> Self {
        ResourceAddress::sample_mainnet_nft_gc_membership()
            .try_into()
            .expect("Valid sample")
    }

    pub fn sample_mainnet_other() -> Self {
        ResourceAddress::sample_mainnet_nft_other()
            .try_into()
            .expect("Valid sample")
    }

    pub fn sample_stokenet() -> Self {
        ResourceAddress::sample_stokenet_nft_gc_membership()
            .try_into()
            .expect("Valid sample")
    }

    pub fn sample_stokenet_other() -> Self {
        ResourceAddress::sample_stokenet_nft_other()
            .try_into()
            .expect("Valid sample")
    }
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_mainnet(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_mainnet_other(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_stokenet(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_stokenet_other(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_stokenet_other()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleResourceAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample_mainnet(), SUT::sample_mainnet());
        assert_eq!(SUT::sample_mainnet_other(), SUT::sample_mainnet_other());
        assert_eq!(SUT::sample_stokenet(), SUT::sample_stokenet());
        assert_eq!(SUT::sample_stokenet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::sample_mainnet(), SUT::sample_stokenet());
        assert_ne!(SUT::sample_mainnet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn display() {
        let s = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa";
        let a = s.parse::<SUT>().unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s = "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa";
        let a = s.parse::<SUT>().unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

    #[test]
    fn manual_perform_uniffi_conversion() {
        type RetAddr = <ResourceAddress as FromRetAddress>::RetAddress;
        let sut = SUT::sample();
        let bech32 = sut.to_string();
        let ret = RetAddr::try_from_bech32(&bech32).unwrap();

        let ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::from_custom(ret);
        assert_eq!(ffi_side, bech32);
        let from_ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::into_custom(
                ffi_side,
            )
            .unwrap();
        assert_eq!(ret, from_ffi_side);
    }

    #[test]
    fn json_roundtrip() {
        let a = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &a,
            json!("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd"),
        );
    }

    #[test]
    fn deref() {
        assert_eq!(*SUT::sample(), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa".parse::<ResourceAddress>().unwrap());
    }

    #[test]
    fn into_resource_address() {
        assert_eq!(Into::<ResourceAddress>::into(SUT::sample()), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa".parse::<ResourceAddress>().unwrap());
    }

    #[test]
    fn new_ok() {
        assert!(
            SUT::new(ResourceAddress::sample_mainnet_nft_gc_membership())
                .is_ok()
        );
    }

    #[test]
    fn try_from_err() {
        assert_eq!(SUT::try_from(ResourceAddress::sample_mainnet_xrd()), Err(CommonError::FungibleResourceAddressNotAcceptedInNonFungibleContext));
    }
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleResourceAddress;

    #[test]
    fn from_bech32() {
        assert_eq!(new_non_fungible_resource_address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa".to_owned()).unwrap(), SUT::sample());
        assert_eq!(new_non_fungible_resource_address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd".to_owned()).unwrap(), SUT::sample_other());
    }

    #[test]
    fn to_bech32() {
        assert_eq!(non_fungible_resource_address_bech32_address(&SUT::sample()), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa");
        assert_eq!(non_fungible_resource_address_bech32_address(&SUT::sample_other()), "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd");
    }

    #[test]
    fn network_id() {
        assert_eq!(
            non_fungible_resource_address_network_id(&SUT::sample_mainnet()),
            NetworkID::Mainnet
        );
        assert_eq!(
            non_fungible_resource_address_network_id(&SUT::sample_stokenet()),
            NetworkID::Stokenet
        );
    }

    #[test]
    fn hash_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_non_fungible_resource_address_sample_mainnet(),
                new_non_fungible_resource_address_sample_mainnet_other(),
                new_non_fungible_resource_address_sample_stokenet(),
                new_non_fungible_resource_address_sample_stokenet_other(),
                // duplicates should be removed
                new_non_fungible_resource_address_sample_mainnet(),
                new_non_fungible_resource_address_sample_mainnet_other(),
                new_non_fungible_resource_address_sample_stokenet(),
                new_non_fungible_resource_address_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
