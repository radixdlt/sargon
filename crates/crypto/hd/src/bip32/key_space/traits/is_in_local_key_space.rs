use crate::prelude::*;

pub trait IsMaybeKeySpaceAware {
    fn maybe_key_space(&self) -> Option<KeySpace>;
}

pub trait IsKeySpaceAware: IsSecurityStateAware {
    fn key_space(&self) -> KeySpace;
}

impl<T: IsKeySpaceAware> IsMaybeKeySpaceAware for T {
    fn maybe_key_space(&self) -> Option<KeySpace> {
        Some(self.key_space())
    }
}
impl<T: IsKeySpaceAware> IsSecurityStateAware for T {
    fn is_securified(&self) -> bool {
        self.key_space().is_securified()
    }
}

pub trait HasIndexInLocalKeySpace {
    fn index_in_local_key_space(&self) -> U31;
}
pub trait IsInLocalKeySpace: HasIndexInLocalKeySpace + IsKeySpaceAware {}

impl<T: HasIndexInLocalKeySpace + IsKeySpaceAware> IsInLocalKeySpace for T {}
