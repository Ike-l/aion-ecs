use std::{any::TypeId, collections::HashMap};

use crate::prelude::{TypeAccess, TypeTracker};

pub mod type_tracker;


type ArchetypeId = Vec<TypeId>;

#[derive(Default)]
pub struct ArchetypeTracker {
    type_trackers: HashMap<ArchetypeId, TypeTracker>
}

impl ArchetypeTracker {
    pub fn try_access(&mut self, archetype_id: ArchetypeId, type_id: TypeId, access: TypeAccess) -> bool {
        self.type_trackers.entry(archetype_id).or_default().try_access(type_id, access)
    }

    fn split_archetype_tracker(&mut self, other: impl Iterator<Item = (ArchetypeId, TypeTracker)>) {
        for (archetype_id, other_type_tracker) in other {
            if let Some(type_tracker) = self.type_trackers.get_mut(&archetype_id) {
                type_tracker.split(other_type_tracker);
            }
        }
    }

    pub fn split(&mut self, other: &mut Self) { 
        self.split_archetype_tracker(other.type_trackers.drain());
    }
}