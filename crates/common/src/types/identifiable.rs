use crate::prelude::*;

/// The `Identifiable` trait allows you to use the
/// `IdentifiedVecOf<User> instead of the more verbose
/// `IdentifiedVec<SomeUserID, User>` but also allows you to
/// skip the `id_of_element: fn(&Element) -> ID` closure when
/// initializing a new identified vec.
pub trait Identifiable {
    /// The type that your `Element` will use as its globally unique and stable ID,
    /// must impl `Hash` since it is used as a key in `IdentifiedVecOf`'s internal
    /// `HashMap`. Must impl `Clone` since we need to be able to clone it as a key
    type ID: Eq + StdHash + Clone + Debug;

    /// Return `Element`'s globally unique and stable ID, used to uniquely identify
    /// the `Element` in the `IdentifiedVecOf` collection of elements.
    fn id(&self) -> Self::ID;
}
