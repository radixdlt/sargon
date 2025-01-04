use crate::prelude::*;

pub trait AddViaDeref:
    std::ops::Deref<Target = u32> + TryFrom<u32, Error = CommonError>
{
    fn checked_add_one(&self) -> Result<Self> {
        self.checked_add_n(1)
    }
    fn checked_add_n(&self, n: u32) -> Result<Self> {
        Self::try_from(**self + n)
    }
}

pub trait AddSelfViaDeref: AddViaDeref {
    fn checked_add(&self, rhs: &Self) -> Result<Self> {
        Self::try_from(*(*self) + *(*rhs))
    }
}
