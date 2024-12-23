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

// impl HasSampleValues for ReferencesToAuthorizedPersonas {
//     /// A sample used to facilitate unit tests.
//     fn sample() -> Self {
//         Self::from_iter([
//             AuthorizedPersonaSimple::sample_mainnet(),
//             AuthorizedPersonaSimple::sample_mainnet_other(),
//         ])
//     }

//     /// A sample used to facilitate unit tests.
//     fn sample_other() -> Self {
//         Self::from_iter([
//             AuthorizedPersonaSimple::sample_stokenet(),
//             AuthorizedPersonaSimple::sample_stokenet_other(),
//         ])
//     }
// }
