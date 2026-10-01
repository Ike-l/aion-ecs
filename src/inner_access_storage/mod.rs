use aion_state::prelude::AccessStorage;

use crate::prelude::Access;

pub struct InnerAccessStorage<ValueId> {
    _p: ValueId
}

impl<ValueId> AccessStorage for InnerAccessStorage<ValueId> {
    type ValueId = ValueId;

    type Access = Access;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Access> {
        todo!()
    }

    fn get(
        &self, 
        value_id: &Self::ValueId
    ) -> Option<&Self::Access> {
        todo!()
    }

    fn insert(
        &mut self,
        value_id: Self::ValueId,
        access: Self::Access
    ) -> Option<Self::Access> {
        todo!()
    }

    fn drain(&mut self) -> impl Iterator<Item = (
        Self::ValueId, 
        Self::Access
    )> {
        todo!()
    }
}