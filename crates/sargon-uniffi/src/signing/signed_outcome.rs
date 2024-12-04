use crate::prelude::*;
use paste::paste;
use sargon::SigningAbandonedReason as InternalSigningAbandonedReason;

/// The reasons signing a signable may be abandoned.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum SigningAbandonedReason {
    /// The user rejected the signing process
    Rejected,

    /// The signing process started with no profile present
    ProfileMissing,

    /// Preprocessing of signatures collector state failed.
    PreprocessingError(String),

    /// Could not validate signatures
    InvalidSignatures,
}

macro_rules! decl_signed_outcome {
    (
        struct_name: $struct_name:ident,
        internal_struct_name: $internal_struct_name:ident,
        signed: $signed:ident,
    ) => {
        /// Outcome of signing a transaction intent
        #[derive(Clone, PartialEq, Eq, uniffi::Enum)]
        pub enum $struct_name {
            /// The user has provided all needed signatures, the transaction intent is considered signed
            Signed($signed),

            /// The signing process was abandoned
            Abandoned(SigningAbandonedReason),
        }

        impl From<$internal_struct_name> for $struct_name {
            fn from(value: $internal_struct_name) -> Self {
                match value {
                    $internal_struct_name::Signed(signed) => {
                        $struct_name::Signed(signed.into())
                    }
                    $internal_struct_name::Abandoned(reason) => {
                        $struct_name::Abandoned(reason.into())
                    }
                }
            }
        }

        impl From<$struct_name> for $internal_struct_name {
            fn from(value: $struct_name) -> Self {
                match value {
                    $struct_name::Signed(signed) => {
                        Self::Signed(signed.into())
                    }
                    $struct_name::Abandoned(reason) => {
                        Self::Abandoned(reason.into())
                    }
                }
            }
        }
    };
    (signable: $signable:ty, signed: $signed:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $signed >] as [< Internal $signed >];

            type [< InternalSignedOutcomeOf $signable >] = sargon::SignedOutcome<[< Internal $signable >]>;

            decl_signed_outcome!(
                struct_name: [< SignedOutcomeOf $signable >],
                internal_struct_name: [< InternalSignedOutcomeOf $signable >],
                signed: [< $signed >],
            );
        }
    };
}

decl_signed_outcome!(
    signable: TransactionIntent,
    signed: SignedIntent
);
decl_signed_outcome!(
    signable: Subintent,
    signed: SignedSubintent
);
