use crate::prelude::*;
use sargon::HDPathComponent as InternalHDPathComponent;

use sargon::{
    FromGlobalKeySpace, FromLocalKeySpace, IsInLocalKeySpace,
    IsMappableToGlobalKeySpace, ToBIP32Str,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum HDPathComponent {
    UnsecurifiedComponent(Unsecurified),
    SecurifiedComponent(SecurifiedU30),
}

impl From<InternalHDPathComponent> for HDPathComponent {
    fn from(value: InternalHDPathComponent) -> Self {
        match value {
            InternalHDPathComponent::Unsecurified(unsecurified) => {
                Self::UnsecurifiedComponent(unsecurified.into())
            }
            InternalHDPathComponent::Securified(securified) => {
                Self::SecurifiedComponent(securified.into())
            }
        }
    }
}

impl From<HDPathComponent> for InternalHDPathComponent {
    fn from(value: HDPathComponent) -> Self {
        match value {
            HDPathComponent::UnsecurifiedComponent(unsecurified) => {
                Self::Unsecurified(unsecurified.into())
            }
            HDPathComponent::SecurifiedComponent(securified) => {
                Self::Securified(securified.into())
            }
        }
    }
}

impl HDPathComponent {
    pub fn into_internal(&self) -> InternalHDPathComponent {
        self.clone().into()
    }
}

#[uniffi::export]
pub fn new_hd_path_component_sample() -> HDPathComponent {
    InternalHDPathComponent::sample().into()
}

#[uniffi::export]
pub fn new_hd_path_component_sample_other() -> HDPathComponent {
    InternalHDPathComponent::sample_other().into()
}

#[uniffi::export]
pub fn new_hd_path_component_from_local_key_space(
    value: u32,
    key_space: KeySpace,
) -> Result<HDPathComponent> {
    InternalHDPathComponent::from_local_key_space(value, key_space.into())
        .into_result()
}

#[uniffi::export]
pub fn new_hd_path_component_from_global_key_space(
    value: u32,
) -> HDPathComponent {
    InternalHDPathComponent::from_global_key_space(value)
        .into_result()
        .expect("Should always be able to create an HDPathComponent from a u32")
}

#[uniffi::export]
pub fn hd_path_component_to_hardened(
    component: HDPathComponent,
) -> Result<Hardened> {
    sargon::Hardened::try_from(component.into_internal()).into_result()
}

#[uniffi::export]
pub fn hd_path_component_to_bip32_string(component: HDPathComponent) -> String {
    component.into_internal().to_bip32_string()
}

#[uniffi::export]
pub fn hd_path_component_to_bip32_string_debug(
    component: HDPathComponent,
) -> String {
    component.into_internal().to_bip32_string_debug()
}

#[uniffi::export]
pub fn hd_path_component_index_in_global_key_space(
    component: HDPathComponent,
) -> u32 {
    component.into_internal().map_to_global_key_space()
}

#[uniffi::export]
pub fn hd_path_component_index_in_local_key_space(
    component: HDPathComponent,
) -> u32 {
    component.into_internal().index_in_local_key_space().into()
}

#[uniffi::export]
pub fn hd_path_component_get_key_space(component: HDPathComponent) -> KeySpace {
    component.into_internal().key_space().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(
            new_hd_path_component_sample(),
            new_hd_path_component_sample()
        );
        assert_eq!(
            new_hd_path_component_sample_other(),
            new_hd_path_component_sample_other()
        );
        assert_ne!(
            new_hd_path_component_sample(),
            new_hd_path_component_sample_other()
        );
    }

    #[test]
    fn test_hd_path_component_get_key_space() {
        let ks = KeySpace::Unsecurified { is_hardened: false };
        assert_eq!(
            hd_path_component_get_key_space(
                new_hd_path_component_from_local_key_space(1, ks,).unwrap(),
            ),
            ks
        );
    }

    #[test]
    fn test_hd_path_component_index_in_local_key_space() {
        assert_eq!(
            hd_path_component_index_in_local_key_space(
                new_hd_path_component_from_local_key_space(
                    99,
                    KeySpace::Securified
                )
                .unwrap(),
            ),
            99
        );
    }

    #[test]
    fn test_hd_path_component_to_hardened() {
        assert_eq!(
            hd_path_component_to_hardened(
                new_hd_path_component_from_local_key_space(
                    99,
                    KeySpace::Securified
                )
                .unwrap(),
            )
            .unwrap(),
            Hardened::Securified(new_securified(new_u30(99).unwrap()))
        );
    }

    #[test]
    fn test_hd_path_component_to_bip32_string_debug() {
        assert_eq!(
            hd_path_component_to_bip32_string_debug(
                new_hd_path_component_from_local_key_space(
                    99,
                    KeySpace::Securified
                )
                .unwrap(),
            ),
            "99^"
        );

        assert_eq!(
            hd_path_component_to_bip32_string_debug(
                new_hd_path_component_from_global_key_space(3,)
            ),
            "3"
        );

        assert_eq!(
            hd_path_component_to_bip32_string_debug(
                new_hd_path_component_from_global_key_space(4 + 0x8000_0000,)
            ),
            "4'"
        );
    }

    #[test]
    fn test_hd_path_component_to_bip32_string() {
        assert_eq!(
            hd_path_component_to_bip32_string(
                new_hd_path_component_from_local_key_space(
                    99,
                    KeySpace::Securified
                )
                .unwrap(),
            ),
            "99S"
        );

        assert_eq!(
            hd_path_component_to_bip32_string(
                new_hd_path_component_from_global_key_space(3,)
            ),
            "3"
        );

        assert_eq!(
            hd_path_component_to_bip32_string(
                new_hd_path_component_from_global_key_space(4 + 0x8000_0000,)
            ),
            "4H"
        );
    }

    #[test]
    fn test_hd_path_component_index_in_global_key_space() {
        assert_eq!(
            hd_path_component_index_in_global_key_space(
                new_hd_path_component_from_local_key_space(
                    1,
                    KeySpace::Unsecurified { is_hardened: false },
                )
                .unwrap(),
            ),
            1
        );

        assert_eq!(
            hd_path_component_index_in_global_key_space(
                new_hd_path_component_from_local_key_space(
                    1,
                    KeySpace::Securified
                )
                .unwrap(),
            ),
            1 + 0x8000_0000 + 0x4000_0000
        );
    }

    #[test]
    fn test_new_hd_path_component_from_global_key_space() {
        assert_eq!(
            new_hd_path_component_from_global_key_space(1),
            HDPathComponent::UnsecurifiedComponent(
                Unsecurified::UnhardenedComponent(new_unhardened(
                    new_u31(1).unwrap()
                ))
            )
        );

        assert_eq!(
            new_hd_path_component_from_global_key_space(2 + 0x8000_0000),
            HDPathComponent::UnsecurifiedComponent(
                Unsecurified::HardenedComponent(new_unsecurified_hardened(
                    new_u30(2).unwrap()
                ))
            )
        );

        assert_eq!(
            new_hd_path_component_from_global_key_space(
                3 + 0x8000_0000 + 0x4000_0000
            ),
            HDPathComponent::SecurifiedComponent(new_securified(
                new_u30(3).unwrap()
            ))
        );
    }

    #[test]
    fn test_new_hd_path_component_from_local_key_space() {
        assert_eq!(
            new_hd_path_component_from_local_key_space(
                1,
                KeySpace::Unsecurified { is_hardened: false }
            )
            .unwrap(),
            HDPathComponent::UnsecurifiedComponent(
                Unsecurified::UnhardenedComponent(new_unhardened(
                    new_u31(1).unwrap()
                ))
            )
        );

        assert_eq!(
            new_hd_path_component_from_local_key_space(
                2,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap(),
            HDPathComponent::UnsecurifiedComponent(
                Unsecurified::HardenedComponent(new_unsecurified_hardened(
                    new_u30(2).unwrap()
                ))
            )
        );

        assert_eq!(
            new_hd_path_component_from_local_key_space(3, KeySpace::Securified)
                .unwrap(),
            HDPathComponent::SecurifiedComponent(new_securified(
                new_u30(3).unwrap()
            ))
        );
    }
}
