use crate::prelude::*;

use sargon::IdentifiedVecOf as InternalIdentifiedVecOf;


impl<InternalElement: Debug + PartialEq + Eq + Clone + Identifiable, Element> From<InternalIdentifiedVecOf<InternalElement>> for Vec<Element>
where
    Element: From<InternalElement>,
{
    fn from(value: InternalIdentifiedVecOf<InternalElement>) -> Self {
        Self(value.into_iter().map(Element::from).collect())
    }
}

impl<InternalElement: Debug + PartialEq + Eq + Clone + Identifiable, Element: Debug + PartialEq + Eq + Clone + Identifiable> Into<InternalIdentifiedVecOf<InternalElement>> for Vec<Element>
where
    Element: Into<InternalElement>,
{
    fn into(self) -> InternalIdentifiedVecOf<InternalElement> {
        self.0.into_iter().map(Into::into).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::User;
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn index() {
        let sut = SUT::sample();
        assert_eq!(sut[0], User::alice());
        assert_eq!(sut[1], User::carol());
    }
}
