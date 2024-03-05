use crate::prelude::*;
use std::ops::Deref;

macro_rules! decl_specialized_address {
    ($specialized_address_type: ident, $base_addr: ty, $validate: ident, $validation_err: ident) => {
        uniffi::custom_newtype!($specialized_address_type, $base_addr);

        #[derive(
            Clone,
            Debug,
            PartialEq,
            Eq,
            Hash,
            derive_more::Display,
            derive_more::FromStr,
            SerializeDisplay,
            DeserializeFromStr,
        )]
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
}
