use crate::prelude::*;

#[macro_export]
macro_rules! create_application_for_securified_entity_with_payloads {
    (
        $name:ident,
        $payload_ty: ty,
        $(
            #[doc = $expr: expr]
        )*
        $entity_name:ident: $entity_type:ty
    ) => {

        preinterpret::preinterpret!{
            /// This struct hold `4` different combinations of manifests for different
            /// combinations of roles.
            ///
            /// Later when we want to sign these manifests using the `SignaturesCollector`,
            /// which currently (2025-01-16) can only be used with `1` Role at a time (later
            /// we might change this). Meaning we need to do `3` passes to the  SignaturesCollector, to sign the different manifests.
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name {
                $(
                    #[doc = $expr]
                )*
                pub $entity_name: $entity_type,

                pub initiate_with_recovery_complete_with_primary: $payload_ty,
                pub initiate_with_recovery_complete_with_confirmation: $payload_ty,
                pub initiate_with_recovery_delayed_completion: $payload_ty,
                pub initiate_with_primary_complete_with_confirmation: $payload_ty,
                pub initiate_with_primary_delayed_completion: $payload_ty,

            }

            impl $name {
                pub fn new(
                    $entity_name: $entity_type,
                    initiate_with_recovery_complete_with_primary: $payload_ty,
                    initiate_with_recovery_complete_with_confirmation: $payload_ty,
                    initiate_with_recovery_delayed_completion: $payload_ty,
                    initiate_with_primary_complete_with_confirmation: $payload_ty,
                    initiate_with_primary_delayed_completion: $payload_ty,
                ) -> Self {
                    Self {
                        $entity_name,
                        initiate_with_recovery_complete_with_primary,
                        initiate_with_recovery_complete_with_confirmation,
                        initiate_with_recovery_delayed_completion,
                        initiate_with_primary_complete_with_confirmation,
                        initiate_with_primary_delayed_completion,
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! create_application_for_securified_entity_with_intents {
    (
        $name:ident,
        $(
            #[doc = $expr: expr]
        )*
        $entity_name:ident: $entity_type:ty
    ) => {
        $crate::create_application_for_securified_entity_with_payloads!(
            $name,
            TransactionIntent,
            $(
                #[doc = $expr]
            )*
            $entity_name: $entity_type
        );
    }
}

#[macro_export]
macro_rules! create_application_for_securified_entity_with_manifests {
    (
        $name:ident,
        $(
            #[doc = $expr: expr]
        )*
        $entity_name:ident: $entity_type:ty
    ) => {
        $crate::create_application_for_securified_entity_with_payloads!(
            $name,
            TransactionManifest,
            $(
                #[doc = $expr]
            )*
            $entity_name: $entity_type
        );
    }
}
