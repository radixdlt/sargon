use crate::prelude::*;

macro_rules! dummy_sargon {
    ($struct_name:ident) => {
        #[derive(
            Serialize,
            Deserialize,
            Clone,
            Debug,
            Default,
            PartialEq,
            Eq,
            Hash,
            uniffi::Record,
        )]
        pub struct $struct_name {}
    };
}

dummy_sargon!(PackageAddress);
dummy_sargon!(ComponentAddress);
dummy_sargon!(AccessControllerAddress);
dummy_sargon!(VaultAddress);
dummy_sargon!(ValidatorAddress);
dummy_sargon!(ResourcePoolAddress);

// Rename and use Decimal type!
dummy_sargon!(RETDecimal);
