#[cfg(test)]
mod mock_xrd_balances;
mod xrd_balances;
#[allow(clippy::module_inception)]
mod xrd_balances_fetcher;

pub(super) use xrd_balances::*;
pub use xrd_balances_fetcher::*;

#[cfg(test)]
pub(crate) use mock_xrd_balances::*;
