use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    ProfileNetwork
);