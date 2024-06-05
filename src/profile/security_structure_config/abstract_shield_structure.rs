use crate::prelude::*;

macro_rules! decl_role_security_shield_at_level {
    (
        $role_name: ident,
        $shield_type_name: ident,
        $factor_level_type: ident
    ) => {
        paste! {
            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< $shield_type_name $role_name Role >] {

                /// Factors which are used in combination with other instances, amounting to at
                /// least `threshold` many instances to perform some function with this role.
                pub threshold_factors: Vec<$factor_level_type>, // FIXME: Change to `IdentifiedVecOf`

                /// How many threshold factors that must be used to perform some function with this role.
                pub threshold: u16,

                /// Overriding / Super admin / "sudo" / God / factors, **ANY**
                /// single of these factor which can perform the function of this role,
                /// disregarding of `threshold`.
                pub override_factors: Vec<$factor_level_type>, // FIXME: Change to `IdentifiedVecOf`
            }

            impl [< $shield_type_name $role_name Role >] {
                pub fn new(
                    threshold_factors: impl IntoIterator<Item = $factor_level_type>,
                    threshold: u16,
                    override_factors: impl IntoIterator<Item = $factor_level_type>
                ) -> Self {
                    let _self = Self {
                        threshold_factors: threshold_factors.into_iter().collect(),
                        threshold,
                        override_factors: override_factors.into_iter().collect(),
                    };
                    assert!(_self.threshold_factors.len() >= _self.threshold as usize);
                    _self
                }

                pub fn threshold_of(factors_count: usize) -> u16 {
                    if factors_count > 2 {
                        (factors_count - 1) as u16
                    } else {
                        1
                    }
                }
            }
        }
    };
}

pub(crate) use decl_role_security_shield_at_level;

macro_rules! decl_security_shield_at_level {
    (
        $(
            #[doc = $expr: expr]
        )*
        $shield_type_name: ident,
        $factor_level_type: ident,
    ) => {

        decl_role_security_shield_at_level!(
            Primary,
            $shield_type_name,
            $factor_level_type
        );


        decl_role_security_shield_at_level!(
            Recovery,
            $shield_type_name,
            $factor_level_type
        );

        decl_role_security_shield_at_level!(
            Confirmation,
            $shield_type_name,
            $factor_level_type
        );

        paste! {

            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct $shield_type_name {
                pub metadata: SecurityStructureMetadata,
                pub configuration: [< $shield_type_name Configuration >],
            }

            impl $shield_type_name {
                pub fn new(metadata: SecurityStructureMetadata, configuration: [< $shield_type_name Configuration >]) -> Self {
                    Self {
                        metadata,
                        configuration
                    }
                }
            }

            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< $shield_type_name Configuration >] {
                /// Used for Signing transactions
                pub primary_role: [< $shield_type_name PrimaryRole >],

                /// Used to initiate recovery - resetting the used Security Shield
                /// of an entity.
                pub recovery_role: [< $shield_type_name RecoveryRole >],

                /// To confirm recovery.
                pub confirmation_role: [< $shield_type_name ConfirmationRole >],

                /// End-user selects number of Days in UI, we translate it into
                /// epochs.
                pub number_of_epochs_until_auto_confirmation: u64,
            }

            impl [< $shield_type_name Configuration >] {
                pub fn new(
                    primary_role: [< $shield_type_name PrimaryRole >],
                    recovery_role: [< $shield_type_name RecoveryRole >],
                    confirmation_role: [< $shield_type_name ConfirmationRole >],
                    number_of_epochs_until_auto_confirmation: u64,
                ) -> Self {
                    Self {
                        primary_role,
                        recovery_role,
                        confirmation_role,
                        number_of_epochs_until_auto_confirmation
                    }
                }
            }
        }
    };
}

pub(crate) use decl_security_shield_at_level;
