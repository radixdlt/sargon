use crate::prelude::*;

/// Sargon representation for [`RetEitherGuaranteedOrPredicted`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GuaranteedOrPredicted<T> {
    Guaranteed(T),
    Predicted(Predicted<T>),
}

impl<T> GuaranteedOrPredicted<T> {
    pub fn new_guaranteed(value: impl Into<T>) -> Self {
        Self::Guaranteed(value.into())
    }

    pub fn new_predicted(value: impl Into<T>, instruction_index: u64) -> Self {
        Self::Predicted(Predicted::new(value, instruction_index))
    }
}

impl<T> GuaranteedOrPredicted<T>
where
    T: Clone,
{
    pub fn get_value(&self) -> T {
        match self {
            Self::Guaranteed(value) => value.clone(),
            Self::Predicted(predicted) => predicted.value.clone(),
        }
    }
}

/// Sargon representation for [`RetTracked`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Predicted<T> {
    pub value: T,
    pub instruction_index: u64,
}

impl<T> Predicted<T> {
    pub fn new(value: impl Into<T>, instruction_index: u64) -> Self {
        Self {
            value: value.into(),
            instruction_index,
        }
    }
}
