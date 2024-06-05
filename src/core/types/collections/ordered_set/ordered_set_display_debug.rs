use crate::prelude::*;

// ===============
// where V: Display
// ===============
impl<V> Display for OrderedSet<V>
where
    V: PartialEq + Eq + Clone + std::hash::Hash + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())?;
        Ok(())
    }
}

impl<V> OrderedSet<V>
where
    V: PartialEq + Eq + Clone + std::hash::Hash + Display,
{
    fn description(&self) -> String {
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
impl<V> Debug for OrderedSet<V>
where
    V: PartialEq + Eq + Clone + std::hash::Hash + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_description())?;
        Ok(())
    }
}

impl<V> OrderedSet<V>
where
    V: PartialEq + Eq + Clone + std::hash::Hash + Debug,
{
    fn debug_description(&self) -> String {
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

    use super::super::super::User;
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OrderedSet<User>;

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
