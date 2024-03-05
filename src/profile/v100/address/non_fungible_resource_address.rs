use crate::prelude::*;
use std::ops::Deref;

macro_rules! decl_specialized_address {
    ($specialized_address_type: ident, $base_addr: ty, $validate: ident, $validation_err: ident) => {
        uniffi::custom_newtype!($specialized_address_type, $base_addr);

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
        )]
        #[debug("{:?}", self.0)]
        pub struct $specialized_address_type($base_addr);
        impl $specialized_address_type {
            pub fn new(address: $base_addr) -> Result<Self> {
                if <$base_addr>::$validate(&address) {
                    Ok(Self(address))
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
                &self.0
            }
        }

        impl From<$specialized_address_type> for $base_addr {
            fn from(value: $specialized_address_type) -> Self {
                value.0
            }
        }
    };
}

decl_specialized_address!(
    NonFungibleResourceAddress,
    ResourceAddress,
    is_non_fungible,
    FungibleResourceAddressNotAcceptedInNonFungibleContext
);

impl HasSampleValues for NonFungibleResourceAddress {
    fn sample() -> Self {
        "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"
            .parse()
            .expect("Valid GC NFT Global ID")
    }

    fn sample_other() -> Self {
        "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd"
            .parse()
            .expect("Valid Scorpion NFT Global ID")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleResourceAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
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

    #[test]
    fn manual_perform_uniffi_conversion_successful() {
        let sut = SUT::sample();
        let builtin = sut.clone().0;

        let ffi_side =
            <SUT as crate::UniffiCustomTypeConverter>::from_custom(sut.clone());

        assert_eq!(ffi_side.clone(), builtin.clone());

        let from_ffi_side =
            <SUT as crate::UniffiCustomTypeConverter>::into_custom(ffi_side)
                .unwrap();

        assert_eq!(sut, from_ffi_side);
    }
}
