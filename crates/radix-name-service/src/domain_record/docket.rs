use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Docket {
    pub context: DocketContext,
    pub directive: Directive,
}

impl Docket {
    pub fn new(context: DocketContext, directive: Directive) -> Self {
        Self { context, directive }
    }

    pub fn to_non_fungible_id(
        &self,
        domain: Domain,
    ) -> Result<NonFungibleLocalId> {
        let domain_id = domain.to_non_fungible_id()?;
        let context_str = self.context.to_string();
        let directive_str = self.directive.0.clone();

        let id_str = format!("{}-{}-{}", domain_id, context_str, directive_str);

        domain_to_non_fungible_id(&id_str)
    }
}

impl Docket {
    pub fn wildcard_receiver() -> Self {
        Self::new(DocketContext::Receivers, Directive::wildcard())
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Directive(String);

impl Directive {
    pub fn new(directive: String) -> Self {
        Self(directive)
    }

    pub fn wildcard() -> Self {
        Self("*".to_owned())
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DocketContext {
    Receivers,
    Delegation,
    Navigation,
    Social,
    Discovery,
    Widgets,
}

impl FromStr for DocketContext {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "receivers" => Ok(DocketContext::Receivers),
            "delegation" => Ok(DocketContext::Delegation),
            "navigation" => Ok(DocketContext::Navigation),
            "social" => Ok(DocketContext::Social),
            "discovery" => Ok(DocketContext::Discovery),
            "widgets" => Ok(DocketContext::Widgets),
            _ => Err(CommonError::RnsInvalidRecordContext { context: s.to_owned() }),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_non_fungible_id() {
        let docket = Docket::wildcard_receiver();
        let domain = Domain::new("s1.grenadine.xrd".to_string());

        let id = docket.to_non_fungible_id(domain).unwrap();
        let expected_id =
            NonFungibleLocalId::from_str("[663c8eb2eaf0907ea4dd742be3b4c606]")
                .unwrap();

        assert_eq!(id, expected_id)
    }
}
