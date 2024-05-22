uniffi::setup_scaffolding!();

mod interaction_id;
mod interaction_version;
mod p2p_links;
mod wallet_account;
mod wallet_interaction;
mod wallet_persona;

#[allow(dead_code)]
mod mobile;

uniffi::remote_type!(Url, common);
uniffi::remote_type!(Uuid, common);

pub mod prelude {
    pub use crate::interaction_id::*;
    pub use crate::interaction_version::*;
    pub use crate::p2p_links::*;
    pub use crate::wallet_account::*;
    pub use crate::wallet_interaction::*;
    pub use crate::wallet_persona::*;

    pub use clients::prelude::*;
}

pub use prelude::*;
