use crate::{
    bip32::{
        hd_path::HDPath,
        hd_path_component::{HDPathComponent, HDPathValue},
    },
    derivation::derivation::Derivation,
};

use super::cap26_error::CAP26Error;

pub(crate) enum CAP26 {}
impl CAP26 {
    pub(crate) fn parse_try_map<T, F>(
        path: &Vec<HDPathComponent>,
        index: usize,
        try_map: F,
    ) -> Result<T, CAP26Error>
    where
        F: Fn(HDPathValue) -> Result<T, CAP26Error>,
    {
        let got = &path[index];
        try_map(got.value())
    }

    pub(crate) fn parse<F>(
        path: &Vec<HDPathComponent>,
        index: usize,
        expected: HDPathComponent,
        err: F,
    ) -> Result<&HDPathComponent, CAP26Error>
    where
        F: Fn(HDPathValue) -> CAP26Error,
    {
        let got = &path[index];
        if got != &expected {
            return Err(err(got.value()));
        }
        Ok(got)
    }

    pub(crate) fn try_parse_base(s: &str) -> Result<(HDPath, Vec<HDPathComponent>), CAP26Error> {
        use CAP26Error::*;
        let path = HDPath::from_str(s).map_err(|_| CAP26Error::InvalidBIP32Path(s.to_string()))?;
        if path.depth() < 2 {
            return Err(InvalidDepthOfCAP26Path);
        }
        let components = path.components();

        if !components.clone().iter().all(|c| c.is_hardened()) {
            return Err(NotAllComponentsAreHardened);
        }
        _ = Self::parse(
            components,
            0,
            HDPathComponent::bip44_purpose(),
            Box::new(|v| BIP44PurposeNotFound(v)),
        )?;

        _ = Self::parse(
            components,
            1,
            HDPathComponent::bip44_cointype(),
            Box::new(|v| CoinTypeNotFound(v)),
        )?;
        return Ok((path.clone(), components.clone()));
    }
}
