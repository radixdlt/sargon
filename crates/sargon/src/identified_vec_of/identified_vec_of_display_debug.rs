use std::fmt::Formatter;

use crate::prelude::*;

// ===============
// where V: Display
// ===============
impl<V> Display for IdentifiedVecOf<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())?;
        Ok(())
    }
}

impl<V> IdentifiedVecOf<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable + Display,
{
    pub fn description(&self) -> String {
        [
            "[".to_owned(),
            self.clone()
                .into_iter()
                .map(|e| format!("{}", e))
                .join(", "),
            "]".to_owned(),
        ]
        .join("")
    }
}

// ===============
// where V: Debug
// ===============
impl<V> Debug for IdentifiedVecOf<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_description())?;
        Ok(())
    }
}

impl<V> IdentifiedVecOf<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable,
{
    pub fn debug_description(&self) -> String {
        [
            "[".to_owned(),
            self.clone()
                .into_iter()
                .map(|e| format!("{:?}", e))
                .join(", "),
            "]".to_owned(),
        ]
        .join("")
    }
}

#[cfg(test)]
mod tests {

    use crate::identified_vec_of::User;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn display() {
        let sut = SUT::sample_other();
        assert_eq!(format!("{}", sut), "[Bob, David, Frank]")
    }

    #[test]
    fn debug() {
        let sut = SUT::sample_other();
        assert_eq!(format!("{:?}", sut), "[(1: Bob), (3: David), (5: Frank)]")
    }
}
