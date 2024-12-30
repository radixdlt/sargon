use crate::prelude::*;

pub trait FromLocalKeySpace: Sized + From<Self::Magnitude> {
    type Magnitude: TryFrom<U31>;

    fn from_local_key_space(value: impl TryInto<U31>) -> Result<Self> {
        let value = value.try_into().map_err(|_| CommonError::IndexOverflow)?;
        let magnitude = Self::Magnitude::try_from(value)
            .map_err(|_| CommonError::IndexOverflow)?;
        Ok(Self::from(magnitude))
    }
}
