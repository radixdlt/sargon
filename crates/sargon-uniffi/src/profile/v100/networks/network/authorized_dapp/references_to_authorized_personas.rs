use crate::prelude::*;

decl_identified_vec_of!(
    /// An order set of `AuthorizedPersonaSimple`s, which is a collection of all
    /// the Personas the user has used to interact with this Dapp, it is called
    /// "references to", since the Personas are not stored in full, that would be
    /// bad duplication of data (which might go stale), instead we refer to the
    /// necessary data by IDs.
    ReferencesToAuthorizedPersonas,
    AuthorizedPersonaSimple
);
