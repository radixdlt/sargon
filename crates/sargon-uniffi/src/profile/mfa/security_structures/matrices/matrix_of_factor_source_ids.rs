use crate::prelude::*;

use super::decl_matrix_macro::matrix_conversion;

matrix_conversion!(FactorSourceID);

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

export_sample_config!(11);
export_sample_config!(12);
export_sample_config!(13);
export_sample_config!(14);
export_sample_config!(15);
export_sample_config!(21);
export_sample_config!(22);
export_sample_config!(23);
export_sample_config!(24);
export_sample_config!(30);
export_sample_config!(40);
export_sample_config!(51);
export_sample_config!(52);
export_sample_config!(60);
export_sample_config!(70);
export_sample_config!(80);
export_sample_config!(90);

