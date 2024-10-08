use crate::prelude::*;
use sargon::SavedGateways as InternalSavedGateways;

decl_identified_vec_of!(
    /// An ordered collection of unique [`Gateway`]s.
    /// It might be Gateways with different URLs on the SAME network, or
    /// other networks, the identifier of a Gateway is the URL.
    Gateway
);

/// The currently used Gateway and a collection of other by user added
/// or predefined Gateways the user can switch to.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct SavedGateways {
    /// The currently used Gateway, when a user query's asset balances of
    /// accounts or submits transactions, this Gateway will be used.
    pub current: Gateway,

    /// Other by user added or predefined Gateways the user can switch to.
    /// It might be Gateways with different URLs on the SAME network, or
    /// other networks, the identifier of a Gateway is the URL.
    pub other: Gateways,
}

impl From<InternalSavedGateways> for SavedGateways {
    fn from(value: InternalSavedGateways) -> Self {
        Self {
            current: value.current.into(),
            other: value.other.into(),
        }
    }
}

impl Into<InternalSavedGateways> for SavedGateways {
    fn into(self) -> InternalSavedGateways {
        InternalSavedGateways {
            current: self.current.into(),
            other: self.other.into(),
        }
    }
}

/// Constructs `Gateways` with `current` set as active Gateway.
#[uniffi::export]
pub fn new_saved_gateways(current: Gateway) -> SavedGateways {
    InternalSavedGateways::new(current.into()).into()
}

/// Constructs `Gateways` with default preset values.
#[uniffi::export]
pub fn new_saved_gateways_default() -> SavedGateways {
    InternalSavedGateways::default().into()
}

/// A sample value useful for tests and previews.
#[uniffi::export]
pub fn new_saved_gateways_sample() -> SavedGateways {
    InternalSavedGateways::sample().into()
}

/// A sample value useful for tests and previews.
#[uniffi::export]
pub fn new_saved_gateways_sample_other() -> SavedGateways {
    InternalSavedGateways::sample_other().into()
}

/// Returns the current and the other gateways of `gateways`.
#[uniffi::export]
pub fn saved_gateways_get_all_elements(
    gateways: &SavedGateways,
) -> Vec<Gateway> {
    gateways.into_internal().all().into_iter().map(|x| x.into()).collect()
}

#[uniffi::export]
pub fn new_saved_gateways_changing_current(
    to: Gateway,
    gateways: &SavedGateways,
) -> Result<SavedGateways> {
    let mut gateways = gateways.clone().into_internal();
    let _ = gateways.change_current(to.into());
    Ok(gateways.into())
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SavedGateways;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_saved_gateways_sample());
        assert_eq!(SUT::sample_other(), new_saved_gateways_sample_other());
    }

    #[test]
    fn new_with_current() {
        assert_eq!(
            new_saved_gateways(Gateway::mardunet()).all(),
            [Gateway::mardunet()]
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(new_saved_gateways_default(), SUT::default(),)
    }

    #[test]
    fn test_saved_gateways_get_all_elements() {
        assert_eq!(
            saved_gateways_get_all_elements(&SUT::sample()),
            SUT::sample().all(),
        )
    }

    #[test]
    fn test_new_saved_gateways_changing_current() {
        let sut =
            SUT::new_with_other(Gateway::stokenet(), [Gateway::mainnet()])
                .unwrap();
        let changed =
            new_saved_gateways_changing_current(Gateway::mainnet(), &sut)
                .unwrap();
        assert_eq!(changed, SUT::default());
    }
}
