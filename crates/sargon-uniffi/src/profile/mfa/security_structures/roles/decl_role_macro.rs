use crate::prelude::*;

// use sargon::ConfirmationRoleWithFactorSources as InternalConfirmationRoleWithFactorSources;
// use sargon::PrimaryRoleWithFactorSources as InternalPrimaryRoleWithFactorSources;
// use sargon::RecoveryRoleWithFactorSources as InternalRecoveryRoleWithFactorSources;

// #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
// pub struct PrimaryRoleWithFactorSources {
//     pub threshold: u8,
//     pub threshold_factors: Vec<FactorSource>,
//     pub override_factors: Vec<FactorSource>,
// }

macro_rules! role_conversion {
    // Impl From<Internal> -> crate
    (from_internal: $internal:ident, $uniffi:ident) => {
        impl From<$internal> for $uniffi {
            fn from(value: $internal) -> Self {
                Self {
                    threshold: value.get_threshold(),
                    threshold_factors: value
                        .get_threshold_factors()
                        .into_iter()
                        .map(|x| x.clone().into())
                        .collect(),
                    override_factors: value
                        .get_override_factors()
                        .into_iter()
                        .map(|x| x.clone().into())
                        .collect(),
                }
            }
        }
    };

    // Impl From<crate> -> Internal
    (to_internal: $internal_factor:ty => $uniffi:ident, $internal:ident) => {
        impl From<$uniffi> for $internal {
            fn from(value: $uniffi) -> Self {
                unsafe {
                    Self::unbuilt_with_factors(
                        value.threshold,
                        value.threshold_factors.into_iter().map(|x| Into::<$internal_factor>::into(x.clone())).collect::<Vec<_>>(),
                        value.override_factors.into_iter().map(|x| Into::<$internal_factor>::into(x.clone())).collect::<Vec<_>>(),
                    )
                }
            }
        }
    };

    (impl_from: $factor_level:ty => $uniffi_name:ident => $internal_name:ident ) => {
        role_conversion!(
            from_internal: $internal_name, $uniffi_name
        );
        role_conversion!(
            to_internal: $factor_level => $uniffi_name, $internal_name
        );
    };

    (struct: $struct_name:ident, $factor_level:ty) => {
        #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
        pub struct $struct_name {
            pub threshold: u8,
            pub threshold_factors: Vec<$factor_level>,
            pub override_factors: Vec<$factor_level>,
        }
        paste! {
            use sargon::$struct_name as [< Internal $struct_name>];
            role_conversion!(
                impl_from: [< Internal $factor_level>] => $struct_name => [< Internal $struct_name >]
            );
        }
    };
    (for: $role:ident $factor_level:ty) => {
        paste! {
            role_conversion!(
                struct: [< $role RoleWith $factor_level s >],
                $factor_level
            );
        }
    };
    ($factor_level:ty) => {
        paste! {
            use sargon::$factor_level as [< Internal $factor_level>];
        }
        role_conversion!(for: Primary $factor_level);
        role_conversion!(for: Recovery $factor_level);
        role_conversion!(for: Confirmation $factor_level);
    };
}
pub(crate) use role_conversion;
