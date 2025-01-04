use sargon_core_metadata::prelude::MetadataKey;

use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct EntityMetadataCollection {
    pub items: Vec<EntityMetadataItem>,
}

impl EntityMetadataCollection {
    pub fn new(items: Vec<EntityMetadataItem>) -> EntityMetadataCollection {
        EntityMetadataCollection { items }
    }

    pub fn empty() -> EntityMetadataCollection {
        EntityMetadataCollection::new(vec![])
    }
}

impl EntityMetadataCollection {
    fn get_value(&self, key: MetadataKey) -> Option<MetadataTypedValue> {
        let item = self
            .items
            .clone()
            .into_iter()
            .find(|x| x.key == key.to_string())?;

        Some(item.value.typed)
    }

    pub fn get_icon_url(&self) -> Option<Url> {
        let typed = self.get_value(MetadataKey::IconUrl)?;

        match typed {
            MetadataTypedValue::MetadataUrlValue { value } => Some(value),
            _ => None,
        }
    }
}
