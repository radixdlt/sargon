use crate::prelude::*;

/// A list of entities which would fail in a transaction if we would
/// neglect certain factor source, either by user explicitly skipping
/// it or if implicitly neglected due to failure.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct InvalidTransactionIfNeglected {
    /// The intent hash of the transaction which would be invalid if a
    /// certain factor source would be neglected, either if user
    /// explicitly skipped it or implicitly neglected due to failure.
    pub intent_hash: IntentHash,

    /// The entities in the transaction which would fail auth.
    entities_which_would_fail_auth: Vec<AddressOfAccountOrPersona>,
}

impl InvalidTransactionIfNeglected {
    /// Constructs a new `InvalidTransactionIfNeglected` from an IndexSet of
    /// entities which would fail auth..
    ///
    /// # Panics
    /// Panics if `entities_which_would_fail_auth` is empty.
    pub fn new(
        intent_hash: IntentHash,
        entities_which_would_fail_auth: impl IntoIterator<
            Item = AddressOfAccountOrPersona,
        >,
    ) -> Self {
        let entities_which_would_fail_auth =
            entities_which_would_fail_auth.into_iter().collect_vec();
        let len = entities_which_would_fail_auth.len();
        let entities_which_would_fail_auth = entities_which_would_fail_auth
            .into_iter()
            .collect::<IndexSet<_>>();

        assert!(!entities_which_would_fail_auth.is_empty(), "'entities_which_would_fail_auth' must not be empty, this type is not useful if it is empty.");

        assert_eq!(
            entities_which_would_fail_auth.len(),
            len,
            "entities_which_would_fail_auth must not contain duplicates."
        );

        Self {
            intent_hash,
            entities_which_would_fail_auth: entities_which_would_fail_auth
                .into_iter()
                .collect_vec(),
        }
    }

    pub fn entities_which_would_fail_auth(
        &self,
    ) -> IndexSet<AddressOfAccountOrPersona> {
        IndexSet::from_iter(self.entities_which_would_fail_auth.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = InvalidTransactionIfNeglected;

    #[test]
    #[should_panic(
        expected = "'entities_which_would_fail_auth' must not be empty, this type is not useful if it is empty."
    )]
    fn panics_if_empty() {
        Sut::new(IntentHash::sample(), IndexSet::new());
    }

    #[test]
    #[should_panic(
        expected = "entities_which_would_fail_auth must not contain duplicates."
    )]
    fn panics_if_duplicates() {
        Sut::new(
            IntentHash::sample(),
            [
                AddressOfAccountOrPersona::sample(),
                AddressOfAccountOrPersona::sample(),
            ],
        );
    }

    #[test]
    fn new() {
        let entities = [
            AddressOfAccountOrPersona::sample(),
            AddressOfAccountOrPersona::sample_other(),
        ];
        let sut = Sut::new(IntentHash::sample(), entities.clone());
        assert_eq!(
            sut.entities_which_would_fail_auth(),
            IndexSet::<_>::from_iter(entities.into_iter())
        );
    }
}