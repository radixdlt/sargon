use crate::prelude::*;

use super::decl_matrix_macro::matrix_conversion;

matrix_conversion!(
    /// Matrix of `FactorSourceID`s containing the primary, recovery, and confirmation roles with `FactorSourceID`s
    FactorSourceID
);

macro_rules! export_sample_config {
    ($config_ident:literal) => {
        paste! {
            #[uniffi::export]
            pub fn [<new_ sample_config_ $config_ident>]() -> MatrixOfFactorSourceIDs {
                <sargon::MatrixOfFactorSourceIDs>::[< sample_config_  $config_ident>]().into()
            }
        }
    };
}

pub(crate) use export_sample_config;

export_sample_config!(1_1);
export_sample_config!(1_2);
export_sample_config!(1_3);
export_sample_config!(1_4);
export_sample_config!(1_5);
export_sample_config!(2_1);
export_sample_config!(2_2);
export_sample_config!(2_3);
export_sample_config!(2_4);
export_sample_config!(3_0);
export_sample_config!(4_0);
export_sample_config!(5_1);
export_sample_config!(5_2);
export_sample_config!(6_0);
export_sample_config!(7_0);
export_sample_config!(8_0);
export_sample_config!(9_0);
