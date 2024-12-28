use crate::prelude::*;
use sargon::IdentifiedVecOf;
use sargon::Result as InternalResult;

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

/// Utility trait to map `Result` to `InternalResult`
pub trait IntoInternalResult<Type, InternalType> {
    fn into_internal_result(self) -> InternalResult<InternalType>;
}

impl<Type, InternalType> IntoInternalResult<Type, InternalType> for Result<Type>
where
    Type: Into<InternalType>,
{
    fn into_internal_result(self) -> InternalResult<InternalType> {
        self.map(Type::into).map_err(Into::into)
    }
}

impl<Type, InternalType> IntoInternalResult<Type, Option<InternalType>>
    for Result<Option<Type>>
where
    Type: Into<InternalType>,
{
    fn into_internal_result(self) -> InternalResult<Option<InternalType>> {
        self.map(|opt| opt.map(Type::into)).map_err(Into::into)
    }
}

/// Utility trait to map `InternalResult` to `Result`
pub trait FromInternalResult<InternalType, Type> {
    fn into_result(self) -> Result<Type>;
}

/// Utility trait to map `InternalResult` to `Result`
pub trait FromInternalIterResult<InternalType, Type> {
    fn into_iter_result(self) -> Result<Type>;
}

/// Implementation for InternalResult<InternalType>
impl<InternalType, Type> FromInternalResult<InternalType, Type>
    for InternalResult<InternalType>
where
    Type: From<InternalType>,
{
    fn into_result(self) -> Result<Type> {
        self.map(Type::from) // Converts Ok variant using From trait
            .map_err(Into::into) // Converts Err variant using Into
    }
}

/// Implementation for InternalResult<Vec<InternalType>>
impl<T, InternalElement, Element> FromInternalIterResult<T, Vec<Element>>
    for InternalResult<T>
where
    T: IntoIterator<Item = InternalElement>,
    Element: From<InternalElement>, // Ensures `Type` can be constructed from `InternalType`
{
    fn into_iter_result(self) -> Result<Vec<Element>> {
        // self.map(|vec| vec.into_type()) // Converts Ok variant using From trait
        //     .map_err(Into::into) // Converts Err variant using Into
        todo!()
    }
}
// `core::result::result::FromInternalResult<P2PLinks, Vec<p2p_link::P2PLink>>`
// impl<T> From<>
