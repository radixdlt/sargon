use crate::prelude::*;

macro_rules! decl_security_structure_config {
    (
        $(
            #[doc = $expr: expr]
        )*
        $factor_level_name: ident,
        $factor_level_type: ident
        $super_admin_factor_sample: expr,
        $super_admin_factor_sample_other: expr,
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
                pub fn new(threshold_factors: impl IntoIterator<Item = $factor_level_type>, threshold: u16, super_admin_factors: impl IntoIterator<Item = $factor_level_type>) -> Self {

                }
            }

            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< SecurityStructure $factor_level_name >] {
                pub primary_role: [< RoleOfTier $factor_level_name>],
                pub number_of_days_until_auto_confirmation: u64,
            }

        }
    };
}

pub(crate) use decl_security_structure_config;
