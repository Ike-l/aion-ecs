use aion_program::prelude::StoredResourceTrait;
use aion_state::prelude::Accessor;

use crate::prelude::{Archetype, StoredArchetype};

#[derive(Debug, PartialEq, Clone)]
pub enum ArchetypeAccess {
    Shared(usize),
    Replace,
}

pub enum ArchetypeAccessResult<'a> {
    Shared(&'a Archetype)
}

impl Accessor for ArchetypeAccess {
    type StoredValue = StoredArchetype;

    type Value = Archetype;

    type AccessResult<'a> = ArchetypeAccessResult<'a>;

    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        match (self, incoming_access) {
            (ArchetypeAccess::Shared(_), ArchetypeAccess::Shared(_)) => true,
            (ArchetypeAccess::Replace, ArchetypeAccess::Replace) => true,
            (ArchetypeAccess::Shared(0), ArchetypeAccess::Replace) => true,
            (ArchetypeAccess::Shared(_), ArchetypeAccess::Replace) => false,
            (ArchetypeAccess::Replace, ArchetypeAccess::Shared(_)) => true,
        }
    }

    fn can_insert_resource(&self) -> bool {
        *self == Self::Replace
    }

    fn can_remove_resource(&self) -> bool {
        *self == Self::Replace
    }

    fn acquire<'a>(
        &self, 
        stored_value: &'a mut Self::StoredValue
    ) -> Self::AccessResult<'a> {
        match self {
            ArchetypeAccess::Shared(0) => unreachable!(),
            ArchetypeAccess::Shared(_) => ArchetypeAccessResult::Shared(stored_value.get()),
            ArchetypeAccess::Replace => unreachable!(),
        }
    }

    fn merge(
        &mut self,
        incoming_access: Self
    ) {
        match (self, incoming_access) {
            (ArchetypeAccess::Shared(n), ArchetypeAccess::Shared(m)) => *n += m,
            (m @ ArchetypeAccess::Shared(0), ArchetypeAccess::Replace) => *m = ArchetypeAccess::Replace,
            (ArchetypeAccess::Shared(_), ArchetypeAccess::Replace) => unreachable!(),
            (m @ ArchetypeAccess::Replace, n @ ArchetypeAccess::Shared(_)) => *m = n,
            (ArchetypeAccess::Replace, ArchetypeAccess::Replace) => (),
        }
    }

    fn release(
        &mut self,
        other: &Self
    ) {
        // probably could panic in other scenarios cause something probably went wrong?
        match (self, other) {
            (ArchetypeAccess::Shared(n), ArchetypeAccess::Shared(m)) => *n = n.saturating_sub(*m),
            _ => ()
        }
    }

    fn insert(
        &self,
        value: Self::Value
    ) -> Self::StoredValue {
        Self::StoredValue::new(value)
    }

    fn remove(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::StoredValue {
        stored_value
    }
}