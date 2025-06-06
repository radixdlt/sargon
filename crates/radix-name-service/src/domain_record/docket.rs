use crate::prelude::*;

/// Definition https://docs.rns.foundation/#/wiki/resolution/dockets
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
        domain: RnsDomain,
    ) -> Result<NonFungibleLocalId> {
        let domain_id = domain.to_non_fungible_id()?;
        let context_str = self.context.to_string();
        let directive_str = self.directive.0.clone();

        let id_str = format!("{}-{}-{}", domain_id, context_str, directive_str);

        domain_to_non_fungible_id(&id_str)
    }
}

impl Docket {
    /// Creates a new Docket for the Receivers context with a wildcard directive.
    /// This is used to match any receiver in the Receivers context.
    pub fn wildcard_receiver() -> Self {
        Self::new(DocketContext::Receivers, Directive::wildcard())
    }
}

/// Narrows down the context to a more granular level.
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

/// The wider context of the record.
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
            _ => Err(CommonError::RnsInvalidRecordContext {
                context: s.to_owned(),
            }),
        }
    }
}

impl Display for DocketContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DocketContext::Receivers => "receivers",
            DocketContext::Delegation => "delegation",
            DocketContext::Navigation => "navigation",
            DocketContext::Social => "social",
            DocketContext::Discovery => "discovery",
            DocketContext::Widgets => "widgets",
        }
        .to_owned();
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_non_fungible_id() {
        let docket = Docket::wildcard_receiver();
        let domain = RnsDomain::new("s1.gvp.xrd".to_string());

        let id = docket.to_non_fungible_id(domain).unwrap();
        let expected_id =
            NonFungibleLocalId::from_str("[8df41db37972d92e9dd411f6e7dbed10]")
                .unwrap();

        assert_eq!(id, expected_id)
    }
}
