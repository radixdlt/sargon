use crate::prelude::*;

macro_rules! matrix_conversion {
    (struct: $struct_name:ident, $factor_level:ty) => {
        paste! {
            #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
            pub struct $struct_name {
                pub primary_role: [< PrimaryRoleWith $factor_level s >],
                pub recovery_role: [< RecoveryRoleWith $factor_level s >],
                pub confirmation_role: [< ConfirmationRoleWith $factor_level s >],
                
                pub number_of_days_until_auto_confirm: u16,
            }
        }
    };
    ($factor_level:ty) => {
        paste! {
            use sargon::$factor_level as [< Internal $factor_level>];
            matrix_conversion!(
                struct: [< $role Of $factor_level s >],
                $factor_level
            );
        }
    };
}

pub(crate) use matrix_conversion;

