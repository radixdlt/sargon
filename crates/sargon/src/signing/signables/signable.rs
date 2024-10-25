use crate::prelude::*;
use std::hash::Hasher;

/// Any type conforming to `Signable` can be used with `SignaturesCollector` and collect
/// signatures from all involved entities according to their security structure.
pub trait Signable: std::hash::Hash + PartialEq + Eq + Clone + Debug + HasSampleValues {
    /// A stable identifier for this `Signable`.
    type ID: SignableID;

    /// A compiled version of the `Signable` that is passed down to the interactors.
    type Payload: PartialEq
        + Eq
        + Clone
        + Debug
        + std::hash::Hash
        + Into<Self::ID>
        + From<Self>
        + HasSampleValues;

    /// A function that extracts the involved entities that require signing.
    fn entities_requiring_signing(
        &self,
        profile: &Profile,
    ) -> Result<IndexSet<AccountOrPersona>>;

    fn get_payload(&self) -> Self::Payload {
        From::<Self>::from(self.clone())
    }

    /// Retrieves the stable identifier from the `Signable`
    fn get_id(&self) -> Self::ID {
        self.get_payload().into()
    }
}

/// An identifier that is unique for each `Signable`
pub trait SignableID: Eq + StdHash + Clone + Debug + Into<Hash> + HasSampleValues {}
