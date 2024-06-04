use crate::prelude::*;

macro_rules! decl_security_shield_at_level {
    (
        $(
            #[doc = $expr: expr]
        )*
        $shield_type_name: ident,
        $factor_level_type: ident,
        $primary_role_threshold_factors_sample: expr,
        $primary_role_threshold_factors_sample_other: expr,
        $primary_role_override_factors_sample: expr,
        $primary_role_override_factors_sample_other: expr,
        $recovery_role_threshold_factors_sample: expr,
        $recovery_role_threshold_factors_sample_other: expr,
        $recovery_role_override_factors_sample: expr,
        $recovery_role_override_factors_sample_other: expr,
        $confirmation_role_threshold_factors_sample: expr,
        $confirmation_role_threshold_factors_sample_other: expr,
        $confirmation_role_override_factors_sample: expr,
        $confirmation_role_override_factors_sample_other: expr,
    ) => {
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

            impl HasSampleValues for $shield_type_name {
                fn sample() -> Self {
                    Self::new(
                        SecurityStructureMetadata::sample(),
                        [< $shield_type_name Configuration >]::sample()
                    )
                }

                fn sample_other() -> Self {
                    Self::new(
                        SecurityStructureMetadata::sample_other(),
                        [< $shield_type_name Configuration >]::sample_other()
                    )
                }
            }

            macro_rules! decl_role_security_shield_at_level {
                (
                    $role_name: ident
                ) => {

                    #[derive(
                        Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
                    )]
                    #[serde(rename_all = "camelCase")]
                    pub struct [< $role_name:camel Role $shield_type_name >] {

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

                    impl [< $role_name Role $shield_type_name >] {
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

                        fn theshold_of(factors_count: usize) -> u16 {
                            if factors_count > 2 {
                                (factors_count - 1) as u16
                            } else {
                                1
                            }
                        }
                    }

                    impl HasSampleValues for [< $role_name Role $shield_type_name >] {
                        fn sample() -> Self {
                            let override_factors: Vec<$factor_level_type> = [< $ $role_name _role_override_factors_sample >].into_iter().collect_vec();
                            let threshold_factors: Vec<$factor_level_type> = [< $ $role_name _role_override_threshold_sample >].into_iter().collect_vec();
                            let threshold = Self::theshold_of(threshold_factors.len());
                            Self::new(
                                threshold_factors,
                                threshold as u16,
                                override_factors
                            )
                        }

                        fn sample_other() -> Self {
                            let override_factors: Vec<$factor_level_type> = [< $ $role_name _role_override_factors_sample_other >].into_iter().collect_vec();
                            let threshold_factors: Vec<$factor_level_type> = [< $ $role_name _role_override_threshold_sample_other >].into_iter().collect_vec();
                            let threshold = Self::theshold_of(threshold_factors.len());
                            Self::new(
                                threshold_factors,
                                threshold as u16,
                                override_factors
                            )
                        }

                    }
                };
            }

            decl_role_security_shield_at_level!(primary);
            decl_role_security_shield_at_level!(confirmation);
            decl_role_security_shield_at_level!(recovery);


            // impl HasSampleValues for [< Role $shield_type_name >] {
            //     fn sample() -> Self {
            //         Self::sample_primary()
            //     }
            //     fn sample_other() -> Self {
            //         Self::sample_confirmation()
            //     }
            // }

            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< $shield_type_name Configuration >] {
                /// Used for Signing transactions
                pub primary_role: [< PrimaryRole $factor_level_name>],

                /// Used to initiate recovery - resetting the used Security Shield
                /// of an entity.
                pub recovery_role: [< RecoveryRole $factor_level_name>],

                /// To confirm recovery.
                pub confirmation_role: [< ConfirmationRole $factor_level_name>],

                /// End-user selects number of Days in UI, we translate it into
                /// epochs.
                pub number_of_epochs_until_auto_confirmation: u64,
            }

            impl [< $shield_type_name Configuration >] {
                pub fn new(
                    primary_role: [< PrimaryRole $factor_level_name>],
                    recovery_role: [< RecoveryRole $factor_level_name>],
                    confirmation_role: [< ConfirmationRole $factor_level_name>],
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

            impl HasSampleValues for [< $shield_type_name Configuration >] {
                fn sample() -> Self {
                    Self::new(
                        [< PrimaryRole $factor_level_name>]::sample_primary(),
                        [< RecoveryRole $factor_level_name>]::sample_recovery(),
                        [< ConfirmationRole $factor_level_name>]::sample_confirmation(),
                        4096, // number of epochs => ca 14.2 days (288 epochs / day)
                    )
                }

                fn sample_other() -> Self {
                    Self::new(
                        [< PrimaryRole $factor_level_name>]::sample_primary_other(),
                        [< RecoveryRole $factor_level_name>]::sample_recovery_other(),
                        [< ConfirmationRole $factor_level_name>]::sample_confirmation_other(),
                        8192, // number of epochs => ca 28.2 days (288 epochs / day)
                    )
                }
            }
        }
    };
}

pub(crate) use decl_security_shield_at_level;
