mod entity_for_display;

pub mod prelude {
    pub use crate::entity_for_display::*;

    pub(crate) use account_for_display::prelude::*;
    pub(crate) use addresses::AddressOfAccountOrPersona;
    pub(crate) use has_sample_values::prelude::HasSampleValues;
    pub(crate) use identified_vec_of::prelude::Identifiable;
    pub(crate) use network::prelude::{IsNetworkAware, NetworkID};
    pub(crate) use persona_for_display::prelude::*;
    pub(crate) use short_string::prelude::DisplayName;

    pub(crate) use serde::{Deserialize, Serialize};
}

pub use prelude::*;
