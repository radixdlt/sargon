use std::any::TypeId;

use crate::prelude::*;

pub fn import_identified_vec_of_from<V>(
    iter: impl IntoIterator<Item = V>,
) -> Result<IdentifiedVecOf<V>>
where
    V: Debug + Eq + Clone + Identifiable + 'static,
{
    let mut id_vec = IdentifiedVecOf::new();

    for item in iter {
        id_vec.try_insert_unique(item)?;
    }

    export_identified_vec_of(&id_vec)?;
    Ok(id_vec)
}

pub fn export_identified_vec_of<V>(
    id_vec: &IdentifiedVecOf<V>,
) -> Result<&IdentifiedVecOf<V>>
where
    V: Debug + Eq + Clone + Identifiable + 'static,
{
    if id_vec.is_empty() {
        if TypeId::of::<V>() == TypeId::of::<FactorSource>() {
            return Err(CommonError::FactorSourcesMustNotBeEmpty);
        }
        if TypeId::of::<V>() == TypeId::of::<SLIP10Curve>() {
            return Err(CommonError::SupportedCurvesMustNotBeEmpty);
        }
    }

    Ok(id_vec)
}
