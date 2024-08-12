#![cfg(test)]

use crate::prelude::*;
use std::sync::RwLock;

#[derive(Debug)]
pub struct RustProfileChangeDriver {
    recorded: RwLock<Vec<Profile>>,
    spy: fn(Profile) -> (),
}

#[async_trait::async_trait]
impl ProfileChangeDriver for RustProfileChangeDriver {
    async fn handle_profile_change(&self, changed_profile: Profile) {
        self.recorded
            .try_write()
            .unwrap()
            .push(changed_profile.clone());
        (self.spy)(changed_profile)
    }
}

impl RustProfileChangeDriver {
    pub fn recorded(&self) -> Vec<Profile> {
        self.recorded.try_read().unwrap().clone()
    }
    pub fn new() -> Arc<Self> {
        Self::with_spy(|_| {})
    }
    pub fn with_spy(spy: fn(Profile) -> ()) -> Arc<Self> {
        Arc::new(Self {
            spy,
            recorded: RwLock::new(Vec::new()),
        })
    }
}
