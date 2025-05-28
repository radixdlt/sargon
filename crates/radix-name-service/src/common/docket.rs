use std::fmt::format;

use crate::prelude::*;

pub struct Docket {
    pub context: DocketContext,
    pub derivative: Derivative,
}

impl Docket {
    pub fn new(context: DocketContext, derivative: Derivative) -> Self {
        Self {
            context,
            derivative,
        }
    }

    pub fn to_non_fungible_id(
        &self,
        domain: Domain,
    ) -> Result<NonFungibleLocalId> {
        let domain_id = domain.to_non_fungible_id()?;
        let composed_id = format!(
            "{:?}-{:?}-{:?}",
            domain_id.to_string(),
            self.context.to_string(),
            self.derivative.0
        );
        return domain_to_non_fungible_id(&composed_id, true);
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
    Widgets,
}

impl ToString for DocketContext {
    fn to_string(&self) -> String {
        match self {
            DocketContext::Receivers => "receivers",
            DocketContext::Delegation => "delegation",
            DocketContext::Navigation => "navigation",
            DocketContext::Social => "social",
            DocketContext::Discovery => "discovery",
            DocketContext::Widgets => "widgets",
        }
        .to_owned()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn to_non_fungible_id() {
//         let docket = Docket::wildcard_receiver();
//         let domain =
//     }
// }
