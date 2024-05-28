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
        $primary_role_super_admin_factors_sample: expr,
        $primary_role_super_admin_factors_sample_other: expr,
        $confirmation_role_threshold_factors_sample: expr,
        $confirmation_role_threshold_factors_sample_other: expr,
        $confirmation_role_super_admin_factors_sample: expr,
        $confirmation_role_super_admin_factors_sample_other: expr,
        $recovery_role_threshold_factors_sample: expr,
        $recovery_role_threshold_factors_sample_other: expr,
        $recovery_role_super_admin_factors_sample: expr,
        $recovery_role_super_admin_factors_sample_other: expr
    ) => {
        paste! {

            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< SecurityStructureConfiguration $factor_level_name  >] {
                pub metadata: SecurityStructureMetadata,
                pub configuration: [< SecurityStructure $factor_level_name >],
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

                /// "sudo" factors, any **single** factor which can perform some function with this role,
	            /// disregarding of `threshold`.
                pub super_admin_factors: Vec<$factor_level_type>, // FIXME: Change to `IdentifiedVecOf`
            }

            impl [< RoleOfTier $factor_level_name >] {
                pub fn new(
                    threshold_factors: impl IntoIterator<Item = $factor_level_type>,
                    threshold: u16,
                    super_admin_factors: impl IntoIterator<Item = $factor_level_type>
                ) -> Self {
                    let _self = Self {
                        threshold_factors: threshold_factors.into_iter().collect(),
                        threshold,
                        super_admin_factors: super_admin_factors.into_iter().collect(),
                    };
                    assert!(_self.threshold_factors.len() >= _self.threshold as usize);
                    _self
                }

                pub fn sample_primary() -> Self {
                    let super_admin_factors: Vec<$factor_level_type> = $primary_role_super_admin_factors_sample.into_iter().collect_vec();
                    let threshold_factors: Vec<$factor_level_type> = $primary_role_threshold_factors_sample.into_iter().collect_vec();
                    let threshold = threshold_factors.len() - 1;
                    Self::new(
                        threshold_factors,
                        threshold as u16,
                        super_admin_factors
                    )
                }

                // pub fn sample_primary_other() -> Self {
                //     Self::new(
                //         $threshold_factors_sample_primary_other,
                //         2,
                //         $super_admin_factors_sample_primary_other
                //     )
                // }
            }

            // impl HasSampleValues for [< RoleOfTier $factor_level_name >] {
            //     fn sample() -> Self {
            //         Self::new(
            //             $threshold_factor_sample,
            //             3,
            //             $super_admin_factors_sample
            //         )
            //     }
            //     fn sample_other() -> Self {
            //         Self::new(
            //             $threshold_factor_sample_other,
            //             2,
            //             $super_admin_factors_sample_other
            //         )
            //     }
            // }

            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< SecurityStructure $factor_level_name >] {
                pub primary_role: [< RoleOfTier $factor_level_name>],
                pub number_of_days_until_auto_confirmation: u64,
            }

            impl [< SecurityStructure $factor_level_name >] {
                pub fn new(
                    primary_role: [< RoleOfTier $factor_level_name>],
                    number_of_days_until_auto_confirmation: u64,
                ) -> Self {
                    Self {
                        primary_role,
                        number_of_days_until_auto_confirmation
                    }
                }
            }

            // impl HasSampleValues for [< SecurityStructure $factor_level_name >] {
            //     pub fn sample() -> Self {
            //         Self::new(

            //         )
            //     }
            //     pub fn sample_other() -> Self {
            //         Self::new(
            //             $threshold_factor_sample_other,
            //             2,
            //             $super_admin_factors_sample_other
            //         )
            //     }
            // }
        }
    };
}

pub(crate) use decl_security_structure_config;
