use crate::prelude::*;

macro_rules! matrix_conversion {
       // Impl From<Internal> -> crate
       (from_internal: $internal:ident, $uniffi:ident) => {
        impl From<$internal> for $uniffi {
            fn from(value: $internal) -> Self {
                    Self {
            primary_role: value.primary().clone().into(),
            recovery_role: value.recovery().clone().into(),
            confirmation_role: value.confirmation().clone().into(),
            number_of_days_until_auto_confirm: value
                .number_of_days_until_auto_confirm,
        }
            }
        }
    };

    // Impl From<crate> -> Internal
    (to_internal: $internal_factor:ty => $uniffi:ident, $internal:ident) => {
        impl From<$uniffi> for $internal {
            fn from(value: $uniffi) -> Self {
                    unsafe {
            Self::unbuilt_with_roles_and_days(
                value.primary_role.into(),
                value.recovery_role.into(),
                value.confirmation_role.into(),
                value.number_of_days_until_auto_confirm,
            )
        }
            }
        }
    };
       // Impl `From<InternalX> for X` and `From<X> for InternalX`
    // and impl `HasSampleValues` for X` and UniFFI export `new_X_sample` and `new_X_sample_other`.
    (impl_from: $factor_level:ty => $uniffi_name:ident => $internal_name:ident ) => {
        matrix_conversion!(
            from_internal: $internal_name, $uniffi_name
        );
        matrix_conversion!(
            to_internal: $factor_level => $uniffi_name, $internal_name
        );

        impl HasSampleValues for $uniffi_name {
            fn sample() -> Self {
                $internal_name::sample().into()
            }
            fn sample_other() -> Self {
                $internal_name::sample().into()
            }
        }

        paste! {
            #[uniffi::export]
            pub fn [< new_ $uniffi_name:snake _ sample >]() -> $uniffi_name {
                $uniffi_name::sample()
            }

            #[uniffi::export]
            pub fn [< new_ $uniffi_name:snake _ sample_other >]() -> $uniffi_name {
                $uniffi_name::sample_other()
            }
        }
    };
    (struct: $struct_name:ident, $factor_level:ty) => {
        paste! {
            #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
            pub struct $struct_name {
                pub primary_role: [< PrimaryRoleWith $factor_level s >],
                pub recovery_role: [< RecoveryRoleWith $factor_level s >],
                pub confirmation_role: [< ConfirmationRoleWith $factor_level s >],

                pub number_of_days_until_auto_confirm: u16,
            }

            use sargon::$struct_name as [< Internal $struct_name>];
            matrix_conversion!(
                impl_from: [< Internal $factor_level>] => $struct_name => [< Internal $struct_name >]
            );
        }
    };
    ($factor_level:ty) => {
        paste! {
            use sargon::$factor_level as [< Internal $factor_level>];
            matrix_conversion!(
                struct: [< MatrixOf $factor_level s >],
                $factor_level
            );
        }
    };
}

pub(crate) use matrix_conversion;
