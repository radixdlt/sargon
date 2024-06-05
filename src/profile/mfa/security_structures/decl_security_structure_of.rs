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
            $(
                #[doc = $expr]
            )*
            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< $role RoleWith $factor s >] {

                /// Factors which are used in combination with other instances, amounting to at
                /// least `threshold` many instances to perform some function with this role.
                pub threshold_factors: OrderedSet<$factor>,

                /// How many threshold factors that must be used to perform some function with this role.
                pub threshold: u16,

                /// Overriding / Super admin / "sudo" / God / factors, **ANY**
                /// single of these factor which can perform the function of this role,
                /// disregarding of `threshold`.
                pub override_factors: OrderedSet<$factor>,
            }

            impl [< $role RoleWith $factor s >] {
                pub fn new(
                    threshold_factors: impl IntoIterator<Item = $factor>,
                    threshold: u16,
                    override_factors: impl IntoIterator<Item = $factor>
                ) -> Self {
                    let _self = Self {
                        threshold_factors: threshold_factors.into_iter().collect(),
                        threshold,
                        override_factors: override_factors.into_iter().collect(),
                    };
                    assert!(_self.threshold_factors.len() >= _self.threshold as usize);
                    _self
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
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< MatrixOf $factor s >] {
                /// Used for Signing transactions
                pub primary_role: [< PrimaryRoleWith $factor s >],

                /// Used to initiate recovery - resetting the used Security Shield
                /// of an entity.
                pub recovery_role: [< RecoveryRoleWith $factor s >],

                /// To confirm recovery.
                pub confirmation_role: [< ConfirmationRoleWith $factor s >],
            }

            impl [< MatrixOf $factor s >] {
                pub fn new(
                    primary_role: [< PrimaryRoleWith $factor s >],
                    recovery_role: [< RecoveryRoleWith $factor s >],
                    confirmation_role: [< ConfirmationRoleWith $factor s >],
                ) -> Self {
                    Self {
                        primary_role,
                        recovery_role,
                        confirmation_role,
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

            $(
                #[doc = $expr]
            )*
            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< SecurityStructureOf $factor s >] {
                /// Metadata of this Security Structure, such as globally unique and
                /// stable identifier, creation date and user chosen label (name).
                pub metadata: SecurityStructureMetadata,

                /// The amount of time until Confirmation Role is automatically
                /// excercised, inputted by user in Days in UI, but translate it into
                /// epochs ("block time").
                pub number_of_epochs_until_auto_confirmation: u64,

                /// The structure of factors to use for certain roles, Primary, Recovery
                /// and Confirmation role.
                pub matrix_of_factors: [< MatrixOf $factor s >],
            }

            impl [< SecurityStructureOf $factor s >] {
                pub fn new(metadata: SecurityStructureMetadata, number_of_epochs_until_auto_confirmation: u64, matrix_of_factors: [< MatrixOf $factor s >]) -> Self {
                    Self {
                        metadata,
                        number_of_epochs_until_auto_confirmation,
                        matrix_of_factors
                    }
                }
            }
        }
    };
}

pub(crate) use decl_security_structure_of;
