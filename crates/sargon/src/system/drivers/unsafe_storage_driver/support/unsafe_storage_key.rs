use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnsafeStorageKey {
    FactorSourceUserHasWrittenDown,
}

impl UnsafeStorageKey {
    pub fn identifier(&self) -> String {
        // format!(
        //     "unsafe_storage_key_{}",
        //     match self {
        //         UnsafeStorageKey::FactorSourceUserHasWrittenDown =>
        //             "factor_source_user_has_written_down".to_owned(),
        //     }
        // )
        match self {
            UnsafeStorageKey::FactorSourceUserHasWrittenDown => {
                "mnemonicsUserClaimsToHaveBackedUp".to_owned()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // #[test]
    // fn identifier() {
    //     assert_eq!(
    //         UnsafeStorageKey::FactorSourceUserHasWrittenDown.identifier(),
    //         "unsafe_storage_key_factor_source_user_has_written_down"
    //     );
    // }
}
