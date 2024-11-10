#[cfg(test)]
mod stateless_dummy_indices;
#[cfg(test)]
mod test_keys_collector;

#[cfg(test)]
pub(crate) use stateless_dummy_indices::*;
#[cfg(test)]
pub(crate) use test_keys_collector::*;

use crate::prelude::*;
use std::future::ready;
