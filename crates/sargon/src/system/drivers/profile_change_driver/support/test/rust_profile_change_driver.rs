#![cfg(test)]

use crate::prelude::*;
use std::sync::RwLock;

#[derive(Debug)]
pub struct RustProfileStateChangeDriver {
    recorded: RwLock<Vec<ProfileState>>,
    spy: fn(ProfileState) -> (),
}

#[async_trait::async_trait]
impl ProfileStateChangeDriver for RustProfileStateChangeDriver {
    async fn handle_profile_state_change(
        &self,
        changed_profile_state: ProfileState,
    ) {
        self.recorded
            .try_write()
            .unwrap()
            .push(changed_profile_state.clone());
        (self.spy)(changed_profile_state)
    }
}

impl RustProfileStateChangeDriver {
    pub fn recorded(&self) -> Vec<ProfileState> {
        self.recorded.try_read().unwrap().clone()
    }
    pub fn new() -> Arc<Self> {
        Self::with_spy(|_| {})
    }
    pub fn with_spy(spy: fn(ProfileState) -> ()) -> Arc<Self> {
        Arc::new(Self {
            spy,
            recorded: RwLock::new(Vec::new()),
        })
    }
}
