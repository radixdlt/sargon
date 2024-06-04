use crate::prelude::*;

macro_rules! decl_security_structure_config {
    (
        $(
            #[doc = $expr: expr]
        )*
        $factor_level_name: ident,
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
            pub struct [< SecurityStructureConfiguration $factor_level_name >] {
                pub metadata: SecurityStructureMetadata,
                pub configuration: [< SecurityStructure $factor_level_name >],
            }

            impl [< SecurityStructureConfiguration $factor_level_name >] {
                pub fn new(metadata: SecurityStructureMetadata, configuration: [< SecurityStructure $factor_level_name >]) -> Self {
                    Self {
                        metadata,
                        configuration
                    }
                }
            }

            impl HasSampleValues for [< SecurityStructureConfiguration $factor_level_name >] {
                fn sample() -> Self {
                    Self::new(
                        SecurityStructureMetadata::sample(),
                        [< SecurityStructure $factor_level_name >]::sample()
                    )
                }

                fn sample_other() -> Self {
                    Self::new(
                        SecurityStructureMetadata::sample_other(),
                        [< SecurityStructure $factor_level_name >]::sample_other()
                    )
                }
            }

            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< RoleOfTier $factor_level_name >] {

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

            impl [< RoleOfTier $factor_level_name >] {
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

                pub fn sample_primary() -> Self {
                    let override_factors: Vec<$factor_level_type> = $primary_role_override_factors_sample.into_iter().collect_vec();
                    let threshold_factors: Vec<$factor_level_type> = $primary_role_threshold_factors_sample.into_iter().collect_vec();
                    let threshold = Self::theshold_of(threshold_factors.len());
                    Self::new(
                        threshold_factors,
                        threshold as u16,
                        override_factors
                    )
                }

                pub fn sample_primary_other() -> Self {
                    let override_factors: Vec<$factor_level_type> = $primary_role_override_factors_sample_other.into_iter().collect_vec();
                    let threshold_factors: Vec<$factor_level_type> = $primary_role_threshold_factors_sample_other.into_iter().collect_vec();
                    let threshold = Self::theshold_of(threshold_factors.len());
                    Self::new(
                        threshold_factors,
                        threshold as u16,
                        override_factors
                    )
                }

                pub fn sample_recovery() -> Self {
                    let override_factors: Vec<$factor_level_type> = $recovery_role_override_factors_sample.into_iter().collect_vec();
                    let threshold_factors: Vec<$factor_level_type> = $recovery_role_threshold_factors_sample.into_iter().collect_vec();
                    let threshold = Self::theshold_of(threshold_factors.len());
                    Self::new(
                        threshold_factors,
                        threshold as u16,
                        override_factors
                    )
                }

                pub fn sample_recovery_other() -> Self {
                    let override_factors: Vec<$factor_level_type> = $recovery_role_override_factors_sample_other.into_iter().collect_vec();
                    let threshold_factors: Vec<$factor_level_type> = $recovery_role_threshold_factors_sample_other.into_iter().collect_vec();
                    let threshold = Self::theshold_of(threshold_factors.len());
                    Self::new(
                        threshold_factors,
                        threshold as u16,
                        override_factors
                    )
                }

                pub fn sample_confirmation() -> Self {
                    let override_factors: Vec<$factor_level_type> = $confirmation_role_override_factors_sample.into_iter().collect_vec();
                    let threshold_factors: Vec<$factor_level_type> = $confirmation_role_threshold_factors_sample.into_iter().collect_vec();
                    let threshold = Self::theshold_of(threshold_factors.len());
                    Self::new(
                        threshold_factors,
                        threshold as u16,
                        override_factors
                    )
                }

                pub fn sample_confirmation_other() -> Self {
                    let override_factors: Vec<$factor_level_type> = $confirmation_role_override_factors_sample_other.into_iter().collect_vec();
                    let threshold_factors: Vec<$factor_level_type> = $confirmation_role_threshold_factors_sample_other.into_iter().collect_vec();
                    let threshold = Self::theshold_of(threshold_factors.len());
                    Self::new(
                        threshold_factors,
                        threshold as u16,
                        override_factors
                    )
                }
            }

            impl HasSampleValues for [< RoleOfTier $factor_level_name >] {
                fn sample() -> Self {
                    Self::sample_primary()
                }
                fn sample_other() -> Self {
                    Self::sample_confirmation()
                }
            }

            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< SecurityStructure $factor_level_name >] {
                /// Used for Signing transactions
                pub primary_role: [< RoleOfTier $factor_level_name>],

                /// Used to initiate recovery - resetting the used Security Shield
                /// of an entity.
                pub recovery_role: [< RoleOfTier $factor_level_name>],

                /// To confirm recovery.
                pub confirmation_role: [< RoleOfTier $factor_level_name>],

                /// End-user selects number of Days in UI, we translate it into
                /// epochs.
                pub number_of_epochs_until_auto_confirmation: u64,
            }

            impl [< SecurityStructure $factor_level_name >] {
                pub fn new(
                    primary_role: [< RoleOfTier $factor_level_name>],
                    recovery_role: [< RoleOfTier $factor_level_name>],
                    confirmation_role: [< RoleOfTier $factor_level_name>],
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

            impl HasSampleValues for [< SecurityStructure $factor_level_name >] {
                fn sample() -> Self {
                    Self::new(
                        [< RoleOfTier $factor_level_name>]::sample_primary(),
                        [< RoleOfTier $factor_level_name>]::sample_recovery(),
                        [< RoleOfTier $factor_level_name>]::sample_confirmation(),
                        4096, // number of epochs => ca 14.2 days (288 epochs / day)
                    )
                }

                fn sample_other() -> Self {
                    Self::new(
                        [< RoleOfTier $factor_level_name>]::sample_primary_other(),
                        [< RoleOfTier $factor_level_name>]::sample_recovery_other(),
                        [< RoleOfTier $factor_level_name>]::sample_confirmation_other(),
                        8192, // number of epochs => ca 28.2 days (288 epochs / day)
                    )
                }
            }
        }
    };
}

pub(crate) use decl_security_structure_config;
