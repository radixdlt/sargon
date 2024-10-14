use crate::prelude::*;
use sargon::IdentifiedVecOf;
use sargon::Result as InternalResult;

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

pub trait IntoInternalResult<Type, InternalType> {
    fn into_internal_result(self) -> InternalResult<InternalType>;
}

impl<Type, InternalType> IntoInternalResult<Type, InternalType>
    for Result<Type>
where
    Type: Into<InternalType>,
{
    fn into_internal_result(self) -> InternalResult<InternalType>{
        self.map(Type::into) 
            .map_err(Into::into)
    }
}

impl<Type, InternalType> IntoInternalResult<Type, Option<InternalType>>
    for Result<Option<Type>>
where
    Type: Into<InternalType>,
{
    fn into_internal_result(self) -> InternalResult<Option<InternalType>>{
        self.map(|opt| opt.map(Type::into))
            .map_err(Into::into)
    }
}


/// Utility trait to map `InternalResult` to `Result`
pub trait FromInternalResult<InternalType, Type> {
    fn into_result(self) -> Result<Type>;
}

/// Implementation for InternalResult<InternalType>
impl<InternalType, Type> FromInternalResult<InternalType, Type>
    for InternalResult<InternalType>
where
    Type: From<InternalType>
{
    fn into_result(self) -> Result<Type> {
        self.map(Type::from) // Converts Ok variant using From trait
            .map_err(Into::into) // Converts Err variant using Into
    }
}

/// Implementation for InternalResult<Vec<InternalType>>
impl<InternalType, Type> FromInternalResult<InternalType, Vec<Type>>
    for InternalResult<Vec<InternalType>>
where
    Type: From<InternalType>, // Ensures `Type` can be constructed from `InternalType`
{
    fn into_result(self) -> Result<Vec<Type>> {
        self.map(|vec| vec.into_vec()) // Converts Ok variant using From trait
            .map_err(Into::into) // Converts Err variant using Into
    }
}

/// Implementation for InternalResult<IdentifiedVecOf<InternalType>>
impl<
        InternalType: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
        Type,
    > FromInternalResult<InternalType, Vec<Type>>
    for InternalResult<IdentifiedVecOf<InternalType>>
where
    Type: From<InternalType>, // Ensures `Type` can be constructed from `InternalType`
{
    fn into_result(self) -> Result<Vec<Type>> {
        self.map(|vec| vec.into_vec()) // Converts Ok variant using From trait
            .map_err(Into::into) // Converts Err variant using Into
    }
}