use crate::prelude::*;
use paste::paste;
use sargon::SigningAbandonedReason as InternalSigningAbandonedReason;

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
    (signable: $signable:ty, signed: $signed:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $signed >] as [< Internal $signed >];

            type [< InternalSignedOutcomeOf $signable >] = sargon::SignedOutcome<[< Internal $signable >]>;

            /// Outcome of signing a transaction intent
            #[derive(Clone, PartialEq, Eq, uniffi::Enum)]
            pub enum [< SignedOutcomeOf $signable >] {
                /// The user has provided all needed signatures, the transaction intent is considered signed
                Signed([< $signed >]),

                /// The signing process was abandoned
                Abandoned(SigningAbandonedReason),
            }

            impl From<[< InternalSignedOutcomeOf $signable >]> for [< SignedOutcomeOf $signable >] {
                fn from(value: [< InternalSignedOutcomeOf $signable >]) -> Self {
                    match value {
                        [< InternalSignedOutcomeOf $signable >]::Signed(signed) => {
                            [< SignedOutcomeOf $signable >]::Signed(signed.into())
                        }
                        [< InternalSignedOutcomeOf $signable >]::Abandoned(reason) => {
                            [< SignedOutcomeOf $signable >]::Abandoned(reason.into())
                        }
                    }
                }
            }

            impl From<[< SignedOutcomeOf $signable >]> for [< InternalSignedOutcomeOf $signable >] {
                fn from(value: [< SignedOutcomeOf $signable >]) -> Self {
                    match value {
                        [< SignedOutcomeOf $signable >]::Signed(signed) => {
                            Self::Signed(signed.into())
                        }
                        [< SignedOutcomeOf $signable >]::Abandoned(reason) => {
                            Self::Abandoned(reason.into())
                        }
                    }
                }
            }
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
