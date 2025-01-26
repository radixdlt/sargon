use crate::prelude::*;

use preinterpret::*;

macro_rules! matrix_conversion {
    (
        $(#[$attributes:meta])*
        $factor_level: ident
    ) => {
        preinterpret::preinterpret! {
            [!set! #internal_factor = [!ident! Internal $factor_level]]
            [!set! #struct_name = [!ident! MatrixOf $factor_level s]]
            [!set! #internal_struct_name = [!ident! Internal #struct_name]]
            [!set! #primary_role_type = [!ident! PrimaryRoleWith $factor_level s ]]
            [!set! #recovery_role_type = [!ident! RecoveryRoleWith $factor_level s ]]
            [!set! #confirmation_role_type = [!ident! ConfirmationRoleWith $factor_level s ]]

            use sargon::#struct_name as #internal_struct_name;
            use sargon::$factor_level as #internal_factor;

            $(#[$attributes])*
            #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
            pub struct #struct_name {
                pub primary_role: #primary_role_type,
                pub recovery_role: #recovery_role_type,
                pub confirmation_role: #confirmation_role_type,

                pub time_until_delayed_confirmation_is_callable: TimePeriod,
            }

            delegate_debug_into!(#struct_name, #internal_struct_name);

            impl From<#internal_struct_name> for #struct_name {
                fn from(value: #internal_struct_name) -> Self {
                    Self {
                        primary_role: value.primary().clone().into(),
                        recovery_role: value.recovery().clone().into(),
                        confirmation_role: value.confirmation().clone().into(),
                        time_until_delayed_confirmation_is_callable: value
                            .time_until_delayed_confirmation_is_callable.into(),
                    }
                }
            }

            impl #struct_name {
                pub fn into_internal(&self) -> #internal_struct_name {
                    unsafe {
                        #internal_struct_name::unbuilt_with_roles_and_days(
                            self.primary_role.clone().into(),
                            self.recovery_role.clone().into(),
                            self.confirmation_role.clone().into(),
                            self.time_until_delayed_confirmation_is_callable.into(),
                        )
                    }
                }
            }
            impl From<#struct_name> for #internal_struct_name {
                fn from(value: #struct_name) -> Self {
                   value.into_internal()
                }
            }

            impl HasSampleValues for #struct_name {
                fn sample() -> Self {
                    #internal_struct_name::sample().into()
                }
                fn sample_other() -> Self {
                    #internal_struct_name::sample_other().into()
                }
            }

            [!set! #fn_name_prefix = new_[!snake! #struct_name]]

            #[uniffi::export]
            pub fn [!ident! #fn_name_prefix _sample ]() -> #struct_name {
                #struct_name::sample()
            }

            #[uniffi::export]
            pub fn [!ident! #fn_name_prefix _sample_other ]() -> #struct_name {
                #struct_name::sample_other()
            }

        }
    };
}

pub(crate) use matrix_conversion;
