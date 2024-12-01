use crate::prelude::*;

// This macro generates "Role" types, for each RoleKind: Primary, Recovery, Confirmation
// for the specified "Factor Level", so input is `FactorSource` or `FactorInstance`
// or `FactorSourceID` etc.
// It will generate the following:
// struct PrimaryRoleWithFactor<$FACTOR_LEVEL> {
//     threshold: u8,
//     threshold_factors: Vec<$FACTOR_LEVEL>,
//     override_factors: Vec<$FACTOR_LEVEL>,
// }
// and `struct RecoveryRoleWithFactor<$FACTOR_LEVEL>`
// and `struct ConfirmationRoleWithFactor<$FACTOR_LEVEL>`
//
// And it will generate:
// * `From<InternalPrimaryRoleWithFactor<$FACTOR_LEVEL>> for PrimaryRoleWithFactor<$FACTOR_LEVEL>`
// * `From<InternalRecoveryRoleWithFactor<$FACTOR_LEVEL>> for RecoveryRoleWithFactor<$FACTOR_LEVEL>`
// * `From<InternalConfirmationRoleWithFactor<$FACTOR_LEVEL>> for ConfirmationRoleWithFactor<$FACTOR_LEVEL>`
//
// And analogously it will impl the inverse conversion:
// * `From<PrimaryRoleWithFactor<$FACTOR_LEVEL>> for InternalPrimaryRoleWithFactor<$FACTOR_LEVEL>`
// * `From<RecoveryRoleWithFactor<$FACTOR_LEVEL>> for InternalRecoveryRoleWithFactor<$FACTOR_LEVEL>`
// * `From<ConfirmationRoleWithFactor<$FACTOR_LEVEL>> for InternalConfirmationRoleWithFactor<$FACTOR_LEVEL>`
//
// Furthermore it will generate `HasSampleValues` impl for each of the generated structs.
// and also uniffi export them.
macro_rules! role_conversion {
    ($factor_level:ty) => {
        paste! {
            use sargon::$factor_level as [< Internal $factor_level>];
        }
        role_conversion_inner!(for: Primary $factor_level);
        role_conversion_inner!(for: Recovery $factor_level);
        role_conversion_inner!(for: Confirmation $factor_level);
    };
}

pub(crate) use role_conversion;

macro_rules! role_conversion_inner {
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

    // Impl `From<InternalX> for X` and `From<X> for InternalX`
    // and impl `HasSampleValues` for X` and UniFFI export `new_X_sample` and `new_X_sample_other`.
    (impl_from: $factor_level:ty => $uniffi_name:ident => $internal_name:ident ) => {
        role_conversion_inner!(
            from_internal: $internal_name, $uniffi_name
        );
        role_conversion_inner!(
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

    // Declare the struct and impl `From<Internal>
    // and by recursively calling `role_conversion_inner` also impl
    // `From` conversions.
    (struct: $struct_name:ident, $factor_level:ty) => {
        /// A role with a threshold, threshold_factors and override_factors.
        #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
        pub struct $struct_name {

            /// How many threshold factors that must be used to perform some function with
            /// this role.
            pub threshold: u8,

            /// Factors which are used in combination with other factors, amounting to at
            /// least `threshold` many factors to perform some function with this role.
            pub threshold_factors: Vec<$factor_level>,

            /// Overriding / Super admin / "sudo" / God / factors, **ANY**
            /// single of these factor which can perform the function of this role,
            /// disregarding of `threshold`.
            pub override_factors: Vec<$factor_level>,
        }
        paste! {
            use sargon::$struct_name as [< Internal $struct_name>];
            role_conversion_inner!(
                impl_from: [< Internal $factor_level>] => $struct_name => [< Internal $struct_name >]
            );
        }
    };
    (for: $role:ident $factor_level:ty) => {
        paste! {
            role_conversion_inner!(
                struct: [< $role RoleWith $factor_level s >],
                $factor_level
            );
        }
    };
}

pub(crate) use role_conversion_inner;
