use crate::prelude::*;

macro_rules! decl_role_with_factors_additional_impl {
    (
        $role: ident,
        FactorInstance
    ) => {
        paste! {
            impl From<[< $role RoleWithFactorInstance s >]> for ScryptoAccessRule {
                fn from(value: [< $role RoleWithFactorInstance s >]) -> Self {
                    ScryptoAccessRule::Protected(ScryptoCompositeRequirement::AnyOf(vec![
                        ScryptoCompositeRequirement::BasicRequirement(ScryptoBasicRequirement::CountOf(
                            value.threshold,
                            value
                                .threshold_factors
                                .into_iter()
                                .map(|instance| instance.badge)
                                .map(ScryptoResourceOrNonFungible::from)
                                .collect(),
                        )),
                        ScryptoCompositeRequirement::BasicRequirement(ScryptoBasicRequirement::AnyOf(
                            value
                                .override_factors
                                .into_iter()
                                .map(|instance| instance.badge)
                                .map(ScryptoResourceOrNonFungible::from)
                                .collect(),
                        )),
                    ]))
                }
            }
        }
    };
    (
        $role: ident,
        $factor: ident
    ) => {}
}

pub(crate) use decl_role_with_factors_additional_impl;

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
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash,
            )]
            #[serde(rename_all = "camelCase")]
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

            impl [< $role RoleWith $factor s >] {
                // # Panics
                /// Panics if threshold > threshold_factor.len()
                ///
                /// Panics if the same factor is present in both lists
                pub fn new(
                    threshold_factors: impl IntoIterator<Item = $factor>,
                    threshold: u8,
                    override_factors: impl IntoIterator<Item = $factor>
                ) -> Result<Self> {
                    let threshold_factors = threshold_factors.into_iter().collect_vec();

                    if threshold_factors.len() < threshold as usize {
                        return Err(CommonError::InvalidSecurityStructureThresholdExceedsFactors {
                            threshold,
                            factors: threshold_factors.len() as u8
                        })
                    }

                    let override_factors = override_factors.into_iter().collect_vec();

                    if !HashSet::<$factor>::from_iter(threshold_factors.clone())
                            .intersection(&HashSet::<$factor>::from_iter(override_factors.clone()))
                            .collect_vec()
                            .is_empty() {
                        return Err(CommonError::InvalidSecurityStructureFactorInBothThresholdAndOverride)
                    }

                    Ok(Self {
                        threshold_factors,
                        threshold,
                        override_factors,
                    })
                }

                pub fn all_factors(&self) -> HashSet<&$factor> {
                    let mut factors = HashSet::from_iter(self.threshold_factors.iter());
                    factors.extend(self.override_factors.iter());
                    factors
                }
            }

            decl_role_with_factors_additional_impl!($role, $factor);
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
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash,
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

                pub fn all_factors(&self) -> HashSet<&$factor> {
                    let mut factors = HashSet::new();
                    factors.extend(self.primary_role.all_factors());
                    factors.extend(self.recovery_role.all_factors());
                    factors.extend(self.confirmation_role.all_factors());
                    factors
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
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash,
            )]
            #[serde(rename_all = "camelCase")]
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

            impl [< SecurityStructureOf $factor s >] {
                pub fn new(metadata: SecurityStructureMetadata, number_of_epochs_until_auto_confirmation: u64, matrix_of_factors: [< MatrixOf $factor s >]) -> Self {
                    Self {
                        metadata,
                        number_of_epochs_until_auto_confirmation,
                        matrix_of_factors
                    }
                }

                pub fn all_factors(&self) -> HashSet<&$factor> {
                    self.matrix_of_factors.all_factors()
                }
            }
        }
    };
}

pub(crate) use decl_security_structure_of;
