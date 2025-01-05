mod entity_by_address;

pub mod prelude {
    pub use crate::entity_by_address::*;

    pub(crate) use addresses::prelude::*;
    pub(crate) use error::prelude::*;
    pub(crate) use profile_account::prelude::*;
    pub(crate) use profile_account_or_persona::prelude::*;
    pub(crate) use profile_persona::prelude::*;
}

pub use prelude::*;
