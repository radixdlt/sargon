use crate::prelude::*;

/// Some value with a known owner - an account or persona.
#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{:#?}: {:#?}", owner, value)]
pub struct Owned<T> {
    /// The known owner - an account or persona - of `value`.
    pub owner: AddressOfAccountOrPersona,
    /// Some value known to be owned by `owner` - an account or persona.
    pub value: T,
}

impl<T> Owned<T> {
    pub fn new(owner: AddressOfAccountOrPersona, value: T) -> Self {
        Self { owner, value }
    }
}

impl<T: HasSampleValues> HasSampleValues for Owned<T> {
    fn sample() -> Self {
        Self::new(AddressOfAccountOrPersona::sample(), T::sample())
    }
    fn sample_other() -> Self {
        Self::new(AddressOfAccountOrPersona::sample_other(), T::sample_other())
    }
}

pub type OwnedIntentSignature = Owned<IntentSignature>;

impl TryFrom<(AddressOfAccountOrPersona, IntentSignature)>
    for OwnedIntentSignature
{
    type Error = CommonError;

    fn try_from(
        (owner, intent_signature): (AddressOfAccountOrPersona, IntentSignature),
    ) -> Result<Self> {
        if !owner.matches_public_key(intent_signature.public_key()) {
            return Err(CommonError::Unknown); // TODO - better error
        }
        Ok(Self::new(owner, intent_signature))
    }
}
