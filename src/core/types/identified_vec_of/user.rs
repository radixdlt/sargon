use crate::prelude::*;

#[cfg(test)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[debug("({}: {})", id, name)]
#[display("{}", name)]
pub(super) struct User {
    pub(super) id: u8,
    pub(super) name: String,
}

#[cfg(test)]
impl User {
    pub(super) fn new(id: u8, name: impl AsRef<str>) -> Self {
        Self {
            id,
            name: name.as_ref().to_owned(),
        }
    }
}

#[cfg(test)]
impl Identifiable for User {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[cfg(test)]
impl User {
    /// id 0
    pub(super) fn alice() -> Self {
        Self::new(0, "Alice")
    }

    /// id 1
    pub(super) fn bob() -> Self {
        Self::new(1, "Bob")
    }

    /// id 2
    pub(super) fn carol() -> Self {
        Self::new(2, "Carol")
    }

    /// id 3
    pub(super) fn david() -> Self {
        Self::new(3, "David")
    }

    /// id 4
    pub(super) fn erin() -> Self {
        Self::new(4, "Erin")
    }

    /// id 5
    pub(super) fn frank() -> Self {
        Self::new(5, "Frank")
    }

    /// id 6
    pub(super) fn grace() -> Self {
        Self::new(6, "Grace")
    }
}

#[cfg(test)]
impl crate::HasSampleValues for IdentifiedVecOf<User> {
    /// Alice(0), Carol(2), Erin(4), Grace(6)
    fn sample() -> Self {
        Self::from_iter([
            User::alice(),
            User::carol(),
            User::erin(),
            User::grace(),
        ])
    }

    /// Bob(1), David(3), Frank(5)
    fn sample_other() -> Self {
        Self::from_iter([User::bob(), User::david(), User::frank()])
    }
}
