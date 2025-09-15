use crate::prelude::*;

/// A discriminator identifying the kind of `Event`, this has no associated
/// values and flattens the otherwise nested `Event` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum EventKind {
    /* Sort lexicographically */
    /// Profile updated with a new account.
    AccountAdded,

    /// Profile updated with new accounts.
    AccountsAdded,

    /// An existing account has been updated
    AccountUpdated,

    /// Existing accounts have been updated
    AccountsUpdated,

    /// SargonOS did boot.
    Booted,

    /// Current Gateway changed
    GatewayChangedCurrent,

    /// Profile updated with a new persona.
    PersonaAdded,

    /// Profile updated with new personas.
    PersonasAdded,

    /// An existing persona has been updated
    PersonaUpdated,

    /// Existing personas have been updated
    PersonasUpdated,

    /// Profile was saved.
    ProfileSaved,

    /// A profile has been imported and has been set to active profile,
    /// and saved into secure storage.
    ProfileImported,

    /// Profile was last used on another device.
    ProfileUsedOnOtherDevice,

    /// Profile updated with a new factor source.
    FactorSourceAdded,

    /// Profile updated with multiple new factor sources.
    FactorSourcesAdded,

    /// An existing factor source has been updated
    FactorSourceUpdated,

    /// A collection of existing factor sources have been updated
    FactorSourcesUpdated,

    /// Profile updated with a new Security Structure.
    SecurityStructureAdded,

    /// Security structures have been updated
    SecurityStructuresUpdated,
}

impl EventKind {
    /// Returns collection of all different EventKinds
    pub fn all() -> Vec<Self> {
        all::<Self>().collect()
    }

    /// If hosts should fetch account list due to an action which triggered the
    /// event of this kind to be emitted.
    ///
    /// E.g. if an account was saved into Profile, an event with the kind
    /// `EventKind::AccountAdded` will be emitted, which hosts SHOULD react to
    /// and thus fetch the account list and possibly update UI.
    pub fn affects_current_accounts(&self) -> bool {
        use EventKind::*;
        matches!(
            *self,
            Booted
                | ProfileImported
                | AccountAdded
                | AccountsAdded
                | AccountUpdated
                | AccountsUpdated
                | GatewayChangedCurrent
        )
    }

    /// If hosts should fetch persona list due to an action which triggered the
    /// event of this kind to be emitted.
    ///
    /// E.g. if a persona was saved into Profile, an event with the kind
    /// `EventKind::PersonaAdded` will be emitted, which hosts SHOULD react to
    /// and thus fetch the persona list and possibly update UI.
    pub fn affects_current_personas(&self) -> bool {
        use EventKind::*;
        matches!(
            *self,
            Booted
                | ProfileImported
                | PersonaAdded
                | PersonasAdded
                | PersonaUpdated
                | PersonasUpdated
                | GatewayChangedCurrent
        )
    }

    /// If hosts should check the current network due to an action which triggered the
    /// event of this kind to be emitted.
    ///
    /// E.g. if the current gateway was changed by the user, an event with the kind
    /// `EventKind::GatewayChangedCurrent` will be emitted, which hosts SHOULD react to
    /// and check the network of the new gateway and possibly update UI.
    pub fn affects_current_network(&self) -> bool {
        use EventKind::*;
        matches!(*self, Booted | ProfileImported | GatewayChangedCurrent)
    }

    /// If hosts should check saved gateways due to an action which triggered the
    /// event of this kind to be emitted.
    ///
    /// E.g. if the current gateway was changed by the user, an event with the kind
    /// `EventKind::GatewayChangedCurrent` will be emitted, which hosts SHOULD
    /// react to and fetch saved gateways and possibly update UI.
    pub fn affects_saved_gateways(&self) -> bool {
        use EventKind::*;
        matches!(*self, Booted | ProfileImported | GatewayChangedCurrent)
    }

    /// If security structures have changed
    pub fn affects_security_structures(&self) -> bool {
        use EventKind::*;
        matches!(
            *self,
            Booted
                | ProfileImported
                | SecurityStructureAdded
                | SecurityStructuresUpdated
        )
    }

    /// If hosts UI displaying factor sources (of any kind) should re-fetch
    /// the list from SargonOS.
    ///
    /// E.g. if a new account is created using factor source `X` then `x.common.last_used`,
    /// is updated and an event of kind `FactorSourceUpdated` is emitted, which does
    /// affect factor sources shown by host.
    pub fn affects_factor_sources(&self) -> bool {
        use EventKind::*;
        matches!(
            *self,
            Booted
                | ProfileImported
                | FactorSourceAdded
                | FactorSourceUpdated
                | FactorSourcesAdded
                | FactorSourcesUpdated
        )
    }
}

