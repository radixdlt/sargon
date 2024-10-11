use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered set of entities of mixed type, either [`Account`] or [`Persona`].
    AccountsOrPersonas,
    AccountOrPersona
);
