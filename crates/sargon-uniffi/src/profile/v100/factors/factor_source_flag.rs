use crate::prelude::*;
use sargon::FactorSourceFlag as InternalFactorSourceFlag;

/// Flags which describe a certain state a FactorSource might be in, primarily used
/// by DeviceFactorSource's to mark which "Babylon" FactorSource is the **main** one.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum FactorSourceFlag {
    /// Used to mark a "babylon" `.device` FactorSource as "main". All new accounts
    /// and Personas are created using the `main` `DeviceFactorSource`.
    ///
    /// We can only ever have one.
    /// We might have zero `main` flags across all  `DeviceFactorSource`s if and only if we have only one  `DeviceFactorSource`s. If we have two or more  `DeviceFactorSource`s one of them MUST
    /// be marked with `main`.
    Main,

    /// Until we have implemented "proper" deletion, we will "flag" a
    /// FactorSource as deleted by the user and hide it, meaning e.g.
    /// that in Multi-Factor Setup flows it will not show up.
    DeletedByUser,
}