impl HasSampleValues for EventKind {
    fn sample() -> Self {
        Self::Booted
    }

    fn sample_other() -> Self {
        Self::ProfileSaved
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EventKind;

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
    fn test_event_kind_affects_current_accounts() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_current_accounts()))
            .for_each(|(sut, affects)| match sut {
                Booted
                | ProfileImported
                | AccountAdded
                | AccountsAdded
                | AccountUpdated
                | AccountsUpdated
                | GatewayChangedCurrent => assert!(affects),
                ProfileUsedOnOtherDevice
                | ProfileSaved
                | SecurityStructureAdded
                | SecurityStructuresUpdated
                | FactorSourceAdded
                | FactorSourcesAdded
                | FactorSourceUpdated
                | FactorSourcesUpdated
                | PersonaAdded
                | PersonasAdded
                | PersonasUpdated
                | PersonaUpdated => {
                    assert!(!affects)
                }
            })
    }

    #[test]
    fn test_event_kind_affects_current_personas() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_current_personas()))
            .for_each(|(sut, affects)| match sut {
                Booted
                | ProfileImported
                | PersonaAdded
                | PersonasAdded
                | PersonasUpdated
                | PersonaUpdated
                | GatewayChangedCurrent => assert!(affects),
                ProfileUsedOnOtherDevice
                | ProfileSaved
                | SecurityStructureAdded
                | SecurityStructuresUpdated
                | FactorSourceAdded
                | FactorSourcesAdded
                | FactorSourceUpdated
                | FactorSourcesUpdated
                | AccountAdded
                | AccountsAdded
                | AccountUpdated
                | AccountsUpdated => {
                    assert!(!affects)
                }
            })
    }

    #[test]
    fn event_kind_affects_current_network() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_current_network()))
            .for_each(|(sut, affects)| match sut {
                Booted | ProfileImported | GatewayChangedCurrent => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | FactorSourceAdded
                | FactorSourceUpdated
                | FactorSourcesUpdated
                | ProfileSaved
                | AccountAdded
                | SecurityStructureAdded
                | SecurityStructuresUpdated
                | FactorSourcesAdded
                | AccountsAdded
                | AccountUpdated
                | AccountsUpdated
                | PersonaAdded
                | PersonasAdded
                | PersonasUpdated
                | PersonaUpdated => assert!(!affects),
            })
    }

    #[test]
    fn event_kind_affects_security_structures() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_security_structures()))
            .for_each(|(sut, affects)| match sut {
                Booted
                | ProfileImported
                | SecurityStructureAdded
                | SecurityStructuresUpdated => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | FactorSourceAdded
                | FactorSourceUpdated
                | FactorSourcesUpdated
                | ProfileSaved
                | AccountAdded
                | GatewayChangedCurrent
                | FactorSourcesAdded
                | AccountsAdded
                | AccountUpdated
                | AccountsUpdated
                | PersonaAdded
                | PersonasAdded
                | PersonaUpdated
                | PersonasUpdated => assert!(!affects),
            })
    }

    #[test]
    fn event_kind_affects_saved_gateways() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_saved_gateways()))
            .for_each(|(sut, affects)| match sut {
                Booted | ProfileImported | GatewayChangedCurrent => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | FactorSourceAdded
                | FactorSourceUpdated
                | FactorSourcesUpdated
                | ProfileSaved
                | AccountAdded
                | FactorSourcesAdded
                | SecurityStructureAdded
                | SecurityStructuresUpdated
                | AccountsAdded
                | AccountUpdated
                | AccountsUpdated
                | PersonaAdded
                | PersonasAdded
                | PersonaUpdated
                | PersonasUpdated => assert!(!affects),
            })
    }

    #[test]
    fn event_kind_affects_factor_source() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_factor_sources()))
            .for_each(|(sut, affects)| match sut {
                Booted | ProfileImported | FactorSourceAdded
                | FactorSourcesAdded | FactorSourceUpdated
                | FactorSourcesUpdated => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | GatewayChangedCurrent
                | ProfileSaved
                | SecurityStructureAdded
                | SecurityStructuresUpdated
                | AccountAdded
                | AccountsAdded
                | AccountUpdated
                | AccountsUpdated
                | PersonaAdded
                | PersonasAdded
                | PersonaUpdated
                | PersonasUpdated => assert!(!affects),
            })
    }
}
