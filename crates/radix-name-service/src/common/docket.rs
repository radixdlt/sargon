use crate::prelude::*;

pub struct Docket {
    pub context: DocketContext,
    pub derivative: Derivative
}

impl Docket {
    pub fn new(
        context: DocketContext,
        derivative: Derivative,
    ) -> Self {
        Self {
            context,
            derivative
        }
    }
}

impl Docket {
    pub fn wildcard_receiver() -> Self {
        Self::new(DocketContext::Receivers, Derivative::wildcard())
    }
}

pub struct Derivative(String);

impl Derivative {
    pub fn wildcard() -> Self {
        Self("*".to_owned())
    }
}

pub enum DocketContext {
    Receivers,
    Delegation,
    Navigation,
    Social,
    Discovery,
    Widgets
}