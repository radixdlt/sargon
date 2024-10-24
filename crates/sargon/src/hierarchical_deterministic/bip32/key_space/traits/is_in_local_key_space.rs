use crate::{KeySpace, U31};

pub trait IsInLocalKeySpace {
    fn key_space(&self) -> KeySpace;
    fn index_in_local_key_space(&self) -> U31;
}
