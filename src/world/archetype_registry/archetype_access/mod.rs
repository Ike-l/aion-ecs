use aion_state::prelude::Accessor;

use crate::prelude::{Archetype, StoredArchetype};

#[derive(Debug, PartialEq, Clone)]
pub enum ArchetypeAccess {
    Shared(usize)
}

pub enum ArchetypeAccessResult {}

impl Accessor for ArchetypeAccess {
    type StoredValue = StoredArchetype;

    type Value = Archetype;

    type AccessResult<'a> = ArchetypeAccessResult;

    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        todo!()
    }

    fn can_insert_resource(&self) -> bool {
        todo!()
    }

    fn can_remove_resource(&self) -> bool {
        todo!()
    }

    fn acquire<'a>(
        &self, 
        stored_value: &'a mut Self::StoredValue
    ) -> Self::AccessResult<'a> {
        todo!()
    }

    fn merge(
        &mut self,
        incoming_access: Self
    ) {
        todo!()
    }

    fn release(
        &mut self,
        other: &Self
    ) {
        todo!()
    }

    fn insert(
        &self,
        value: Self::Value
    ) -> Self::StoredValue {
        todo!()
    }

    fn remove(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::StoredValue {
        todo!()
    }
}