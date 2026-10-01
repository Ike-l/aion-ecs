use std::{collections::HashMap, hash::Hash};

use aion_state::prelude::AccessStorage;

use crate::prelude::Access;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "ValueId: serde::Serialize + std::cmp::Eq + std::hash::Hash",
    deserialize = "ValueId: serde::Deserialize<'de> + std::cmp::Eq + std::hash::Hash"
))]
pub struct InnerAccessStorage<ValueId> {
    inner: HashMap<ValueId, Access>
}

impl<ValueId> Default for InnerAccessStorage<ValueId> {
    fn default() -> Self {
        Self {
            inner: HashMap::new()
        }
    }
}

impl<ValueId: Eq + Hash> AccessStorage for InnerAccessStorage<ValueId> {
    type ValueId = ValueId;
    type Access = Access;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Access> {
        self.inner.get_mut(value_id)
    }

    fn get(
        &self, 
        value_id: &Self::ValueId
    ) -> Option<&Self::Access> {
        self.inner.get(value_id)
    }

    fn insert(
        &mut self,
        value_id: Self::ValueId,
        access: Self::Access
    ) -> Option<Self::Access> {
        self.inner.insert(value_id, access)
    }

    fn drain(&mut self) -> impl Iterator<Item = (
        Self::ValueId, 
        Self::Access
    )> {
        self.inner.drain()
    }
}