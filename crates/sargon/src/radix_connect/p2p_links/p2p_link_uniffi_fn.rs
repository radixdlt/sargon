use crate::prelude::*;

json_data_convertible!(P2PLink);

#[uniffi::export]
pub fn p2p_link_id(link: &P2PLink) -> <P2PLink as Identifiable>::ID {
    link.id()
}

#[uniffi::export]
pub fn new_p2p_link_sample() -> P2PLink {
    P2PLink::sample()
}

#[uniffi::export]
pub fn new_p2p_link_sample_other() -> P2PLink {
    P2PLink::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = P2PLink;

    #[test]
    fn id_of_link() {
        let sut = SUT::sample();
        assert_eq!(p2p_link_id(&sut), sut.id())
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_p2p_link_sample(),
                new_p2p_link_sample_other(),
                // duplicates should get removed
                new_p2p_link_sample(),
                new_p2p_link_sample_other(),
            ])
            .len(),
            2
        );
    }
}
