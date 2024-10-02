//! This mod contains higher level abstraction methods that use underlying
//! Gateway Endpoints, e.g. applying some post-process logic, such as
//! returning the XRD balance of a single account, using the endpoint
//! `/state/entity/details` and then refining the response.

mod state_methods;
mod transaction_methods;

pub use state_methods::*;
pub use transaction_methods::*;
