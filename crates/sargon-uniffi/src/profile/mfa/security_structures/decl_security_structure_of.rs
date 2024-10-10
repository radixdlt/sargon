use crate::prelude::*;

macro_rules! decl_role_with_factors {
    (
        $(
            #[doc = $expr: expr]
        )*
        $role: ident,
        $factor: ident
    ) => {
        paste! {
            use sargon::[< $role RoleWith $factor s >] as [< Internal $role RoleWith $factor s >];

            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,  PartialEq, Eq, Hash,  uniffi::Record,
            )]
            pub struct [< $role RoleWith $factor s >] {

                /// Factors which are used in combination with other instances, amounting to at
                /// least `threshold` many instances to perform some function with this role.
                ///
                /// # Implementation
                /// Must allow duplicates, thus using `Vec` since at FactorSourceKind level
                /// we might wanna use duplicates, allowing us to build a "template"
                /// structure where a role might contain two `FactorSourceKind::TrustedContact`,
                /// meaning an instance of this template at FactorSource level
                /// (`SecurityStructureOfFactorSources`) will contain two different
                /// `TrustedContactFactorSource`s.
                pub threshold_factors: Vec<$factor>,

                /// How many threshold factors that must be used to perform some function with this role.
                pub threshold: u8,

                /// Overriding / Super admin / "sudo" / God / factors, **ANY**
                /// single of these factor which can perform the function of this role,
                /// disregarding of `threshold`.
                pub override_factors: Vec<$factor>,
            }

            impl From<[< Internal $role RoleWith $factor s >]> for [< $role RoleWith $factor s >] {
                fn from(value: [< Internal $role RoleWith $factor s >]) -> Self {
                    Self {
                        threshold_factors: value.threshold_factors.into_vec(),
                        threshold: value.threshold,
                        override_factors: value.override_factors.into_vec(),
                    }
                }
            }

            impl Into<[< Internal $role RoleWith $factor s >]> for [< $role RoleWith $factor s >] {
                fn into(self) -> [< Internal $role RoleWith $factor s >] {
                    [< Internal $role RoleWith $factor s >] {
                        threshold_factors: self.threshold_factors.into_internal_vec(),
                        threshold: self.threshold,
                        override_factors: self.override_factors.into_internal_vec(),
                    }
                }
            }
        }
    };
}

pub(crate) use decl_role_with_factors;

macro_rules! decl_matrix_of_factors {
    (
        $(
            #[doc = $expr: expr]
        )*
        $factor: ident
    ) => {
        paste! {
            use sargon::[< MatrixOf $factor s >] as [< InternalMatrixOf $factor s >];

            decl_role_with_factors!(
                /// PrimaryRole is used for Signing Transactions.
                Primary,
                $factor
            );

            decl_role_with_factors!(
                /// RecoveryRole is used to recover lost access to an entity.
                Recovery,
                $factor
            );

            decl_role_with_factors!(
                /// ConfirmationRole is used to confirm recovery.
                Confirmation,
                $factor
            );

            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,  PartialEq, Eq, Hash,  uniffi::Record,
            )]
            pub struct [< MatrixOf $factor s >] {
                /// Used for Signing transactions
                pub primary_role: [< PrimaryRoleWith $factor s >],

                /// Used to initiate recovery - resetting the used Security Shield
                /// of an entity.
                pub recovery_role: [< RecoveryRoleWith $factor s >],

                /// To confirm recovery.
                pub confirmation_role: [< ConfirmationRoleWith $factor s >],
            }

            impl From<[< InternalMatrixOf $factor s >]> for [< MatrixOf $factor s >] {
                fn from(value: [< InternalMatrixOf $factor s >]) -> Self {
                    Self {
                        primary_role: value.primary_role.into(),
                        recovery_role: value.recovery_role.into(),
                        confirmation_role: value.confirmation_role.into(),
                    }
                }
            }

            impl Into<[< InternalMatrixOf $factor s >]> for [< MatrixOf $factor s >] {
                fn into(self) -> [< InternalMatrixOf $factor s >] {
                    [< InternalMatrixOf $factor s >] {
                        primary_role: self.primary_role.into(),
                        recovery_role: self.recovery_role.into(),
                        confirmation_role: self.confirmation_role.into(),
                    }
                }
            }
        }
    };
}

pub(crate) use decl_matrix_of_factors;

macro_rules! decl_security_structure_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $factor: ident,
    ) => {

        decl_matrix_of_factors!($factor);

        paste! {
            use sargon::[< SecurityStructureOf $factor s >] as InternalSecurityStructureOfFactors;

            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,  PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
            )]
            pub struct [< SecurityStructureOf $factor s >] {
                /// Metadata of this Security Structure, such as globally unique and
                /// stable identifier, creation date and user chosen label (name).
                pub metadata: SecurityStructureMetadata,

                /// The amount of time until Confirmation Role is automatically
                /// exercised, inputted by user in Days in UI, but translate it into
                /// epochs ("block time").
                pub number_of_epochs_until_auto_confirmation: u64,

                /// The structure of factors to use for certain roles, Primary, Recovery
                /// and Confirmation role.
                pub matrix_of_factors: [< MatrixOf $factor s >],
            }

            impl From<InternalSecurityStructureOfFactors> for [< SecurityStructureOf $factor s >] {
                fn from(value: InternalSecurityStructureOfFactors) -> Self {
                    Self {
                        metadata: value.metadata.into(),
                        number_of_epochs_until_auto_confirmation: value.number_of_epochs_until_auto_confirmation,
                        matrix_of_factors: value.matrix_of_factors.into(),
                    }
                }
            }

            impl Into<InternalSecurityStructureOfFactors> for [< SecurityStructureOf $factor s >] {
                fn into(self) -> InternalSecurityStructureOfFactors {
                    InternalSecurityStructureOfFactors {
                        metadata: self.metadata.into(),
                        number_of_epochs_until_auto_confirmation: self.number_of_epochs_until_auto_confirmation,
                        matrix_of_factors: self.matrix_of_factors.into(),
                    }
                }
            }
        }
    };
}

pub(crate) use decl_security_structure_of;
